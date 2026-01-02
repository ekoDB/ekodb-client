//! # ekoDB Client Library
//!
//! Official Rust client for ekoDB - A high-performance database with intelligent caching,
//! real-time capabilities, and automatic optimization.
//!
//! ## Features
//!
//! - **Async/Await**: Built on Tokio for high-performance async operations
//! - **Type-Safe**: Strong typing with Rust's type system
//! - **Auto-Retry**: Automatic retry with exponential backoff for transient failures
//! - **Connection Pooling**: Efficient connection management
//! - **WebSocket Support**: Real-time subscriptions and updates
//! - **Batch Operations**: Efficient bulk inserts, updates, and deletes
//! - **Query Builder**: Fluent API for building complex queries
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ekodb_client::{Client, Record};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client
//!     let client = Client::builder()
//!         .base_url("https://api.ekodb.net")
//!         .api_key("your-api-key")
//!         .build()?;
//!
//!     // Insert a record
//!     let mut record = Record::new();
//!     record.insert("name", "John Doe");
//!     record.insert("age", 30);
//!     
//!     let result = client.insert("users", record, None).await?;
//!     println!("Inserted: {:?}", result);
//!
//!     Ok(())
//! }
//! ```

mod auth;
mod batch;
mod chat;
mod client;
mod error;
mod functions;
mod http;
mod join;
mod query;
mod query_builder;
mod retry;
mod schema;
mod search;
mod types;
mod websocket;

// Public API exports
pub use batch::BatchBuilder;
pub use chat::{
    ChatMessageRequest, ChatRequest, ChatResponse, ChatSession, ChatSessionResponse,
    CollectionConfig, ContextSnippet, CreateChatSessionRequest, FieldSearchOptions,
    GetMessagesQuery, GetMessagesResponse, ListSessionsQuery, ListSessionsResponse,
    MergeSessionsRequest, MergeStrategy, Models, TextSearchOptions as ChatTextSearchOptions,
    ToggleForgottenRequest, TokenUsage, UpdateMessageRequest, UpdateSessionRequest,
};
pub use client::{Client, ClientBuilder, RateLimitInfo};
pub use error::{Error, Result};
pub use functions::{
    ChatMessage, Function, FunctionResult, FunctionStats, GroupFunctionConfig, GroupFunctionOp,
    ParameterDefinition, Script, ScriptCondition, SortFieldConfig, StageStats, UserFunction,
};
pub use join::JoinConfig;
pub use query_builder::{QueryBuilder, SortOrder};
pub use schema::{
    CollectionMetadata, DistanceMetric, FieldTypeSchema, IndexConfig, Schema, VectorIndexAlgorithm,
};
pub use search::{SearchQuery, SearchResponse, SearchResult};
pub use types::{FieldType, NumberValue, Query, QueryOperator, Record, SerializationFormat};
pub use websocket::WebSocketClient;

/// Client version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
