/**
 * ekoDB TypeScript Client
 */

import { QueryBuilder, Query as QueryBuilderQuery } from "./query-builder";
import { SearchQuery, SearchQueryBuilder, SearchResponse } from "./search";
import { Schema, SchemaBuilder, CollectionMetadata } from "./schema";

export interface Record {
  [key: string]: any;
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
  private rateLimitInfo: RateLimitInfo | null = null;

  constructor(config: string | ClientConfig, apiKey?: string) {
    // Support both old (baseURL, apiKey) and new (config object) signatures
    if (typeof config === "string") {
      this.baseURL = config;
      this.apiKey = apiKey!;
      this.shouldRetry = true;
      this.maxRetries = 3;
      this.timeout = 30000;
    } else {
      this.baseURL = config.baseURL;
      this.apiKey = config.apiKey;
      this.shouldRetry = config.shouldRetry ?? true;
      this.maxRetries = config.maxRetries ?? 3;
      this.timeout = config.timeout ?? 30000;
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
      throw new Error(`Auth failed with status: ${response.status}`);
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
   * Make an HTTP request to the ekoDB API with retry logic
   */
  private async makeRequest<T>(
    method: string,
    path: string,
    data?: any,
    attempt: number = 0,
  ): Promise<T> {
    if (!this.token) {
      await this.refreshToken();
    }

    // Note: Modern fetch API automatically handles gzip/deflate decompression
    // when server sends Content-Encoding header. No additional configuration needed.
    const options: RequestInit = {
      method,
      headers: {
        Authorization: `Bearer ${this.token}`,
        "Content-Type": "application/json",
      },
    };

    if (data) {
      options.body = JSON.stringify(data);
    }

    try {
      const response = await fetch(`${this.baseURL}${path}`, options);

      // Extract rate limit info from successful responses
      if (response.ok) {
        this.extractRateLimitInfo(response);
        return response.json() as Promise<T>;
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
          return this.makeRequest<T>(method, path, data, attempt + 1);
        }

        throw new RateLimitError(retryAfter);
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
        return this.makeRequest<T>(method, path, data, attempt + 1);
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
        return this.makeRequest<T>(method, path, data, attempt + 1);
      }

      throw error;
    }
  }

  /**
   * Insert a document into a collection
   */
  async insert(
    collection: string,
    record: Record,
    ttl?: string,
  ): Promise<Record> {
    const data = { ...record };
    if (ttl) {
      data.ttl_duration = ttl;
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
  async findByID(collection: string, id: string): Promise<Record> {
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
  async batchInsert(collection: string, records: Record[]): Promise<Record[]> {
    const inserts = records.map((data) => ({ data }));
    const result = await this.makeRequest<BatchOperationResult>(
      "POST",
      `/api/batch/insert/${collection}`,
      { inserts },
    );
    return result.successful.map((id) => ({ id: id }));
  }

  /**
   * Batch update multiple documents
   */
  async batchUpdate(
    collection: string,
    updates: Array<{ id: string; data: Record }>,
  ): Promise<Record[]> {
    const result = await this.makeRequest<BatchOperationResult>(
      "PUT",
      `/api/batch/update/${collection}`,
      { updates },
    );
    return result.successful.map((id) => ({ id: id }));
  }

  /**
   * Batch delete multiple documents
   */
  async batchDelete(collection: string, ids: string[]): Promise<number> {
    const deletes = ids.map((id) => ({ id: id }));
    const result = await this.makeRequest<BatchOperationResult>(
      "DELETE",
      `/api/batch/delete/${collection}`,
      { deletes },
    );
    return result.successful.length;
  }

  /**
   * Set a key-value pair
   */
  async kvSet(key: string, value: any): Promise<void> {
    await this.makeRequest<void>(
      "POST",
      `/api/kv/set/${encodeURIComponent(key)}`,
      { value },
    );
  }

  /**
   * Get a value by key
   */
  async kvGet(key: string): Promise<any> {
    const result = await this.makeRequest<{ value: any }>(
      "GET",
      `/api/kv/get/${encodeURIComponent(key)}`,
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
    );
  }

  /**
   * List all collections
   */
  async listCollections(): Promise<string[]> {
    const result = await this.makeRequest<{ collections: string[] }>(
      "GET",
      "/api/collections",
    );
    return result.collections;
  }

  /**
   * Delete a collection
   */
  async deleteCollection(collection: string): Promise<void> {
    await this.makeRequest<void>("DELETE", `/api/collections/${collection}`);
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
    );
  }

  // ========== Chat Methods ==========

  /**
   * Create a new chat session
   */
  async createChatSession(
    request: CreateChatSessionRequest,
  ): Promise<ChatResponse> {
    return this.makeRequest<ChatResponse>("POST", "/api/chat", request);
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
    );
  }

  /**
   * Get a chat session by ID
   */
  async getChatSession(sessionId: string): Promise<ChatSessionResponse> {
    return this.makeRequest<ChatSessionResponse>(
      "GET",
      `/api/chat/${sessionId}`,
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
    return this.makeRequest<ListSessionsResponse>("GET", path);
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
    return this.makeRequest<GetMessagesResponse>("GET", path);
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
    );
  }

  /**
   * Branch a chat session
   */
  async branchChatSession(
    request: CreateChatSessionRequest,
  ): Promise<ChatResponse> {
    return this.makeRequest<ChatResponse>("POST", "/api/chat/branch", request);
  }

  /**
   * Delete a chat session
   */
  async deleteChatSession(sessionId: string): Promise<void> {
    await this.makeRequest<void>("DELETE", `/api/chat/${sessionId}`);
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
    );
  }

  /**
   * Delete a specific message
   */
  async deleteChatMessage(sessionId: string, messageId: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/chat/${sessionId}/messages/${messageId}`,
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
    );
  }

  /**
   * Create a WebSocket client
   */
  websocket(wsURL: string): WebSocketClient {
    return new WebSocketClient(wsURL, this.token!);
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
