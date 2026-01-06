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
pub mod chat;
pub mod client;
pub mod error;
pub mod functions;
mod http;
mod join;
pub mod options;
mod query;
mod query_builder;
pub mod retry;
pub mod schema;
pub mod search;
pub mod types;
mod utils;
mod websocket;

// Public API exports
pub use auth::*;
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
pub use utils::{
    extract_record, get_array_value, get_binary_value, get_bool_value, get_bytes_value,
    get_datetime_value, get_decimal_value, get_duration_value, get_float_value, get_int_value,
    get_object_value, get_set_value, get_string_value, get_uuid_value, get_value, get_values,
    get_vector_value,
};
pub use websocket::WebSocketClient;

/// Client version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
