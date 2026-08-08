//! Authentication management for ekoDB client

use crate::error::{Error, Result};
use reqwest::Client;
use serde_json::{Value, json};
use tokio::sync::RwLock;
use url::Url;

/// Manages authentication tokens with automatic expiry detection.
pub struct AuthManager {
    api_key: String,
    base_url: Url,
    client: Client,
    /// Cached token + Unix timestamp when it expires (seconds since epoch).
    /// The token is proactively refreshed 60s before expiry.
    cached_token: RwLock<Option<(String, u64)>>,
}

impl AuthManager {
    pub fn new(api_key: String, base_url: Url, client: Client) -> Self {
        Self {
            api_key,
            base_url,
            client,
            cached_token: RwLock::new(None),
        }
    }

    /// Get a valid authentication token.
    ///
    /// Returns a cached token if it has more than 60s of validity remaining.
    /// Otherwise fetches a new one via `/api/auth/token`. This means callers
    /// never need to handle token refresh themselves — every `get_token()` call
    /// returns a token that's valid for at least 60 more seconds.
    pub async fn get_token(&self) -> Result<String> {
        // Check if we have a cached token that's not about to expire
        let cached = self.cached_token.read().await;
        if let Some((token, expires_at)) = cached.as_ref() {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            // Refresh 60s before expiry to avoid edge-case races
            if now + 60 < *expires_at {
                return Ok(token.clone());
            }
            log::debug!(
                "Token expiring soon ({}s left), refreshing",
                expires_at.saturating_sub(now)
            );
        }
        drop(cached);

        self.fetch_new_token().await
    }

    /// Fetch a fresh token from the server and cache it.
    async fn fetch_new_token(&self) -> Result<String> {
        let token_url = self.base_url.join("/api/auth/token")?;
        let response = self
            .client
            .post(token_url)
            .json(&json!({ "api_key": self.api_key }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::Authentication(format!(
                "Failed to get token ({}): {}",
                status, text
            )));
        }

        let token_data: Value = response.json().await?;
        let token = token_data["token"]
            .as_str()
            .ok_or_else(|| Error::Authentication("No token in response".to_string()))?
            .to_string();

        // Extract expiry from JWT payload (base64-decode the middle segment)
        let expires_at = Self::extract_jwt_expiry(&token).unwrap_or_else(|| {
            // Fallback: assume 1 hour from now if we can't decode
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                + 3600
        });

        // Cache the token with expiry
        let mut cached = self.cached_token.write().await;
        *cached = Some((token.clone(), expires_at));

        Ok(token)
    }

    /// Extract the `exp` claim from a JWT without verifying the signature.
    /// Returns the Unix timestamp of expiry, or None if parsing fails.
    fn extract_jwt_expiry(token: &str) -> Option<u64> {
        use base64::Engine;
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(parts[1])
            .ok()?;
        let claims: Value = serde_json::from_slice(&payload).ok()?;
        claims.get("exp").and_then(|v| v.as_u64())
    }

    /// Refresh the authentication token.
    ///
    /// Clears the cached token and fetches a new one from the server.
    /// Normally not needed — `get_token()` auto-refreshes before expiry.
    /// Use this when you receive a 401 and want to force immediate refresh.
    pub async fn refresh_token(&self) -> Result<String> {
        self.clear_cache().await;
        self.fetch_new_token().await
    }

    /// Clear the cached token.
    ///
    /// Forces a new token to be fetched on the next `get_token()` call.
    pub async fn clear_cache(&self) {
        let mut cached = self.cached_token.write().await;
        *cached = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_token() {
        // Note: This test requires a mock server to properly test token exchange
        // For now, we just test that the auth manager can be created
        let base_url = Url::parse("http://localhost:8080").unwrap();
        let client = Client::new();
        let auth = AuthManager::new("test-key".to_string(), base_url, client);

        // Token exchange would happen here, but requires a running server
        // In real usage, this would call /api/auth/token
        assert!(auth.api_key == "test-key");
    }

    #[tokio::test]
    async fn test_refresh_token() {
        let base_url = Url::parse("http://localhost:8080").unwrap();
        let client = Client::new();
        let auth = AuthManager::new("test-key".to_string(), base_url, client);

        // Manually set a cached token with future expiry
        {
            let mut cached = auth.cached_token.write().await;
            *cached = Some(("old-token".to_string(), u64::MAX));
        }

        auth.clear_cache().await;

        let cached = auth.cached_token.read().await;
        assert!(cached.is_none());
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let base_url = Url::parse("http://localhost:8080").unwrap();
        let client = Client::new();
        let auth = AuthManager::new("test-key".to_string(), base_url, client);

        {
            let mut cached = auth.cached_token.write().await;
            *cached = Some(("old-token".to_string(), u64::MAX));
        }

        auth.clear_cache().await;

        let cached = auth.cached_token.read().await;
        assert!(cached.is_none());
    }

    #[tokio::test]
    async fn test_expired_token_not_returned() {
        let base_url = Url::parse("http://localhost:8080").unwrap();
        let client = Client::new();
        let auth = AuthManager::new("test-key".to_string(), base_url, client);

        // Cache a token that expired 10 minutes ago
        {
            let mut cached = auth.cached_token.write().await;
            let past = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - 600;
            *cached = Some(("expired-token".to_string(), past));
        }

        // get_token should NOT return the expired token — it will try to
        // fetch a new one (and fail since no server is running, which is fine)
        let result = auth.get_token().await;
        assert!(
            result.is_err(),
            "should fail to fetch new token (no server)"
        );
        // Crucially, it should NOT return "expired-token"
    }

    #[test]
    fn test_extract_jwt_expiry() {
        // Construct a minimal JWT: header.payload.signature
        // payload: {"exp": 1700000000}
        use base64::Engine;
        let payload = serde_json::json!({"exp": 1700000000, "sub": "test"});
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(serde_json::to_vec(&payload).unwrap());
        let fake_jwt = format!("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.{encoded}.fakesig");
        let exp = AuthManager::extract_jwt_expiry(&fake_jwt);
        assert_eq!(exp, Some(1700000000));
    }

    #[test]
    fn test_extract_jwt_expiry_invalid() {
        assert_eq!(AuthManager::extract_jwt_expiry("not-a-jwt"), None);
        assert_eq!(AuthManager::extract_jwt_expiry("a.b"), None);
        assert_eq!(AuthManager::extract_jwt_expiry("a.!!!.c"), None);
    }
}
