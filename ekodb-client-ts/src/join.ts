/**
 * Join support for multi-collection queries
 *
 * This module provides support for joining data across multiple collections,
 * similar to SQL joins but with document-oriented semantics.
 */

/**
 * Configuration for joining collections
 */
export interface JoinConfig {
  /** Target collections to join with */
  collections: string[];
  /** Field in the current collection */
  local_field: string;
  /** Field in the target collection(s) */
  foreign_field: string;
  /** Name of the field to store joined data */
  as_field: string;
}

/**
 * Builder for constructing join configurations
 *
 * @example
 * ```typescript
 * // Single collection join
 * const join = JoinBuilder.single("users", "user_id", "id", "user");
 *
 * // Multi-collection join
 * const join = JoinBuilder.multi(
 *   ["users", "profiles", "settings"],
 *   "user_id",
 *   "id",
 *   "user_info"
 * );
 *
 * // Use in query
 * const query = new QueryBuilder()
 *   .eq("status", "active")
 *   .join(join)
 *   .build();
 * ```
 */
export class JoinBuilder {
  private config: JoinConfig;

  private constructor(
    collections: string[],
    localField: string,
    foreignField: string,
    asField: string,
  ) {
    this.config = {
      collections,
      local_field: localField,
      foreign_field: foreignField,
      as_field: asField,
    };
  }

  /**
   * Create a join with a single collection
   *
   * @param collection - Target collection name
   * @param localField - Field in the current collection
   * @param foreignField - Field in the target collection
   * @param asField - Name of the field to store joined data
   */
  static single(
    collection: string,
    localField: string,
    foreignField: string,
    asField: string,
  ): JoinConfig {
    return new JoinBuilder(
      [collection],
      localField,
      foreignField,
      asField,
    ).build();
  }

  /**
   * Create a join with multiple collections
   *
   * @param collections - Array of target collection names
   * @param localField - Field in the current collection
   * @param foreignField - Field in the target collections
   * @param asField - Name of the field to store joined data
   */
  static multi(
    collections: string[],
    localField: string,
    foreignField: string,
    asField: string,
  ): JoinConfig {
    return new JoinBuilder(
      collections,
      localField,
      foreignField,
      asField,
    ).build();
  }

  /**
   * Build the final JoinConfig
   */
  build(): JoinConfig {
    return this.config;
  }
}
