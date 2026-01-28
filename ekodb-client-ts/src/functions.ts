/**
 * Scripts API for ekoDB TypeScript client
 */

export interface Script {
  label: string;
  name: string;
  description?: string;
  version?: string;
  parameters: { [key: string]: ParameterDefinition };
  functions: FunctionStageConfig[];
  tags?: string[];
  created_at?: string;
  updated_at?: string;
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
      condition: ScriptCondition;
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

// ScriptCondition uses adjacently-tagged format: { type: "...", value: { ...data } }
// Unit variants (HasRecords) have no value field
export type ScriptCondition =
  | { type: "FieldEquals"; value: { field: string; value: any } }
  | { type: "FieldExists"; value: { field: string } }
  | { type: "HasRecords" }
  | { type: "CountEquals"; value: { count: number } }
  | { type: "CountGreaterThan"; value: { count: number } }
  | { type: "CountLessThan"; value: { count: number } }
  | { type: "And"; value: { conditions: ScriptCondition[] } }
  | { type: "Or"; value: { conditions: ScriptCondition[] } }
  | { type: "Not"; value: { condition: ScriptCondition } };

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

// Stage builder functions
export const Stage = {
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
    record: Record<string, any>,
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
    updates: Record<string, any>,
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
    updates: Record<string, any>,
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
    condition: ScriptCondition,
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
    updates: Record<string, any>,
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
};
