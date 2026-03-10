//! WebSocket client for real-time subscriptions and chat streaming
//!
//! This module provides WebSocket support for real-time updates, queries,
//! and streaming chat responses with bidirectional client tool callbacks.

use crate::Record;
use crate::error::{Error, Result};
use futures_util::{SinkExt, StreamExt, stream::SplitSink, stream::SplitStream};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite::protocol::Message,
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
    /// Send a chat message for streaming response
    ChatSend { payload: ChatSendPayload },
    /// Register client-side tools for a chat session
    RegisterClientTools { payload: RegisterClientToolsPayload },
    /// Return result of a client-side tool execution
    ClientToolResult { payload: ClientToolResultPayload },
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
    /// Optional filter: only receive mutations where this field matches the value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSendPayload {
    pub chat_id: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_ripple: Option<bool>,
    /// Client-side tool definitions to merge with ekoDB's built-in tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_tools: Option<Vec<ClientToolDefinition>>,
    /// Maximum tool-calling iterations for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iterations: Option<u32>,
    /// Server-side tools that require client confirmation before execution.
    /// The server sends a ClientToolCall for these; client approves/denies via ClientToolResult.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_tools: Option<Vec<String>>,
    /// Server-side tools to exclude from the LLM's tool list.
    /// Denied tools are filtered out so the LLM never sees or attempts to call them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_tools: Option<Vec<String>>,
}

/// Definition of a client-side tool that ekoDB can route back to the client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientToolDefinition {
    /// Tool name (must not conflict with ekoDB built-in tool names)
    pub name: String,
    /// Human-readable description for the LLM
    pub description: String,
    /// JSON Schema describing the tool's parameters
    pub parameters: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterClientToolsPayload {
    pub chat_id: String,
    pub tools: Vec<ClientToolDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientToolResultPayload {
    pub chat_id: String,
    pub call_id: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// WebSocket response types from the server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketResponse {
    Success {
        payload: ResponsePayload,
    },
    Error {
        code: u16,
        message: String,
    },
    /// A streamed text chunk from the LLM
    ChatStreamChunk {
        payload: ChatStreamChunkPayload,
    },
    /// Final message after streaming completes
    ChatStreamEnd {
        payload: ChatStreamEndPayload,
    },
    /// Error during chat streaming
    ChatStreamError {
        payload: ChatStreamErrorPayload,
    },
    /// Mutation notification from a subscription
    MutationNotification {
        payload: MutationNotificationPayload,
    },
    /// Server requests the client to execute a tool
    ClientToolCall {
        payload: ClientToolCallPayload,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsePayload {
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatStreamChunkPayload {
    pub chat_id: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatStreamEndPayload {
    pub chat_id: String,
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_history: Option<Value>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatStreamErrorPayload {
    pub chat_id: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationNotificationPayload {
    pub collection: String,
    pub event: String,
    pub record_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Value>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientToolCallPayload {
    pub chat_id: String,
    pub call_id: String,
    pub tool_name: String,
    pub arguments: Value,
}

/// Event emitted by the chat stream listener
#[derive(Debug, Clone)]
pub enum ChatStreamEvent {
    /// A text chunk from the LLM
    Chunk(String),
    /// Stream completed with metadata
    End {
        message_id: String,
        token_usage: Option<Value>,
        tool_call_history: Option<Value>,
        execution_time_ms: u64,
    },
    /// Server requests the client to execute a tool.
    /// The client must call `send_tool_result()` with the result.
    ToolCall {
        call_id: String,
        tool_name: String,
        arguments: Value,
    },
    /// An error occurred during streaming
    Error(String),
}

type WsWrite = SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, Message>;
type WsRead = SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>;

/// WebSocket client for real-time communication with persistent connection.
///
/// Read and write halves are held in separate mutexes so that a spawned reader
/// task never blocks writes (e.g. sending `ClientToolResult` while the reader
/// is waiting for the next frame).
#[derive(Clone)]
pub struct WebSocketClient {
    ws_url: Url,
    token: String,
    writer: Arc<Mutex<Option<WsWrite>>>,
    reader: Arc<Mutex<Option<WsRead>>>,
}

impl WebSocketClient {
    /// Create a new WebSocket client
    pub fn new(ws_url: impl AsRef<str>, token: impl Into<String>) -> Result<Self> {
        let ws_url = Url::parse(ws_url.as_ref())?;
        Ok(Self {
            ws_url,
            token: token.into(),
            writer: Arc::new(Mutex::new(None)),
            reader: Arc::new(Mutex::new(None)),
        })
    }

    /// Ensure we have an active connection, reconnecting if needed
    async fn ensure_connected(&self) -> Result<()> {
        // Quick check: if writer exists, we're connected
        {
            let w = self.writer.lock().await;
            if w.is_some() {
                return Ok(());
            }
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

        let (write, read) = ws_stream.split();
        *self.writer.lock().await = Some(write);
        *self.reader.lock().await = Some(read);
        Ok(())
    }

    /// Helper: send a message through the writer half
    async fn ws_send(&self, msg: &impl Serialize) -> Result<()> {
        let mut w = self.writer.lock().await;
        let write = w
            .as_mut()
            .ok_or_else(|| Error::WebSocket("Not connected".to_string()))?;
        if let Err(e) = write
            .send(Message::Text(serde_json::to_string(msg)?.into()))
            .await
        {
            *w = None;
            return Err(Error::WebSocket(format!("Send failed: {}", e)));
        }
        Ok(())
    }

    /// Helper: read the next text frame from the reader half, skipping empty frames
    async fn ws_recv(&self) -> Result<Option<String>> {
        let mut r = self.reader.lock().await;
        let read = r
            .as_mut()
            .ok_or_else(|| Error::WebSocket("Not connected".to_string()))?;
        loop {
            match read.next().await {
                Some(Ok(msg)) => {
                    if let Ok(text) = msg.into_text() {
                        let text = text.to_string();
                        if text.is_empty() {
                            continue;
                        }
                        return Ok(Some(text));
                    }
                }
                Some(Err(e)) => {
                    *r = None;
                    return Err(Error::WebSocket(format!("Receive failed: {}", e)));
                }
                None => {
                    *r = None;
                    return Ok(None);
                }
            }
        }
    }

    /// Find all records in a collection via WebSocket
    pub async fn find_all(&self, collection: &str) -> Result<Vec<Record>> {
        self.ensure_connected().await?;

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

        self.ws_send(&request).await?;

        // Wait for response
        loop {
            let text = self
                .ws_recv()
                .await?
                .ok_or_else(|| Error::WebSocket("Connection closed unexpectedly".to_string()))?;

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
                _ => continue,
            };
        }
    }

    /// Subscribe to collection changes with an optional field filter.
    ///
    /// Returns a receiver that yields `MutationNotificationPayload` events.
    pub async fn subscribe(
        &self,
        collection: &str,
        filter_field: Option<&str>,
        filter_value: Option<&str>,
    ) -> Result<tokio::sync::mpsc::Receiver<MutationNotificationPayload>> {
        self.ensure_connected().await?;

        let message_id = format!(
            "{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| Error::WebSocket(e.to_string()))?
                .as_nanos()
        );

        let request = WebSocketRequest::Subscribe {
            message_id,
            payload: SubscribePayload {
                collection: collection.to_string(),
                filter_field: filter_field.map(|s| s.to_string()),
                filter_value: filter_value.map(|s| s.to_string()),
            },
        };

        self.ws_send(&request).await?;

        let (tx, rx) = tokio::sync::mpsc::channel(256);

        // Spawn a listener task — only holds the reader lock
        let reader = self.reader.clone();
        tokio::spawn(async move {
            loop {
                let mut r = reader.lock().await;
                let Some(read) = r.as_mut() else {
                    break;
                };

                match read.next().await {
                    Some(Ok(msg)) => {
                        // Drop the lock before processing so writes aren't blocked
                        drop(r);
                        if let Ok(text) = msg.into_text() {
                            if text.is_empty() {
                                continue;
                            }
                            if let Ok(response) = serde_json::from_str::<WebSocketResponse>(&text) {
                                if let WebSocketResponse::MutationNotification { payload } =
                                    response
                                {
                                    if tx.send(payload).await.is_err() {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Some(Err(_)) | None => break,
                }
            }
        });

        Ok(rx)
    }

    /// Send a chat message and receive streaming responses with client tool support.
    ///
    /// Returns a receiver that yields `ChatStreamEvent` items:
    /// - `Chunk(text)` for each token/chunk from the LLM
    /// - `ToolCall { call_id, tool_name, arguments }` when ekoDB needs a client tool executed
    /// - `End { ... }` when the stream completes
    /// - `Error(msg)` if something goes wrong
    ///
    /// When a `ToolCall` event is received, call `send_tool_result()` to return the
    /// result. ekoDB will feed the result back to the LLM and continue streaming.
    pub async fn chat_send(
        &self,
        chat_id: &str,
        message: &str,
    ) -> Result<tokio::sync::mpsc::Receiver<ChatStreamEvent>> {
        self.chat_send_with_tools(chat_id, message, None, None, None, None)
            .await
    }

    /// Send a chat message with client tool definitions.
    ///
    /// Same as `chat_send` but includes client-side tool definitions that ekoDB
    /// merges with its built-in tools when calling the LLM.
    pub async fn chat_send_with_tools(
        &self,
        chat_id: &str,
        message: &str,
        client_tools: Option<Vec<ClientToolDefinition>>,
        max_iterations: Option<u32>,
        confirm_tools: Option<Vec<String>>,
        exclude_tools: Option<Vec<String>>,
    ) -> Result<tokio::sync::mpsc::Receiver<ChatStreamEvent>> {
        self.ensure_connected().await?;

        let request = WebSocketRequest::ChatSend {
            payload: ChatSendPayload {
                chat_id: chat_id.to_string(),
                message: message.to_string(),
                bypass_ripple: None,
                client_tools,
                max_iterations,
                confirm_tools,
                exclude_tools,
            },
        };

        self.ws_send(&request).await?;

        let (tx, rx) = tokio::sync::mpsc::channel(256);
        let target_chat_id = chat_id.to_string();

        // Spawn a reader task — only holds the reader lock, never the writer lock.
        // This lets `send_tool_result` write concurrently without blocking.
        let reader = self.reader.clone();
        tokio::spawn(async move {
            loop {
                let msg = {
                    let mut r = reader.lock().await;
                    let Some(read) = r.as_mut() else {
                        break;
                    };
                    read.next().await
                };
                // Lock is dropped here — writer is free to send

                match msg {
                    Some(Ok(msg)) => {
                        if let Ok(text) = msg.into_text() {
                            if text.is_empty() {
                                continue;
                            }
                            if let Ok(response) = serde_json::from_str::<WebSocketResponse>(&text) {
                                match response {
                                    WebSocketResponse::ChatStreamChunk { payload }
                                        if payload.chat_id == target_chat_id =>
                                    {
                                        if tx
                                            .send(ChatStreamEvent::Chunk(payload.content))
                                            .await
                                            .is_err()
                                        {
                                            break;
                                        }
                                    }
                                    WebSocketResponse::ChatStreamEnd { payload }
                                        if payload.chat_id == target_chat_id =>
                                    {
                                        let _ = tx
                                            .send(ChatStreamEvent::End {
                                                message_id: payload.message_id,
                                                token_usage: payload.token_usage,
                                                tool_call_history: payload.tool_call_history,
                                                execution_time_ms: payload.execution_time_ms,
                                            })
                                            .await;
                                        break;
                                    }
                                    WebSocketResponse::ChatStreamError { payload }
                                        if payload.chat_id == target_chat_id =>
                                    {
                                        let _ =
                                            tx.send(ChatStreamEvent::Error(payload.error)).await;
                                        break;
                                    }
                                    WebSocketResponse::ClientToolCall { payload }
                                        if payload.chat_id == target_chat_id =>
                                    {
                                        if tx
                                            .send(ChatStreamEvent::ToolCall {
                                                call_id: payload.call_id,
                                                tool_name: payload.tool_name,
                                                arguments: payload.arguments,
                                            })
                                            .await
                                            .is_err()
                                        {
                                            break;
                                        }
                                    }
                                    _ => {
                                        // Skip non-chat messages (subscriptions, pings, etc.)
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    Some(Err(_)) | None => {
                        let _ = tx
                            .send(ChatStreamEvent::Error("Connection closed".to_string()))
                            .await;
                        break;
                    }
                }
            }
        });

        Ok(rx)
    }

    /// Register client-side tools for a chat session.
    ///
    /// These tools persist across messages within the session. When the LLM
    /// invokes one, ekoDB sends a `ClientToolCall` event back to the client.
    pub async fn register_client_tools(
        &self,
        chat_id: &str,
        tools: Vec<ClientToolDefinition>,
    ) -> Result<()> {
        self.ensure_connected().await?;

        let request = WebSocketRequest::RegisterClientTools {
            payload: RegisterClientToolsPayload {
                chat_id: chat_id.to_string(),
                tools,
            },
        };

        self.ws_send(&request).await?;

        // Wait for ack
        loop {
            let text = self.ws_recv().await?.ok_or_else(|| {
                Error::WebSocket("Connection closed before registration ack".to_string())
            })?;

            let response: WebSocketResponse = serde_json::from_str(&text)?;
            return match response {
                WebSocketResponse::Success { .. } => Ok(()),
                WebSocketResponse::Error { code, message } => Err(Error::api(code, message)),
                _ => continue,
            };
        }
    }

    /// Send the result of a client-side tool execution back to ekoDB.
    ///
    /// Call this after receiving a `ChatStreamEvent::ToolCall` event.
    /// ekoDB will feed the result back to the LLM and continue the conversation.
    pub async fn send_tool_result(
        &self,
        chat_id: &str,
        call_id: &str,
        success: bool,
        result: Option<Value>,
        error: Option<String>,
    ) -> Result<()> {
        // Only locks the writer — reader task can keep running
        let request = WebSocketRequest::ClientToolResult {
            payload: ClientToolResultPayload {
                chat_id: chat_id.to_string(),
                call_id: call_id.to_string(),
                success,
                result,
                error,
            },
        };

        self.ws_send(&request).await
    }

    /// Close the WebSocket connection with a proper close frame.
    pub async fn close(&self) -> Result<()> {
        if let Some(writer) = self.writer.lock().await.as_mut() {
            // Send a close frame before dropping
            let _ = writer.close().await;
        }
        *self.writer.lock().await = None;
        *self.reader.lock().await = None;
        Ok(())
    }
}
