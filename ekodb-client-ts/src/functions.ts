/**
 * Saved Functions API for ekoDB TypeScript client
 */

export interface SavedFunction {
  label: string;
  name: string;
  description?: string;
  version: string;
  parameters: { [key: string]: ParameterDefinition };
  pipeline: FunctionStageConfig[];
  tags: string[];
  created_at?: string;
  updated_at?: string;
}

export interface ParameterDefinition {
  required: boolean;
  default?: any;
  description?: string;
  param_type?: string;
}

export interface ParameterValue {
  type: "Literal" | "Parameter";
  value: any;
}

export const ParameterValue = {
  literal: (value: any): ParameterValue => ({
    type: "Literal",
    value,
  }),
  parameter: (name: string): ParameterValue => ({
    type: "Parameter",
    value: name,
  }),
};

export type FunctionStageConfig =
  | { type: "FindAll"; collection: string }
  | { type: "Query"; collection: string; expression: Record<string, any> }
  | { type: "Project"; fields: string[] }
  | {
      type: "Group";
      by_fields: string[];
      functions: GroupFunctionConfig[];
    }
  | { type: "Count" }
  | {
      type: "Insert";
      collection: string;
      data: Record<string, any>;
      bypass_ripple?: boolean;
    }
  | {
      type: "Delete";
      collection: string;
      id: ParameterValue;
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
      ids: ParameterValue[];
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
      options?: Record<string, any>;
    }
  | {
      type: "TextSearch";
      collection: string;
      query: string;
      options?: Record<string, any>;
    }
  | {
      type: "HybridSearch";
      collection: string;
      text_query: string;
      vector_query: number[];
      options?: Record<string, any>;
    }
  | {
      type: "Chat";
      messages: ChatMessage[];
      model?: string;
      temperature?: number;
    }
  | {
      type: "Embed";
      texts: any;
      model?: string;
    };

export interface ChatMessage {
  role: ParameterValue;
  content: ParameterValue;
}

export const ChatMessage = {
  system: (content: string): ChatMessage => ({
    role: ParameterValue.literal("system"),
    content: ParameterValue.literal(content),
  }),
  user: (content: string): ChatMessage => ({
    role: ParameterValue.literal("user"),
    content: ParameterValue.literal(content),
  }),
  assistant: (content: string): ChatMessage => ({
    role: ParameterValue.literal("assistant"),
    content: ParameterValue.literal(content),
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
    expression: Record<string, any>,
  ): FunctionStageConfig => ({
    type: "Query",
    collection,
    expression,
  }),

  project: (fields: string[]): FunctionStageConfig => ({
    type: "Project",
    fields,
  }),

  group: (
    by_fields: string[],
    functions: GroupFunctionConfig[],
  ): FunctionStageConfig => ({
    type: "Group",
    by_fields,
    functions,
  }),

  count: (): FunctionStageConfig => ({ type: "Count" }),

  insert: (
    collection: string,
    data: Record<string, any>,
    bypassRipple = false,
  ): FunctionStageConfig => ({
    type: "Insert",
    collection,
    data,
    bypass_ripple: bypassRipple,
  }),

  delete: (
    collection: string,
    id: ParameterValue,
    bypassRipple = false,
  ): FunctionStageConfig => ({
    type: "Delete",
    collection,
    id,
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
    ids: ParameterValue[],
    bypassRipple = false,
  ): FunctionStageConfig => ({
    type: "BatchDelete",
    collection,
    ids,
    bypass_ripple: bypassRipple,
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
    queryVector: number[],
    options?: Record<string, any>,
  ): FunctionStageConfig => ({
    type: "VectorSearch",
    collection,
    query_vector: queryVector,
    options,
  }),

  textSearch: (
    collection: string,
    query: string,
    options?: Record<string, any>,
  ): FunctionStageConfig => ({
    type: "TextSearch",
    collection,
    query,
    options,
  }),

  hybridSearch: (
    collection: string,
    textQuery: string,
    vectorQuery: number[],
    options?: Record<string, any>,
  ): FunctionStageConfig => ({
    type: "HybridSearch",
    collection,
    text_query: textQuery,
    vector_query: vectorQuery,
    options,
  }),

  chat: (
    messages: ChatMessage[],
    model?: string,
    temperature?: number,
  ): FunctionStageConfig => ({
    type: "Chat",
    messages,
    model,
    temperature,
  }),

  embed: (texts: any, model?: string): FunctionStageConfig => ({
    type: "Embed",
    texts,
    model,
  }),
};
