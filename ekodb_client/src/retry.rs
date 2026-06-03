//! Retry logic with exponential backoff

use crate::error::Result;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

/// Hard cap on a single *exponential-backoff* sleep, so the computed schedule
/// can never produce an unbounded delay. A server-supplied `Retry-After` is
/// honored on its own, higher ceiling (`MAX_RETRY_AFTER_SECS`) rather than this
/// one, matching the other ekoDB clients.
const MAX_BACKOFF: Duration = Duration::from_secs(30);
/// Hard cap on a server-supplied `Retry-After`, bounding even a hostile value.
const MAX_RETRY_AFTER_SECS: u64 = 60;
/// Cap on the backoff exponent so `2^exp` can never overflow.
const MAX_BACKOFF_EXPONENT: u32 = 16;

/// Retry policy for handling transient failures
pub struct RetryPolicy {
    max_retries: u32,
    base_delay: Duration,
}

impl RetryPolicy {
    /// Create a new retry policy
    pub fn new(max_retries: u32) -> Self {
        Self {
            max_retries,
            base_delay: Duration::from_millis(100),
        }
    }

    /// Execute a function with retry logic
    pub async fn execute<F, Fut, T>(&self, mut f: F) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let mut attempts = 0;

        loop {
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) if e.is_retryable() && attempts < self.max_retries => {
                    attempts += 1;
                    sleep(self.backoff_delay(attempts, e.retry_delay_secs())).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Compute the backoff delay for a given attempt. A server-supplied
    /// `Retry-After` is honored but capped; otherwise an exponential schedule is
    /// used with a bounded exponent and saturating math, then clamped to
    /// `MAX_BACKOFF`. Never panics or overflows regardless of attempt count.
    fn backoff_delay(&self, attempt: u32, retry_after_secs: Option<u64>) -> Duration {
        if let Some(secs) = retry_after_secs {
            return Duration::from_secs(secs.min(MAX_RETRY_AFTER_SECS));
        }
        let exp = attempt.saturating_sub(1).min(MAX_BACKOFF_EXPONENT);
        let factor = 1u32 << exp; // exp <= 16, so this never overflows
        self.base_delay
            .checked_mul(factor)
            .unwrap_or(MAX_BACKOFF)
            .min(MAX_BACKOFF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn test_backoff_delay_capped_and_never_overflows() {
        let policy = RetryPolicy::new(100);
        // Normal exponential growth for small attempts.
        assert_eq!(policy.backoff_delay(1, None), Duration::from_millis(100));
        assert_eq!(policy.backoff_delay(2, None), Duration::from_millis(200));
        assert_eq!(policy.backoff_delay(3, None), Duration::from_millis(400));
        // A huge attempt count must not panic/overflow and must be capped.
        assert_eq!(policy.backoff_delay(1000, None), MAX_BACKOFF);
        assert!(policy.backoff_delay(u32::MAX, None) <= MAX_BACKOFF);
        // Server Retry-After is honored but capped.
        assert_eq!(policy.backoff_delay(1, Some(5)), Duration::from_secs(5));
        assert_eq!(
            policy.backoff_delay(1, Some(99_999)),
            Duration::from_secs(MAX_RETRY_AFTER_SECS)
        );
    }

    #[tokio::test]
    async fn test_retry_success_on_first_attempt() {
        let policy = RetryPolicy::new(3);
        let call_count = Arc::new(AtomicU32::new(0));
        let call_count_clone = call_count.clone();

        let result = policy
            .execute(|| {
                let count = call_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Ok::<_, Error>(42)
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retry_success_after_failures() {
        let policy = RetryPolicy::new(3);
        let call_count = Arc::new(AtomicU32::new(0));
        let call_count_clone = call_count.clone();

        let result = policy
            .execute(|| {
                let count = call_count_clone.clone();
                async move {
                    let current = count.fetch_add(1, Ordering::SeqCst) + 1;
                    if current < 3 {
                        Err(Error::Timeout)
                    } else {
                        Ok(42)
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(call_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_exhausted() {
        let policy = RetryPolicy::new(2);
        let call_count = Arc::new(AtomicU32::new(0));
        let call_count_clone = call_count.clone();

        let result = policy
            .execute(|| {
                let count = call_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, _>(Error::Timeout)
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(call_count.load(Ordering::SeqCst), 3); // Initial attempt + 2 retries
    }

    #[tokio::test]
    async fn test_no_retry_on_non_retryable_error() {
        let policy = RetryPolicy::new(3);
        let call_count = Arc::new(AtomicU32::new(0));
        let call_count_clone = call_count.clone();

        let result = policy
            .execute(|| {
                let count = call_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, _>(Error::NotFound)
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(call_count.load(Ordering::SeqCst), 1); // No retries for NotFound
    }
}
