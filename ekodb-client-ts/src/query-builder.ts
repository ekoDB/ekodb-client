/**
 * Query Builder for constructing complex queries with fluent API
 */

export enum SortOrder {
  Asc = "asc",
  Desc = "desc",
}

export interface Query {
  filter?: any;
  sort?: Array<{ field: string; ascending: boolean }>;
  limit?: number;
  skip?: number;
  join?: any;
  bypass_cache?: boolean;
  bypass_ripple?: boolean;
}

/**
 * Fluent API for building complex database queries
 *
 * @example
 * ```typescript
 * const query = new QueryBuilder()
 *   .eq("status", "active")
 *   .gt("age", 18)
 *   .sortDesc("created_at")
 *   .limit(10)
 *   .build();
 * ```
 */
export class QueryBuilder {
  private filters: any[] = [];
  private sortFields: Array<{ field: string; order: SortOrder }> = [];
  private _limit?: number;
  private _skip?: number;
  private _join?: any;
  private _bypassCache: boolean = false;
  private _bypassRipple: boolean = false;

  // ========================================================================
  // Comparison Operators
  // ========================================================================

  /**
   * Add an equality filter ($eq)
   */
  eq(field: string, value: any): this {
    this.filters.push({
      type: "Condition",
      content: {
        field,
        operator: "Eq",
        value,
      },
    });
    return this;
  }

  /**
   * Add a not-equal filter ($ne)
   */
  ne(field: string, value: any): this {
    this.filters.push({
      type: "Condition",
      content: {
        field,
        operator: "Ne",
        value,
      },
    });
    return this;
  }

  /**
   * Add a greater-than filter ($gt)
   */
  gt(field: string, value: any): this {
    this.filters.push({
      type: "Condition",
      content: {
        field,
        operator: "Gt",
        value,
      },
    });
    return this;
  }

  /**
   * Add a greater-than-or-equal filter ($gte)
   */
  gte(field: string, value: any): this {
    this.filters.push({
      type: "Condition",
      content: {
        field,
        operator: "Gte",
        value,
      },
    });
    return this;
  }

  /**
   * Add a less-than filter ($lt)
   */
  lt(field: string, value: any): this {
    this.filters.push({
      type: "Condition",
      content: {
        field,
        operator: "Lt",
        value,
      },
    });
    return this;
  }

  /**
   * Add a less-than-or-equal filter ($lte)
   */
  lte(field: string, value: any): this {
    this.filters.push({
      type: "Condition",
      content: {
        field,
        operator: "Lte",
        value,
      },
    });
    return this;
  }

  /**
   * Add an in-array filter ($in)
   */
  in(field: string, values: any[]): this {
    this.filters.push({
      type: "Condition",
      content: {
        field,
        operator: "In",
        value: values,
      },
    });
    return this;
  }

  /**
   * Add a not-in-array filter ($nin)
   */
  nin(field: string, values: any[]): this {
    this.filters.push({
      type: "Condition",
      content: {
        field,
        operator: "NotIn",
        value: values,
      },
    });
    return this;
  }

  // ========================================================================
  // String Operators
  // ========================================================================

  /**
   * Add a contains filter (substring match)
   */
  contains(field: string, substring: string): this {
    this.filters.push({
      type: "Condition",
      content: {
        field,
        operator: "Contains",
        value: substring,
      },
    });
    return this;
  }

  /**
   * Add a starts-with filter
   */
  startsWith(field: string, prefix: string): this {
    this.filters.push({
      type: "Condition",
      content: {
        field,
        operator: "StartsWith",
        value: prefix,
      },
    });
    return this;
  }

  /**
   * Add an ends-with filter
   */
  endsWith(field: string, suffix: string): this {
    this.filters.push({
      type: "Condition",
      content: {
        field,
        operator: "EndsWith",
        value: suffix,
      },
    });
    return this;
  }

  /**
   * Add a regex filter
   */
  regex(field: string, pattern: string): this {
    this.filters.push({
      type: "Condition",
      content: {
        field,
        operator: "Regex",
        value: pattern,
      },
    });
    return this;
  }

  // ========================================================================
  // Logical Operators
  // ========================================================================

  /**
   * Combine filters with AND logic
   */
  and(conditions: any[]): this {
    this.filters.push({
      type: "Logical",
      content: {
        operator: "And",
        expressions: conditions,
      },
    });
    return this;
  }

  /**
   * Combine filters with OR logic
   */
  or(conditions: any[]): this {
    this.filters.push({
      type: "Logical",
      content: {
        operator: "Or",
        expressions: conditions,
      },
    });
    return this;
  }

  /**
   * Negate a filter
   */
  not(condition: any): this {
    this.filters.push({
      type: "Logical",
      content: {
        operator: "Not",
        expressions: [condition],
      },
    });
    return this;
  }

  /**
   * Add a raw filter expression
   */
  rawFilter(filter: any): this {
    this.filters.push(filter);
    return this;
  }

  // ========================================================================
  // Sorting
  // ========================================================================

  /**
   * Add a sort field in ascending order
   */
  sortAsc(field: string): this {
    this.sortFields.push({ field, order: SortOrder.Asc });
    return this;
  }

  /**
   * Add a sort field in descending order
   */
  sortDesc(field: string): this {
    this.sortFields.push({ field, order: SortOrder.Desc });
    return this;
  }

  // ========================================================================
  // Pagination
  // ========================================================================

  /**
   * Set the maximum number of results
   */
  limit(limit: number): this {
    this._limit = limit;
    return this;
  }

  /**
   * Set the number of results to skip (for pagination)
   */
  skip(skip: number): this {
    this._skip = skip;
    return this;
  }

  /**
   * Set page number and page size (convenience method)
   */
  page(page: number, pageSize: number): this {
    this._skip = page * pageSize;
    this._limit = pageSize;
    return this;
  }

  // ========================================================================
  // Joins
  // ========================================================================

  /**
   * Add a join configuration
   */
  join(joinConfig: any): this {
    this._join = joinConfig;
    return this;
  }

  // ========================================================================
  // Performance Flags
  // ========================================================================

  /**
   * Bypass cache for this query
   */
  bypassCache(bypass: boolean = true): this {
    this._bypassCache = bypass;
    return this;
  }

  /**
   * Bypass ripple for this query
   */
  bypassRipple(bypass: boolean = true): this {
    this._bypassRipple = bypass;
    return this;
  }

  // ========================================================================
  // Build
  // ========================================================================

  /**
   * Build the final Query object
   */
  build(): Query {
    const query: Query = {};

    // Combine all filters with AND logic if multiple filters exist
    if (this.filters.length > 0) {
      query.filter =
        this.filters.length === 1
          ? this.filters[0]
          : {
              type: "Logical",
              content: {
                operator: "And",
                expressions: this.filters,
              },
            };
    }

    // Build sort expression
    if (this.sortFields.length > 0) {
      query.sort = this.sortFields.map(({ field, order }) => ({
        field,
        ascending: order === SortOrder.Asc,
      }));
    }

    if (this._limit !== undefined) {
      query.limit = this._limit;
    }

    if (this._skip !== undefined) {
      query.skip = this._skip;
    }

    if (this._join !== undefined) {
      query.join = this._join;
    }

    if (this._bypassCache) {
      query.bypass_cache = true;
    }

    if (this._bypassRipple) {
      query.bypass_ripple = true;
    }

    return query;
  }
}
