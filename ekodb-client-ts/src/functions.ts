/**
 * Functions API for ekoDB TypeScript client
 */

/** A reusable sequence of Functions stored in ekoDB. */
export interface UserFunction {
  id?: string;
  label: string;
  name: string;
  description?: string;
  version?: string;
  parameters: { [key: string]: ParameterDefinition };
  functions: FunctionStageConfig[];
  tags?: string[];
  created_at?: string;
  updated_at?: string;
  /**
   * REST method this function answers — `"GET"`, `"POST"`, etc.
   * Pair with `http_path` to expose the function under the
   * path-routed dispatcher at `/api/route/{path}`.
   * Requires ekoDB >= 0.43.0.
   */
  http_method?: "GET" | "POST" | "PUT" | "PATCH" | "DELETE";
  /**
   * REST path pattern (e.g. `"/users/:id"`). Path segments
   * starting with `:` are extracted into the function's params
   * map at call time. Requires ekoDB >= 0.43.0.
   */
  http_path?: string;
}

export interface ParameterDefinition {
  required: boolean;
  default?: any;
  description?: string;
  param_type?: string;
}

// ParameterValue removed - use plain values instead

export type FunctionStageConfig =
  | { type: "FindAll"; collection: string }
  | {
      type: "Query";
      collection: string;
      filter?: Record<string, any>;
      sort?: SortFieldConfig[];
      limit?: number;
      skip?: number;
    }
  | { type: "Project"; fields: string[]; exclude: boolean }
  | {
      type: "Group";
      by_fields: string[];
      functions: GroupFunctionConfig[];
    }
  | { type: "Count"; output_field: string }
  | { type: "Filter"; filter: Record<string, any> }
  | { type: "Sort"; sort: SortFieldConfig[] }
  | { type: "Limit"; limit: number }
  | { type: "Skip"; skip: number }
  | {
      type: "Insert";
      collection: string;
      record: Record<string, any>;
      bypass_ripple?: boolean;
      ttl?: number;
    }
  | {
      type: "Update";
      collection: string;
      filter: Record<string, any>;
      updates: Record<string, any>;
      bypass_ripple?: boolean;
      ttl?: number;
    }
  | {
      type: "UpdateById";
      collection: string;
      record_id: string;
      updates: Record<string, any>;
      bypass_ripple?: boolean;
      ttl?: number;
    }
  | {
      type: "Delete";
      collection: string;
      filter: Record<string, any>;
      bypass_ripple?: boolean;
    }
  | {
      type: "DeleteById";
      collection: string;
      record_id: string;
      bypass_ripple?: boolean;
    }
  | {
      type: "BatchInsert";
      collection: string;
      records: Record<string, any>[];
      bypass_ripple?: boolean;
    }
  | {
      type: "BatchDelete";
      collection: string;
      record_ids: string[];
      bypass_ripple?: boolean;
    }
  | {
      type: "HttpRequest";
      url: string;
      method?: string;
      headers?: Record<string, string>;
      body?: any;
    }
  | {
      type: "VectorSearch";
      collection: string;
      query_vector: number[];
      limit?: number;
      threshold?: number;
    }
  | {
      type: "TextSearch";
      collection: string;
      query_text: string;
      fields?: string[];
      limit?: number;
      fuzzy?: boolean;
    }
  | {
      type: "HybridSearch";
      collection: string;
      query_text: string;
      query_vector?: number[];
      limit?: number;
    }
  | {
      type: "Chat";
      messages: ChatMessage[];
      model?: string;
      temperature?: number;
      max_tokens?: number;
    }
  | {
      type: "Embed";
      input_field: string;
      output_field: string;
      model?: string;
    }
  | {
      type: "FindById";
      collection: string;
      record_id: string;
    }
  | {
      type: "FindOne";
      collection: string;
      key: string;
      value: any;
    }
  | {
      type: "If";
      condition: FunctionCondition;
      then_functions: FunctionStageConfig[];
      else_functions?: FunctionStageConfig[];
    }
  | {
      type: "ForEach";
      functions: FunctionStageConfig[];
    }
  | {
      type: "CallFunction";
      function_label: string;
      params?: Record<string, any>;
    }
  | {
      type: "FindOneAndUpdate";
      collection: string;
      record_id: string;
      updates: Record<string, any>;
      bypass_ripple?: boolean;
      ttl?: number;
    }
  | {
      type: "UpdateWithAction";
      collection: string;
      record_id: string;
      action: string;
      field: string;
      value: any;
      bypass_ripple?: boolean;
    }
  | {
      type: "CreateSavepoint";
      name: string;
    }
  | {
      type: "RollbackToSavepoint";
      name: string;
    }
  | {
      type: "ReleaseSavepoint";
      name: string;
    }
  | {
      /** Returns {value: <data>} on hit, {value: null} on miss */
      type: "KvGet";
      key: string;
    }
  | {
      type: "KvSet";
      key: string;
      value: any;
      ttl?: number;
    }
  | {
      type: "KvDelete";
      key: string;
    }
  | {
      type: "KvExists";
      key: string;
      output_field?: string;
    }
  | {
      type: "KvQuery";
      pattern?: string;
      include_expired?: boolean;
    }
  | {
      type: "SWR";
      cache_key: string;
      ttl: string | number;
      url: string;
      method: string;
      headers?: Record<string, string>;
      body?: any;
      timeout_seconds?: number;
      output_field?: string;
      collection?: string;
    }
  | {
      /**
       * Bcrypt-hash a plaintext value and add the result to every record in
       * the working data as `output_field`. Requires ekoDB >= 0.41.0.
       */
      type: "BcryptHash";
      plain: string;
      cost?: number;
      output_field: string;
    }
  | {
      /**
       * Verify a plaintext against a bcrypt hash stored on the first record
       * in the working data and write a boolean result into `output_field`.
       * Pair with an `If` stage for login flows. Requires ekoDB >= 0.41.0.
       */
      type: "BcryptVerify";
      plain: string;
      hash_field: string;
      output_field: string;
    }
  | {
      /**
       * Generate a cryptographically-random token and add it to every
       * record in the working data. Requires ekoDB >= 0.41.0.
       */
      type: "RandomToken";
      bytes: number;
      encoding?: "hex" | "base64" | "base64url";
      output_field: string;
    }
  | {
      /**
       * Sign a JWT and write the resulting token to every working
       * record. Pair with `BcryptVerify` to issue a session token
       * after login. Use `"{{env.JWT_SECRET}}"` for `secret` so the
       * LLM never sees the operator-owned signing key. `iat` and
       * `exp` are auto-stamped when `expires_in_secs` is set.
       * Requires ekoDB >= 0.43.0.
       */
      type: "JwtSign";
      claims: Record<string, unknown>;
      secret: string;
      algorithm?: "HS256" | "HS384" | "HS512";
      expires_in_secs?: number;
      output_field: string;
    }
  | {
      /**
       * Verify a JWT held in `token_field` on the first working
       * record. On success, writes the decoded claims object into
       * `output_field`. On failure, writes `null` so callers can
       * branch with `If { FieldEquals { value: null } }` to reject.
       * Requires ekoDB >= 0.43.0.
       */
      type: "JwtVerify";
      token_field: string;
      secret: string;
      algorithm?: "HS256" | "HS384" | "HS512";
      output_field: string;
    }
  | {
      /**
       * Send a transactional email through a provider's REST API.
       * Today only `provider = "sendgrid"` is supported. Pull the
       * API key from `"{{env.SENDGRID_API_KEY}}"` so the LLM never
       * sees the operator-owned secret. Result envelope
       * `{provider_status, provider_message, provider}` is written
       * to `output_field` (defaults to `"email_send"`).
       * Requires ekoDB >= 0.43.0.
       */
      type: "EmailSend";
      to: string;
      subject: string;
      body: string;
      from: string;
      reply_to?: string;
      api_key: string;
      provider?: "sendgrid";
      html?: boolean;
      output_field?: string;
    }
  | {
      /**
       * Try/Catch error handling for graceful failure recovery.
       * Executes try_functions, and if any fail, executes catch_functions.
       */
      type: "TryCatch";
      try_functions: FunctionStageConfig[];
      catch_functions: FunctionStageConfig[];
      output_error_field?: string;
    }
  | {
      /**
       * Execute multiple functions in parallel (concurrently).
       * All functions run simultaneously, results are merged.
       */
      type: "Parallel";
      functions: FunctionStageConfig[];
      wait_for_all: boolean;
    }
  | {
      /** Sleep/delay execution for rate limiting or timing control. */
      type: "Sleep";
      duration_ms: string | number;
    }
  | {
      /**
       * Return a shaped response (final output formatting).
       * Constructs the final response object from current execution context.
       */
      type: "Return";
      fields: Record<string, any>;
      status_code?: number;
    }
  | {
      /**
       * Validate data against a JSON schema before processing.
       * Prevents invalid data from corrupting database or causing errors downstream.
       */
      type: "Validate";
      schema: Record<string, any>;
      data_field: string;
      on_error?: FunctionStageConfig[];
    };

export interface ChatMessage {
  role: string;
  content: string;
}

export const ChatMessage = {
  system: (content: string): ChatMessage => ({
    role: "system",
    content: content,
  }),
  user: (content: string): ChatMessage => ({
    role: "user",
    content: content,
  }),
  assistant: (content: string): ChatMessage => ({
    role: "assistant",
    content: content,
  }),
};

export interface GroupFunctionConfig {
  output_field: string;
  operation:
    | "Sum"
    | "Average"
    | "Count"
    | "Min"
    | "Max"
    | "First"
    | "Last"
    | "Push";
  input_field?: string;
}

export interface SortFieldConfig {
  field: string;
  ascending: boolean;
}

// FunctionCondition uses adjacently-tagged format: { type: "...", value: { ...data } }
// Unit variants (HasRecords) have no value field
export type FunctionCondition =
  | { type: "FieldEquals"; value: { field: string; value: any } }
  | { type: "FieldExists"; value: { field: string } }
  | { type: "HasRecords" }
  | { type: "CountEquals"; value: { count: number } }
  | { type: "CountGreaterThan"; value: { count: number } }
  | { type: "CountLessThan"; value: { count: number } }
  | { type: "And"; value: { conditions: FunctionCondition[] } }
  | { type: "Or"; value: { conditions: FunctionCondition[] } }
  | { type: "Not"; value: { condition: FunctionCondition } };

export interface FunctionResult {
  records: Record<string, any>[];
  stats: FunctionStats;
}

export interface FunctionStats {
  input_count: number;
  output_count: number;
  execution_time_ms: number;
  stages_executed: number;
  stage_stats: StageStats[];
}

export interface StageStats {
  stage: string;
  input_count: number;
  output_count: number;
  execution_time_ms: number;
}

/**
 * Reference a call-time function parameter inside a stored-function stage
 * body (Insert.record, Update.updates, UpdateById.updates,
 * FindOneAndUpdate.updates, BatchInsert.records, or any nested JSON value).
 *
 * Returns the structural placeholder `{"type": "Parameter", "name": "<name>"}`.
 * ekoDB's `resolve_json_parameters` recognizes this shape and substitutes the
 * actual parameter value at execution time, preserving the original FieldType
 * (Binary, DateTime, UUID, Decimal, Duration, Number, Set, Vector) via the
 * `{type,value}` wrapped form. Safe to use for any type.
 *
 * This is the structural alternative to the text-level `"{{name}}"` form;
 * both are accepted but structural placeholders are preferred when the
 * parameter is a whole-object Record or a value whose type would be lost in
 * raw JSON.
 *
 * @example
 * ```ts
 * const createUser: UserFunction = {
 *   label: "users_create",
 *   name: "Create user",
 *   parameters: {
 *     record: { required: true },
 *   },
 *   functions: [
 *     Stage.insert("users", Stage.param("record")),
 *   ],
 * };
 * ```
 */
export interface ParameterRef {
  type: "Parameter";
  name: string;
}

export function parameterRef(name: string): ParameterRef {
  return { type: "Parameter", name };
}

// Stage builder functions
export const Stage = {
  /**
   * Shorthand for `parameterRef(name)` — builds the structural placeholder
   * `{"type": "Parameter", "name": name}`. See `parameterRef` for the full
   * explanation and example.
   */
  param: (name: string): ParameterRef => parameterRef(name),

  findAll: (collection: string): FunctionStageConfig => ({
    type: "FindAll",
    collection,
  }),

  query: (
    collection: string,
    filter?: Record<string, any>,
    sort?: SortFieldConfig[],
    limit?: number,
    skip?: number,
  ): FunctionStageConfig => ({
    type: "Query",
    collection,
    filter,
    sort,
    limit,
    skip,
  }),

  project: (fields: string[], exclude = false): FunctionStageConfig => ({
    type: "Project",
    fields,
    exclude,
  }),

  group: (
    by_fields: string[],
    functions: GroupFunctionConfig[],
  ): FunctionStageConfig => ({
    type: "Group",
    by_fields,
    functions,
  }),

  count: (output_field = "count"): FunctionStageConfig => ({
    type: "Count",
    output_field,
  }),

  insert: (
    collection: string,
    record: Record<string, any> | ParameterRef,
    bypassRipple = false,
    ttl?: number,
  ): FunctionStageConfig => ({
    type: "Insert",
    collection,
    record,
    bypass_ripple: bypassRipple,
    ttl,
  }),

  update: (
    collection: string,
    filter: Record<string, any>,
    updates: Record<string, any> | ParameterRef,
    bypassRipple = false,
    ttl?: number,
  ): FunctionStageConfig => ({
    type: "Update",
    collection,
    filter,
    updates,
    bypass_ripple: bypassRipple,
    ttl,
  }),

  updateById: (
    collection: string,
    record_id: string,
    updates: Record<string, any> | ParameterRef,
    bypassRipple = false,
    ttl?: number,
  ): FunctionStageConfig => ({
    type: "UpdateById",
    collection,
    record_id,
    updates,
    bypass_ripple: bypassRipple,
    ttl,
  }),

  delete: (
    collection: string,
    filter: Record<string, any>,
    bypassRipple = false,
  ): FunctionStageConfig => ({
    type: "Delete",
    collection,
    filter,
    bypass_ripple: bypassRipple,
  }),

  deleteById: (
    collection: string,
    record_id: string,
    bypassRipple = false,
  ): FunctionStageConfig => ({
    type: "DeleteById",
    collection,
    record_id,
    bypass_ripple: bypassRipple,
  }),

  batchInsert: (
    collection: string,
    records: Record<string, any>[],
    bypassRipple = false,
  ): FunctionStageConfig => ({
    type: "BatchInsert",
    collection,
    records,
    bypass_ripple: bypassRipple,
  }),

  batchDelete: (
    collection: string,
    record_ids: string[],
    bypassRipple = false,
  ): FunctionStageConfig => ({
    type: "BatchDelete",
    collection,
    record_ids,
    bypass_ripple: bypassRipple,
  }),

  filter: (filter: Record<string, any>): FunctionStageConfig => ({
    type: "Filter",
    filter,
  }),

  sort: (sort: SortFieldConfig[]): FunctionStageConfig => ({
    type: "Sort",
    sort,
  }),

  limit: (limit: number): FunctionStageConfig => ({
    type: "Limit",
    limit,
  }),

  skip: (skip: number): FunctionStageConfig => ({
    type: "Skip",
    skip,
  }),

  httpRequest: (
    url: string,
    method = "GET",
    headers?: Record<string, string>,
    body?: any,
  ): FunctionStageConfig => ({
    type: "HttpRequest",
    url,
    method,
    headers,
    body,
  }),

  vectorSearch: (
    collection: string,
    query_vector: number[],
    limit?: number,
    threshold?: number,
  ): FunctionStageConfig => ({
    type: "VectorSearch",
    collection,
    query_vector,
    limit,
    threshold,
  }),

  textSearch: (
    collection: string,
    query_text: string,
    options?: { fields?: string[]; limit?: number; fuzzy?: boolean },
  ): FunctionStageConfig => ({
    type: "TextSearch",
    collection,
    query_text,
    fields: options?.fields,
    limit: options?.limit,
    fuzzy: options?.fuzzy,
  }),

  hybridSearch: (
    collection: string,
    query_text: string,
    query_vector?: number[],
    limit?: number,
  ): FunctionStageConfig => ({
    type: "HybridSearch",
    collection,
    query_text,
    query_vector,
    limit,
  }),

  chat: (
    messages: ChatMessage[],
    model?: string,
    temperature?: number,
    max_tokens?: number,
  ): FunctionStageConfig => ({
    type: "Chat",
    messages,
    model,
    temperature,
    max_tokens,
  }),

  embed: (
    input_field: string,
    output_field: string,
    model?: string,
  ): FunctionStageConfig => ({
    type: "Embed",
    input_field,
    output_field,
    model,
  }),

  findById: (collection: string, record_id: string): FunctionStageConfig => ({
    type: "FindById",
    collection,
    record_id,
  }),

  findOne: (
    collection: string,
    key: string,
    value: any,
  ): FunctionStageConfig => ({
    type: "FindOne",
    collection,
    key,
    value,
  }),

  if: (
    condition: FunctionCondition,
    thenFunctions: FunctionStageConfig[],
    elseFunctions?: FunctionStageConfig[],
  ): FunctionStageConfig => ({
    type: "If",
    condition,
    then_functions: thenFunctions,
    else_functions: elseFunctions,
  }),

  forEach: (functions: FunctionStageConfig[]): FunctionStageConfig => ({
    type: "ForEach",
    functions,
  }),

  callFunction: (
    function_label: string,
    params?: Record<string, any>,
  ): FunctionStageConfig => ({
    type: "CallFunction",
    function_label,
    params,
  }),

  findOneAndUpdate: (
    collection: string,
    record_id: string,
    updates: Record<string, any> | ParameterRef,
    bypassRipple = false,
    ttl?: number,
  ): FunctionStageConfig => ({
    type: "FindOneAndUpdate",
    collection,
    record_id,
    updates,
    bypass_ripple: bypassRipple,
    ttl,
  }),

  updateWithAction: (
    collection: string,
    record_id: string,
    action: string,
    field: string,
    value: any,
    bypassRipple = false,
  ): FunctionStageConfig => ({
    type: "UpdateWithAction",
    collection,
    record_id,
    action,
    field,
    value,
    bypass_ripple: bypassRipple,
  }),

  createSavepoint: (name: string): FunctionStageConfig => ({
    type: "CreateSavepoint",
    name,
  }),

  rollbackToSavepoint: (name: string): FunctionStageConfig => ({
    type: "RollbackToSavepoint",
    name,
  }),

  releaseSavepoint: (name: string): FunctionStageConfig => ({
    type: "ReleaseSavepoint",
    name,
  }),

  // KV Store operations - faster than collection lookups for simple key-value data
  // kvGet returns {value: <data>} on hit, {value: null} on miss
  kvGet: (key: string): FunctionStageConfig => ({
    type: "KvGet",
    key,
  }),

  kvSet: (key: string, value: any, ttl?: number): FunctionStageConfig => ({
    type: "KvSet",
    key,
    value,
    ttl,
  }),

  kvDelete: (key: string): FunctionStageConfig => ({
    type: "KvDelete",
    key,
  }),

  kvExists: (key: string, output_field?: string): FunctionStageConfig => ({
    type: "KvExists",
    key,
    output_field,
  }),

  kvQuery: (
    pattern?: string,
    include_expired?: boolean,
  ): FunctionStageConfig => ({
    type: "KvQuery",
    pattern,
    include_expired,
  }),

  /**
   * SWR (Stale-While-Revalidate) pattern for external API caching.
   * Automatically handles: KV cache check → HTTP request → KV cache set → optional audit storage.
   *
   * @param cache_key - KV key for caching (supports parameter substitution like "user:{{user_id}}")
   * @param ttl - Cache TTL - supports duration strings ("15m", "1h"), integers (seconds), or ISO timestamps
   * @param url - HTTP URL to fetch from (supports parameter substitution)
   * @param method - HTTP method (default: "GET")
   * @param headers - Optional HTTP headers
   * @param body - Optional HTTP request body
   * @param timeout_seconds - Optional HTTP timeout
   * @param output_field - Field name for response in enriched params (default: "response")
   * @param collection - Optional collection for audit trail storage
   */
  swr: (
    cache_key: string,
    ttl: string | number,
    url: string,
    method: string = "GET",
    headers?: Record<string, string>,
    body?: any,
    timeout_seconds?: number,
    output_field?: string,
    collection?: string,
  ): FunctionStageConfig => ({
    type: "SWR",
    cache_key,
    ttl,
    url,
    method,
    headers,
    body,
    timeout_seconds,
    output_field,
    collection,
  }),

  /**
   * Bcrypt-hash a plaintext value and write the result into every record
   * in the working data as `output_field`. Requires ekoDB >= 0.41.0.
   *
   * @param plain - Plaintext to hash. Typically a `"{{password}}"`
   *   placeholder that the substituter replaces with the call-time param
   *   before this stage runs.
   * @param output_field - Field name to write the bcrypt hash into.
   * @param cost - bcrypt cost factor (4..=31). Defaults to 12 when undefined.
   */
  bcryptHash: (
    plain: string,
    output_field: string,
    cost?: number,
  ): FunctionStageConfig => ({
    type: "BcryptHash",
    plain,
    cost,
    output_field,
  }),

  /**
   * Verify a plaintext against a bcrypt hash stored on the first record in
   * the working data. Writes a boolean into `output_field` on every
   * working record. Pair with `Stage.if` to branch on success / failure.
   * Requires ekoDB >= 0.41.0.
   *
   * @param plain - Plaintext to verify (typically `"{{password}}"`).
   * @param hash_field - Name of the field on the current record that
   *   holds the stored bcrypt hash (e.g. `"password_hash"`).
   * @param output_field - Field name to write the boolean result into.
   */
  bcryptVerify: (
    plain: string,
    hash_field: string,
    output_field: string,
  ): FunctionStageConfig => ({
    type: "BcryptVerify",
    plain,
    hash_field,
    output_field,
  }),

  /**
   * Generate a cryptographically-random token and add it to every record
   * in the working data. Requires ekoDB >= 0.41.0.
   *
   * @param bytes - Number of random bytes to draw (1..=1024).
   * @param output_field - Field name to write the encoded token into.
   * @param encoding - `"hex"` (default) | `"base64"` | `"base64url"`.
   */
  randomToken: (
    bytes: number,
    output_field: string,
    encoding?: "hex" | "base64" | "base64url",
  ): FunctionStageConfig => ({
    type: "RandomToken",
    bytes,
    encoding,
    output_field,
  }),

  /**
   * Sign a JWT and write the resulting token to every working
   * record. Pair with `Stage.bcryptVerify` to issue a session
   * token after login. Use `"{{env.JWT_SECRET}}"` for `secret` so
   * the LLM never sees the operator-owned signing key. `iat` and
   * `exp` are auto-stamped when `expires_in_secs` is set.
   * Requires ekoDB >= 0.43.0.
   *
   * @param claims - JWT payload claims.
   * @param secret - Signing secret (typically `"{{env.JWT_SECRET}}"`).
   * @param output_field - Field name to write the signed JWT into.
   * @param expires_in_secs - Lifetime in seconds (auto-stamps `iat` + `exp`).
   * @param algorithm - `"HS256"` (default) | `"HS384"` | `"HS512"`.
   */
  jwtSign: (
    claims: Record<string, unknown>,
    secret: string,
    output_field: string,
    expires_in_secs?: number,
    algorithm?: "HS256" | "HS384" | "HS512",
  ): FunctionStageConfig => ({
    type: "JwtSign",
    claims,
    secret,
    algorithm,
    expires_in_secs,
    output_field,
  }),

  /**
   * Verify a JWT held in `token_field` on the first working record.
   * On success writes the decoded claims object into `output_field`;
   * on failure writes `null`. Branch with `Stage.if` matching
   * `output_field == null` to reject. Requires ekoDB >= 0.43.0.
   *
   * @param token_field - Field on the working record holding the JWT.
   * @param secret - Verification secret (must match the signing secret).
   * @param output_field - Field name to write decoded claims into.
   * @param algorithm - Expected algorithm (default `"HS256"`).
   */
  jwtVerify: (
    token_field: string,
    secret: string,
    output_field: string,
    algorithm?: "HS256" | "HS384" | "HS512",
  ): FunctionStageConfig => ({
    type: "JwtVerify",
    token_field,
    secret,
    algorithm,
    output_field,
  }),

  /**
   * Send a transactional email. Today only the `"sendgrid"`
   * provider is supported. Use `"{{env.SENDGRID_API_KEY}}"` for
   * `api_key` so the LLM never sees the operator-owned secret.
   * Set `html: true` to send `text/html`. The result envelope
   * (`{provider_status, provider_message, provider}`) is written
   * to `output_field` (default `"email_send"`).
   * Requires ekoDB >= 0.43.0.
   */
  emailSend: (
    to: string,
    subject: string,
    body: string,
    from: string,
    api_key: string,
    options?: {
      reply_to?: string;
      provider?: "sendgrid";
      html?: boolean;
      output_field?: string;
    },
  ): FunctionStageConfig => ({
    type: "EmailSend",
    to,
    subject,
    body,
    from,
    reply_to: options?.reply_to,
    api_key,
    provider: options?.provider,
    html: options?.html,
    output_field: options?.output_field,
  }),

  /**
   * Try/Catch error handling for graceful failure recovery.
   * Executes tryFunctions, and if any fail, executes catchFunctions.
   *
   * @param tryFunctions - Functions to attempt.
   * @param catchFunctions - Functions to execute on failure.
   * @param outputErrorField - Field name to store error details (default: "error").
   */
  tryCatch: (
    tryFunctions: FunctionStageConfig[],
    catchFunctions: FunctionStageConfig[],
    outputErrorField?: string,
  ): FunctionStageConfig => ({
    type: "TryCatch",
    try_functions: tryFunctions,
    catch_functions: catchFunctions,
    output_error_field: outputErrorField,
  }),

  /**
   * Execute multiple functions in parallel (concurrently).
   * All functions run simultaneously, results are merged.
   *
   * @param functions - Functions to execute concurrently.
   * @param waitForAll - true = wait for all to complete, false = return on first completion.
   */
  parallel: (
    functions: FunctionStageConfig[],
    waitForAll = true,
  ): FunctionStageConfig => ({
    type: "Parallel",
    functions,
    wait_for_all: waitForAll,
  }),

  /**
   * Sleep/delay execution for rate limiting or timing control.
   *
   * @param durationMs - Duration in milliseconds: `1000` or `"{{delay_param}}"`.
   */
  sleep: (durationMs: string | number): FunctionStageConfig => ({
    type: "Sleep",
    duration_ms: durationMs,
  }),

  /**
   * Return a shaped response (final output formatting).
   * Constructs the final response object from current execution context.
   *
   * @param fields - Fields to include in response with `{{param}}` substitution.
   * @param statusCode - HTTP status code (default: 200).
   */
  returnResponse: (
    fields: Record<string, any>,
    statusCode?: number,
  ): FunctionStageConfig => ({
    type: "Return",
    fields,
    status_code: statusCode,
  }),

  /**
   * Validate data against a JSON schema before processing.
   *
   * @param schema - JSON Schema to validate against.
   * @param dataField - Field containing data to validate.
   * @param onError - Functions to execute on validation failure.
   */
  validate: (
    schema: Record<string, any>,
    dataField: string,
    onError?: FunctionStageConfig[],
  ): FunctionStageConfig => ({
    type: "Validate",
    schema,
    data_field: dataField,
    on_error: onError,
  }),
};
