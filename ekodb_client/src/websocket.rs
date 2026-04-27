//! WebSocket client for real-time subscriptions and chat streaming
//!
//! This module provides WebSocket support for real-time updates, queries,
//! and streaming chat responses with bidirectional client tool callbacks.
//!
//! Uses a single reader loop (dispatcher) that demultiplexes incoming frames
//! to per-request/per-subscription/per-chat channels, avoiding reader contention.

use crate::Record;
use crate::error::{Error, Result};
use futures_util::{SinkExt, StreamExt, stream::SplitSink, stream::SplitStream};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
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
    /// Stateless raw LLM completion (no session, no history, no RAG).
    /// Mirrors POST /api/chat/complete. Response: Success { data: { "content": "..." } }.
    RawComplete {
        system_prompt: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_tokens: Option<i32>,
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
        /// Server-echoed messageId for request-response correlation (top-level)
        #[serde(rename = "messageId", default)]
        message_id: Option<String>,
    },
    Error {
        code: u16,
        message: String,
        /// Server-echoed messageId for request-response correlation (top-level)
        #[serde(rename = "messageId", default)]
        message_id: Option<String>,
    },
    /// Schema change notification from server (collection config changed)
    SchemaChanged { payload: SchemaChangedPayload },
    /// A streamed text chunk from the LLM
    ChatStreamChunk { payload: ChatStreamChunkPayload },
    /// Final message after streaming completes
    ChatStreamEnd { payload: ChatStreamEndPayload },
    /// Error during chat streaming
    ChatStreamError { payload: ChatStreamErrorPayload },
    /// Mutation notification from a subscription
    MutationNotification {
        payload: MutationNotificationPayload,
    },
    /// Server requests the client to execute a tool
    ClientToolCall { payload: ClientToolCallPayload },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsePayload {
    pub data: Value,
    /// Server echoes back the message_id for request-response correlation
    #[serde(rename = "messageId", default)]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaChangedPayload {
    pub collection: String,
    pub version: u64,
    pub primary_key_alias: String,
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
    /// Model's context window size in tokens (for client display).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_window: Option<u32>,
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
        context_window: Option<u32>,
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

/// Shared state for the dispatcher to route messages
struct DispatchState {
    /// One-shot channels for request-response (keyed by message_id)
    pending_requests: HashMap<String, tokio::sync::oneshot::Sender<WebSocketResponse>>,
    /// Subscription channels: (collection_name, sender) — only matching notifications are routed
    subscription_senders: Vec<(
        String,
        tokio::sync::mpsc::Sender<MutationNotificationPayload>,
    )>,
    /// Chat stream channels (keyed by chat_id)
    chat_senders: HashMap<String, tokio::sync::mpsc::Sender<ChatStreamEvent>>,
    /// Dedicated ack channel for register_client_tools (protocol has no messageId for this)
    register_tools_ack: Option<tokio::sync::oneshot::Sender<WebSocketResponse>>,
    /// Optional schema cache to invalidate on SchemaChanged events
    schema_cache: Option<std::sync::Arc<crate::schema_cache::SchemaCache>>,
}

/// WebSocket client for real-time communication with persistent connection.
///
/// Uses a single dispatcher task that reads all incoming frames and routes them
/// to the appropriate consumer (request-response, subscription, or chat stream).
/// The writer is held in a separate mutex so sends never block on reads.
#[derive(Clone)]
pub struct WebSocketClient {
    ws_url: Url,
    token: String,
    writer: Arc<Mutex<Option<WsWrite>>>,
    dispatch: Arc<Mutex<DispatchState>>,
    connected: Arc<Mutex<bool>>,
}

impl WebSocketClient {
    /// Create a new WebSocket client
    pub fn new(ws_url: impl AsRef<str>, token: impl Into<String>) -> Result<Self> {
        let ws_url = Url::parse(ws_url.as_ref())?;
        Ok(Self {
            ws_url,
            token: token.into(),
            writer: Arc::new(Mutex::new(None)),
            dispatch: Arc::new(Mutex::new(DispatchState {
                pending_requests: HashMap::new(),
                subscription_senders: Vec::new(),
                chat_senders: HashMap::new(),
                register_tools_ack: None,
                schema_cache: None,
            })),
            connected: Arc::new(Mutex::new(false)),
        })
    }

    /// Ensure we have an active connection, reconnecting if needed
    pub(crate) async fn ensure_connected(&self) -> Result<()> {
        // Quick check: if connected, return
        {
            let w = self.writer.lock().await;
            let c = self.connected.lock().await;
            if w.is_some() && *c {
                return Ok(());
            }
        }

        // Clear stale state if partially connected
        {
            *self.writer.lock().await = None;
            *self.connected.lock().await = false;
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
        *self.connected.lock().await = true;

        // Spawn the single dispatcher task that reads all incoming frames
        self.spawn_dispatcher(read);

        Ok(())
    }

    /// Spawn the single reader loop that demultiplexes incoming frames
    fn spawn_dispatcher(&self, mut read: WsRead) {
        let dispatch = self.dispatch.clone();
        let writer = self.writer.clone();
        let connected = self.connected.clone();

        tokio::spawn(async move {
            loop {
                match read.next().await {
                    Some(Ok(msg)) => {
                        let text = match msg.into_text() {
                            Ok(t) if !t.is_empty() => t.to_string(),
                            _ => continue,
                        };

                        let response: WebSocketResponse = match serde_json::from_str(&text) {
                            Ok(r) => r,
                            Err(_) => continue,
                        };

                        // Collect send targets while holding the lock, then drop
                        // the lock before any async .send().await to avoid blocking
                        // the dispatcher when a channel is full.
                        enum SendAction {
                            None,
                            ChatEvent(tokio::sync::mpsc::Sender<ChatStreamEvent>, ChatStreamEvent),
                        }

                        let action = {
                            let mut state = dispatch.lock().await;

                            match response {
                                // Request-response: route by message_id (oneshot, no async needed)
                                // Try top-level messageId first, then payload.messageId, then single-pending fallback
                                WebSocketResponse::Success {
                                    ref payload,
                                    ref message_id,
                                } => {
                                    let mid = message_id.as_ref().or(payload.message_id.as_ref());
                                    if let Some(mid) = mid {
                                        if let Some(tx) = state.pending_requests.remove(mid) {
                                            let _ = tx.send(response);
                                            SendAction::None
                                        } else if let Some(tx) = state.register_tools_ack.take() {
                                            let _ = tx.send(response);
                                            SendAction::None
                                        } else {
                                            SendAction::None
                                        }
                                    } else if let Some(tx) = state.register_tools_ack.take() {
                                        let _ = tx.send(response);
                                        SendAction::None
                                    } else if state.pending_requests.len() == 1 {
                                        // Legacy fallback: no messageId, single pending request
                                        let key =
                                            state.pending_requests.keys().next().cloned().unwrap();
                                        if let Some(tx) = state.pending_requests.remove(&key) {
                                            let _ = tx.send(response);
                                        }
                                        SendAction::None
                                    } else {
                                        SendAction::None
                                    }
                                }
                                WebSocketResponse::Error { ref message_id, .. } => {
                                    if let Some(mid) = message_id {
                                        if let Some(tx) = state.pending_requests.remove(mid) {
                                            let _ = tx.send(response);
                                        }
                                    } else if let Some(tx) = state.register_tools_ack.take() {
                                        let _ = tx.send(response);
                                    } else if state.pending_requests.len() == 1 {
                                        let key =
                                            state.pending_requests.keys().next().cloned().unwrap();
                                        if let Some(tx) = state.pending_requests.remove(&key) {
                                            let _ = tx.send(response);
                                        }
                                    }
                                    SendAction::None
                                }

                                // Schema change: invalidate cached schema
                                WebSocketResponse::SchemaChanged { ref payload } => {
                                    if let Some(ref cache) = state.schema_cache {
                                        cache.handle_schema_changed(
                                            &payload.collection,
                                            payload.version,
                                            &payload.primary_key_alias,
                                        );
                                    }
                                    SendAction::None
                                }

                                // Subscription: filter by collection, use try_send (non-async)
                                WebSocketResponse::MutationNotification { payload } => {
                                    state.subscription_senders.retain(|(_, tx)| !tx.is_closed());
                                    for (col, tx) in &state.subscription_senders {
                                        if *col == payload.collection {
                                            let _ = tx.try_send(payload.clone());
                                        }
                                    }
                                    SendAction::None
                                }

                                // Chat stream: clone sender, build event, send outside lock
                                WebSocketResponse::ChatStreamChunk { payload } => {
                                    if let Some(tx) = state.chat_senders.get(&payload.chat_id) {
                                        SendAction::ChatEvent(
                                            tx.clone(),
                                            ChatStreamEvent::Chunk(payload.content),
                                        )
                                    } else {
                                        SendAction::None
                                    }
                                }
                                WebSocketResponse::ChatStreamEnd { payload } => {
                                    if let Some(tx) = state.chat_senders.remove(&payload.chat_id) {
                                        SendAction::ChatEvent(
                                            tx,
                                            ChatStreamEvent::End {
                                                message_id: payload.message_id,
                                                token_usage: payload.token_usage,
                                                tool_call_history: payload.tool_call_history,
                                                execution_time_ms: payload.execution_time_ms,
                                                context_window: payload.context_window,
                                            },
                                        )
                                    } else {
                                        SendAction::None
                                    }
                                }
                                WebSocketResponse::ChatStreamError { payload } => {
                                    if let Some(tx) = state.chat_senders.remove(&payload.chat_id) {
                                        SendAction::ChatEvent(
                                            tx,
                                            ChatStreamEvent::Error(payload.error),
                                        )
                                    } else {
                                        SendAction::None
                                    }
                                }
                                WebSocketResponse::ClientToolCall { payload } => {
                                    if let Some(tx) = state.chat_senders.get(&payload.chat_id) {
                                        SendAction::ChatEvent(
                                            tx.clone(),
                                            ChatStreamEvent::ToolCall {
                                                call_id: payload.call_id,
                                                tool_name: payload.tool_name,
                                                arguments: payload.arguments,
                                            },
                                        )
                                    } else {
                                        SendAction::None
                                    }
                                }
                            }
                        };
                        // Lock is dropped — perform async sends outside the lock
                        match action {
                            SendAction::ChatEvent(tx, event) => {
                                let _ = tx.send(event).await;
                            }
                            SendAction::None => {}
                        }
                    }
                    Some(Err(_)) | None => {
                        // Connection closed — mark as disconnected and notify all waiters
                        *writer.lock().await = None;
                        *connected.lock().await = false;

                        // Collect chat senders, then drop lock before async sends
                        let (pending, chat_senders, tools_ack) = {
                            let mut state = dispatch.lock().await;
                            let pending: Vec<_> =
                                state.pending_requests.drain().map(|(_, tx)| tx).collect();
                            let chats: Vec<_> = state.chat_senders.drain().collect();
                            let tools = state.register_tools_ack.take();
                            state.subscription_senders.clear();
                            (pending, chats, tools)
                        };

                        for tx in pending {
                            let _ = tx.send(WebSocketResponse::Error {
                                code: 0,
                                message: "Connection closed".to_string(),
                                message_id: None,
                            });
                        }
                        if let Some(tx) = tools_ack {
                            let _ = tx.send(WebSocketResponse::Error {
                                code: 0,
                                message: "Connection closed".to_string(),
                                message_id: None,
                            });
                        }
                        for (_, tx) in chat_senders {
                            let _ = tx
                                .send(ChatStreamEvent::Error("Connection closed".to_string()))
                                .await;
                        }
                        break;
                    }
                }
            }
        });
    }

    /// Generate a unique message ID for request-response correlation
    fn gen_message_id() -> Result<String> {
        Ok(format!(
            "{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| Error::WebSocket(e.to_string()))?
                .as_nanos()
        ))
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
            *self.connected.lock().await = false;
            return Err(Error::WebSocket(format!("Send failed: {}", e)));
        }
        Ok(())
    }

    /// Register a one-shot channel for a request-response and wait for the response
    async fn send_and_wait(
        &self,
        msg: &impl Serialize,
        message_id: &str,
    ) -> Result<WebSocketResponse> {
        let (tx, rx) = tokio::sync::oneshot::channel();

        {
            let mut state = self.dispatch.lock().await;
            state.pending_requests.insert(message_id.to_string(), tx);
        }

        if let Err(e) = self.ws_send(msg).await {
            // Clean up on send failure
            let mut state = self.dispatch.lock().await;
            state.pending_requests.remove(message_id);
            return Err(e);
        }

        rx.await
            .map_err(|_| Error::WebSocket("Response channel closed".to_string()))
    }

    /// Find all records in a collection via WebSocket
    pub async fn find_all(&self, collection: &str) -> Result<Vec<Record>> {
        self.ensure_connected().await?;

        let message_id = Self::gen_message_id()?;

        let request = WebSocketRequest::FindAll {
            message_id: message_id.clone(),
            payload: FindAllPayload {
                collection: collection.to_string(),
            },
        };

        let response = self.send_and_wait(&request, &message_id).await?;

        match response {
            WebSocketResponse::Success { payload, .. } => {
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
            WebSocketResponse::Error { code, message, .. } => Err(Error::api(code, message)),
            _ => Err(Error::WebSocket("Unexpected response type".to_string())),
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

        let message_id = Self::gen_message_id()?;

        let request = WebSocketRequest::Subscribe {
            message_id: message_id.clone(),
            payload: SubscribePayload {
                collection: collection.to_string(),
                filter_field: filter_field.map(|s| s.to_string()),
                filter_value: filter_value.map(|s| s.to_string()),
            },
        };

        // Send subscribe request and wait for ack
        let response = self.send_and_wait(&request, &message_id).await?;
        match response {
            WebSocketResponse::Success { .. } => {}
            WebSocketResponse::Error { code, message, .. } => {
                return Err(Error::api(code, message));
            }
            _ => {
                return Err(Error::WebSocket(
                    "Unexpected response to subscribe".to_string(),
                ));
            }
        }

        // Register the subscription channel — dispatcher routes only matching collection notifications
        let (tx, rx) = tokio::sync::mpsc::channel(256);
        {
            let mut state = self.dispatch.lock().await;
            state
                .subscription_senders
                .push((collection.to_string(), tx));
        }

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

        // Register the chat stream channel before sending so we don't miss early frames
        let (tx, rx) = tokio::sync::mpsc::channel(256);
        {
            let mut state = self.dispatch.lock().await;
            state.chat_senders.insert(chat_id.to_string(), tx);
        }

        if let Err(e) = self.ws_send(&request).await {
            // Clean up on send failure
            let mut state = self.dispatch.lock().await;
            state.chat_senders.remove(chat_id);
            return Err(e);
        }

        Ok(rx)
    }

    /// Register client-side tools for a chat session.
    ///
    /// These tools persist across messages within the session. When the LLM
    /// invokes one, ekoDB sends a `ClientToolCall` event back to the client.
    ///
    /// Note: The WS protocol for RegisterClientTools does not include a messageId
    /// field, so we use a dedicated `register_tools_ack` slot in the dispatcher
    /// instead of `pending_requests`. This means only one registration can be
    /// in-flight at a time, which is fine since tool registration is typically
    /// done once before chat starts.
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

        let (tx, rx_ack) = tokio::sync::oneshot::channel();
        {
            let mut state = self.dispatch.lock().await;
            state.register_tools_ack = Some(tx);
        }

        if let Err(e) = self.ws_send(&request).await {
            let mut state = self.dispatch.lock().await;
            state.register_tools_ack = None;
            return Err(e);
        }

        let response = rx_ack
            .await
            .map_err(|_| Error::WebSocket("Response channel closed".to_string()))?;

        match response {
            WebSocketResponse::Success { .. } => Ok(()),
            WebSocketResponse::Error { code, message, .. } => Err(Error::api(code, message)),
            _ => Err(Error::WebSocket(
                "Unexpected response to register_client_tools".to_string(),
            )),
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
        // Only locks the writer — dispatcher keeps running
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

    /// Stateless raw LLM completion via WebSocket.
    ///
    /// Sends a `RawComplete` message and waits for the `Success` response.
    /// Preferred over HTTP for deployed instances: the persistent WSS connection
    /// is already authenticated and won't be killed by reverse proxy timeouts.
    pub async fn raw_completion(
        &self,
        request: &crate::chat::RawCompletionRequest,
    ) -> Result<crate::chat::RawCompletionResponse> {
        self.ensure_connected().await?;

        let message_id = Self::gen_message_id()?;

        let ws_request = WebSocketRequest::RawComplete {
            system_prompt: request.system_prompt.clone(),
            message: request.message.clone(),
            provider: request.provider.clone(),
            model: request.model.clone(),
            max_tokens: request.max_tokens,
        };

        let response = self.send_and_wait(&ws_request, &message_id).await?;

        match response {
            WebSocketResponse::Success { payload, .. } => {
                let content = payload
                    .data
                    .get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                Ok(crate::chat::RawCompletionResponse { content })
            }
            WebSocketResponse::Error { code, message, .. } => Err(Error::api(code, message)),
            _ => Err(Error::WebSocket(
                "Unexpected response to RawComplete".to_string(),
            )),
        }
    }

    /// Close the WebSocket connection with a proper close frame.
    pub async fn close(&self) -> Result<()> {
        if let Some(writer) = self.writer.lock().await.as_mut() {
            // Send a close frame before dropping
            let _ = writer.close().await;
        }
        *self.writer.lock().await = None;
        *self.connected.lock().await = false;

        // Clean up dispatch state
        let mut state = self.dispatch.lock().await;
        state.pending_requests.clear();
        state.subscription_senders.clear();
        state.chat_senders.clear();

        Ok(())
    }

    /// Attach a schema cache for automatic invalidation on SchemaChanged events.
    pub async fn set_schema_cache(&self, cache: std::sync::Arc<crate::schema_cache::SchemaCache>) {
        let mut state = self.dispatch.lock().await;
        state.schema_cache = Some(cache);
    }

    // =========================================================================
    // WS CRUD Methods — Full Parity with Server
    // =========================================================================

    /// Helper: send a typed WS request as raw JSON and wait for response.
    /// Attaches messageId at the top level for concurrent correlation.
    /// Send a typed CRUD request as raw JSON and return the data from the response.
    pub async fn send_crud(&self, msg_type: &str, payload: Value) -> Result<Value> {
        self.ensure_connected().await?;
        let message_id = Self::gen_message_id()?;

        let request = serde_json::json!({
            "type": msg_type,
            "messageId": message_id,
            "payload": payload,
        });

        // Use raw JSON send (not typed enum) for flexibility
        let response = self.send_and_wait(&request, &message_id).await?;

        match response {
            WebSocketResponse::Success { payload, .. } => Ok(payload.data),
            WebSocketResponse::Error { code, message, .. } => Err(Error::api(code, message)),
            _ => Err(Error::WebSocket("Unexpected response type".to_string())),
        }
    }

    /// Insert a single record into a collection via WebSocket.
    pub async fn insert(
        &self,
        collection: &str,
        record: Value,
        bypass_ripple: Option<bool>,
    ) -> Result<Value> {
        let mut payload = serde_json::json!({
            "collection": collection,
            "record": record,
        });
        if let Some(br) = bypass_ripple {
            payload["bypass_ripple"] = serde_json::json!(br);
        }
        self.send_crud("Insert", payload).await
    }

    /// Query records from a collection via WebSocket.
    pub async fn query(
        &self,
        collection: &str,
        filter: Option<Value>,
        sort: Option<Value>,
        limit: Option<u64>,
        skip: Option<u64>,
    ) -> Result<Vec<Value>> {
        let mut payload = serde_json::json!({"collection": collection});
        if let Some(f) = filter {
            payload["filter"] = f;
        }
        if let Some(s) = sort {
            payload["sort"] = s;
        }
        if let Some(l) = limit {
            payload["limit"] = serde_json::json!(l);
        }
        if let Some(s) = skip {
            payload["skip"] = serde_json::json!(s);
        }
        let data = self.send_crud("Query", payload).await?;
        Ok(data.as_array().cloned().unwrap_or_default())
    }

    /// Find a record by ID via WebSocket.
    pub async fn find_by_id(&self, collection: &str, id: &str) -> Result<Value> {
        self.send_crud(
            "FindById",
            serde_json::json!({"collection": collection, "id": id}),
        )
        .await
    }

    /// Update a record by ID via WebSocket.
    pub async fn update(
        &self,
        collection: &str,
        id: &str,
        record: Value,
        bypass_ripple: Option<bool>,
    ) -> Result<Value> {
        let mut payload = serde_json::json!({
            "collection": collection,
            "id": id,
            "record": record,
        });
        if let Some(br) = bypass_ripple {
            payload["bypass_ripple"] = serde_json::json!(br);
        }
        self.send_crud("Update", payload).await
    }

    /// Delete a record by ID via WebSocket.
    pub async fn delete(
        &self,
        collection: &str,
        id: &str,
        bypass_ripple: Option<bool>,
    ) -> Result<()> {
        let mut payload = serde_json::json!({"collection": collection, "id": id});
        if let Some(br) = bypass_ripple {
            payload["bypass_ripple"] = serde_json::json!(br);
        }
        self.send_crud("Delete", payload).await?;
        Ok(())
    }

    /// Batch insert multiple records via WebSocket.
    pub async fn batch_insert(
        &self,
        collection: &str,
        records: Vec<Value>,
        bypass_ripple: Option<bool>,
    ) -> Result<Value> {
        let mut payload = serde_json::json!({
            "collection": collection,
            "records": records,
        });
        if let Some(br) = bypass_ripple {
            payload["bypass_ripple"] = serde_json::json!(br);
        }
        self.send_crud("BatchInsert", payload).await
    }

    /// Batch update multiple records via WebSocket.
    /// Each update is a tuple of (id, record_data).
    pub async fn batch_update(
        &self,
        collection: &str,
        updates: Vec<(String, Value)>,
        bypass_ripple: Option<bool>,
    ) -> Result<Value> {
        let updates_json: Vec<Value> = updates
            .into_iter()
            .map(|(id, data)| serde_json::json!([id, data]))
            .collect();
        let mut payload = serde_json::json!({
            "collection": collection,
            "updates": updates_json,
        });
        if let Some(br) = bypass_ripple {
            payload["bypass_ripple"] = serde_json::json!(br);
        }
        self.send_crud("BatchUpdate", payload).await
    }

    /// Batch delete multiple records by IDs via WebSocket.
    pub async fn batch_delete(
        &self,
        collection: &str,
        ids: Vec<String>,
        bypass_ripple: Option<bool>,
    ) -> Result<()> {
        let mut payload = serde_json::json!({"collection": collection, "ids": ids});
        if let Some(br) = bypass_ripple {
            payload["bypass_ripple"] = serde_json::json!(br);
        }
        self.send_crud("BatchDelete", payload).await?;
        Ok(())
    }

    /// Full-text search via WebSocket.
    pub async fn text_search(
        &self,
        collection: &str,
        query: &str,
        fields: Option<Vec<String>>,
        limit: Option<u64>,
    ) -> Result<Vec<Value>> {
        let mut options = serde_json::Map::new();
        if let Some(f) = fields {
            options.insert("fields".to_string(), serde_json::json!(f));
        }
        if let Some(l) = limit {
            options.insert("limit".to_string(), serde_json::json!(l));
        }
        let payload = serde_json::json!({
            "collection": collection,
            "query": query,
            "options": if options.is_empty() { Value::Null } else { Value::Object(options) },
        });
        let data = self.send_crud("TextSearch", payload).await?;
        Ok(data.as_array().cloned().unwrap_or_default())
    }

    /// Get distinct values for a field via WebSocket.
    pub async fn distinct_values(
        &self,
        collection: &str,
        field: &str,
        filter: Option<Value>,
    ) -> Result<Value> {
        let mut payload = serde_json::json!({"collection": collection, "field": field});
        if let Some(f) = filter {
            payload["filter"] = f;
        }
        self.send_crud("DistinctValues", payload).await
    }

    /// Apply an atomic field action to a record via WebSocket.
    pub async fn update_with_action(
        &self,
        collection: &str,
        id: &str,
        action: &str,
        field: &str,
        value: Option<Value>,
    ) -> Result<Value> {
        let mut payload = serde_json::json!({
            "collection": collection,
            "id": id,
            "action": action,
            "field": field,
        });
        if let Some(v) = value {
            payload["value"] = v;
        }
        self.send_crud("UpdateWithAction", payload).await
    }

    /// Create a new collection with optional schema via WebSocket.
    pub async fn create_collection(&self, name: &str, schema: Option<Value>) -> Result<()> {
        let payload = serde_json::json!({
            "name": name,
            "schema": schema.unwrap_or(serde_json::json!({})),
        });
        self.send_crud("CreateCollection", payload).await?;
        Ok(())
    }

    /// List all collections via WebSocket.
    pub async fn list_collections(&self) -> Result<Vec<String>> {
        let data = self
            .send_crud("GetCollections", serde_json::json!({}))
            .await?;
        Ok(data
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default())
    }

    /// Delete a collection via WebSocket.
    pub async fn delete_collection(&self, name: &str) -> Result<()> {
        self.send_crud("DeleteCollection", serde_json::json!({"name": name}))
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_stream_end_payload_with_context_window() {
        let json_str = r#"{
            "chat_id": "chat-1",
            "message_id": "msg-1",
            "execution_time_ms": 250,
            "context_window": 128000
        }"#;
        let payload: ChatStreamEndPayload = serde_json::from_str(json_str).unwrap();
        assert_eq!(payload.context_window, Some(128000));
        assert_eq!(payload.message_id, "msg-1");
        assert_eq!(payload.execution_time_ms, 250);
    }

    #[test]
    fn chat_stream_end_payload_without_context_window() {
        let json_str = r#"{
            "chat_id": "chat-2",
            "message_id": "msg-2",
            "execution_time_ms": 100
        }"#;
        let payload: ChatStreamEndPayload = serde_json::from_str(json_str).unwrap();
        assert_eq!(payload.context_window, None);
    }

    #[test]
    fn chat_stream_end_payload_serializes_without_context_window() {
        let payload = ChatStreamEndPayload {
            chat_id: "c1".into(),
            message_id: "m1".into(),
            token_usage: None,
            tool_call_history: None,
            execution_time_ms: 100,
            context_window: None,
        };
        let json = serde_json::to_value(&payload).unwrap();
        assert!(json.get("context_window").is_none());
    }

    #[test]
    fn chat_stream_end_payload_serializes_with_context_window() {
        let payload = ChatStreamEndPayload {
            chat_id: "c1".into(),
            message_id: "m1".into(),
            token_usage: None,
            tool_call_history: None,
            execution_time_ms: 100,
            context_window: Some(200000),
        };
        let json = serde_json::to_value(&payload).unwrap();
        assert_eq!(json["context_window"], 200000);
    }

    #[test]
    fn chat_stream_event_end_carries_context_window() {
        let event = ChatStreamEvent::End {
            message_id: "m1".into(),
            token_usage: None,
            tool_call_history: None,
            execution_time_ms: 50,
            context_window: Some(128000),
        };
        match event {
            ChatStreamEvent::End { context_window, .. } => {
                assert_eq!(context_window, Some(128000));
            }
            _ => panic!("expected End variant"),
        }
    }

    #[test]
    fn websocket_response_chat_stream_end_deserializes() {
        let json_str = r#"{
            "type": "ChatStreamEnd",
            "payload": {
                "chat_id": "chat-1",
                "message_id": "msg-1",
                "execution_time_ms": 300,
                "context_window": 128000,
                "token_usage": {"prompt_tokens": 100, "completion_tokens": 50}
            }
        }"#;
        let msg: WebSocketResponse = serde_json::from_str(json_str).unwrap();
        match msg {
            WebSocketResponse::ChatStreamEnd { payload } => {
                assert_eq!(payload.context_window, Some(128000));
                assert_eq!(payload.message_id, "msg-1");
                assert!(payload.token_usage.is_some());
            }
            _ => panic!("expected ChatStreamEnd"),
        }
    }
}
