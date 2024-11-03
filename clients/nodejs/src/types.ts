export type ValueType = 'String' | 'Integer' | 'Float' | 'Boolean' | 'Date';

export interface TypedValue<T> {
  type: ValueType;
  value: T;
}

export type FilterOperator = 'Eq' | 'Gt' | 'Lt' | 'Gte' | 'Lte' | 'Contains';
export type LogicalOperator = 'And' | 'Or';

export interface ConditionFilter {
  type: 'Condition';
  content: {
    field: string;
    operator: FilterOperator;
    value: TypedValue<unknown>;
  };
}

export interface LogicalFilter {
  type: 'Logical';
  content: {
    operator: LogicalOperator;
    expressions: Filter[];
  };
}

export type Filter = ConditionFilter | LogicalFilter;

export interface SortOption {
  field: string;
  ascending: boolean;
}

export interface FindOptions {
  filter?: Filter;
  sort?: SortOption[];
  limit?: number;
  skip?: number;
}

export interface ClientConfig {
  baseURL?: string;
  apiKey: string;
}

export interface BatchInsertResponse {
  insertedIds: string[];
  success: boolean;
}

export interface BatchUpdateResponse {
  updatedCount: number;
  success: boolean;
}

export interface BatchDeleteResponse {
  deletedCount: number;
  success: boolean;
}

export interface TokenResponse {
  token: string;
}

// Schema types
export interface SchemaField {
  type: ValueType;
  required?: boolean;
  default?: unknown;
}

export interface CollectionSchema {
  fields: Record<string, SchemaField>;
}

// Record type helper
export type TypedRecord<T> = {
  [K in keyof T]: TypedValue<T[K]>;
};
