//! Retry logic with exponential backoff

use crate::error::Result;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

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

                    // Calculate delay with exponential backoff
                    let delay = if let Some(retry_delay) = e.retry_delay_secs() {
                        Duration::from_secs(retry_delay)
                    } else {
                        self.base_delay * 2_u32.pow(attempts - 1)
                    };

                    log::debug!(
                        "Retrying request (attempt {}/{}) after {:?}",
                        attempts,
                        self.max_retries,
                        delay
                    );

                    sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

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
