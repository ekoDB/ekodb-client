/**
 * Full-text and vector search support for ekoDB
 *
 * This module provides comprehensive search capabilities including:
 * - Full-text search with fuzzy matching
 * - Vector/semantic search
 * - Hybrid search (text + vector)
 * - Field weighting and boosting
 */

export interface SearchQuery {
  /** Search query string */
  query: string;
  /** Language for stemming (e.g., "english", "spanish", "french") */
  language?: string;
  /** Case-sensitive search */
  case_sensitive?: boolean;
  /** Enable fuzzy matching (typo tolerance) */
  fuzzy?: boolean;
  /** Minimum score threshold (0.0-1.0) */
  min_score?: number;
  /** Fields to search in (comma-separated or array) */
  fields?: string | string[];
  /** Field weights (format: "field1:2.0,field2:1.5" or object) */
  weights?: string | Record<string, number>;
  /** Enable stemming */
  enable_stemming?: boolean;
  /** Boost exact matches */
  boost_exact?: boolean;
  /** Maximum edit distance for fuzzy matching (0-5) */
  max_edit_distance?: number;
  /** Bypass ripple cache */
  bypass_ripple?: boolean;
  /** Bypass cache */
  bypass_cache?: boolean;
  /** Maximum number of results to return */
  limit?: number;

  // Vector search parameters
  /** Query vector for semantic search */
  vector?: number[];
  /** Field containing vectors (default: "embedding") */
  vector_field?: string;
  /** Similarity metric: "cosine", "euclidean", "dotproduct" */
  vector_metric?: string;
  /** Number of vector results (k-nearest neighbors) */
  vector_k?: number;
  /** Minimum similarity threshold */
  vector_threshold?: number;

  // Hybrid search parameters
  /** Weight for text search (0.0-1.0) */
  text_weight?: number;
  /** Weight for vector search (0.0-1.0) */
  vector_weight?: number;
}

/**
 * Search result with score and matched fields
 */
export interface SearchResult {
  /** The matched record */
  record: any;
  /** Relevance score */
  score: number;
  /** Fields that matched the search query */
  matched_fields: string[];
}

/**
 * Search response containing results and metadata
 */
export interface SearchResponse {
  /** Array of search results */
  results: SearchResult[];
  /** Total number of results found */
  total: number;
  /** Query execution time in milliseconds */
  took_ms?: number;
}

/**
 * Builder for constructing search queries with fluent API
 *
 * @example
 * ```typescript
 * const query = new SearchQueryBuilder("john")
 *   .fields(["name", "email"])
 *   .fuzzy(true)
 *   .minScore(0.5)
 *   .limit(10)
 *   .build();
 *
 * const results = await client.search("users", query);
 * ```
 */
export class SearchQueryBuilder {
  private query: SearchQuery;

  constructor(queryString: string) {
    this.query = { query: queryString };
  }

  /**
   * Set the language for stemming
   */
  language(language: string): this {
    this.query.language = language;
    return this;
  }

  /**
   * Enable case-sensitive search
   */
  caseSensitive(enabled: boolean = true): this {
    this.query.case_sensitive = enabled;
    return this;
  }

  /**
   * Enable fuzzy matching
   */
  fuzzy(enabled: boolean = true): this {
    this.query.fuzzy = enabled;
    return this;
  }

  /**
   * Set minimum score threshold
   */
  minScore(score: number): this {
    this.query.min_score = score;
    return this;
  }

  /**
   * Set fields to search in
   */
  fields(fields: string | string[]): this {
    this.query.fields = fields;
    return this;
  }

  /**
   * Set field weights
   */
  weights(weights: string | Record<string, number>): this {
    this.query.weights = weights;
    return this;
  }

  /**
   * Enable stemming
   */
  enableStemming(enabled: boolean = true): this {
    this.query.enable_stemming = enabled;
    return this;
  }

  /**
   * Boost exact matches
   */
  boostExact(enabled: boolean = true): this {
    this.query.boost_exact = enabled;
    return this;
  }

  /**
   * Set maximum edit distance for fuzzy matching
   */
  maxEditDistance(distance: number): this {
    this.query.max_edit_distance = distance;
    return this;
  }

  /**
   * Set query vector for semantic search
   */
  vector(vector: number[]): this {
    this.query.vector = vector;
    return this;
  }

  /**
   * Set vector field name
   */
  vectorField(field: string): this {
    this.query.vector_field = field;
    return this;
  }

  /**
   * Set vector similarity metric
   */
  vectorMetric(metric: "cosine" | "euclidean" | "dotproduct"): this {
    this.query.vector_metric = metric;
    return this;
  }

  /**
   * Set number of vector results (k-nearest neighbors)
   */
  vectorK(k: number): this {
    this.query.vector_k = k;
    return this;
  }

  /**
   * Set minimum similarity threshold
   */
  vectorThreshold(threshold: number): this {
    this.query.vector_threshold = threshold;
    return this;
  }

  /**
   * Set text search weight for hybrid search
   */
  textWeight(weight: number): this {
    this.query.text_weight = weight;
    return this;
  }

  /**
   * Set vector search weight for hybrid search
   */
  vectorWeight(weight: number): this {
    this.query.vector_weight = weight;
    return this;
  }

  /**
   * Bypass ripple cache
   */
  bypassRipple(bypass: boolean = true): this {
    this.query.bypass_ripple = bypass;
    return this;
  }

  /**
   * Bypass cache
   */
  bypassCache(bypass: boolean = true): this {
    this.query.bypass_cache = bypass;
    return this;
  }

  /**
   * Set maximum number of results to return
   */
  limit(limit: number): this {
    this.query.limit = limit;
    return this;
  }

  /**
   * Build the final SearchQuery object
   */
  build(): SearchQuery {
    // Normalize fields to comma-separated string if array
    if (Array.isArray(this.query.fields)) {
      this.query.fields = this.query.fields.join(",");
    }

    // Normalize weights to string format if object
    if (this.query.weights && typeof this.query.weights === "object") {
      const weightEntries = Object.entries(this.query.weights)
        .map(([field, weight]) => `${field}:${weight}`)
        .join(",");
      this.query.weights = weightEntries;
    }

    return this.query;
  }
}
