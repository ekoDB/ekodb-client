/**
 * ekoDB TypeScript Client
 */

import { encode, decode } from "@msgpack/msgpack";
import { QueryBuilder, Query as QueryBuilderQuery } from "./query-builder";
import { SearchQuery, SearchResponse } from "./search";
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

// ========== Operation Options Interfaces ==========

export interface InsertOptions {
  ttl?: string;
  bypassRipple?: boolean;
  transactionId?: string;
  bypassCache?: boolean;
}

export interface UpdateOptions {
  bypassRipple?: boolean;
  transactionId?: string;
  bypassCache?: boolean;
  selectFields?: string[];
  excludeFields?: string[];
}

export interface DeleteOptions {
  bypassRipple?: boolean;
  transactionId?: string;
}

export interface UpsertOptions {
  ttl?: string;
  bypassRipple?: boolean;
  transactionId?: string;
  bypassCache?: boolean;
}

export interface FindOptions {
  filter?: any;
  sort?: any;
  limit?: number;
  skip?: number;
  join?: any;
  bypassRipple?: boolean;
  bypassCache?: boolean;
  selectFields?: string[];
  excludeFields?: string[];
}

export interface BatchInsertOptions {
  bypassRipple?: boolean;
  transactionId?: string;
}

export interface BatchUpdateOptions {
  bypassRipple?: boolean;
  transactionId?: string;
}

export interface BatchDeleteOptions {
  bypassRipple?: boolean;
  transactionId?: string;
}

export interface DistinctValuesOptions {
  /** Optional filter expression (same format as find() filter). */
  filter?: any;
  /** Bypass ripple propagation for this query. */
  bypassRipple?: boolean;
  /** Bypass cache for this query. */
  bypassCache?: boolean;
}

export interface DistinctValuesResponse {
  /** Collection that was queried. */
  collection: string;
  /** Field whose distinct values were returned. */
  field: string;
  /** Unique values, sorted alphabetically. */
  values: any[];
  /** Number of distinct values. */
  count: number;
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

export interface ToolChoice {
  type: "auto" | "none" | "required" | "tool";
  name?: string;
}

export interface ToolConfig {
  enabled: boolean;
  allowed_tools?: string[];
  allowed_collections?: string[];
  max_iterations?: number;
  allow_write_operations?: boolean;
  tool_choice?: ToolChoice;
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
  max_tokens?: number;
  temperature?: number;
  tool_config?: ToolConfig;
}

export interface ChatMessageRequest {
  message: string;
  bypass_ripple?: boolean;
  force_summarize?: boolean;
  max_iterations?: number;
  tool_config?: ToolConfig;
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
  bypass_ripple?: boolean;
  title?: string;
  memory?: any;
}

export enum MergeStrategy {
  Chronological = "Chronological",
  Summarized = "Summarized",
  LatestOnly = "LatestOnly",
  Interleaved = "Interleaved",
}

export interface MergeSessionsRequest {
  source_chat_ids: string[];
  target_chat_id: string;
  merge_strategy: MergeStrategy;
  bypass_ripple?: boolean;
}

/**
 * Available chat models by provider
 */
export interface ChatModels {
  openai: string[];
  anthropic: string[];
  perplexity: string[];
}

/**
 * Request to generate embeddings
 */
export interface EmbedRequest {
  text?: string;
  texts?: string[];
  model?: string;
}

/**
 * Response from embedding generation
 */
export interface EmbedResponse {
  embeddings: number[][];
  model: string;
  dimensions: number;
}

/**
 * Request for stateless raw LLM completion — no session, no history, no RAG.
 */
export interface RawCompletionRequest {
  system_prompt: string;
  message: string;
  provider?: string;
  model?: string;
  max_tokens?: number;
}

/**
 * Response from a raw LLM completion request.
 */
export interface RawCompletionResponse {
  content: string;
}

/**
 * User function definition - reusable sequence of Functions that can be called by Scripts
 */
export interface UserFunction {
  label: string;
  name: string;
  description?: string;
  version?: string;
  parameters: { [key: string]: ParameterDefinition };
  functions: FunctionStageConfig[];
  tags?: string[];
  id?: string;
  created_at?: string;
  updated_at?: string;
}

/**
 * Parameter definition for functions
 */
export interface ParameterDefinition {
  required: boolean;
  default?: any;
  description?: string;
}

/**
 * Function stage configuration for pipelines
 */
export interface FunctionStageConfig {
  type: string;
  [key: string]: any;
}

export class EkoDBClient {
  private baseURL: string;
  private apiKey: string;
  private token: string | null = null;
  private tokenExpiry: number = 0;
  private shouldRetry: boolean;
  private maxRetries: number;
  private format: SerializationFormat;
  private rateLimitInfo: RateLimitInfo | null = null;

  constructor(config: string | ClientConfig, apiKey?: string) {
    // Support both old (baseURL, apiKey) and new (config object) signatures
    if (typeof config === "string") {
      this.baseURL = config;
      this.apiKey = apiKey!;
      this.shouldRetry = true;
      this.maxRetries = 3;
      this.format = SerializationFormat.MessagePack; // Default to MessagePack for 2-3x performance
    } else {
      this.baseURL = config.baseURL;
      this.apiKey = config.apiKey;
      this.shouldRetry = config.shouldRetry ?? true;
      this.maxRetries = config.maxRetries ?? 3;
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
  async refreshToken(): Promise<void> {
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

    // Extract and cache JWT expiry for proactive refresh
    const expiry = this.extractJWTExpiry(result.token);
    this.tokenExpiry = expiry ?? Math.floor(Date.now() / 1000) + 3600; // fallback: 1 hour
  }

  /**
   * Get a valid authentication token.
   *
   * Returns a cached token if it has more than 60s of validity remaining.
   * Otherwise fetches a new one via refreshToken(). This means callers
   * never need to handle token refresh themselves — every getToken() call
   * returns a token that's valid for at least 60 more seconds.
   */
  async getToken(): Promise<string | null> {
    if (this.token) {
      const now = Math.floor(Date.now() / 1000);
      if (now + 60 >= this.tokenExpiry) {
        // Token is about to expire or already expired — refresh proactively
        await this.refreshToken();
      }
    } else {
      // No token yet — fetch one
      await this.refreshToken();
    }
    return this.token;
  }

  /**
   * Clear the cached authentication token and expiry.
   * The next request will trigger a fresh token exchange.
   */
  clearTokenCache(): void {
    this.token = null;
    this.tokenExpiry = 0;
  }

  /**
   * Extract the `exp` claim from a JWT without verifying the signature.
   * Returns the Unix timestamp (seconds) of expiry, or null if parsing fails.
   */
  private extractJWTExpiry(token: string): number | null {
    try {
      const parts = token.split(".");
      if (parts.length !== 3) {
        return null;
      }

      // Convert base64url to standard base64
      let payload = parts[1];
      payload = payload.replace(/-/g, "+").replace(/_/g, "/");

      // Pad to multiple of 4
      const pad = payload.length % 4;
      if (pad) {
        payload += "=".repeat(4 - pad);
      }

      const decoded = atob(payload);
      const claims = JSON.parse(decoded);

      if (typeof claims.exp === "number") {
        return claims.exp;
      }
      return null;
    } catch {
      return null;
    }
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
   * @param options - Optional parameters (ttl, bypassRipple, transactionId, bypassCache)
   */
  async insert(
    collection: string,
    record: Record,
    options?: InsertOptions,
  ): Promise<Record> {
    const data = { ...record };
    if (options?.ttl) {
      data.ttl = options.ttl;
    }

    const params = new URLSearchParams();
    if (options?.bypassRipple !== undefined) {
      params.append("bypass_ripple", String(options.bypassRipple));
    }
    if (options?.transactionId) {
      params.append("transaction_id", options.transactionId);
    }

    const url = params.toString()
      ? `/api/insert/${collection}?${params.toString()}`
      : `/api/insert/${collection}`;

    return this.makeRequest<Record>("POST", url, data);
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
   * Find a document by ID with field projection
   * @param collection - Collection name
   * @param id - Document ID
   * @param selectFields - Fields to include in the result
   * @param excludeFields - Fields to exclude from the result
   */
  async findByIdWithProjection(
    collection: string,
    id: string,
    selectFields?: string[],
    excludeFields?: string[],
  ): Promise<Record> {
    const params = new URLSearchParams();
    if (selectFields?.length) {
      params.append("select_fields", selectFields.join(","));
    }
    if (excludeFields?.length) {
      params.append("exclude_fields", excludeFields.join(","));
    }
    const url = params.toString()
      ? `/api/find/${collection}/${id}?${params.toString()}`
      : `/api/find/${collection}/${id}`;
    return this.makeRequest<Record>("GET", url);
  }

  /**
   * Update a document
   * @param collection - Collection name
   * @param id - Document ID
   * @param record - Update data
   * @param options - Optional parameters (bypassRipple, transactionId, bypassCache, selectFields, excludeFields)
   */
  async update(
    collection: string,
    id: string,
    record: Record,
    options?: UpdateOptions,
  ): Promise<Record> {
    const params = new URLSearchParams();
    if (options?.bypassRipple !== undefined) {
      params.append("bypass_ripple", String(options.bypassRipple));
    }
    if (options?.transactionId) {
      params.append("transaction_id", options.transactionId);
    }

    const url = params.toString()
      ? `/api/update/${collection}/${id}?${params.toString()}`
      : `/api/update/${collection}/${id}`;

    return this.makeRequest<Record>("PUT", url, record);
  }

  /**
   * Apply an atomic field action to a single field of a record.
   *
   * Use this instead of `update()` for safe concurrent modifications like
   * incrementing counters, pushing to arrays, or arithmetic operations.
   *
   * @param collection - Collection name
   * @param id - Record ID
   * @param action - The atomic action: increment, decrement, multiply, divide, modulo,
   *                 push, pop, shift, unshift, remove, append, clear
   * @param field - The field name to apply the action to
   * @param value - The value for the action (omit for pop/shift/clear)
   */
  async updateWithAction(
    collection: string,
    id: string,
    action: string,
    field: string,
    value?: any,
  ): Promise<Record> {
    const url = `/api/update/${collection}/${id}/action/${action}`;
    return this.makeRequest<Record>("PUT", url, {
      field,
      value: value ?? null,
    });
  }

  /**
   * Apply a sequence of atomic field actions to a record in a single request.
   *
   * All actions are applied atomically — the record is fetched once, all actions
   * run in order, and the result is persisted in a single update.
   *
   * @param collection - Collection name
   * @param id - Record ID
   * @param actions - Array of [action, field, value] tuples
   */
  async updateWithActionSequence(
    collection: string,
    id: string,
    actions: [string, string, any][],
  ): Promise<Record> {
    const url = `/api/update/sequence/${collection}/${id}`;
    return this.makeRequest<Record>("PUT", url, actions);
  }

  /**
   * Delete a document
   * @param collection - Collection name
   * @param id - Document ID
   * @param options - Optional parameters (bypassRipple, transactionId)
   */
  async delete(
    collection: string,
    id: string,
    options?: DeleteOptions,
  ): Promise<void> {
    const params = new URLSearchParams();
    if (options?.bypassRipple !== undefined) {
      params.append("bypass_ripple", String(options.bypassRipple));
    }
    if (options?.transactionId) {
      params.append("transaction_id", options.transactionId);
    }

    const url = params.toString()
      ? `/api/delete/${collection}/${id}?${params.toString()}`
      : `/api/delete/${collection}/${id}`;

    await this.makeRequest<void>("DELETE", url);
  }

  /**
   * Batch insert multiple documents
   * @param collection - Collection name
   * @param records - Array of documents to insert
   * @param options - Optional parameters (bypassRipple, transactionId)
   */
  async batchInsert(
    collection: string,
    records: Record[],
    options?: BatchInsertOptions,
  ): Promise<BatchOperationResult> {
    const params = new URLSearchParams();
    if (options?.bypassRipple !== undefined) {
      params.append("bypass_ripple", String(options.bypassRipple));
    }
    if (options?.transactionId) {
      params.append("transaction_id", options.transactionId);
    }

    const inserts = records.map((data) => ({ data }));
    const url = params.toString()
      ? `/api/batch/insert/${collection}?${params.toString()}`
      : `/api/batch/insert/${collection}`;

    return this.makeRequest<BatchOperationResult>("POST", url, { inserts });
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
   * Batch get multiple keys
   * @param keys - Array of keys to retrieve
   * @returns Array of records corresponding to the keys
   */
  async kvBatchGet(keys: string[]): Promise<any[]> {
    const result = await this.makeRequest<any[]>(
      "POST",
      "/api/kv/batch/get",
      { keys },
      0,
      true, // Force JSON for KV operations
    );
    return result;
  }

  /**
   * Batch set multiple key-value pairs.
   * Note: TTL from the first entry with a valid TTL is applied to all entries (server limitation).
   * @param entries - Array of objects with key, value, and optional ttl
   * @returns Array of tuples [key, wasSet]
   */
  async kvBatchSet(
    entries: Array<{ key: string; value: any; ttl?: number }>,
  ): Promise<Array<[string, boolean]>> {
    const keys = entries.map((e) => e.key);
    const values = entries.map((e) => e.value);
    // Server applies a single TTL to all entries - use first entry's TTL if provided
    const ttl = entries[0]?.ttl;

    const result = await this.makeRequest<Array<[string, boolean]>>(
      "POST",
      "/api/kv/batch/set",
      { keys, values, ttl },
      0,
      true, // Force JSON for KV operations
    );
    return result;
  }

  /**
   * Batch delete multiple keys
   * @param keys - Array of keys to delete
   * @returns Array of tuples [key, wasDeleted]
   */
  async kvBatchDelete(keys: string[]): Promise<Array<[string, boolean]>> {
    const result = await this.makeRequest<Array<[string, boolean]>>(
      "DELETE",
      "/api/kv/batch/delete",
      { keys },
      0,
      true, // Force JSON for KV operations
    );
    return result;
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

  // ============================================================================
  // Convenience Methods
  // ============================================================================

  /**
   * Insert or update a record (upsert operation)
   *
   * Attempts to update the record first. If the record doesn't exist (404 error),
   * it will be inserted instead. This provides atomic insert-or-update semantics.
   *
   * @param collection - Collection name
   * @param id - Record ID
   * @param record - Record data to insert or update
   * @param bypassRipple - Optional flag to bypass ripple effects
   * @returns The inserted or updated record
   *
   * @example
   * ```typescript
   * const record = { name: "John Doe", email: "john@example.com" };
   * // Will update if exists, insert if not
   * const result = await client.upsert("users", "user123", record);
   * ```
   */
  /**
   * Upsert a document (insert or update)
   * @param collection - Collection name
   * @param id - Document ID
   * @param record - Document data
   * @param options - Optional parameters (ttl, bypassRipple, transactionId, bypassCache)
   */
  async upsert(
    collection: string,
    id: string,
    record: Record,
    options?: UpsertOptions,
  ): Promise<Record> {
    try {
      // Try update first
      return await this.update(collection, id, record, {
        bypassRipple: options?.bypassRipple,
        transactionId: options?.transactionId,
        bypassCache: options?.bypassCache,
      });
    } catch (error: any) {
      // If not found, insert instead
      if (
        error.message?.includes("404") ||
        error.message?.includes("Not found")
      ) {
        return await this.insert(collection, record, {
          ttl: options?.ttl,
          bypassRipple: options?.bypassRipple,
          transactionId: options?.transactionId,
          bypassCache: options?.bypassCache,
        });
      }
      throw error;
    }
  }

  /**
   * Find a single record by field value
   *
   * Convenience method for finding one record matching a specific field value.
   * Returns null if no record matches, or the first matching record.
   *
   * @param collection - Collection name
   * @param field - Field name to search
   * @param value - Value to match
   * @returns The matching record or null if not found
   *
   * @example
   * ```typescript
   * // Find user by email
   * const user = await client.findOne("users", "email", "john@example.com");
   * if (user) {
   *   console.log("Found user:", user);
   * }
   * ```
   */
  async findOne(
    collection: string,
    field: string,
    value: any,
  ): Promise<Record | null> {
    const query = new QueryBuilder().eq(field, value).limit(1).build();
    const results = await this.find(collection, query);
    return results.length > 0 ? results[0] : null;
  }

  /**
   * Check if a record exists by ID
   *
   * This is more efficient than fetching the record when you only need to check existence.
   *
   * @param collection - Collection name
   * @param id - Record ID to check
   * @returns true if the record exists, false if it doesn't
   *
   * @example
   * ```typescript
   * if (await client.exists("users", "user123")) {
   *   console.log("User exists");
   * } else {
   *   console.log("User not found");
   * }
   * ```
   */
  async exists(collection: string, id: string): Promise<boolean> {
    try {
      await this.findById(collection, id);
      return true;
    } catch (error: any) {
      if (
        error.message?.includes("404") ||
        error.message?.includes("Not found")
      ) {
        return false;
      }
      throw error;
    }
  }

  /**
   * Paginate through records
   *
   * Convenience method for pagination with page numbers (1-indexed).
   *
   * @param collection - Collection name
   * @param page - Page number (1-indexed, i.e., first page is 1)
   * @param pageSize - Number of records per page
   * @returns Array of records for the requested page
   *
   * @example
   * ```typescript
   * // Get page 2 with 10 records per page
   * const records = await client.paginate("users", 2, 10);
   * ```
   */
  async paginate(
    collection: string,
    page: number,
    pageSize: number,
  ): Promise<Record[]> {
    // Page 1 = skip 0, Page 2 = skip pageSize, etc.
    const skip = page > 0 ? (page - 1) * pageSize : 0;
    const query: QueryBuilderQuery = {
      limit: pageSize,
      skip: skip,
    };
    return this.find(collection, query);
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
    query: SearchQuery,
  ): Promise<SearchResponse> {
    // Ensure all parameters from SearchQuery are sent to server
    return this.makeRequest<SearchResponse>(
      "POST",
      `/api/search/${collection}`,
      query,
      0,
      true, // Force JSON for search operations
    );
  }

  /**
   * Get distinct (unique) values for a field across all records in a collection.
   *
   * Results are deduplicated and sorted alphabetically. Supports an optional filter
   * to restrict which records are examined.
   *
   * @param collection - Collection name
   * @param field - Field to get distinct values for
   * @param options - Optional filter and bypass flags
   *
   * @example
   * // All distinct statuses
   * const resp = await client.distinctValues("orders", "status");
   * console.log(resp.values); // ["active", "cancelled", "shipped"]
   *
   * // Only statuses for US orders
   * const resp = await client.distinctValues("orders", "status", {
   *   filter: { type: "Condition", content: { field: "region", operator: "Eq", value: "us" } }
   * });
   */
  async distinctValues(
    collection: string,
    field: string,
    options: DistinctValuesOptions = {},
  ): Promise<DistinctValuesResponse> {
    const body: {
      filter?: any;
      bypass_ripple?: boolean;
      bypass_cache?: boolean;
    } = {};
    if (options.filter !== undefined) body.filter = options.filter;
    if (options.bypassRipple !== undefined)
      body.bypass_ripple = options.bypassRipple;
    if (options.bypassCache !== undefined)
      body.bypass_cache = options.bypassCache;

    return this.makeRequest<DistinctValuesResponse>(
      "POST",
      `/api/distinct/${collection}/${field}`,
      body,
      0,
      true, // Force JSON
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
   * Execute a tool via ekoDB's server-side tool pipeline.
   *
   * Calls POST /api/chat/tools/execute which goes through the same
   * execute_tool function as the LLM tool-calling loop — with all
   * collection filtering, permission enforcement, and internal collection
   * blocking. No LLM round-trip.
   *
   * @returns The tool result if executed, or null if the server doesn't
   * support the endpoint (older ekoDB versions).
   */
  async executeTool(
    toolName: string,
    params: { [key: string]: any },
    chatId?: string,
  ): Promise<{ [key: string]: any } | null> {
    const body: { [key: string]: any } = { tool: toolName, params };
    if (chatId) {
      body.chat_id = chatId;
    }

    try {
      const result = await this.makeRequest<{ [key: string]: any }>(
        "POST",
        "/api/chat/tools/execute",
        body,
        0,
        true, // Force JSON for chat operations
      );

      if (result.success) {
        return result.result;
      } else {
        throw new Error(result.error || "tool execution failed");
      }
    } catch (err: any) {
      // Server doesn't have the endpoint (404) or route mismatch (405)
      if (err?.message?.includes("404") || err?.message?.includes("405") || err?.status === 404 || err?.status === 405) {
        return null;
      }
      throw err;
    }
  }

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
   * Stateless raw LLM completion — no session, no history, no RAG.
   *
   * Sends a system prompt and user message directly to the LLM via ekoDB
   * and returns the raw text response without any context injection or
   * conversation management. Use this for structured-output tasks such as
   * planning where the response must be parsed programmatically.
   *
   * @example
   * const resp = await client.rawCompletion({
   *   system_prompt: "You are a helpful assistant.",
   *   message: "Summarize this in JSON.",
   *   max_tokens: 2048,
   * });
   * console.log(resp.content);
   */
  async rawCompletion(
    request: RawCompletionRequest,
  ): Promise<RawCompletionResponse> {
    return this.makeRequest<RawCompletionResponse>(
      "POST",
      "/api/chat/complete",
      request,
      0,
      true, // Force JSON
    );
  }

  /**
   * Stateless raw LLM completion via SSE streaming.
   *
   * Same as rawCompletion() but uses Server-Sent Events to keep the
   * connection alive. Preferred for deployed instances where reverse proxies
   * may kill idle HTTP connections before the LLM responds.
   */
  async rawCompletionStream(
    request: RawCompletionRequest,
  ): Promise<RawCompletionResponse> {
    let token = await this.getToken();
    const url = `${this.baseURL}/api/chat/complete/stream`;

    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Accept: "text/event-stream",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      const body = await response.text();
      throw new Error(
        `SSE raw completion failed (${response.status}): ${body}`,
      );
    }

    const body = await response.text();
    let content = "";
    let lastError: string | null = null;

    for (const line of body.split("\n")) {
      if (line.startsWith("data:")) {
        const dataStr = line.slice(5).trim();
        if (!dataStr) continue;
        try {
          const eventData = JSON.parse(dataStr);
          if (eventData.token) content += eventData.token;
          if (eventData.content) content = eventData.content;
          if (eventData.error) lastError = eventData.error;
        } catch {
          // skip malformed SSE data
        }
      }
    }

    if (lastError) throw new Error(lastError);
    return { content };
  }

  /**
   * Stateless raw LLM completion via SSE streaming with token-level progress.
   *
   * Same as rawCompletionStream() but invokes `onToken` with each token as it
   * arrives, allowing callers to show real-time progress.
   */
  async rawCompletionStreamWithProgress(
    request: RawCompletionRequest,
    onToken: (token: string) => void,
  ): Promise<RawCompletionResponse> {
    let token = await this.getToken();
    const url = `${this.baseURL}/api/chat/complete/stream`;

    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Accept: "text/event-stream",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      const body = await response.text();
      throw new Error(
        `SSE raw completion failed (${response.status}): ${body}`,
      );
    }

    const body = await response.text();
    let content = "";
    let lastError: string | null = null;

    for (const line of body.split("\n")) {
      if (line.startsWith("data:")) {
        const dataStr = line.slice(5).trim();
        if (!dataStr) continue;
        try {
          const eventData = JSON.parse(dataStr);
          if (eventData.token) {
            content += eventData.token;
            onToken(eventData.token);
          }
          if (eventData.content) content = eventData.content;
          if (eventData.error) lastError = eventData.error;
        } catch {
          // skip malformed SSE data
        }
      }
    }

    if (lastError) throw new Error(lastError);
    return { content };
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
   * Send a message in an existing chat session via SSE streaming.
   *
   * Returns an EventStream that emits ChatStreamEvent objects as they arrive:
   * - `{ type: "chunk", content: "..." }` for each token
   * - `{ type: "end", messageId, executionTimeMs, tokenUsage?, contextWindow? }` when complete
   * - `{ type: "error", error: "..." }` on failure
   *
   * Preferred over chatMessage() for long-running responses where reverse
   * proxies may kill idle HTTP connections before the LLM responds.
   */
  chatMessageStream(
    chatId: string,
    request: ChatMessageRequest,
  ): EventStream<ChatStreamEvent> {
    const stream = new EventStream<ChatStreamEvent>();

    (async () => {
      try {
        let token = this.getToken();
        if (!token) {
          await this.refreshToken();
          token = this.getToken();
        }
        const url = `${this.baseURL}/api/chat/${chatId}/messages/stream`;

        const response = await fetch(url, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Accept: "text/event-stream",
            Authorization: `Bearer ${token}`,
          },
          body: JSON.stringify(request),
        });

        if (!response.ok) {
          const body = await response.text();
          stream.emit("event", {
            type: "error",
            error: `SSE chat message stream failed (${response.status}): ${body}`,
          } as ChatStreamEvent);
          stream.close();
          return;
        }

        const body = await response.text();
        for (const line of body.split("\n")) {
          if (!line.startsWith("data:")) continue;
          const dataStr = line.slice(5).trim();
          if (!dataStr) continue;
          try {
            const eventData = JSON.parse(dataStr);
            if (eventData.error) {
              stream.emit("event", {
                type: "error",
                error: eventData.error,
              } as ChatStreamEvent);
            } else if (eventData.content && eventData.message_id) {
              // Done event — has full content + message_id
              stream.emit("event", {
                type: "end",
                messageId: eventData.message_id,
                executionTimeMs: eventData.execution_time_ms ?? 0,
                tokenUsage: eventData.token_usage,
                contextWindow: eventData.context_window,
              } as ChatStreamEvent);
            } else if (eventData.token) {
              stream.emit("event", {
                type: "chunk",
                content: eventData.token,
              } as ChatStreamEvent);
            }
          } catch {
            // skip malformed SSE data
          }
        }
        stream.close();
      } catch (err: any) {
        stream.emit("event", {
          type: "error",
          error: err.message ?? String(err),
        } as ChatStreamEvent);
        stream.close();
      }
    })();

    return stream;
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

  /**
   * Get all available chat models from all providers
   * @returns ChatModels object with models organized by provider
   */
  async getChatModels(): Promise<ChatModels> {
    return this.makeRequest<ChatModels>(
      "GET",
      "/api/chat_models",
      undefined,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Get all built-in server-side chat tool definitions.
   * Returns a list of tool objects with name, description, and parameters fields.
   * Used by planning agents to discover available tools dynamically.
   */
  async getChatTools(): Promise<object[]> {
    return this.makeRequest<object[]>(
      "GET",
      "/api/chat/tools",
      undefined,
      0,
      true, // Force JSON
    );
  }

  /**
   * Get available models for a specific provider
   * @param provider - Provider name (e.g., "openai", "anthropic", "perplexity")
   * @returns Array of model names for the provider
   */
  async getChatModel(provider: string): Promise<string[]> {
    return this.makeRequest<string[]>(
      "GET",
      `/api/chat_models/${encodeURIComponent(provider)}`,
      undefined,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Get a specific chat message by ID
   * @param sessionId - Chat session ID
   * @param messageId - Message ID
   * @returns The chat message record
   */
  async getChatMessage(sessionId: string, messageId: string): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      `/api/chat/${sessionId}/messages/${messageId}`,
      undefined,
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

  // ========================================================================
  // USER FUNCTIONS API
  // ========================================================================

  /**
   * Save a new user function
   * @param userFunction - The user function definition
   * @returns The ID of the created user function
   */
  async saveUserFunction(userFunction: UserFunction): Promise<string> {
    const result = await this.makeRequest<{ id: string }>(
      "POST",
      "/api/functions",
      userFunction,
      0,
      true, // Force JSON
    );
    return result.id;
  }

  /**
   * Get a user function by label
   * @param label - The user function label
   * @returns The user function definition
   */
  async getUserFunction(label: string): Promise<UserFunction> {
    return this.makeRequest<UserFunction>(
      "GET",
      `/api/functions/${encodeURIComponent(label)}`,
      undefined,
      0,
      true, // Force JSON
    );
  }

  /**
   * List all user functions, optionally filtered by tags
   * @param tags - Optional array of tags to filter by
   * @returns Array of user functions
   */
  async listUserFunctions(tags?: string[]): Promise<UserFunction[]> {
    const params = tags ? `?tags=${tags.join(",")}` : "";
    return this.makeRequest<UserFunction[]>(
      "GET",
      `/api/functions${params}`,
      undefined,
      0,
      true, // Force JSON
    );
  }

  /**
   * Update an existing user function by label
   * @param label - The user function label
   * @param userFunction - The updated user function definition
   */
  async updateUserFunction(
    label: string,
    userFunction: UserFunction,
  ): Promise<void> {
    await this.makeRequest<void>(
      "PUT",
      `/api/functions/${encodeURIComponent(label)}`,
      userFunction,
      0,
      true, // Force JSON
    );
  }

  /**
   * Delete a user function by label
   * @param label - The user function label
   */
  async deleteUserFunction(label: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/functions/${encodeURIComponent(label)}`,
      undefined,
      0,
      true, // Force JSON
    );
  }

  // ========================================================================
  // GOAL API
  // ========================================================================

  /** Create a new goal */
  async goalCreate(data: Record): Promise<Record> {
    return this.makeRequest<Record>("POST", "/api/chat/goals", data, 0, true);
  }

  /** List all goals */
  async goalList(): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      "/api/chat/goals",
      undefined,
      0,
      true,
    );
  }

  /** Get a goal by ID */
  async goalGet(id: string): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      `/api/chat/goals/${encodeURIComponent(id)}`,
      undefined,
      0,
      true,
    );
  }

  /** Update a goal by ID */
  async goalUpdate(id: string, data: Record): Promise<Record> {
    return this.makeRequest<Record>(
      "PUT",
      `/api/chat/goals/${encodeURIComponent(id)}`,
      data,
      0,
      true,
    );
  }

  /** Delete a goal by ID */
  async goalDelete(id: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/chat/goals/${encodeURIComponent(id)}`,
      undefined,
      0,
      true,
    );
  }

  // ── Goal Templates ──

  /** Create a new goal template */
  async goalTemplateCreate(data: Record): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      "/api/chat/goal-templates",
      data,
      0,
      true,
    );
  }

  /** List all goal templates */
  async goalTemplateList(): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      "/api/chat/goal-templates",
      undefined,
      0,
      true,
    );
  }

  /** Get a goal template by ID */
  async goalTemplateGet(id: string): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      `/api/chat/goal-templates/${encodeURIComponent(id)}`,
      undefined,
      0,
      true,
    );
  }

  /** Update a goal template by ID */
  async goalTemplateUpdate(id: string, data: Record): Promise<Record> {
    return this.makeRequest<Record>(
      "PUT",
      `/api/chat/goal-templates/${encodeURIComponent(id)}`,
      data,
      0,
      true,
    );
  }

  /** Delete a goal template by ID */
  async goalTemplateDelete(id: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/chat/goal-templates/${encodeURIComponent(id)}`,
      undefined,
      0,
      true,
    );
  }

  /** Search goals */
  async goalSearch(query: string): Promise<Record> {
    const params = new URLSearchParams({ q: query });
    return this.makeRequest<Record>(
      "GET",
      `/api/chat/goals/search?${params}`,
      undefined,
      0,
      true,
    );
  }

  /** Mark a goal as complete (status -> pending_review) */
  async goalComplete(id: string, data: Record): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/chat/goals/${encodeURIComponent(id)}/complete`,
      data,
      0,
      true,
    );
  }

  /** Approve a goal (status -> in_progress) */
  async goalApprove(id: string): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/chat/goals/${encodeURIComponent(id)}/approve`,
      undefined,
      0,
      true,
    );
  }

  /** Reject a goal (status -> failed) */
  async goalReject(id: string, data: Record): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/chat/goals/${encodeURIComponent(id)}/reject`,
      data,
      0,
      true,
    );
  }

  /** Start a goal step (status -> in_progress) */
  async goalStepStart(id: string, stepIndex: number): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/chat/goals/${encodeURIComponent(id)}/steps/${stepIndex}/start`,
      undefined,
      0,
      true,
    );
  }

  /** Complete a goal step with result */
  async goalStepComplete(
    id: string,
    stepIndex: number,
    data: Record,
  ): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/chat/goals/${encodeURIComponent(id)}/steps/${stepIndex}/complete`,
      data,
      0,
      true,
    );
  }

  /** Fail a goal step with error */
  async goalStepFail(
    id: string,
    stepIndex: number,
    data: Record,
  ): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/chat/goals/${encodeURIComponent(id)}/steps/${stepIndex}/fail`,
      data,
      0,
      true,
    );
  }

  // ========================================================================
  // TASK API
  // ========================================================================

  /** Create a new scheduled task */
  async taskCreate(data: Record): Promise<Record> {
    return this.makeRequest<Record>("POST", "/api/chat/tasks", data, 0, true);
  }

  /** List all scheduled tasks */
  async taskList(): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      "/api/chat/tasks",
      undefined,
      0,
      true,
    );
  }

  /** Get a task by ID */
  async taskGet(id: string): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      `/api/chat/tasks/${encodeURIComponent(id)}`,
      undefined,
      0,
      true,
    );
  }

  /** Update a task by ID */
  async taskUpdate(id: string, data: Record): Promise<Record> {
    return this.makeRequest<Record>(
      "PUT",
      `/api/chat/tasks/${encodeURIComponent(id)}`,
      data,
      0,
      true,
    );
  }

  /** Delete a task by ID */
  async taskDelete(id: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/chat/tasks/${encodeURIComponent(id)}`,
      undefined,
      0,
      true,
    );
  }

  /** Get tasks that are due at the given time */
  async taskDue(now: string): Promise<Record> {
    const params = new URLSearchParams({ now });
    return this.makeRequest<Record>(
      "GET",
      `/api/chat/tasks/due?${params}`,
      undefined,
      0,
      true,
    );
  }

  /** Start a task (status -> running) */
  async taskStart(id: string): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/chat/tasks/${encodeURIComponent(id)}/start`,
      undefined,
      0,
      true,
    );
  }

  /** Mark a task as succeeded */
  async taskSucceed(id: string, data: Record): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/chat/tasks/${encodeURIComponent(id)}/succeed`,
      data,
      0,
      true,
    );
  }

  /** Mark a task as failed */
  async taskFail(id: string, data: Record): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/chat/tasks/${encodeURIComponent(id)}/fail`,
      data,
      0,
      true,
    );
  }

  /** Pause a task */
  async taskPause(id: string): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/chat/tasks/${encodeURIComponent(id)}/pause`,
      undefined,
      0,
      true,
    );
  }

  /** Resume a paused task */
  async taskResume(id: string, data: Record): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/chat/tasks/${encodeURIComponent(id)}/resume`,
      data,
      0,
      true,
    );
  }

  // ========================================================================
  // AGENT API
  // ========================================================================

  /** Create a new agent */
  async agentCreate(data: Record): Promise<Record> {
    return this.makeRequest<Record>("POST", "/api/chat/agents", data, 0, true);
  }

  /** List all agents */
  async agentList(): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      "/api/chat/agents",
      undefined,
      0,
      true,
    );
  }

  /** Get an agent by ID */
  async agentGet(id: string): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      `/api/chat/agents/${encodeURIComponent(id)}`,
      undefined,
      0,
      true,
    );
  }

  /** Get an agent by name */
  async agentGetByName(name: string): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      `/api/chat/agents/by-name/${encodeURIComponent(name)}`,
      undefined,
      0,
      true,
    );
  }

  /** Update an agent by ID */
  async agentUpdate(id: string, data: Record): Promise<Record> {
    return this.makeRequest<Record>(
      "PUT",
      `/api/chat/agents/${encodeURIComponent(id)}`,
      data,
      0,
      true,
    );
  }

  /** Delete an agent by ID */
  async agentDelete(id: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/chat/agents/${encodeURIComponent(id)}`,
      undefined,
      0,
      true,
    );
  }

  /** Get agents by deployment ID */
  async agentsByDeployment(deploymentId: string): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      `/api/chat/agents/by-deployment/${encodeURIComponent(deploymentId)}`,
      undefined,
      0,
      true,
    );
  }

  // ========================================================================
  // KV DOCUMENT LINKING
  // ========================================================================

  /** Get documents linked to a KV key */
  async kvGetLinks(key: string): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      `/api/kv/links/${encodeURIComponent(key)}`,
      undefined,
      0,
      true,
    );
  }

  /** Link a document to a KV key */
  async kvLink(
    key: string,
    collection: string,
    documentId: string,
  ): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/kv/link`,
      { key, collection, document_id: documentId },
      0,
      true,
    );
  }

  /** Unlink a document from a KV key */
  async kvUnlink(
    key: string,
    collection: string,
    documentId: string,
  ): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/kv/unlink`,
      { key, collection, document_id: documentId },
      0,
      true,
    );
  }

  // ========================================================================
  // SCHEDULE MANAGEMENT
  // ========================================================================

  /** Create a new schedule */
  async createSchedule(data: Record): Promise<Record> {
    return this.makeRequest<Record>("POST", `/api/schedules`, data, 0, true);
  }

  /** List all schedules */
  async listSchedules(): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      `/api/schedules`,
      undefined,
      0,
      true,
    );
  }

  /** Get a schedule by ID */
  async getSchedule(id: string): Promise<Record> {
    return this.makeRequest<Record>(
      "GET",
      `/api/schedules/${encodeURIComponent(id)}`,
      undefined,
      0,
      true,
    );
  }

  /** Update a schedule */
  async updateSchedule(id: string, data: Record): Promise<Record> {
    return this.makeRequest<Record>(
      "PUT",
      `/api/schedules/${encodeURIComponent(id)}`,
      data,
      0,
      true,
    );
  }

  /** Delete a schedule */
  async deleteSchedule(id: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/schedules/${encodeURIComponent(id)}`,
      undefined,
      0,
      true,
    );
  }

  /** Pause a schedule */
  async pauseSchedule(id: string): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/schedules/${encodeURIComponent(id)}/pause`,
      undefined,
      0,
      true,
    );
  }

  /** Resume a schedule */
  async resumeSchedule(id: string): Promise<Record> {
    return this.makeRequest<Record>(
      "POST",
      `/api/schedules/${encodeURIComponent(id)}/resume`,
      undefined,
      0,
      true,
    );
  }

  // ========================================================================
  // COLLECTION UTILITIES
  // ========================================================================

  /**
   * Check if a collection exists
   * @param collection - Collection name to check
   * @returns true if the collection exists, false otherwise
   */
  async collectionExists(collection: string): Promise<boolean> {
    try {
      const collections = await this.listCollections();
      return collections.includes(collection);
    } catch {
      return false;
    }
  }

  /**
   * Count documents in a collection
   * @param collection - Collection name
   * @returns Number of documents in the collection
   */
  async countDocuments(collection: string): Promise<number> {
    const query = new QueryBuilder().limit(100000).build();
    const records = await this.find(collection, query);
    return records.length;
  }

  /**
   * Create a WebSocket client
   */
  websocket(wsURL: string): WebSocketClient {
    return new WebSocketClient(wsURL, this.token!);
  }

  // ========== RAG Helper Methods ==========

  /**
   * Generate embeddings for a single text
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
    const response = await this.embedRequest({ text, model });
    if (response.embeddings.length === 0) {
      throw new Error("No embedding returned");
    }
    return response.embeddings[0];
  }

  /**
   * Generate embeddings for multiple texts in a single batch request
   *
   * @param texts - Array of texts to generate embeddings for
   * @param model - The embedding model to use
   * @returns Array of embedding vectors
   */
  async embedBatch(texts: string[], model: string): Promise<number[][]> {
    const response = await this.embedRequest({ texts, model });
    return response.embeddings;
  }

  /**
   * Internal: make embed API request
   */
  private async embedRequest(request: EmbedRequest): Promise<EmbedResponse> {
    return this.makeRequest<EmbedResponse>(
      "POST",
      "/api/embed",
      request,
      0,
      true, // Force JSON
    );
  }

  /**
   * Perform text search without embeddings
   *
   * Simplified text search with full-text matching, fuzzy search, and stemming.
   *
   * @param collection - Collection name to search
   * @param query - Search query text
   * @param options - Additional search options
   * @returns Search response with results and metadata
   *
   * @example
   * ```typescript
   * const results = await client.textSearch(
   *   "documents",
   *   "ownership system",
   *   {
   *     limit: 10,
   *     select_fields: ["title", "content"],
   *     exclude_fields: ["author"]
   *   }
   * );
   * ```
   */
  async textSearch(
    collection: string,
    query: string,
    options?: Partial<SearchQuery>,
  ): Promise<SearchResponse> {
    const searchQuery: SearchQuery = {
      query,
      ...options,
    };
    return this.search(collection, searchQuery);
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

/** Mutation notification from a subscription. */
export interface MutationNotification {
  collection: string;
  event: string;
  recordIds: string[];
  records?: any;
  timestamp: string;
}

/** A chunk/event from a streaming chat response. */
export type ChatStreamEvent =
  | { type: "chunk"; content: string }
  | {
      type: "end";
      messageId: string;
      tokenUsage?: any;
      toolCallHistory?: any;
      executionTimeMs: number;
      /** Model's context window size in tokens. */
      contextWindow?: number;
    }
  | {
      type: "toolCall";
      chatId: string;
      callId: string;
      toolName: string;
      arguments: any;
    }
  | { type: "error"; error: string };

/** Definition for a client-side tool the LLM can call. */
export interface ClientToolDefinition {
  name: string;
  description: string;
  parameters: any;
}

/** Options for chatSend. */
export interface ChatSendOptions {
  bypassRipple?: boolean;
  clientTools?: ClientToolDefinition[];
  maxIterations?: number;
  confirmTools?: string[];
  excludeTools?: string[];
}

/** Options for subscribe. */
export interface SubscribeOptions {
  filterField?: string;
  filterValue?: string;
}

/** EventEmitter-like interface for subscriptions and chat streams. */
export class EventStream<_T = unknown> {
  private listeners: Map<string, Array<(data: any) => void>> = new Map();
  private _closed = false;

  on(event: string, listener: (data: any) => void): this {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event)!.push(listener);
    return this;
  }

  /** @internal */
  emit(event: string, data?: any): void {
    const handlers = this.listeners.get(event);
    if (handlers) {
      for (const handler of handlers) {
        handler(data);
      }
    }
  }

  get closed(): boolean {
    return this._closed;
  }

  /** @internal */
  close(): void {
    this._closed = true;
    this.emit("close");
  }
}

/**
 * WebSocket client for real-time queries, subscriptions, and chat streaming.
 */
export class WebSocketClient {
  private wsURL: string;
  private token: string;
  private ws: any = null;
  private dispatcherRunning = false;

  // Dispatcher state
  private pendingRequests: Map<
    string,
    { resolve: (value: any) => void; reject: (reason: any) => void }
  > = new Map();
  private subscriptions: Map<string, EventStream<MutationNotification>> =
    new Map();
  private chatStreams: Map<string, EventStream<ChatStreamEvent>> = new Map();
  private registerToolsAck: {
    resolve: (value: any) => void;
    reject: (reason: any) => void;
  } | null = null;

  constructor(wsURL: string, token: string) {
    this.wsURL = wsURL;
    this.token = token;
  }

  private messageCounter = 0;

  private genMessageId(): string {
    const counter = this.messageCounter++;
    return `${Date.now()}-${counter}-${Math.random().toString(36).slice(2, 8)}`;
  }

  /**
   * Connect and start the dispatcher.
   */
  private async ensureConnected(): Promise<void> {
    if (this.ws && this.dispatcherRunning) return;

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

    await new Promise<void>((resolve, reject) => {
      this.ws.on("open", () => resolve());
      this.ws.on("error", (err: Error) => reject(err));
    });

    this.spawnDispatcher();
  }

  private spawnDispatcher(): void {
    if (this.dispatcherRunning) return;
    this.dispatcherRunning = true;

    this.ws.on("message", (data: Buffer) => {
      try {
        const msg = JSON.parse(data.toString());
        this.routeMessage(msg);
      } catch {
        // Ignore malformed messages
      }
    });

    this.ws.on("close", () => {
      this.dispatcherRunning = false;
      // Notify all pending requests
      for (const [, pending] of this.pendingRequests) {
        pending.reject(new Error("WebSocket connection closed"));
      }
      this.pendingRequests.clear();
      // Close all chat streams
      for (const [, stream] of this.chatStreams) {
        stream.emit("event", { type: "error", error: "Connection closed" });
        stream.close();
      }
      this.chatStreams.clear();
      // Close all subscriptions
      for (const [, stream] of this.subscriptions) {
        stream.close();
      }
      this.subscriptions.clear();
      this.ws = null;
    });
  }

  private routeMessage(msg: any): void {
    switch (msg.type) {
      case "Success":
      case "Error": {
        // Try messageId from top-level, then from payload
        const messageId =
          msg.messageId ||
          msg.message_id ||
          msg.payload?.message_id ||
          msg.payload?.messageId;
        let matched = false;
        if (messageId && this.pendingRequests.has(messageId)) {
          const pending = this.pendingRequests.get(messageId)!;
          this.pendingRequests.delete(messageId);
          if (msg.type === "Error") {
            pending.reject(new Error(msg.message || "Unknown error"));
          } else {
            pending.resolve(msg.payload);
          }
          matched = true;
        }
        if (!matched && this.registerToolsAck) {
          const ack = this.registerToolsAck;
          this.registerToolsAck = null;
          if (msg.type === "Error") {
            ack.reject(new Error(msg.message || "Tool registration failed"));
          } else {
            ack.resolve(msg.payload);
          }
          matched = true;
        }
        // Server doesn't echo messageId — if there's exactly one pending
        // request, deliver the response to it (sequential request/response).
        if (!matched && this.pendingRequests.size === 1) {
          const entry = this.pendingRequests.entries().next().value!;
          const key = entry[0];
          const pending = entry[1];
          this.pendingRequests.delete(key);
          if (msg.type === "Error") {
            pending.reject(new Error(msg.message || "Unknown error"));
          } else {
            pending.resolve(msg.payload);
          }
        }
        break;
      }

      case "MutationNotification": {
        const payload = msg.payload;
        const notification: MutationNotification = {
          collection: payload.collection,
          event: payload.event,
          recordIds: payload.record_ids || payload.recordIds || [],
          records: payload.records,
          timestamp: payload.timestamp,
        };
        for (const [collection, stream] of this.subscriptions) {
          if (collection === notification.collection) {
            stream.emit("mutation", notification);
          }
        }
        break;
      }

      case "ChatStreamChunk": {
        const chatId = msg.payload?.chat_id || msg.payload?.chatId;
        const stream = this.chatStreams.get(chatId);
        if (stream) {
          stream.emit("event", {
            type: "chunk",
            content: msg.payload.content,
          } as ChatStreamEvent);
        }
        break;
      }

      case "ChatStreamEnd": {
        const chatId = msg.payload?.chat_id || msg.payload?.chatId;
        const stream = this.chatStreams.get(chatId);
        if (stream) {
          stream.emit("event", {
            type: "end",
            messageId: msg.payload.message_id || msg.payload.messageId || "",
            tokenUsage: msg.payload.token_usage || msg.payload.tokenUsage,
            toolCallHistory:
              msg.payload.tool_call_history || msg.payload.toolCallHistory,
            executionTimeMs:
              msg.payload.execution_time_ms || msg.payload.executionTimeMs || 0,
            contextWindow:
              msg.payload.context_window || msg.payload.contextWindow,
          } as ChatStreamEvent);
          this.chatStreams.delete(chatId);
          stream.close();
        }
        break;
      }

      case "ChatStreamError": {
        const chatId = msg.payload?.chat_id || msg.payload?.chatId;
        const stream = this.chatStreams.get(chatId);
        if (stream) {
          stream.emit("event", {
            type: "error",
            error: msg.payload.error || msg.payload.message || "Unknown error",
          } as ChatStreamEvent);
          this.chatStreams.delete(chatId);
          stream.close();
        }
        break;
      }

      case "ClientToolCall": {
        const chatId = msg.payload?.chat_id || msg.payload?.chatId;
        const stream = this.chatStreams.get(chatId);
        if (stream) {
          stream.emit("event", {
            type: "toolCall",
            chatId,
            callId: msg.payload.call_id || msg.payload.callId,
            toolName: msg.payload.tool_name || msg.payload.toolName,
            arguments: msg.payload.arguments,
          } as ChatStreamEvent);
        }
        break;
      }
    }
  }

  private async sendRequest(request: any): Promise<any> {
    await this.ensureConnected();
    const messageId = request.messageId || request.message_id;

    return new Promise((resolve, reject) => {
      this.pendingRequests.set(messageId, { resolve, reject });
      try {
        this.ws.send(JSON.stringify(request));
      } catch (err) {
        this.pendingRequests.delete(messageId);
        reject(err);
      }
    });
  }

  /**
   * Find all records in a collection via WebSocket.
   */
  async findAll(collection: string): Promise<Record[]> {
    const messageId = this.genMessageId();
    const payload = await this.sendRequest({
      type: "FindAll",
      messageId,
      payload: { collection },
    });
    return payload?.data || [];
  }

  /**
   * Subscribe to mutation notifications on a collection.
   * Returns an EventStream that emits "mutation" events.
   */
  async subscribe(
    collection: string,
    options?: SubscribeOptions,
  ): Promise<EventStream<MutationNotification>> {
    await this.ensureConnected();

    if (this.subscriptions.has(collection)) {
      throw new Error(`Already subscribed to collection "${collection}"`);
    }

    const messageId = this.genMessageId();
    const stream = new EventStream<MutationNotification>();
    this.subscriptions.set(collection, stream);

    const request: any = {
      type: "Subscribe",
      messageId,
      payload: {
        collection,
        ...(options?.filterField && { filter_field: options.filterField }),
        ...(options?.filterValue && { filter_value: options.filterValue }),
      },
    };

    // Send subscribe request and wait for ack
    try {
      await this.sendRequest(request);
    } catch (err) {
      this.subscriptions.delete(collection);
      throw err;
    }
    return stream;
  }

  /**
   * Send a chat message and receive a streaming response.
   * Returns an EventStream that emits "event" with ChatStreamEvent objects.
   */
  async chatSend(
    chatId: string,
    message: string,
    options?: ChatSendOptions,
  ): Promise<EventStream<ChatStreamEvent>> {
    await this.ensureConnected();

    if (this.chatStreams.has(chatId)) {
      throw new Error(`Chat stream already active for chatId "${chatId}"`);
    }

    const stream = new EventStream<ChatStreamEvent>();
    this.chatStreams.set(chatId, stream);

    const request: any = {
      type: "ChatSend",
      payload: {
        chat_id: chatId,
        message,
        ...(options?.bypassRipple != null && {
          bypass_ripple: options.bypassRipple,
        }),
        ...(options?.clientTools && { client_tools: options.clientTools }),
        ...(options?.maxIterations != null && {
          max_iterations: options.maxIterations,
        }),
        ...(options?.confirmTools && { confirm_tools: options.confirmTools }),
        ...(options?.excludeTools && { exclude_tools: options.excludeTools }),
      },
    };

    this.ws.send(JSON.stringify(request));
    return stream;
  }

  /**
   * Register client-side tools for a chat session.
   */
  async registerClientTools(
    chatId: string,
    tools: ClientToolDefinition[],
  ): Promise<void> {
    await this.ensureConnected();

    const request = {
      type: "RegisterClientTools",
      payload: {
        chat_id: chatId,
        tools,
      },
    };

    await new Promise<void>((resolve, reject) => {
      this.registerToolsAck = {
        resolve: () => resolve(),
        reject: (err) => reject(err),
      };
      this.ws.send(JSON.stringify(request));
    });
  }

  /**
   * Send a tool result back to the server during a chat stream.
   */
  async sendToolResult(
    chatId: string,
    callId: string,
    success: boolean,
    result?: any,
    error?: string,
  ): Promise<void> {
    await this.ensureConnected();

    const request = {
      type: "ClientToolResult",
      payload: {
        chat_id: chatId,
        call_id: callId,
        success,
        ...(result !== undefined && { result }),
        ...(error !== undefined && { error }),
      },
    };

    this.ws.send(JSON.stringify(request));
  }

  /**
   * Stateless raw LLM completion via WebSocket.
   *
   * Sends a RawComplete message and waits for the Success response.
   * Preferred over HTTP for deployed instances: the persistent WSS
   * connection is already authenticated and won't be killed by reverse
   * proxy timeouts.
   */
  async rawCompletion(
    request: RawCompletionRequest,
  ): Promise<RawCompletionResponse> {
    await this.ensureConnected();
    const messageId = this.genMessageId();
    const payload = await this.sendRequest({
      type: "RawComplete",
      messageId,
      payload: {
        system_prompt: request.system_prompt,
        message: request.message,
        ...(request.provider && { provider: request.provider }),
        ...(request.model && { model: request.model }),
        ...(request.max_tokens != null && { max_tokens: request.max_tokens }),
      },
    });
    return { content: payload?.data?.content || "" };
  }

  /**
   * Close the WebSocket connection.
   */
  close(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
      this.dispatcherRunning = false;
    }
  }
}
