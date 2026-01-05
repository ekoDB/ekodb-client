/**
 * ekoDB TypeScript Client
 */

import { encode, decode } from "@msgpack/msgpack";
import { QueryBuilder, Query as QueryBuilderQuery } from "./query-builder";
import { SearchQuery, SearchQueryBuilder, SearchResponse } from "./search";
import { Schema, SchemaBuilder, CollectionMetadata } from "./schema";
import { Script, FunctionResult } from "./functions";

export interface Record {
  [key: string]: any;
}

/**
 * Serialization format for client-server communication
 */
export enum SerializationFormat {
  /** JSON format (default, human-readable) */
  Json = "Json",
  /** MessagePack format (binary, 2-3x faster) */
  MessagePack = "MessagePack",
}

/**
 * Rate limit information from the server
 */
export interface RateLimitInfo {
  /** Maximum requests allowed per window */
  limit: number;
  /** Requests remaining in current window */
  remaining: number;
  /** Unix timestamp when the rate limit resets */
  reset: number;
}

/**
 * Client configuration options
 */
export interface ClientConfig {
  /** Base URL of the ekoDB server */
  baseURL: string;
  /** API key for authentication */
  apiKey: string;
  /** Enable automatic retries for rate limiting and transient errors (default: true) */
  shouldRetry?: boolean;
  /** Maximum number of retry attempts (default: 3) */
  maxRetries?: number;
  /** Request timeout in milliseconds (default: 30000) */
  timeout?: number;
  /** Serialization format (default: MessagePack for best performance, use Json for debugging) */
  format?: SerializationFormat;
}

/**
 * Rate limit error
 */
export class RateLimitError extends Error {
  constructor(
    public retryAfterSecs: number,
    message?: string,
  ) {
    super(
      message || `Rate limit exceeded. Retry after ${retryAfterSecs} seconds`,
    );
    this.name = "RateLimitError";
  }
}

export interface Query {
  limit?: number;
  offset?: number;
  filter?: Record;
}

export interface BatchOperationResult {
  successful: string[];
  failed: Array<{ id?: string; error: string }>;
}

// ========== Chat Interfaces ==========

export interface CollectionConfig {
  collection_name: string;
  fields?: string[];
  search_options?: any;
}

export interface ChatRequest {
  collections: CollectionConfig[];
  llm_provider: string;
  llm_model?: string;
  message: string;
  system_prompt?: string;
  bypass_ripple?: boolean;
}

export interface CreateChatSessionRequest {
  collections: CollectionConfig[];
  llm_provider: string;
  llm_model?: string;
  system_prompt?: string;
  bypass_ripple?: boolean;
  parent_id?: string;
  branch_point_idx?: number;
  max_context_messages?: number;
}

export interface ChatMessageRequest {
  message: string;
  bypass_ripple?: boolean;
  force_summarize?: boolean;
}

export interface TokenUsage {
  prompt_tokens: number;
  completion_tokens: number;
  total_tokens: number;
}

export interface ChatResponse {
  chat_id: string;
  message_id: string;
  responses: string[];
  context_snippets: any[];
  execution_time_ms: number;
  token_usage?: TokenUsage;
}

export interface ChatSession {
  chat_id: string;
  created_at: string;
  updated_at: string;
  llm_provider: string;
  llm_model: string;
  collections: CollectionConfig[];
  system_prompt?: string;
  title?: string;
  message_count: number;
}

export interface ChatSessionResponse {
  session: Record;
  message_count: number;
}

export interface ListSessionsQuery {
  limit?: number;
  skip?: number;
  sort?: string;
}

export interface ListSessionsResponse {
  sessions: ChatSession[];
  total: number;
  returned: number;
  skip: number;
  limit?: number;
}

export interface GetMessagesQuery {
  limit?: number;
  skip?: number;
  sort?: string;
}

export interface GetMessagesResponse {
  messages: Record[];
  total: number;
  skip: number;
  limit?: number;
  returned: number;
}

export interface UpdateSessionRequest {
  system_prompt?: string;
  llm_model?: string;
  collections?: CollectionConfig[];
  max_context_messages?: number;
}

export enum MergeStrategy {
  Chronological = "Chronological",
  Summarized = "Summarized",
  LatestOnly = "LatestOnly",
}

export interface MergeSessionsRequest {
  source_chat_ids: string[];
  target_chat_id: string;
  merge_strategy: MergeStrategy;
}

export class EkoDBClient {
  private baseURL: string;
  private apiKey: string;
  private token: string | null = null;
  private shouldRetry: boolean;
  private maxRetries: number;
  private timeout: number;
  private format: SerializationFormat;
  private rateLimitInfo: RateLimitInfo | null = null;

  constructor(config: string | ClientConfig, apiKey?: string) {
    // Support both old (baseURL, apiKey) and new (config object) signatures
    if (typeof config === "string") {
      this.baseURL = config;
      this.apiKey = apiKey!;
      this.shouldRetry = true;
      this.maxRetries = 3;
      this.timeout = 30000;
      this.format = SerializationFormat.MessagePack; // Default to MessagePack for 2-3x performance
    } else {
      this.baseURL = config.baseURL;
      this.apiKey = config.apiKey;
      this.shouldRetry = config.shouldRetry ?? true;
      this.maxRetries = config.maxRetries ?? 3;
      this.timeout = config.timeout ?? 30000;
      this.format = config.format ?? SerializationFormat.MessagePack; // Default to MessagePack for 2-3x performance
    }
  }

  /**
   * Initialize the client by getting an auth token
   */
  async init(): Promise<void> {
    await this.refreshToken();
  }

  /**
   * Get the current rate limit information
   */
  getRateLimitInfo(): RateLimitInfo | null {
    return this.rateLimitInfo;
  }

  /**
   * Check if approaching rate limit (less than 10% remaining)
   */
  isNearRateLimit(): boolean {
    if (!this.rateLimitInfo) return false;
    const threshold = this.rateLimitInfo.limit * 0.1;
    return this.rateLimitInfo.remaining <= threshold;
  }

  /**
   * Refresh the authentication token
   */
  private async refreshToken(): Promise<void> {
    const response = await fetch(`${this.baseURL}/api/auth/token`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ api_key: this.apiKey }),
    });

    if (!response.ok) {
      let errorMsg = `Auth failed with status: ${response.status}`;
      try {
        const errorBody = (await response.json()) as { error?: string };
        if (errorBody.error) {
          errorMsg = errorBody.error;
        }
      } catch {
        // Ignore JSON parse errors, use default message
      }
      throw new Error(errorMsg);
    }

    const result = (await response.json()) as { token: string };
    this.token = result.token;
  }

  /**
   * Extract rate limit information from response headers
   */
  private extractRateLimitInfo(response: Response): void {
    const limit = response.headers.get("x-ratelimit-limit");
    const remaining = response.headers.get("x-ratelimit-remaining");
    const reset = response.headers.get("x-ratelimit-reset");

    if (limit && remaining && reset) {
      this.rateLimitInfo = {
        limit: parseInt(limit, 10),
        remaining: parseInt(remaining, 10),
        reset: parseInt(reset, 10),
      };

      // Log warning if approaching rate limit
      if (this.isNearRateLimit()) {
        console.warn(
          `Approaching rate limit: ${this.rateLimitInfo.remaining}/${this.rateLimitInfo.limit} remaining`,
        );
      }
    }
  }

  /**
   * Sleep for a specified number of seconds
   */
  private sleep(seconds: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, seconds * 1000));
  }

  /**
   * Helper to determine if a path should use JSON
   * Only CRUD operations (insert/update/delete/batch) use MessagePack
   * Everything else uses JSON for compatibility
   */
  private shouldUseJSON(path: string): boolean {
    // ONLY these operations support MessagePack
    const msgpackPaths = [
      "/api/insert/",
      "/api/batch_insert/",
      "/api/update/",
      "/api/batch_update/",
      "/api/delete/",
      "/api/batch_delete/",
    ];

    // Check if path starts with any MessagePack-supported operation
    for (const prefix of msgpackPaths) {
      if (path.startsWith(prefix)) {
        return false; // Use MessagePack
      }
    }

    // Everything else uses JSON
    return true;
  }

  /**
   * Make an HTTP request to the ekoDB API with retry logic
   */
  private async makeRequest<T>(
    method: string,
    path: string,
    data?: any,
    attempt: number = 0,
    forceJson: boolean = false,
  ): Promise<T> {
    if (!this.token) {
      await this.refreshToken();
    }

    // Determine content type and serialization based on path
    // Only CRUD operations support MessagePack, everything else uses JSON
    const shouldForceJson = forceJson || this.shouldUseJSON(path);
    const isMessagePack =
      !shouldForceJson && this.format === SerializationFormat.MessagePack;
    const contentType = isMessagePack
      ? "application/msgpack"
      : "application/json";

    // Note: Modern fetch API automatically handles gzip/deflate decompression
    // when server sends Content-Encoding header. No additional configuration needed.
    const options: RequestInit = {
      method,
      headers: {
        Authorization: `Bearer ${this.token}`,
        "Content-Type": contentType,
        Accept: contentType,
      },
    };

    if (data) {
      options.body = isMessagePack
        ? (encode(data) as any)
        : JSON.stringify(data);
    }

    try {
      const response = await fetch(`${this.baseURL}${path}`, options);

      // Extract rate limit info from successful responses
      if (response.ok) {
        this.extractRateLimitInfo(response);

        // Deserialize based on format
        if (isMessagePack) {
          const buffer = await response.arrayBuffer();
          return decode(new Uint8Array(buffer)) as T;
        } else {
          return response.json() as Promise<T>;
        }
      }

      // Handle rate limiting (429)
      if (response.status === 429) {
        const retryAfter = parseInt(
          response.headers.get("retry-after") || "60",
          10,
        );

        if (this.shouldRetry && attempt < this.maxRetries) {
          console.log(`Rate limited. Retrying after ${retryAfter} seconds...`);
          await this.sleep(retryAfter);
          return this.makeRequest<T>(
            method,
            path,
            data,
            attempt + 1,
            forceJson,
          );
        }

        throw new RateLimitError(retryAfter);
      }

      // Handle unauthorized (401) - try refreshing token
      if (response.status === 401 && attempt === 0) {
        console.log("Authentication failed, refreshing token...");
        await this.refreshToken();
        // Retry with new token
        return this.makeRequest<T>(method, path, data, attempt + 1, forceJson);
      }

      // Handle service unavailable (503)
      if (
        response.status === 503 &&
        this.shouldRetry &&
        attempt < this.maxRetries
      ) {
        const retryDelay = 10;
        console.log(
          `Service unavailable. Retrying after ${retryDelay} seconds...`,
        );
        await this.sleep(retryDelay);
        return this.makeRequest<T>(method, path, data, attempt + 1, forceJson);
      }

      // Handle other errors
      const text = await response.text();
      throw new Error(`Request failed with status ${response.status}: ${text}`);
    } catch (error) {
      // Handle network errors with retry
      if (
        error instanceof TypeError &&
        this.shouldRetry &&
        attempt < this.maxRetries
      ) {
        const retryDelay = 3;
        console.log(`Network error. Retrying after ${retryDelay} seconds...`);
        await this.sleep(retryDelay);
        return this.makeRequest<T>(method, path, data, attempt + 1, forceJson);
      }

      throw error;
    }
  }

  /**
   * Insert a document into a collection
   * @param collection - Collection name
   * @param record - Document to insert
   * @param ttl - Optional TTL: duration string ("1h", "30m"), seconds ("3600"), or ISO8601 timestamp
   */
  async insert(
    collection: string,
    record: Record,
    ttl?: string,
  ): Promise<Record> {
    const data = { ...record };
    if (ttl) {
      data.ttl = ttl;
    }
    return this.makeRequest<Record>("POST", `/api/insert/${collection}`, data);
  }

  /**
   * Find documents in a collection
   *
   * @param collection - Collection name
   * @param query - Query object or QueryBuilder instance
   * @returns Array of matching records
   *
   * @example
   * ```typescript
   * // Using QueryBuilder
   * const results = await client.find("users",
   *   new QueryBuilder()
   *     .eq("status", "active")
   *     .gt("age", 18)
   *     .sortDesc("created_at")
   *     .limit(10)
   * );
   *
   * // Using plain Query object
   * const results = await client.find("users", { limit: 10 });
   * ```
   */
  async find(
    collection: string,
    query: Query | QueryBuilder = {},
  ): Promise<Record[]> {
    const queryObj = query instanceof QueryBuilder ? query.build() : query;
    return this.makeRequest<Record[]>(
      "POST",
      `/api/find/${collection}`,
      queryObj,
    );
  }

  /**
   * Find a document by ID
   */
  async findById(collection: string, id: string): Promise<Record> {
    return this.makeRequest<Record>("GET", `/api/find/${collection}/${id}`);
  }

  /**
   * Update a document
   */
  async update(
    collection: string,
    id: string,
    record: Record,
  ): Promise<Record> {
    return this.makeRequest<Record>(
      "PUT",
      `/api/update/${collection}/${id}`,
      record,
    );
  }

  /**
   * Delete a document
   */
  async delete(collection: string, id: string): Promise<void> {
    await this.makeRequest<void>("DELETE", `/api/delete/${collection}/${id}`);
  }

  /**
   * Batch insert multiple documents
   */
  async batchInsert(
    collection: string,
    records: Record[],
    bypassRipple?: boolean,
  ): Promise<BatchOperationResult> {
    const inserts = records.map((data) => ({
      data,
      bypass_ripple: bypassRipple,
    }));
    return this.makeRequest<BatchOperationResult>(
      "POST",
      `/api/batch/insert/${collection}`,
      { inserts },
    );
  }

  /**
   * Batch update multiple documents
   */
  async batchUpdate(
    collection: string,
    updates: Array<{ id: string; data: Record; bypassRipple?: boolean }>,
  ): Promise<BatchOperationResult> {
    const formattedUpdates = updates.map((u) => ({
      id: u.id,
      data: u.data,
      bypass_ripple: u.bypassRipple,
    }));
    return this.makeRequest<BatchOperationResult>(
      "PUT",
      `/api/batch/update/${collection}`,
      { updates: formattedUpdates },
    );
  }

  /**
   * Batch delete multiple documents
   */
  async batchDelete(
    collection: string,
    ids: string[],
    bypassRipple?: boolean,
  ): Promise<BatchOperationResult> {
    const deletes = ids.map((id) => ({
      id: id,
      bypass_ripple: bypassRipple,
    }));
    return this.makeRequest<BatchOperationResult>(
      "DELETE",
      `/api/batch/delete/${collection}`,
      { deletes },
    );
  }

  /**
   * Set a key-value pair with optional TTL
   * @param key - The key to set
   * @param value - The value to store
   * @param ttl - Optional TTL in seconds
   */
  async kvSet(key: string, value: any, ttl?: number): Promise<void> {
    const body: any = { value };
    if (ttl !== undefined) {
      body.ttl = ttl;
    }
    await this.makeRequest<void>(
      "POST",
      `/api/kv/set/${encodeURIComponent(key)}`,
      body,
      0,
      true, // Force JSON for KV operations
    );
  }

  /**
   * Get a value by key
   */
  async kvGet(key: string): Promise<any> {
    const result = await this.makeRequest<{ value: any }>(
      "GET",
      `/api/kv/get/${encodeURIComponent(key)}`,
      undefined,
      0,
      true, // Force JSON for KV operations
    );
    return result.value;
  }

  /**
   * Delete a key
   */
  async kvDelete(key: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/kv/delete/${encodeURIComponent(key)}`,
      undefined,
      0,
      true, // Force JSON for KV operations
    );
  }

  /**
   * Check if a key exists
   * @param key - The key to check
   * @returns true if the key exists, false otherwise
   */
  async kvExists(key: string): Promise<boolean> {
    try {
      const result = await this.kvGet(key);
      return result !== null && result !== undefined;
    } catch {
      return false;
    }
  }

  /**
   * Query/find KV entries with pattern matching
   * @param options - Query options including pattern and include_expired
   * @returns Array of matching records
   */
  async kvFind(options?: {
    pattern?: string;
    include_expired?: boolean;
  }): Promise<any[]> {
    const result = await this.makeRequest<any[]>(
      "POST",
      "/api/kv/find",
      options || {},
      0,
      true, // Force JSON for KV operations
    );
    return result;
  }

  /**
   * Alias for kvFind - query KV store with pattern
   */
  async kvQuery(options?: {
    pattern?: string;
    include_expired?: boolean;
  }): Promise<any[]> {
    return this.kvFind(options);
  }

  // ============================================================================
  // Transaction Operations
  // ============================================================================

  /**
   * Begin a new transaction
   * @param isolationLevel - Transaction isolation level (default: "ReadCommitted")
   * @returns Transaction ID
   */
  async beginTransaction(
    isolationLevel: string = "ReadCommitted",
  ): Promise<string> {
    const result = await this.makeRequest<{ transaction_id: string }>(
      "POST",
      "/api/transactions",
      { isolation_level: isolationLevel },
      0,
      true,
    );
    return result.transaction_id;
  }

  /**
   * Get transaction status
   * @param transactionId - The transaction ID
   * @returns Transaction status object
   */
  async getTransactionStatus(
    transactionId: string,
  ): Promise<{ state: string; operations_count: number }> {
    return this.makeRequest<{ state: string; operations_count: number }>(
      "GET",
      `/api/transactions/${encodeURIComponent(transactionId)}`,
      undefined,
      0,
      true,
    );
  }

  /**
   * Commit a transaction
   * @param transactionId - The transaction ID to commit
   */
  async commitTransaction(transactionId: string): Promise<void> {
    await this.makeRequest<void>(
      "POST",
      `/api/transactions/${encodeURIComponent(transactionId)}/commit`,
      undefined,
      0,
      true,
    );
  }

  /**
   * Rollback a transaction
   * @param transactionId - The transaction ID to rollback
   */
  async rollbackTransaction(transactionId: string): Promise<void> {
    await this.makeRequest<void>(
      "POST",
      `/api/transactions/${encodeURIComponent(transactionId)}/rollback`,
      undefined,
      0,
      true,
    );
  }

  /**
   * List all collections
   */
  async listCollections(): Promise<string[]> {
    const result = await this.makeRequest<{ collections: string[] }>(
      "GET",
      "/api/collections",
      undefined,
      0,
      true, // Force JSON for metadata operations
    );
    return result.collections;
  }

  /**
   * Delete a collection
   */
  async deleteCollection(collection: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/collections/${collection}`,
      undefined,
      0,
      true, // Force JSON for metadata operations
    );
  }

  /**
   * Restore a deleted record from trash
   * Records remain in trash for 30 days before permanent deletion
   *
   * @param collection - Collection name
   * @param id - Record ID to restore
   * @returns true if restored successfully
   */
  async restoreRecord(collection: string, id: string): Promise<boolean> {
    const result = await this.makeRequest<{ status: string }>(
      "POST",
      `/api/trash/${collection}/${id}`,
      undefined,
      0,
      true,
    );
    return result.status === "restored";
  }

  /**
   * Restore all deleted records in a collection from trash
   * Records remain in trash for 30 days before permanent deletion
   *
   * @param collection - Collection name
   * @returns Number of records restored
   */
  async restoreCollection(
    collection: string,
  ): Promise<{ recordsRestored: number }> {
    const result = await this.makeRequest<{
      status: string;
      collection: string;
      records_restored: number;
    }>("POST", `/api/trash/${collection}`, undefined, 0, true);
    return { recordsRestored: result.records_restored };
  }

  /**
   * Create a collection with schema
   *
   * @param collection - Collection name
   * @param schema - Schema definition or SchemaBuilder instance
   *
   * @example
   * ```typescript
   * const schema = new SchemaBuilder()
   *   .addField("name", new FieldTypeSchemaBuilder("string").required())
   *   .addField("email", new FieldTypeSchemaBuilder("string").unique())
   *   .addField("age", new FieldTypeSchemaBuilder("number").range(0, 150));
   *
   * await client.createCollection("users", schema);
   * ```
   */
  async createCollection(
    collection: string,
    schema: Schema | SchemaBuilder,
  ): Promise<void> {
    const schemaObj = schema instanceof SchemaBuilder ? schema.build() : schema;
    await this.makeRequest<void>(
      "POST",
      `/api/collections/${collection}`,
      schemaObj,
      0,
      true, // Force JSON for metadata operations
    );
  }

  /**
   * Get collection metadata and schema
   *
   * @param collection - Collection name
   * @returns Collection metadata including schema and analytics
   */
  async getCollection(collection: string): Promise<CollectionMetadata> {
    return this.makeRequest<CollectionMetadata>(
      "GET",
      `/api/collections/${collection}`,
      undefined,
      0,
      true, // Force JSON for metadata operations
    );
  }

  /**
   * Get collection schema
   *
   * @param collection - Collection name
   * @returns The collection schema
   */
  async getSchema(collection: string): Promise<Schema> {
    const metadata = await this.getCollection(collection);
    return metadata.collection;
  }

  /**
   * Search documents in a collection using full-text, vector, or hybrid search
   *
   * @param collection - Collection name
   * @param searchQuery - Search query object or SearchQueryBuilder instance
   * @returns Search response with results and metadata
   *
   * @example
   * ```typescript
   * // Full-text search
   * const results = await client.search("users",
   *   new SearchQueryBuilder("john")
   *     .fields(["name", "email"])
   *     .fuzzy(true)
   *     .limit(10)
   * );
   *
   * // Vector search
   * const results = await client.search("documents",
   *   new SearchQueryBuilder("")
   *     .vector([0.1, 0.2, 0.3, ...])
   *     .vectorK(5)
   * );
   *
   * // Hybrid search
   * const results = await client.search("products",
   *   new SearchQueryBuilder("laptop")
   *     .vector([0.1, 0.2, ...])
   *     .textWeight(0.7)
   *     .vectorWeight(0.3)
   * );
   * ```
   */
  async search(
    collection: string,
    searchQuery: SearchQuery | SearchQueryBuilder,
  ): Promise<SearchResponse> {
    const queryObj =
      searchQuery instanceof SearchQueryBuilder
        ? searchQuery.build()
        : searchQuery;
    return this.makeRequest<SearchResponse>(
      "POST",
      `/api/search/${collection}`,
      queryObj,
      0,
      true, // Force JSON for search operations
    );
  }

  /**
   * Health check - verify the ekoDB server is responding
   */
  async health(): Promise<boolean> {
    try {
      const result = await this.makeRequest<{ status: string }>(
        "GET",
        "/api/health",
        undefined,
        0,
        true,
      );
      return result.status === "ok";
    } catch {
      return false;
    }
  }

  // ========== Chat Methods ==========

  /**
   * Create a new chat session
   */
  async createChatSession(
    request: CreateChatSessionRequest,
  ): Promise<ChatResponse> {
    return this.makeRequest<ChatResponse>(
      "POST",
      "/api/chat",
      request,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Send a message in an existing chat session
   */
  async chatMessage(
    sessionId: string,
    request: ChatMessageRequest,
  ): Promise<ChatResponse> {
    return this.makeRequest<ChatResponse>(
      "POST",
      `/api/chat/${sessionId}/messages`,
      request,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Get a chat session by ID
   */
  async getChatSession(sessionId: string): Promise<ChatSessionResponse> {
    return this.makeRequest<ChatSessionResponse>(
      "GET",
      `/api/chat/${sessionId}`,
      undefined,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * List all chat sessions
   */
  async listChatSessions(
    query?: ListSessionsQuery,
  ): Promise<ListSessionsResponse> {
    const params = new URLSearchParams();
    if (query?.limit) params.append("limit", query.limit.toString());
    if (query?.skip) params.append("skip", query.skip.toString());
    if (query?.sort) params.append("sort", query.sort);

    const queryString = params.toString();
    const path = queryString ? `/api/chat?${queryString}` : "/api/chat";
    return this.makeRequest<ListSessionsResponse>(
      "GET",
      path,
      undefined,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Get messages from a chat session
   */
  async getChatSessionMessages(
    sessionId: string,
    query?: GetMessagesQuery,
  ): Promise<GetMessagesResponse> {
    const params = new URLSearchParams();
    if (query?.limit) params.append("limit", query.limit.toString());
    if (query?.skip) params.append("skip", query.skip.toString());
    if (query?.sort) params.append("sort", query.sort);

    const queryString = params.toString();
    const path = queryString
      ? `/api/chat/${sessionId}/messages?${queryString}`
      : `/api/chat/${sessionId}/messages`;
    return this.makeRequest<GetMessagesResponse>(
      "GET",
      path,
      undefined,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Update a chat session
   */
  async updateChatSession(
    sessionId: string,
    request: UpdateSessionRequest,
  ): Promise<ChatSessionResponse> {
    return this.makeRequest<ChatSessionResponse>(
      "PUT",
      `/api/chat/${sessionId}`,
      request,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Branch a chat session
   */
  async branchChatSession(
    request: CreateChatSessionRequest,
  ): Promise<ChatResponse> {
    return this.makeRequest<ChatResponse>(
      "POST",
      "/api/chat/branch",
      request,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Delete a chat session
   */
  async deleteChatSession(sessionId: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/chat/${sessionId}`,
      undefined,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Regenerate an AI response message
   */
  async regenerateMessage(
    sessionId: string,
    messageId: string,
  ): Promise<ChatResponse> {
    return this.makeRequest<ChatResponse>(
      "POST",
      `/api/chat/${sessionId}/messages/${messageId}/regenerate`,
      undefined,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Update a specific message
   */
  async updateChatMessage(
    sessionId: string,
    messageId: string,
    content: string,
  ): Promise<void> {
    await this.makeRequest<void>(
      "PUT",
      `/api/chat/${sessionId}/messages/${messageId}`,
      { content },
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Delete a specific message
   */
  async deleteChatMessage(sessionId: string, messageId: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/chat/${sessionId}/messages/${messageId}`,
      undefined,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Toggle the "forgotten" status of a message
   */
  async toggleForgottenMessage(
    sessionId: string,
    messageId: string,
    forgotten: boolean,
  ): Promise<void> {
    await this.makeRequest<void>(
      "PATCH",
      `/api/chat/${sessionId}/messages/${messageId}/forgotten`,
      { forgotten },
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Merge multiple chat sessions into one
   */
  async mergeChatSessions(
    request: MergeSessionsRequest,
  ): Promise<ChatSessionResponse> {
    return this.makeRequest<ChatSessionResponse>(
      "POST",
      "/api/chat/merge",
      request,
      0,
      true, // Force JSON for chat operations
    );
  }

  // ========================================================================
  // SCRIPTS API
  // ========================================================================

  /**
   * Save a new script definition
   */
  async saveScript(script: Script): Promise<string> {
    const result = await this.makeRequest<{ id: string }>(
      "POST",
      "/api/functions",
      script,
    );
    return result.id;
  }

  /**
   * Get a script by ID
   */
  async getScript(id: string): Promise<Script> {
    return this.makeRequest<Script>("GET", `/api/functions/${id}`);
  }

  /**
   * List all scripts, optionally filtered by tags
   */
  async listScripts(tags?: string[]): Promise<Script[]> {
    const params = tags ? `?tags=${tags.join(",")}` : "";
    return this.makeRequest<Script[]>("GET", `/api/functions${params}`);
  }

  /**
   * Update an existing script by ID
   */
  async updateScript(id: string, script: Script): Promise<void> {
    await this.makeRequest<void>("PUT", `/api/functions/${id}`, script);
  }

  /**
   * Delete a script by ID
   */
  async deleteScript(id: string): Promise<void> {
    await this.makeRequest<void>("DELETE", `/api/functions/${id}`);
  }

  /**
   * Call a saved script by ID or label
   */
  async callScript(
    idOrLabel: string,
    params?: { [key: string]: any },
  ): Promise<FunctionResult> {
    return this.makeRequest<FunctionResult>(
      "POST",
      `/api/functions/${idOrLabel}`,
      params || {},
    );
  }

  /**
   * Create a WebSocket client
   */
  websocket(wsURL: string): WebSocketClient {
    return new WebSocketClient(wsURL, this.token!);
  }

  // ========== RAG Helper Methods ==========

  /**
   * Generate embeddings for text using ekoDB's native Functions
   *
   * This helper simplifies embedding generation by:
   * 1. Creating a temporary collection with the text
   * 2. Running a Script with FindAll + Embed Functions
   * 3. Extracting and returning the embedding vector
   * 4. Cleaning up temporary resources
   *
   * @param text - The text to generate embeddings for
   * @param model - The embedding model to use (e.g., "text-embedding-3-small")
   * @returns Array of floats representing the embedding vector
   *
   * @example
   * ```typescript
   * const embedding = await client.embed(
   *   "Hello world",
   *   "text-embedding-3-small"
   * );
   * console.log(`Generated ${embedding.length} dimensions`);
   * ```
   */
  async embed(text: string, model: string): Promise<number[]> {
    const tempCollection = `embed_temp_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

    try {
      // Insert temporary record with the text
      await this.insert(tempCollection, { text }, undefined);

      // Create Script with FindAll + Embed Functions
      const tempLabel = `embed_script_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      const script: Script = {
        label: tempLabel,
        name: "Generate Embedding",
        description: "Temporary script for embedding generation",
        version: "1.0",
        parameters: {},
        functions: [
          {
            type: "FindAll",
            collection: tempCollection,
          },
          {
            type: "Embed",
            input_field: "text",
            output_field: "embedding",
            model: model,
          } as any,
        ],
        tags: [],
      };

      // Save and execute the script
      const scriptId = await this.saveScript(script);
      const result = await this.callScript(scriptId, undefined);

      // Clean up
      await this.deleteScript(scriptId).catch(() => {});
      await this.deleteCollection(tempCollection).catch(() => {});

      // Extract embedding from result
      if (result.records && result.records.length > 0) {
        const record = result.records[0];
        if (record.embedding && Array.isArray(record.embedding)) {
          return record.embedding as number[];
        }
      }

      throw new Error("Failed to extract embedding from result");
    } catch (error) {
      // Ensure cleanup even on error
      await this.deleteCollection(tempCollection).catch(() => {});
      throw error;
    }
  }

  /**
   * Perform text search without embeddings
   *
   * Simplified text search with full-text matching, fuzzy search, and stemming.
   *
   * @param collection - Collection name to search
   * @param queryText - Search query text
   * @param limit - Maximum number of results to return
   * @returns Array of matching records
   *
   * @example
   * ```typescript
   * const results = await client.textSearch(
   *   "documents",
   *   "ownership system",
   *   10
   * );
   * ```
   */
  async textSearch(
    collection: string,
    queryText: string,
    limit: number,
  ): Promise<Record[]> {
    const searchQuery: SearchQuery = {
      query: queryText,
      limit,
    };

    const response = await this.search(collection, searchQuery);
    return response.results.map((r) => r.record);
  }

  /**
   * Perform hybrid search combining text and vector search
   *
   * Combines semantic similarity (vector search) with keyword matching (text search)
   * for more accurate and relevant results.
   *
   * @param collection - Collection name to search
   * @param queryText - Search query text
   * @param queryVector - Embedding vector for semantic search
   * @param limit - Maximum number of results to return
   * @returns Array of matching records
   *
   * @example
   * ```typescript
   * const embedding = await client.embed(query, "text-embedding-3-small");
   * const results = await client.hybridSearch(
   *   "documents",
   *   query,
   *   embedding,
   *   5
   * );
   * ```
   */
  async hybridSearch(
    collection: string,
    queryText: string,
    queryVector: number[],
    limit: number,
  ): Promise<Record[]> {
    const searchQuery: SearchQuery = {
      query: queryText,
      vector: queryVector,
      limit,
    };

    const response = await this.search(collection, searchQuery);
    return response.results.map((r) => r.record);
  }

  /**
   * Find all records in a collection with a limit
   *
   * Simplified method to query all documents in a collection.
   *
   * @param collection - Collection name
   * @param limit - Maximum number of records to return
   * @returns Array of records
   *
   * @example
   * ```typescript
   * const allMessages = await client.findAll("messages", 1000);
   * console.log(`Found ${allMessages.length} messages`);
   * ```
   */
  async findAllWithLimit(collection: string, limit: number): Promise<Record[]> {
    const query = new QueryBuilder().limit(limit).build();
    const results = await this.find(collection, query);
    return results;
  }
}

/**
 * WebSocket client for real-time queries
 */
export class WebSocketClient {
  private wsURL: string;
  private token: string;
  private ws: any = null;

  constructor(wsURL: string, token: string) {
    this.wsURL = wsURL;
    this.token = token;
  }

  /**
   * Connect to WebSocket
   */
  private async connect(): Promise<void> {
    if (this.ws) return;

    // Dynamic import for Node.js WebSocket
    const WebSocket = (await import("ws")).default;

    let url = this.wsURL;
    if (!url.endsWith("/api/ws")) {
      url += "/api/ws";
    }

    this.ws = new WebSocket(url, {
      headers: {
        Authorization: `Bearer ${this.token}`,
      },
    });

    return new Promise((resolve, reject) => {
      this.ws.on("open", () => resolve());
      this.ws.on("error", (err: Error) => reject(err));
    });
  }

  /**
   * Find all records in a collection via WebSocket
   */
  async findAll(collection: string): Promise<Record[]> {
    await this.connect();

    const messageId = Date.now().toString();
    const request = {
      type: "FindAll",
      messageId,
      payload: { collection },
    };

    return new Promise((resolve, reject) => {
      this.ws.send(JSON.stringify(request));

      this.ws.once("message", (data: Buffer) => {
        const response = JSON.parse(data.toString());

        if (response.type === "Error") {
          reject(new Error(response.message));
        } else {
          resolve(response.payload?.data || []);
        }
      });

      this.ws.once("error", reject);
    });
  }

  /**
   * Close the WebSocket connection
   */
  close(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }
}
