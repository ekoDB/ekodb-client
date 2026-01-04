//! WebSocket client for real-time subscriptions
//!
//! This module provides WebSocket support for real-time updates and queries.

use crate::error::{Error, Result};
use crate::Record;
use futures_util::{stream::SplitSink, stream::SplitStream, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use url::Url;

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketRequest {
    /// Find all records in a collection
    FindAll {
        #[serde(rename = "messageId")]
        message_id: String,
        payload: FindAllPayload,
    },
    /// Find a record by ID
    FindById {
        #[serde(rename = "messageId")]
        message_id: String,
        payload: FindByIdPayload,
    },
    /// Subscribe to collection changes
    Subscribe {
        #[serde(rename = "messageId")]
        message_id: String,
        payload: SubscribePayload,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindAllPayload {
    pub collection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindByIdPayload {
    pub collection: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribePayload {
    pub collection: String,
}

/// WebSocket response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketResponse {
    Success { payload: ResponsePayload },
    Error { code: u16, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsePayload {
    pub data: Value,
}

type WsWrite = SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, Message>;
type WsRead = SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>;

/// WebSocket client for real-time communication with persistent connection
#[derive(Clone)]
pub struct WebSocketClient {
    ws_url: Url,
    token: String,
    connection: Arc<Mutex<Option<(WsWrite, WsRead)>>>,
}

impl WebSocketClient {
    /// Create a new WebSocket client
    pub fn new(ws_url: impl AsRef<str>, token: impl Into<String>) -> Result<Self> {
        let ws_url = Url::parse(ws_url.as_ref())?;
        Ok(Self {
            ws_url,
            token: token.into(),
            connection: Arc::new(Mutex::new(None)),
        })
    }

    /// Ensure we have an active connection, reconnecting if needed
    async fn ensure_connected(&self) -> Result<()> {
        let mut conn = self.connection.lock().await;

        // If already connected, return
        if conn.is_some() {
            return Ok(());
        }

        // Create new connection - append /api/ws path if not present
        let mut url = self.ws_url.clone();
        if !url.path().contains("/api/ws") {
            url.set_path("/api/ws");
        }

        // Create request with Authorization header
        let request = tokio_tungstenite::tungstenite::http::Request::builder()
            .uri(url.as_str())
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Host", url.host_str().unwrap_or("localhost"))
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header(
                "Sec-WebSocket-Key",
                tokio_tungstenite::tungstenite::handshake::client::generate_key(),
            )
            .body(())
            .map_err(|e| Error::WebSocket(e.to_string()))?;

        let (ws_stream, _) = connect_async(request)
            .await
            .map_err(|e| Error::WebSocket(e.to_string()))?;

        *conn = Some(ws_stream.split());
        Ok(())
    }

    /// Find all records in a collection via WebSocket
    pub async fn find_all(&self, collection: &str) -> Result<Vec<Record>> {
        // Ensure we're connected
        self.ensure_connected().await?;

        let mut conn = self.connection.lock().await;
        let (write, read) = conn
            .as_mut()
            .ok_or_else(|| Error::WebSocket("Not connected".to_string()))?;

        let message_id = format!(
            "{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| Error::WebSocket(e.to_string()))?
                .as_nanos()
        );

        let request = WebSocketRequest::FindAll {
            message_id,
            payload: FindAllPayload {
                collection: collection.to_string(),
            },
        };

        // Send request
        if let Err(e) = write
            .send(Message::Text(serde_json::to_string(&request)?.into()))
            .await
        {
            // Connection failed, clear it for reconnection
            *conn = None;
            return Err(Error::WebSocket(format!("Send failed: {}", e)));
        }

        // Wait for response - may need to skip ping/pong messages
        while let Some(msg) = read.next().await {
            let msg = match msg {
                Ok(m) => m,
                Err(e) => {
                    // Connection error, clear for reconnection
                    *conn = None;
                    return Err(Error::WebSocket(format!("Receive failed: {}", e)));
                }
            };

            // Only process text messages
            if let Ok(text) = msg.into_text() {
                // Skip empty messages
                if text.is_empty() {
                    continue;
                }

                let response: WebSocketResponse = serde_json::from_str(&text)?;

                return match response {
                    WebSocketResponse::Success { payload } => {
                        if let Some(arr) = payload.data.as_array() {
                            let records: Vec<Record> = arr
                                .iter()
                                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                                .collect();
                            Ok(records)
                        } else {
                            Ok(vec![])
                        }
                    }
                    WebSocketResponse::Error { code, message } => Err(Error::api(code, message)),
                };
            }
        }

        // Connection closed unexpectedly
        *conn = None;
        Err(Error::WebSocket(
            "Connection closed unexpectedly".to_string(),
        ))
    }

    /// Close the WebSocket connection
    pub async fn close(&mut self) -> Result<()> {
        let mut conn = self.connection.lock().await;
        *conn = None;
        Ok(())
    }
}
