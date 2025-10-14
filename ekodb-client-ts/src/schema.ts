/**
 * Schema management for collections
 *
 * This module provides types and utilities for defining and managing
 * collection schemas with field types, constraints, and indexes.
 */

/**
 * Vector index algorithm
 */
export enum VectorIndexAlgorithm {
  /** Simple flat index (brute force) */
  Flat = "flat",
  /** Hierarchical Navigable Small World */
  HNSW = "hnsw",
  /** Inverted File Index (for future) */
  IVF = "ivf",
}

/**
 * Distance metric for vector similarity
 */
export enum DistanceMetric {
  Cosine = "cosine",
  Euclidean = "euclidean",
  DotProduct = "dotproduct",
}

/**
 * Index configuration for a field
 */
export type IndexConfig =
  | {
      type: "text";
      language?: string;
      analyzer?: string;
    }
  | {
      type: "vector";
      algorithm?: VectorIndexAlgorithm;
      metric?: DistanceMetric;
      m?: number;
      ef_construction?: number;
    }
  | {
      type: "btree";
    }
  | {
      type: "hash";
    };

/**
 * Field type schema with constraints
 *
 * Valid field types (case-sensitive):
 * - String, Integer, Float, Boolean, DateTime
 * - Array, Object, Vector, Set
 * - UUID, Decimal, Binary, Bytes, Duration, Number, Null
 */
export interface FieldTypeSchema {
  /** Field type - must be one of the valid types (case-sensitive, e.g., "String", "Integer", "Float") */
  field_type: string;
  /** Default value for the field */
  default?: any;
  /** Whether the field must be unique across records */
  unique?: boolean;
  /** Whether the field is required */
  required?: boolean;
  /** Allowed enum values */
  enums?: any[];
  /** Maximum value (for numbers/dates) */
  max?: any;
  /** Minimum value (for numbers/dates) */
  min?: any;
  /** Regex pattern for string validation */
  regex?: string;
  /** Index configuration */
  index?: IndexConfig;
}

/**
 * Collection schema
 */
export interface Schema {
  /** Field definitions */
  fields: Record<string, FieldTypeSchema>;
  /** Schema version */
  version?: number;
  /** Creation timestamp */
  created_at?: string;
  /** Last modification timestamp */
  last_modified?: string;
  /** Whether to bypass ripple replication */
  bypass_ripple?: boolean;
}

/**
 * Collection metadata with analytics
 */
export interface CollectionMetadata {
  /** Schema definition */
  collection: Schema;
  /** Analytics data (if available) */
  analytics?: any;
}

/**
 * Builder for constructing field type schemas
 *
 * @example
 * ```typescript
 * const field = new FieldTypeSchemaBuilder("string")
 *   .required()
 *   .unique()
 *   .pattern("^[a-z]+$")
 *   .build();
 * ```
 */
export class FieldTypeSchemaBuilder {
  private schema: FieldTypeSchema;

  constructor(fieldType: string) {
    this.schema = { field_type: fieldType };
  }

  /**
   * Set the field as required
   */
  required(): this {
    this.schema.required = true;
    return this;
  }

  /**
   * Set the field as unique
   */
  unique(): this {
    this.schema.unique = true;
    return this;
  }

  /**
   * Set a default value
   */
  defaultValue(value: any): this {
    this.schema.default = value;
    return this;
  }

  /**
   * Set enum values
   */
  enums(values: any[]): this {
    this.schema.enums = values;
    return this;
  }

  /**
   * Set min/max range
   */
  range(min?: any, max?: any): this {
    this.schema.min = min;
    this.schema.max = max;
    return this;
  }

  /**
   * Set regex pattern
   */
  pattern(regex: string): this {
    this.schema.regex = regex;
    return this;
  }

  /**
   * Add a text index
   */
  textIndex(language: string = "english", analyzer?: string): this {
    this.schema.index = {
      type: "text",
      language,
      analyzer,
    };
    return this;
  }

  /**
   * Add a vector index
   */
  vectorIndex(
    algorithm: VectorIndexAlgorithm = VectorIndexAlgorithm.Flat,
    metric: DistanceMetric = DistanceMetric.Cosine,
    m: number = 16,
    efConstruction: number = 200,
  ): this {
    this.schema.index = {
      type: "vector",
      algorithm,
      metric,
      m,
      ef_construction: efConstruction,
    };
    return this;
  }

  /**
   * Add a B-tree index
   */
  btreeIndex(): this {
    this.schema.index = { type: "btree" };
    return this;
  }

  /**
   * Add a hash index
   */
  hashIndex(): this {
    this.schema.index = { type: "hash" };
    return this;
  }

  /**
   * Build the final FieldTypeSchema
   */
  build(): FieldTypeSchema {
    return this.schema;
  }
}

/**
 * Builder for constructing collection schemas
 *
 * @example
 * ```typescript
 * const schema = new SchemaBuilder()
 *   .addField("name", new FieldTypeSchemaBuilder("string").required())
 *   .addField("email", new FieldTypeSchemaBuilder("string").unique())
 *   .addField("age", new FieldTypeSchemaBuilder("number").range(0, 150))
 *   .build();
 *
 * await client.createCollection("users", schema);
 * ```
 */
export class SchemaBuilder {
  private schema: Schema;

  constructor() {
    this.schema = {
      fields: {},
      version: 1,
      bypass_ripple: true,
    };
  }

  /**
   * Add a field to the schema
   */
  addField(
    name: string,
    field: FieldTypeSchema | FieldTypeSchemaBuilder,
  ): this {
    this.schema.fields[name] =
      field instanceof FieldTypeSchemaBuilder ? field.build() : field;
    return this;
  }

  /**
   * Set bypass_ripple flag
   */
  bypassRipple(bypass: boolean): this {
    this.schema.bypass_ripple = bypass;
    return this;
  }

  /**
   * Set schema version
   */
  version(version: number): this {
    this.schema.version = version;
    return this;
  }

  /**
   * Build the final Schema
   */
  build(): Schema {
    return this.schema;
  }
}
