export {
  EkoDBClient,
  WebSocketClient,
  SerializationFormat,
  MergeStrategy,
  RateLimitError,
} from "./client";
export { QueryBuilder, SortOrder } from "./query-builder";
export { SearchQueryBuilder } from "./search";
export {
  SchemaBuilder,
  FieldTypeSchemaBuilder,
  VectorIndexAlgorithm,
  DistanceMetric,
} from "./schema";
export { JoinBuilder } from "./join";
export type { SearchQuery, SearchResult, SearchResponse } from "./search";
export type {
  Schema,
  FieldTypeSchema,
  IndexConfig,
  CollectionMetadata,
} from "./schema";
export type { JoinConfig } from "./join";
export type {
  Record,
  Query,
  BatchOperationResult,
  ClientConfig,
  RateLimitInfo,
  CollectionConfig,
  ChatRequest,
  CreateChatSessionRequest,
  ChatMessageRequest,
  TokenUsage,
  ChatResponse,
  ChatSession,
  ChatSessionResponse,
  ListSessionsQuery,
  ListSessionsResponse,
  GetMessagesQuery,
  GetMessagesResponse,
  UpdateSessionRequest,
  MergeSessionsRequest,
} from "./client";
