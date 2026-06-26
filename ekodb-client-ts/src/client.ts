/**
 * ekoDB TypeScript Client
 */

import { encode, decode } from "@msgpack/msgpack";
import { QueryBuilder, Query } from "./query-builder";
import { SearchQuery, SearchResponse } from "./search";
import { Schema, SchemaBuilder, CollectionMetadata } from "./schema";
import { UserFunction, FunctionResult } from "./functions";

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

// `Query` is the canonical find/query body shape — the server's FindBody —
// re-exported from the query builder so the whole client shares a single type
// (`filter`, `sort`, `limit`, `skip`, `join`, `select_fields`, `exclude_fields`,
// matching the server exactly). `QueryBuilder.build()` returns this same type.
export type { Query };

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
  /**
   * Read within a transaction (read-your-writes). When set, the read is served
   * from the transaction's own view — its uncommitted staged writes, else the
   * committed store — and recorded in the transaction's read set for
   * commit-time conflict detection. Omit for an ordinary committed read.
   */
  transactionId?: string;
}

/**
 * Options for a point read by id. `transactionId` enables read-your-writes
 * within a transaction (see {@link FindOptions.transactionId}).
 */
export interface FindByIdOptions {
  selectFields?: string[];
  excludeFields?: string[];
  bypassRipple?: boolean;
  transactionId?: string;
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
  agent_id?: string;
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
  llm_model?: string;
  client_tools?: ClientToolDefinition[];
  confirm_tools?: string[];
  exclude_tools?: string[];
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
  agent_id?: string;
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
 * Request to compact a chat session's history on demand.
 */
export interface CompactChatRequest {
  keep_recent?: number;
  // No bypass_ripple: compaction writes chat-message records, which the server
  // does not ripple (same convention as all chat-message writes).
}

/**
 * Result of an on-demand chat history compaction.
 */
export interface CompactChatResponse {
  folded: number;
  kept_recent: number;
  summary_chars: number;
  summary_message_id: string | null;
  already_compact: boolean;
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

// UserFunction is defined in functions.ts and re-exported from there.

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

/**
 * Strip trailing slashes from a base URL so path concatenation
 * (`${base}/api/...`) never yields a double-slash path. Uses a linear scan
 * rather than a regex like `/\/+$/`, which CodeQL flags as polynomial-time
 * backtracking on caller-supplied input.
 */
function stripTrailingSlashes(url: string): string {
  let end = url.length;
  while (end > 0 && url.charCodeAt(end - 1) === 47 /* "/" */) {
    end--;
  }
  return end === url.length ? url : url.slice(0, end);
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
      // Strip trailing slashes so `${baseURL}/api/...` never produces a
      // double-slash path (some servers/proxies reject `//api/...`).
      this.baseURL = stripTrailingSlashes(config);
      this.apiKey = apiKey!;
      this.shouldRetry = true;
      this.maxRetries = 3;
      this.format = SerializationFormat.MessagePack; // Default to MessagePack for 2-3x performance
    } else {
      this.baseURL = stripTrailingSlashes(config.baseURL);
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
   * Parse a `Retry-After` header into a non-negative delay in seconds.
   *
   * Per RFC 9110 the value is either delay-seconds (an integer) or an
   * HTTP-date. Anything that doesn't resolve to a finite, non-negative number
   * (missing header, garbage, a past date) falls back to `defaultSecs`.
   */
  private parseRetryAfter(header: string | null, defaultSecs = 60): number {
    if (!header) return defaultSecs;

    // delay-seconds form: a bare integer.
    const secs = Number(header.trim());
    if (Number.isFinite(secs)) return Math.max(0, secs);

    // HTTP-date form: compute the delay from now.
    const dateMs = Date.parse(header);
    if (Number.isFinite(dateMs)) {
      return Math.max(0, (dateMs - Date.now()) / 1000);
    }

    return defaultSecs;
  }

  /**
   * Backoff delay (in seconds) for a 0-indexed retry attempt: a capped
   * exponential schedule (0.2s → 5s) with full jitter, so concurrent clients
   * don't retry in lockstep. Returns a value in [d/2, d].
   */
  private backoffSeconds(attempt: number): number {
    const base = 0.2;
    const max = 5;
    const d = Math.min(base * Math.pow(2, Math.max(0, attempt)), max);
    return d / 2 + Math.random() * (d / 2);
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
        const retryAfter = this.parseRetryAfter(
          response.headers.get("retry-after"),
        );

        if (this.shouldRetry && attempt < this.maxRetries) {
          // Honor the server's Retry-After, but cap it so a hostile/large value
          // can't pin the client for minutes.
          const wait = Math.min(retryAfter, 60);
          console.log(`Rate limited. Retrying after ${wait} seconds...`);
          await this.sleep(wait);
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
        const retryDelay = this.backoffSeconds(attempt);
        console.log(
          `Service unavailable. Retrying after ${retryDelay.toFixed(2)}s...`,
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
        const retryDelay = this.backoffSeconds(attempt);
        console.log(
          `Network error. Retrying after ${retryDelay.toFixed(2)}s...`,
        );
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
      ? `/api/insert/${encodeURIComponent(collection)}?${params.toString()}`
      : `/api/insert/${encodeURIComponent(collection)}`;

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
    options?: { bypassRipple?: boolean; transactionId?: string },
  ): Promise<Record[]> {
    const queryObj = query instanceof QueryBuilder ? query.build() : query;
    // bypass_ripple and transaction_id are query parameters — the same way every
    // other method (insert/update/findById) carries bypass_ripple — not part of
    // the FindBody. Hoist any bypass_ripple carried on the query object (e.g. from
    // QueryBuilder.bypassRipple()) out of the body so it is ALWAYS sent as a query
    // param; an explicit options.bypassRipple wins.
    let body: unknown = queryObj;
    let bypassRipple = options?.bypassRipple;
    if (body && typeof body === "object" && "bypass_ripple" in body) {
      const { bypass_ripple, ...rest } = body as Record & {
        bypass_ripple?: boolean;
      };
      body = rest;
      if (bypassRipple === undefined) bypassRipple = bypass_ripple;
    }
    const params = new URLSearchParams();
    if (options?.transactionId)
      params.append("transaction_id", options.transactionId);
    if (bypassRipple !== undefined)
      params.append("bypass_ripple", String(bypassRipple));
    const qs = params.toString();
    const url = qs
      ? `/api/find/${encodeURIComponent(collection)}?${qs}`
      : `/api/find/${encodeURIComponent(collection)}`;
    return this.makeRequest<Record[]>("POST", url, body);
  }

  /**
   * Find a document by ID.
   * @param options - Optional read options. `transactionId` reads within a
   *   transaction (read-your-writes); see {@link FindByIdOptions}.
   */
  async findById(
    collection: string,
    id: string,
    options?: FindByIdOptions,
  ): Promise<Record> {
    const params = new URLSearchParams();
    if (options?.selectFields?.length) {
      params.append("select_fields", options.selectFields.join(","));
    }
    if (options?.excludeFields?.length) {
      params.append("exclude_fields", options.excludeFields.join(","));
    }
    // bypass_ripple is a GET query param, the same way the non-transactional
    // findById carries it; it rides alongside transaction_id when both are set.
    if (options?.bypassRipple !== undefined) {
      params.append("bypass_ripple", String(options.bypassRipple));
    }
    if (options?.transactionId) {
      params.append("transaction_id", options.transactionId);
    }
    const url = params.toString()
      ? `/api/find/${encodeURIComponent(collection)}/${encodeURIComponent(id)}?${params.toString()}`
      : `/api/find/${encodeURIComponent(collection)}/${encodeURIComponent(id)}`;
    return this.makeRequest<Record>("GET", url);
  }

  /**
   * Find a document by ID with field projection
   * @param collection - Collection name
   * @param id - Document ID
   * @param selectFields - Fields to include in the result
   * @param excludeFields - Fields to exclude from the result
   * @param transactionId - Read within a transaction (read-your-writes)
   */
  async findByIdWithProjection(
    collection: string,
    id: string,
    selectFields?: string[],
    excludeFields?: string[],
    transactionId?: string,
  ): Promise<Record> {
    const params = new URLSearchParams();
    if (selectFields?.length) {
      params.append("select_fields", selectFields.join(","));
    }
    if (excludeFields?.length) {
      params.append("exclude_fields", excludeFields.join(","));
    }
    if (transactionId) {
      params.append("transaction_id", transactionId);
    }
    const url = params.toString()
      ? `/api/find/${encodeURIComponent(collection)}/${encodeURIComponent(id)}?${params.toString()}`
      : `/api/find/${encodeURIComponent(collection)}/${encodeURIComponent(id)}`;
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
      ? `/api/update/${encodeURIComponent(collection)}/${encodeURIComponent(id)}?${params.toString()}`
      : `/api/update/${encodeURIComponent(collection)}/${encodeURIComponent(id)}`;

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
    const url = `/api/update/${encodeURIComponent(collection)}/${encodeURIComponent(id)}/action/${encodeURIComponent(action)}`;
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
    const url = `/api/update/sequence/${encodeURIComponent(collection)}/${encodeURIComponent(id)}`;
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
      ? `/api/delete/${encodeURIComponent(collection)}/${encodeURIComponent(id)}?${params.toString()}`
      : `/api/delete/${encodeURIComponent(collection)}/${encodeURIComponent(id)}`;

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
      ? `/api/batch/insert/${encodeURIComponent(collection)}?${params.toString()}`
      : `/api/batch/insert/${encodeURIComponent(collection)}`;

    return this.makeRequest<BatchOperationResult>("POST", url, { inserts });
  }

  /**
   * Batch update multiple documents
   * @param collection - Collection name
   * @param updates - Array of updates ({ id, data, bypassRipple? })
   * @param options - Optional parameters (bypassRipple, transactionId). When
   *   `transactionId` is set, the batch is staged into that MVCC transaction
   *   (sent as the `transaction_id` query param) instead of committed
   *   immediately — mirrors the single-record update.
   */
  async batchUpdate(
    collection: string,
    updates: Array<{ id: string; data: Record; bypassRipple?: boolean }>,
    options?: BatchUpdateOptions,
  ): Promise<BatchOperationResult> {
    const params = new URLSearchParams();
    if (options?.bypassRipple !== undefined) {
      params.append("bypass_ripple", String(options.bypassRipple));
    }
    if (options?.transactionId) {
      params.append("transaction_id", options.transactionId);
    }

    const formattedUpdates = updates.map((u) => ({
      id: u.id,
      data: u.data,
      bypass_ripple: u.bypassRipple,
    }));
    const url = params.toString()
      ? `/api/batch/update/${encodeURIComponent(collection)}?${params.toString()}`
      : `/api/batch/update/${encodeURIComponent(collection)}`;

    return this.makeRequest<BatchOperationResult>("PUT", url, {
      updates: formattedUpdates,
    });
  }

  /**
   * Batch delete multiple documents
   * @param collection - Collection name
   * @param ids - Document IDs to delete
   * @param bypassRipple - Optional flag to bypass ripple propagation (legacy
   *   positional form, kept for back-compat)
   * @param options - Optional parameters (bypassRipple, transactionId). When
   *   `transactionId` is set, the batch is staged into that MVCC transaction
   *   (sent as the `transaction_id` query param) instead of committed
   *   immediately — mirrors the single-record delete.
   */
  async batchDelete(
    collection: string,
    ids: string[],
    bypassRipple?: boolean,
    options?: BatchDeleteOptions,
  ): Promise<BatchOperationResult> {
    // bypass_ripple is sent per-item in the body (existing behavior). The
    // effective value prefers the explicit positional arg, falling back to the
    // options object so callers can use either form.
    const effectiveBypassRipple =
      bypassRipple !== undefined ? bypassRipple : options?.bypassRipple;

    const params = new URLSearchParams();
    if (options?.transactionId) {
      params.append("transaction_id", options.transactionId);
    }

    const deletes = ids.map((id) => ({
      id: id,
      bypass_ripple: effectiveBypassRipple,
    }));
    const url = params.toString()
      ? `/api/batch/delete/${encodeURIComponent(collection)}?${params.toString()}`
      : `/api/batch/delete/${encodeURIComponent(collection)}`;

    return this.makeRequest<BatchOperationResult>("DELETE", url, { deletes });
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
   * Clear the entire KV store (all keys in the namespace).
   */
  async kvClear(): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      "/api/kv/clear",
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
   * Begin a new transaction.
   *
   * Transactions are buffered: statements issued with this `transactionId`
   * (passed via the `transactionId` option on insert/update/delete/find/…) are
   * staged and applied atomically only at {@link commitTransaction}. They are
   * invisible to everyone else until commit, and visible to this transaction's
   * own reads (read-your-writes) only when those reads also carry the
   * `transactionId`. {@link rollbackTransaction} discards the staged writes.
   * `commitTransaction` may reject with a conflict (HTTP 409) if a record this
   * transaction read or wrote was changed by another committed transaction —
   * retry the transaction in that case.
   *
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
   * Rollback a transaction (discards all staged writes; nothing was applied).
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
   * Create a savepoint within a transaction. A later
   * {@link rollbackToSavepoint} discards everything staged after it.
   */
  async createSavepoint(transactionId: string, name: string): Promise<void> {
    await this.makeRequest<void>(
      "POST",
      `/api/transactions/${encodeURIComponent(transactionId)}/savepoints`,
      { name },
      0,
      true,
    );
  }

  /**
   * Roll the transaction back to a savepoint, discarding writes staged after it.
   */
  async rollbackToSavepoint(
    transactionId: string,
    name: string,
  ): Promise<void> {
    await this.makeRequest<void>(
      "POST",
      `/api/transactions/${encodeURIComponent(transactionId)}/savepoints/${encodeURIComponent(name)}/rollback`,
      undefined,
      0,
      true,
    );
  }

  /**
   * Release (forget) a savepoint. Staged work is unaffected.
   */
  async releaseSavepoint(transactionId: string, name: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/transactions/${encodeURIComponent(transactionId)}/savepoints/${encodeURIComponent(name)}`,
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
    const query: Query = {
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
   * List collections, excluding internal chat/system collections.
   */
  async listUserCollections(): Promise<string[]> {
    const result = await this.makeRequest<{ collections: string[] }>(
      "GET",
      "/api/collections?exclude_internal=true",
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
      `/api/collections/${encodeURIComponent(collection)}`,
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
      `/api/trash/${encodeURIComponent(collection)}/${encodeURIComponent(id)}`,
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
    }>(
      "POST",
      `/api/trash/${encodeURIComponent(collection)}`,
      undefined,
      0,
      true,
    );
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
      `/api/collections/${encodeURIComponent(collection)}`,
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
      `/api/collections/${encodeURIComponent(collection)}`,
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
      `/api/search/${encodeURIComponent(collection)}`,
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
      `/api/distinct/${encodeURIComponent(collection)}/${encodeURIComponent(field)}`,
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
  ): Promise<any | null> {
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
      // Parse status from makeRequest error format: "Request failed with status NNN: ..."
      const message = String(err?.message ?? "");
      const match = message.match(/Request failed with status (\d+):/);
      const status = match ? parseInt(match[1], 10) : undefined;
      if (status === 404 || status === 405) {
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
      `/api/chat/${encodeURIComponent(sessionId)}/messages`,
      request,
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Submit a client tool result for an in-flight SSE chat stream.
   * Unblocks ekoDB's tool loop so it can feed the result to the LLM.
   */
  async submitChatToolResult(
    chatId: string,
    callId: string,
    success: boolean,
    result?: any,
    error?: string,
  ): Promise<void> {
    await this.makeRequest(
      "POST",
      `/api/chat/${encodeURIComponent(chatId)}/tool-result`,
      {
        call_id: callId,
        success,
        ...(result !== undefined && { result }),
        ...(error !== undefined && { error }),
      },
      0,
      true,
    );
  }

  /**
   * Send a client tool keepalive (liveness ping) for an in-flight SSE chat stream.
   *
   * This is NOT a result: it does not unblock the tool loop or feed anything to the
   * LLM. It simply resets the server's per-tool wait deadline (governed by
   * `client_tool_timeout_secs`, default 60s) so that slow user confirmations or
   * long-running client tools don't get the turn timed out before
   * {@link submitChatToolResult} arrives. Send it periodically while a client tool
   * is still working. See ekoDB#530.
   */
  async submitChatToolKeepalive(chatId: string, callId: string): Promise<void> {
    await this.makeRequest(
      "POST",
      `/api/chat/${encodeURIComponent(chatId)}/tool-result`,
      {
        call_id: callId,
        keepalive: true,
      },
      0,
      true,
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
        let token = await this.getToken();
        if (!token) {
          await this.refreshToken();
          token = await this.getToken();
        }
        const url = `${this.baseURL}/api/chat/${encodeURIComponent(chatId)}/messages/stream`;

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

        const emitLine = (line: string) => {
          if (!line.startsWith("data:")) return;
          const dataStr = line.slice(5).trim();
          if (!dataStr) return;
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
        };

        const reader = response.body?.getReader?.();
        if (reader) {
          // True incremental streaming: decode and emit each SSE line as soon as
          // it arrives, rather than buffering the entire response body first.
          const decoder = new TextDecoder();
          let buffer = "";
          for (;;) {
            const { done, value } = await reader.read();
            if (done) break;
            buffer += decoder.decode(value, { stream: true });
            let nl: number;
            while ((nl = buffer.indexOf("\n")) >= 0) {
              emitLine(buffer.slice(0, nl));
              buffer = buffer.slice(nl + 1);
            }
          }
          buffer += decoder.decode();
          if (buffer) emitLine(buffer);
        } else {
          // Fallback for environments/tests without a readable body stream.
          const body = await response.text();
          for (const line of body.split("\n")) emitLine(line);
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
      `/api/chat/${encodeURIComponent(sessionId)}`,
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
      ? `/api/chat/${encodeURIComponent(sessionId)}/messages?${queryString}`
      : `/api/chat/${encodeURIComponent(sessionId)}/messages`;
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
      `/api/chat/${encodeURIComponent(sessionId)}`,
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
      `/api/chat/${encodeURIComponent(sessionId)}`,
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
      `/api/chat/${encodeURIComponent(sessionId)}/messages/${encodeURIComponent(messageId)}/regenerate`,
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
      `/api/chat/${encodeURIComponent(sessionId)}/messages/${encodeURIComponent(messageId)}`,
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
      `/api/chat/${encodeURIComponent(sessionId)}/messages/${encodeURIComponent(messageId)}`,
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
      `/api/chat/${encodeURIComponent(sessionId)}/messages/${encodeURIComponent(messageId)}/forgotten`,
      { forgotten },
      0,
      true, // Force JSON for chat operations
    );
  }

  /**
   * Compact a chat session's history on demand.
   *
   * Folds older messages into a summary while preserving the most recent
   * messages verbatim, reducing context size for long-running sessions.
   *
   * @param chatId - Chat session ID
   * @param keepRecent - Number of recent messages to preserve verbatim (optional)
   * @returns Compaction result with counts and the summary message ID
   */
  async compactChat(
    chatId: string,
    keepRecent?: number,
  ): Promise<CompactChatResponse> {
    const body: CompactChatRequest = {};
    if (keepRecent !== undefined) {
      body.keep_recent = keepRecent;
    }
    return this.makeRequest<CompactChatResponse>(
      "POST",
      `/api/chat/${encodeURIComponent(chatId)}/compact`,
      body,
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
      `/api/chat/${encodeURIComponent(sessionId)}/messages/${encodeURIComponent(messageId)}`,
      undefined,
      0,
      true, // Force JSON for chat operations
    );
  }

  // ========================================================================
  // SCRIPTS API
  // ========================================================================

  /**
   * Save a new function definition
   */
  async saveFunction(script: UserFunction): Promise<string> {
    const result = await this.makeRequest<{ id: string }>(
      "POST",
      "/api/functions",
      script,
    );
    return result.id;
  }

  /**
   * Get a function by ID
   */
  async getFunction(id: string): Promise<UserFunction> {
    return this.makeRequest<UserFunction>(
      "GET",
      `/api/functions/${encodeURIComponent(id)}`,
    );
  }

  /**
   * List all functions, optionally filtered by tags
   */
  async listFunctions(tags?: string[]): Promise<UserFunction[]> {
    // URLSearchParams percent-encodes the value (`&`/`=`/`,`), so a tag
    // containing query-reserved characters can't smuggle extra params.
    const params = tags
      ? `?${new URLSearchParams({ tags: tags.join(",") }).toString()}`
      : "";
    return this.makeRequest<UserFunction[]>("GET", `/api/functions${params}`);
  }

  /**
   * Update an existing function by ID
   */
  async updateFunction(id: string, script: UserFunction): Promise<void> {
    await this.makeRequest<void>(
      "PUT",
      `/api/functions/${encodeURIComponent(id)}`,
      script,
    );
  }

  /**
   * Delete a function by ID
   */
  async deleteFunction(id: string): Promise<void> {
    await this.makeRequest<void>(
      "DELETE",
      `/api/functions/${encodeURIComponent(id)}`,
    );
  }

  /**
   * Call a saved function by ID or label
   */
  async callFunction(
    idOrLabel: string,
    params?: { [key: string]: any },
  ): Promise<FunctionResult> {
    return this.makeRequest<FunctionResult>(
      "POST",
      `/api/functions/${encodeURIComponent(idOrLabel)}`,
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
    // URLSearchParams percent-encodes the value (`&`/`=`/`,`), so a tag
    // containing query-reserved characters can't smuggle extra params.
    const params = tags
      ? `?${new URLSearchParams({ tags: tags.join(",") }).toString()}`
      : "";
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
      `/api/chat/goals/${encodeURIComponent(id)}/steps/${encodeURIComponent(String(stepIndex))}/start`,
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
      `/api/chat/goals/${encodeURIComponent(id)}/steps/${encodeURIComponent(String(stepIndex))}/complete`,
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
      `/api/chat/goals/${encodeURIComponent(id)}/steps/${encodeURIComponent(String(stepIndex))}/fail`,
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
   * Subscribe to collection mutations via SSE (Server-Sent Events).
   *
   * Returns an EventStream that emits MutationNotification events.
   * Use this when WebSocket connections aren't available (e.g. behind
   * reverse proxies that block WS upgrades).
   */
  subscribeSSE(
    collection: string,
    options?: { filterField?: string; filterValue?: string },
  ): EventStream<MutationNotification> {
    const stream = new EventStream<MutationNotification>();

    (async () => {
      try {
        let token = await this.getToken();
        if (!token) {
          await this.refreshToken();
          token = await this.getToken();
        }

        let url = `${this.baseURL}/api/subscribe/${encodeURIComponent(collection)}`;
        const params: string[] = [];
        if (options?.filterField)
          params.push(
            `filter_field=${encodeURIComponent(options.filterField)}`,
          );
        if (options?.filterValue)
          params.push(
            `filter_value=${encodeURIComponent(options.filterValue)}`,
          );
        if (params.length > 0) url += `?${params.join("&")}`;

        const response = await fetch(url, {
          method: "GET",
          headers: {
            Accept: "text/event-stream",
            Authorization: `Bearer ${token}`,
          },
        });

        if (!response.ok) {
          const body = await response.text();
          stream.emit(
            "error",
            `SSE subscribe failed (${response.status}): ${body}`,
          );
          stream.close();
          return;
        }

        const reader = response.body?.getReader();
        if (!reader) {
          stream.emit("error", "SSE subscribe failed: streaming not supported");
          stream.close();
          return;
        }

        const decoder = new TextDecoder("utf-8");
        let buffer = "";
        let eventType = "";
        let dataLines: string[] = [];

        while (!stream.closed) {
          const { value, done } = await reader.read();
          if (done) break;

          buffer += decoder.decode(value, { stream: true });
          const lines = buffer.split("\n");
          buffer = lines.pop() ?? "";

          for (const line of lines) {
            if (line === "") {
              // End of event block
              if (eventType === "mutation" && dataLines.length > 0) {
                try {
                  const payload = JSON.parse(dataLines.join("\n"));
                  stream.emit("event", {
                    collection: payload.collection,
                    event: payload.event,
                    recordIds: payload.record_ids,
                    records: payload.records,
                    timestamp: payload.timestamp,
                  } as MutationNotification);
                } catch {
                  // skip malformed data
                }
              }
              eventType = "";
              dataLines = [];
              continue;
            }
            if (line.startsWith("event: ")) {
              eventType = line.slice(7).trim();
            } else if (line.startsWith("data: ")) {
              dataLines.push(line.slice(6).trim());
            }
          }
        }
        stream.close();
      } catch (err: any) {
        stream.emit("error", err.message ?? String(err));
        stream.close();
      }
    })();

    return stream;
  }

  /**
   * Create a WebSocket client.
   *
   * The token is supplied as a provider bound to this client's
   * {@link getToken}, so every (re)connect re-evaluates (and proactively
   * refreshes) the auth token instead of snapshotting it once. This means a
   * reconnect after a token rotation uses the current token.
   *
   * @param wsURL - The WebSocket URL (e.g. `wss://host`); `/api/ws` is appended if absent.
   * @param options - Optional reconnect/timeout tunables.
   */
  websocket(wsURL: string, options?: WebSocketClientOptions): WebSocketClient {
    return new WebSocketClient(wsURL, () => this.getToken(), options);
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
    return response.results.map((r) => ({
      ...r.record,
      _score: r.score,
    }));
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

/**
 * A token provider: either a static token string, or a (possibly async)
 * function that returns a fresh token. When a function is supplied it is
 * re-invoked on every (re)connect, so a rotated/refreshed token is always
 * used for the new socket instead of a stale snapshot captured once.
 */
export type TokenProvider =
  | string
  | (() => string | null | Promise<string | null>);

/** Tunables for the WebSocket client's reconnect + request-timeout behavior. */
export interface WebSocketClientOptions {
  /**
   * Auto-reconnect after an unexpected socket close/error (not an explicit
   * `close()`/unsubscribe). Defaults to true.
   */
  autoReconnect?: boolean;
  /** Initial backoff delay in ms before the first reconnect attempt. Default 200. */
  reconnectInitialDelayMs?: number;
  /** Maximum backoff delay in ms (the cap for exponential growth). Default 5000. */
  reconnectMaxDelayMs?: number;
  /**
   * Maximum number of consecutive reconnect attempts before giving up.
   * 0 or undefined means unlimited. Default unlimited.
   */
  reconnectMaxAttempts?: number;
  /**
   * Per-request timeout in ms for request/response WS calls. If no response
   * arrives in this window the pending promise rejects. Default 30000.
   */
  requestTimeoutMs?: number;
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
/**
 * In-memory schema cache with TTL for primary_key_alias resolution.
 */
export class SchemaCache {
  private entries: Map<
    string,
    { primaryKeyAlias: string; version: number; cachedAt: number }
  > = new Map();
  private lruOrder: string[] = [];
  private enabled: boolean;
  private maxEntries: number;
  private ttlMs: number;

  constructor(
    options: {
      enabled?: boolean;
      maxEntries?: number;
      ttlSeconds?: number;
    } = {},
  ) {
    this.enabled = options.enabled ?? false;
    this.maxEntries = options.maxEntries ?? 100;
    this.ttlMs = (options.ttlSeconds ?? 300) * 1000;
  }

  isEnabled(): boolean {
    return this.enabled;
  }
  setEnabled(enabled: boolean): void {
    this.enabled = enabled;
  }

  getAlias(collection: string): string | undefined {
    if (!this.enabled) return undefined;
    const entry = this.entries.get(collection);
    if (!entry) return undefined;
    if (Date.now() - entry.cachedAt > this.ttlMs) {
      this.entries.delete(collection);
      this.lruOrder = this.lruOrder.filter((c) => c !== collection);
      return undefined;
    }
    this.touchLRU(collection);
    return entry.primaryKeyAlias;
  }

  insert(collection: string, primaryKeyAlias: string, version: number): void {
    if (!this.enabled) return;
    const isNew = !this.entries.has(collection);
    this.entries.set(collection, {
      primaryKeyAlias,
      version,
      cachedAt: Date.now(),
    });
    if (isNew) {
      this.lruOrder.push(collection);
      while (this.lruOrder.length > this.maxEntries) {
        const oldest = this.lruOrder.shift()!;
        this.entries.delete(oldest);
      }
    } else {
      this.touchLRU(collection);
    }
  }

  invalidate(collection: string): void {
    this.entries.delete(collection);
    this.lruOrder = this.lruOrder.filter((c) => c !== collection);
  }
  invalidateAll(): void {
    this.entries.clear();
    this.lruOrder = [];
  }

  handleSchemaChanged(
    collection: string,
    version: number,
    primaryKeyAlias: string,
  ): void {
    if (!this.enabled) return;
    const existing = this.entries.get(collection);
    if (existing && version <= existing.version) return;
    this.insert(collection, primaryKeyAlias, version);
  }

  get size(): number {
    return this.entries.size;
  }

  private touchLRU(collection: string): void {
    this.lruOrder = this.lruOrder.filter((c) => c !== collection);
    this.lruOrder.push(collection);
  }
}

/**
 * Extract record ID from a record object, trying custom alias, "id", then "_id".
 */
export function extractRecordId(
  record: Record,
  extraCandidates: string[] = [],
): string | undefined {
  for (const key of extraCandidates) {
    const val = record[key];
    if (typeof val === "string") return val;
    // Unwrap only a genuine typed wrapper (both "type" and "value"), matching
    // getValue's rule so a user object like { value: 1 } isn't mistaken for one.
    if (val && typeof val === "object" && "type" in val && "value" in val)
      return String(val.value);
  }
  for (const key of ["id", "_id"]) {
    const val = record[key];
    if (typeof val === "string") return val;
    // Unwrap only a genuine typed wrapper (both "type" and "value"), matching
    // getValue's rule so a user object like { value: 1 } isn't mistaken for one.
    if (val && typeof val === "object" && "type" in val && "value" in val)
      return String(val.value);
  }
  return undefined;
}

export class WebSocketClient {
  private wsURL: string;
  private tokenProvider: () => string | null | Promise<string | null>;
  private ws: any = null;
  private dispatcherRunning = false;
  private schemaCache: SchemaCache | null = null;
  /**
   * Per-connection wire format, set by negotiateFormat() on every (re)connect:
   * true once the server has Welcomed msgpack, so frames are sent/received as
   * binary msgpack; false (JSON text) otherwise, including against an older
   * server that never Welcomes. Keeps the transport fully back-compatible.
   */
  private binary = false;

  // Reconnect config
  private autoReconnect: boolean;
  private reconnectInitialDelayMs: number;
  private reconnectMaxDelayMs: number;
  private reconnectMaxAttempts: number;
  private requestTimeoutMs: number;

  // Reconnect state
  /** Set while close() is in progress so the close handler doesn't reconnect. */
  private closed = false;
  private reconnectAttempts = 0;
  private reconnecting = false;
  private connectPromise: Promise<void> | null = null;

  // Dispatcher state
  private pendingRequests: Map<
    string,
    {
      resolve: (value: any) => void;
      reject: (reason: any) => void;
      timer?: ReturnType<typeof setTimeout>;
    }
  > = new Map();
  private subscriptions: Map<string, EventStream<MutationNotification>> =
    new Map();
  /** Bookkeeping so subscriptions can be replayed on reconnect. */
  private subscriptionParams: Map<string, SubscribeOptions | undefined> =
    new Map();
  private chatStreams: Map<string, EventStream<ChatStreamEvent>> = new Map();
  private registerToolsAck: {
    resolve: (value: any) => void;
    reject: (reason: any) => void;
  } | null = null;

  /**
   * @param wsURL - WebSocket URL; `/api/ws` is appended if absent.
   * @param token - A static token string OR a {@link TokenProvider} function
   *   re-evaluated on every (re)connect (so a refreshed token is used after a drop).
   * @param options - Optional reconnect/timeout tunables.
   */
  constructor(
    wsURL: string,
    token: TokenProvider,
    options: WebSocketClientOptions = {},
  ) {
    // Strip trailing slashes so appending `/api/ws` can't yield `//api/ws`,
    // which warp's exact path match (`api / ws`) would reject.
    this.wsURL = stripTrailingSlashes(wsURL);
    this.tokenProvider = typeof token === "function" ? token : () => token;
    this.autoReconnect = options.autoReconnect ?? true;
    this.reconnectInitialDelayMs = options.reconnectInitialDelayMs ?? 200;
    this.reconnectMaxDelayMs = options.reconnectMaxDelayMs ?? 5000;
    this.reconnectMaxAttempts = options.reconnectMaxAttempts ?? 0;
    this.requestTimeoutMs = options.requestTimeoutMs ?? 30000;
  }

  private messageCounter = 0;

  private genMessageId(): string {
    const counter = this.messageCounter++;
    return `${Date.now()}-${counter}-${Math.random().toString(36).slice(2, 8)}`;
  }

  /**
   * Compute the capped exponential backoff (with jitter) for a reconnect
   * attempt. attempt 0 -> ~initial, growing x2 each time up to the max cap.
   * Jitter is +/-25% to avoid thundering-herd reconnect storms.
   * @internal exposed for testing
   */
  computeBackoff(attempt: number): number {
    const base = Math.min(
      this.reconnectInitialDelayMs * 2 ** attempt,
      this.reconnectMaxDelayMs,
    );
    const jitter = base * 0.25 * (Math.random() * 2 - 1);
    return Math.max(0, Math.round(base + jitter));
  }

  /**
   * Connect and start the dispatcher. Re-evaluates the token provider so the
   * current/refreshed token is used for this socket.
   */
  private async ensureConnected(): Promise<void> {
    if (this.ws && this.dispatcherRunning) return;
    // Coalesce concurrent connect attempts onto a single in-flight promise.
    if (this.connectPromise) return this.connectPromise;
    // Clear the intentional-close flag only for user-initiated connects. During
    // a reconnect cycle this stays untouched so a concurrent close() can't be
    // undone and have the reconnect proceed against the user's intent.
    if (!this.reconnecting) this.closed = false;
    this.connectPromise = this.openSocket().finally(() => {
      this.connectPromise = null;
    });
    return this.connectPromise;
  }

  private async openSocket(): Promise<void> {
    const WebSocket = (await import("ws")).default;

    let url = this.wsURL;
    if (!url.endsWith("/api/ws")) {
      url += "/api/ws";
    }

    // Re-evaluate the token on every (re)connect — never a stale snapshot.
    const token = await this.tokenProvider();
    if (!token) {
      // Fail fast with a clear error instead of sending `Bearer null`, which
      // would surface as a confusing 401 from the server.
      throw new Error(
        "WebSocket auth token is unavailable (the token provider returned null/empty)",
      );
    }

    this.ws = new WebSocket(url, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });

    await new Promise<void>((resolve, reject) => {
      this.ws.on("open", () => resolve());
      this.ws.on("error", (err: Error) => reject(err));
    });

    // Negotiate the wire format before the dispatcher starts so the Welcome is
    // consumed here (not by routeMessage), and before any real frame is sent
    // (resubscribeAll runs only after this resolves).
    await this.negotiateFormat(this.ws);

    this.spawnDispatcher();
  }

  /**
   * Additive capability handshake: offer msgpack and, if the server Welcomes
   * it, switch this connection to binary msgpack frames; otherwise stay on JSON
   * text. The Welcome (a text frame) is read with a one-shot listener and a
   * timeout so an older server that never answers — or answers with an Error —
   * simply leaves the connection on JSON. Best-effort and never throws: JSON
   * always works.
   */
  private async negotiateFormat(socket: any): Promise<void> {
    this.binary = false;
    const welcome = await new Promise<any | null>((resolve) => {
      const onMsg = (data: Buffer) => {
        clearTimeout(timer);
        try {
          resolve(JSON.parse(data.toString()));
        } catch {
          resolve(null);
        }
      };
      // Only caps the wait when no Welcome comes (a silent/old server); the
      // listener resolves immediately when it does arrive. 2s comfortably exceeds
      // the handshake round-trip even on high-latency links.
      const timer = setTimeout(() => {
        socket.off("message", onMsg);
        resolve(null);
      }, 2000);
      socket.once("message", onMsg);
      try {
        socket.send(
          JSON.stringify({
            type: "Hello",
            payload: { formats: ["msgpack", "json"] },
          }),
        );
      } catch {
        clearTimeout(timer);
        socket.off("message", onMsg);
        resolve(null);
      }
    });
    if (
      welcome &&
      welcome.type === "Welcome" &&
      welcome.payload?.format === "msgpack"
    ) {
      this.binary = true;
    }
  }

  /**
   * Send a request object on the active socket using the negotiated format:
   * binary msgpack when the server Welcomed it, JSON text otherwise. The single
   * write point so every request honors the negotiated transport.
   */
  private sendFrame(obj: any): void {
    this.ws.send(this.binary ? encode(obj) : JSON.stringify(obj));
  }

  private spawnDispatcher(): void {
    if (this.dispatcherRunning) return;
    this.dispatcherRunning = true;

    // Capture the socket this dispatcher is bound to. After a reconnect, the old
    // socket may still emit late close/error events; ignore them so they don't
    // tear down the replacement connection.
    const socket = this.ws;

    socket.on("message", (data: Buffer, isBinary: boolean) => {
      if (this.ws !== socket) return;
      try {
        // A binary frame is msgpack (the server only sends binary once it has
        // Welcomed msgpack); a text frame is JSON. Decode by frame type so the
        // routed value is identical regardless of negotiated transport.
        const msg = isBinary
          ? (decode(data) as any)
          : JSON.parse(data.toString());
        this.routeMessage(msg);
      } catch {
        // Ignore malformed messages
      }
    });

    // Both "close" and "error" mean this socket is dead. ws typically emits
    // "error" followed by "close", so route both through one handler and let the
    // identity check dedupe: the first to fire nulls this.ws, the second no-ops.
    const onDown = () => {
      if (this.ws !== socket) return;
      this.handleDisconnect();
    };
    socket.on("close", onDown);
    socket.on("error", onDown);
  }

  /**
   * Reject in-flight requests and tear down the dead socket. If the close was
   * unexpected (not an explicit `close()`) and auto-reconnect is enabled,
   * schedule a reconnect that re-sends the active subscriptions.
   */
  private handleDisconnect(): void {
    this.dispatcherRunning = false;
    this.ws = null;

    // Reject all in-flight pending requests so callers don't hang forever.
    for (const [, pending] of this.pendingRequests) {
      if (pending.timer) clearTimeout(pending.timer);
      pending.reject(new Error("WebSocket connection closed"));
    }
    this.pendingRequests.clear();
    if (this.registerToolsAck) {
      this.registerToolsAck.reject(new Error("WebSocket connection closed"));
      this.registerToolsAck = null;
    }
    // Close all chat streams (they are one-shot; not replayed on reconnect).
    for (const [, stream] of this.chatStreams) {
      stream.emit("event", { type: "error", error: "Connection closed" });
      stream.close();
    }
    this.chatStreams.clear();

    const shouldReconnect =
      this.autoReconnect && !this.closed && this.subscriptionParams.size > 0;

    if (shouldReconnect) {
      this.scheduleReconnect();
    } else {
      // No reconnect: tear down subscriptions too.
      for (const [, stream] of this.subscriptions) {
        stream.close();
      }
      this.subscriptions.clear();
      this.subscriptionParams.clear();
    }
  }

  /**
   * Reconnect with capped exponential backoff + jitter, then re-send the
   * subscribe messages for every active subscription so the SAME EventStream
   * keeps delivering mutations after a transient drop.
   */
  private scheduleReconnect(): void {
    if (this.reconnecting) return;
    this.reconnecting = true;

    const attempt = async (): Promise<void> => {
      // Bail if the client was closed, or if every subscription was torn down
      // (e.g. unsubscribed) while a reconnect was in-flight — reconnect was only
      // opted into because subscriptions existed, so there's nothing to restore.
      if (this.closed || this.subscriptionParams.size === 0) {
        this.reconnecting = false;
        return;
      }
      if (
        this.reconnectMaxAttempts > 0 &&
        this.reconnectAttempts >= this.reconnectMaxAttempts
      ) {
        // Give up: tear down subscriptions and notify consumers.
        this.reconnecting = false;
        for (const [, stream] of this.subscriptions) {
          stream.emit("error", "WebSocket reconnect failed");
          stream.close();
        }
        this.subscriptions.clear();
        this.subscriptionParams.clear();
        return;
      }

      const delay = this.computeBackoff(this.reconnectAttempts);
      this.reconnectAttempts++;
      await new Promise((r) => setTimeout(r, delay));

      // Re-check after the backoff delay: close() or a full unsubscribe may have
      // happened while we were waiting, in which case skip reopening the socket.
      if (this.closed || this.subscriptionParams.size === 0) {
        this.reconnecting = false;
        return;
      }

      try {
        // Route through ensureConnected() so a request-driven connect and this
        // reconnect share one in-flight connectPromise/socket — opening two live
        // sockets would misroute responses.
        await this.ensureConnected();
        // close() may have been called while the connect was in-flight; if so,
        // tear down the freshly-opened socket instead of leaving it orphaned.
        if (this.closed) {
          try {
            this.ws?.close?.();
          } catch {
            /* already closing */
          }
          this.ws = null;
          this.dispatcherRunning = false;
          this.reconnecting = false;
          return;
        }
        // Success — reset backoff and replay subscriptions.
        this.reconnectAttempts = 0;
        this.reconnecting = false;
        await this.resubscribeAll();
      } catch {
        // Connect failed — schedule the next attempt WITHOUT recursive await so
        // a prolonged outage can't build an unbounded promise chain.
        setTimeout(() => void attempt(), 0);
      }
    };

    void attempt();
  }

  /** Re-send Subscribe frames for every tracked subscription after a reconnect. */
  private async resubscribeAll(): Promise<void> {
    for (const [collection, options] of this.subscriptionParams) {
      const stream = this.subscriptions.get(collection);
      if (!stream || stream.closed) continue;
      const messageId = this.genMessageId();
      const request: any = {
        type: "Subscribe",
        messageId,
        payload: {
          collection,
          ...(options?.filterField && { filter_field: options.filterField }),
          ...(options?.filterValue && { filter_value: options.filterValue }),
        },
      };
      try {
        await this.sendRequest(request);
      } catch {
        // If the re-subscribe ack fails, leave it tracked; the next
        // disconnect/reconnect cycle will attempt it again.
      }
    }
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
          matched = this.settlePending(messageId, msg.type === "Error", msg);
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
        // Server doesn't echo messageId at all — if there's exactly one pending
        // request, deliver the response to it (sequential request/response).
        // Only when messageId is absent: a present-but-unmatched id means a late
        // response for an already-settled/timed-out request, which must NOT be
        // misrouted to whatever request happens to still be pending.
        if (!matched && !messageId && this.pendingRequests.size === 1) {
          const key = this.pendingRequests.keys().next().value!;
          this.settlePending(key, msg.type === "Error", msg);
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

      case "SchemaChanged": {
        if (this.schemaCache && msg.payload) {
          this.schemaCache.handleSchemaChanged(
            msg.payload.collection,
            msg.payload.version,
            msg.payload.primary_key_alias,
          );
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
      // Per-request timeout: reject if no response arrives in the window so a
      // dropped/never-answered response can't leave the promise pending forever.
      let timer: ReturnType<typeof setTimeout> | undefined;
      if (this.requestTimeoutMs > 0) {
        timer = setTimeout(() => {
          if (this.pendingRequests.delete(messageId)) {
            reject(
              new Error(
                `WebSocket request "${request.type}" timed out after ${this.requestTimeoutMs}ms`,
              ),
            );
          }
        }, this.requestTimeoutMs);
        // Don't keep the process alive just for this timer.
        (timer as any)?.unref?.();
      }

      this.pendingRequests.set(messageId, { resolve, reject, timer });
      try {
        this.sendFrame(request);
      } catch (err) {
        this.pendingRequests.delete(messageId);
        if (timer) clearTimeout(timer);
        reject(err);
      }
    });
  }

  /** Resolve/reject a pending request, clearing its timeout timer. */
  private settlePending(
    messageId: string,
    isError: boolean,
    msg: any,
  ): boolean {
    const pending = this.pendingRequests.get(messageId);
    if (!pending) return false;
    this.pendingRequests.delete(messageId);
    if (pending.timer) clearTimeout(pending.timer);
    if (isError) {
      pending.reject(new Error(msg.message || "Unknown error"));
    } else {
      pending.resolve(msg.payload);
    }
    return true;
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
    // Track params so the subscription can be replayed on reconnect.
    this.subscriptionParams.set(collection, options);

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
      this.subscriptionParams.delete(collection);
      throw err;
    }
    return stream;
  }

  /**
   * Unsubscribe from a collection's mutation notifications. This is an
   * intentional teardown, so the subscription is NOT replayed on reconnect.
   */
  unsubscribe(collection: string): void {
    const stream = this.subscriptions.get(collection);
    this.subscriptions.delete(collection);
    this.subscriptionParams.delete(collection);
    if (stream && !stream.closed) {
      stream.close();
    }
    // Best-effort: tell the server to stop streaming this collection (the
    // server already handles an Unsubscribe frame). If the socket isn't open
    // the local teardown above suffices, since the server drops subscriptions
    // when the connection closes. A unique messageId is attached so the
    // server's Success ack carries a correlation id: it has no pending request
    // to match, so it is simply ignored — and because the id is present, the
    // single-pending fallback can't misroute it to an unrelated request.
    if (this.ws && this.ws.readyState === 1 /* WebSocket.OPEN */) {
      try {
        this.sendFrame({
          type: "Unsubscribe",
          messageId: this.genMessageId(),
          payload: { collection },
        });
      } catch {
        // Best-effort: the socket can close between the readyState check and the
        // send. Local teardown already happened, so swallow the failure rather
        // than throw out of a void teardown call.
      }
    }
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

    this.sendFrame(request);
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
      this.sendFrame(request);
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

    this.sendFrame(request);
  }

  /**
   * Cancel an in-flight streaming chat. Fire-and-forget: tells the server to
   * stop generating tokens for the given chat.
   */
  async cancelChat(chatId: string): Promise<void> {
    await this.ensureConnected();

    // Attach a unique messageId (same generator as unsubscribe). Any Success ack
    // from the server then carries a correlation id: it has no pending request to
    // match, so it is ignored — and because the id is present, the dispatcher's
    // single-pending fallback can't misroute the ack to an unrelated request.
    const request = {
      type: "CancelChat",
      messageId: this.genMessageId(),
      payload: { chat_id: chatId },
    };

    this.sendFrame(request);
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

  /** Attach a schema cache for automatic invalidation on SchemaChanged events. */
  setSchemaCache(cache: SchemaCache): void {
    this.schemaCache = cache;
  }

  /** Extract record ID using the schema cache's primary_key_alias. */
  extractId(collection: string, record: Record): string | undefined {
    const alias = this.schemaCache?.getAlias(collection);
    return extractRecordId(record, alias ? [alias] : []);
  }

  // =========================================================================
  // WS CRUD Methods — Full Parity with Server
  // =========================================================================

  private async sendCRUD(msgType: string, payload: any): Promise<any> {
    const messageId = this.genMessageId();
    const response = await this.sendRequest({
      type: msgType,
      messageId,
      payload,
    });
    return response?.data ?? response;
  }

  /** Insert a single record via WebSocket. */
  async insert(
    collection: string,
    record: Record,
    bypassRipple?: boolean,
  ): Promise<any> {
    return this.sendCRUD("Insert", {
      collection,
      record,
      ...(bypassRipple !== undefined && { bypass_ripple: bypassRipple }),
    });
  }

  /** Query records via WebSocket. */
  async query(
    collection: string,
    options?: {
      filter?: any;
      sort?: any;
      limit?: number;
      skip?: number;
    },
  ): Promise<any[]> {
    const data = await this.sendCRUD("Query", {
      collection,
      ...options,
    });
    return Array.isArray(data) ? data : [];
  }

  /** Find a record by ID via WebSocket. */
  async findById(collection: string, id: string): Promise<any> {
    return this.sendCRUD("FindById", { collection, id });
  }

  /** Update a record by ID via WebSocket. */
  async update(
    collection: string,
    id: string,
    record: Record,
    bypassRipple?: boolean,
  ): Promise<any> {
    return this.sendCRUD("Update", {
      collection,
      id,
      record,
      ...(bypassRipple !== undefined && { bypass_ripple: bypassRipple }),
    });
  }

  /** Delete a record by ID via WebSocket. */
  async delete(
    collection: string,
    id: string,
    bypassRipple?: boolean,
  ): Promise<void> {
    await this.sendCRUD("Delete", {
      collection,
      id,
      ...(bypassRipple !== undefined && { bypass_ripple: bypassRipple }),
    });
  }

  /** Batch insert multiple records via WebSocket. */
  async batchInsert(
    collection: string,
    records: Record[],
    bypassRipple?: boolean,
  ): Promise<any> {
    return this.sendCRUD("BatchInsert", {
      collection,
      records,
      ...(bypassRipple !== undefined && { bypass_ripple: bypassRipple }),
    });
  }

  /** Batch update multiple records via WebSocket. */
  async batchUpdate(
    collection: string,
    updates: [string, Record][],
    bypassRipple?: boolean,
  ): Promise<any> {
    return this.sendCRUD("BatchUpdate", {
      collection,
      updates,
      ...(bypassRipple !== undefined && { bypass_ripple: bypassRipple }),
    });
  }

  /** Batch delete records by IDs via WebSocket. */
  async batchDelete(
    collection: string,
    ids: string[],
    bypassRipple?: boolean,
  ): Promise<void> {
    await this.sendCRUD("BatchDelete", {
      collection,
      ids,
      ...(bypassRipple !== undefined && { bypass_ripple: bypassRipple }),
    });
  }

  /** Full-text search via WebSocket. */
  async textSearch(
    collection: string,
    query: string,
    fields?: string[],
    limit?: number,
  ): Promise<any[]> {
    const options: any = {};
    if (fields) options.fields = fields;
    if (limit) options.limit = limit;
    const data = await this.sendCRUD("TextSearch", {
      collection,
      query,
      ...(Object.keys(options).length > 0 && { options }),
    });
    return Array.isArray(data) ? data : [];
  }

  /** Get distinct values for a field via WebSocket. */
  async distinctValues(
    collection: string,
    field: string,
    filter?: any,
  ): Promise<any> {
    return this.sendCRUD("DistinctValues", {
      collection,
      field,
      ...(filter && { filter }),
    });
  }

  /** Apply an atomic field action via WebSocket. */
  async updateWithAction(
    collection: string,
    id: string,
    action: string,
    field: string,
    value?: any,
  ): Promise<any> {
    return this.sendCRUD("UpdateWithAction", {
      collection,
      id,
      action,
      field,
      ...(value !== undefined && { value }),
    });
  }

  /** Create a collection with optional schema via WebSocket. */
  async createCollection(name: string, schema?: any): Promise<void> {
    await this.sendCRUD("CreateCollection", {
      name,
      schema: schema ?? {},
    });
  }

  /** List all collections via WebSocket. */
  async listCollections(): Promise<string[]> {
    const data = await this.sendCRUD("GetCollections", {});
    return Array.isArray(data) ? data : [];
  }

  /** Delete a collection via WebSocket. */
  async deleteCollection(name: string): Promise<void> {
    await this.sendCRUD("DeleteCollection", { name });
  }

  /**
   * Close the WebSocket connection.
   *
   * This is an INTENTIONAL close: it disables auto-reconnect, rejects any
   * in-flight requests, and tears down all subscriptions/chat streams so
   * nothing is replayed afterward.
   */
  close(): void {
    // Mark intentional so the close handler doesn't trigger a reconnect.
    this.closed = true;
    this.reconnecting = false;

    // Reject any in-flight requests and clear their timers.
    for (const [, pending] of this.pendingRequests) {
      if (pending.timer) clearTimeout(pending.timer);
      pending.reject(new Error("WebSocket connection closed"));
    }
    this.pendingRequests.clear();

    // Tear down subscriptions + their replay bookkeeping.
    for (const [, stream] of this.subscriptions) {
      if (!stream.closed) stream.close();
    }
    this.subscriptions.clear();
    this.subscriptionParams.clear();

    // Reject any in-flight tool registration ack. Done here (not just in the
    // ws "close" handler) so it's cleaned up even when this.ws is already null.
    if (this.registerToolsAck) {
      this.registerToolsAck.reject(new Error("WebSocket connection closed"));
      this.registerToolsAck = null;
    }

    // Tear down chat streams immediately; they are one-shot and not replayed,
    // and we can't rely on the underlying ws "close" event having fired.
    for (const [, stream] of this.chatStreams) {
      stream.emit("event", { type: "error", error: "Connection closed" });
      stream.close();
    }
    this.chatStreams.clear();

    if (this.ws) {
      this.ws.close();
      this.ws = null;
      this.dispatcherRunning = false;
    }
  }
}
