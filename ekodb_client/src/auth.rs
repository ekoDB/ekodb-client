//! Authentication management for ekoDB client

use crate::error::{Error, Result};
use reqwest::Client;
use serde_json::{json, Value};
use tokio::sync::RwLock;
use url::Url;

/// Manages authentication tokens
pub struct AuthManager {
    api_key: String,
    base_url: Url,
    client: Client,
    cached_token: RwLock<Option<String>>,
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

    /// Get a valid authentication token
    ///
    /// Exchanges the API key for a JWT token via /api/auth/token
    pub async fn get_token(&self) -> Result<String> {
        // Check if we have a cached token
        let cached = self.cached_token.read().await;
        if let Some(token) = cached.as_ref() {
            return Ok(token.clone());
        }
        drop(cached);

        // Exchange API key for JWT token
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

        // Cache the token
        let mut cached = self.cached_token.write().await;
        *cached = Some(token.clone());

        Ok(token)
    }

    /// Refresh the authentication token
    ///
    /// Clears the cached token and fetches a new one from the server.
    /// This is useful when a 401 Unauthorized response is received.
    pub async fn refresh_token(&self) -> Result<String> {
        // Clear the cached token
        self.clear_cache().await;

        // Fetch a new token
        self.get_token().await
    }

    /// Clear the cached token
    ///
    /// This will force a new token to be fetched on the next request.
    /// Useful for testing or when you know the token has expired.
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

        // Manually set a cached token
        {
            let mut cached = auth.cached_token.write().await;
            *cached = Some("old-token".to_string());
        }

        // Note: refresh_token() would call the server to get a new token
        // For unit tests, we just verify the cache is cleared
        // Integration tests would verify the full flow
        auth.clear_cache().await;

        let cached = auth.cached_token.read().await;
        assert!(cached.is_none());
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let base_url = Url::parse("http://localhost:8080").unwrap();
        let client = Client::new();
        let auth = AuthManager::new("test-key".to_string(), base_url, client);

        // Manually set a cached token
        {
            let mut cached = auth.cached_token.write().await;
            *cached = Some("old-token".to_string());
        }

        // Clear the cache
        auth.clear_cache().await;

        // Verify cache is cleared
        let cached = auth.cached_token.read().await;
        assert!(cached.is_none());
    }
}
