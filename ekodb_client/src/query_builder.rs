//! Advanced query builder with comprehensive operator support
//!
//! This module provides a fluent API for building complex queries with
//! logical operators, comparison operators, and advanced filtering.

use crate::types::Query;
use serde_json::{json, Value};

/// Builder for constructing complex queries
#[derive(Debug, Clone, Default)]
pub struct QueryBuilder {
    filters: Vec<Value>,
    sort_fields: Vec<(String, SortOrder)>,
    limit: Option<usize>,
    skip: Option<usize>,
    join: Option<Value>,
    bypass_cache: bool,
    bypass_ripple: bool,
    select_fields: Option<Vec<String>>,
    exclude_fields: Option<Vec<String>>,
}

/// Sort order for query results
#[derive(Debug, Clone, Copy)]
pub enum SortOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

impl QueryBuilder {
    /// Create a new query builder
    pub fn new() -> Self {
        Self::default()
    }

    // ========================================================================
    // Comparison Operators
    // ========================================================================

    /// Add an equality filter (Eq operator)
    pub fn eq(mut self, field: impl Into<String>, value: impl Into<Value>) -> Self {
        self.filters.push(json!({
            "type": "Condition",
            "content": {
                "field": field.into(),
                "operator": "Eq",
                "value": value.into()
            }
        }));
        self
    }

    /// Add a not-equal filter (Ne operator)
    pub fn ne(mut self, field: impl Into<String>, value: impl Into<Value>) -> Self {
        self.filters.push(json!({
            "type": "Condition",
            "content": {
                "field": field.into(),
                "operator": "Ne",
                "value": value.into()
            }
        }));
        self
    }

    /// Add a greater-than filter (Gt operator)
    pub fn gt(mut self, field: impl Into<String>, value: impl Into<Value>) -> Self {
        self.filters.push(json!({
            "type": "Condition",
            "content": {
                "field": field.into(),
                "operator": "Gt",
                "value": value.into()
            }
        }));
        self
    }

    /// Add a greater-than-or-equal filter (Gte operator)
    pub fn gte(mut self, field: impl Into<String>, value: impl Into<Value>) -> Self {
        self.filters.push(json!({
            "type": "Condition",
            "content": {
                "field": field.into(),
                "operator": "Gte",
                "value": value.into()
            }
        }));
        self
    }

    /// Add a less-than filter (Lt operator)
    pub fn lt(mut self, field: impl Into<String>, value: impl Into<Value>) -> Self {
        self.filters.push(json!({
            "type": "Condition",
            "content": {
                "field": field.into(),
                "operator": "Lt",
                "value": value.into()
            }
        }));
        self
    }

    /// Add a less-than-or-equal filter (Lte operator)
    pub fn lte(mut self, field: impl Into<String>, value: impl Into<Value>) -> Self {
        self.filters.push(json!({
            "type": "Condition",
            "content": {
                "field": field.into(),
                "operator": "Lte",
                "value": value.into()
            }
        }));
        self
    }

    /// Add an in-array filter (In operator)
    pub fn in_array(mut self, field: impl Into<String>, values: Vec<Value>) -> Self {
        self.filters.push(json!({
            "type": "Condition",
            "content": {
                "field": field.into(),
                "operator": "In",
                "value": values
            }
        }));
        self
    }

    /// Add a not-in-array filter (NotIn operator)
    pub fn nin(mut self, field: impl Into<String>, values: Vec<Value>) -> Self {
        self.filters.push(json!({
            "type": "Condition",
            "content": {
                "field": field.into(),
                "operator": "NotIn",
                "value": values
            }
        }));
        self
    }

    // ========================================================================
    // String Operators
    // ========================================================================

    /// Add a contains filter (substring match)
    pub fn contains(mut self, field: impl Into<String>, substring: impl Into<String>) -> Self {
        self.filters.push(json!({
            "type": "Condition",
            "content": {
                "field": field.into(),
                "operator": "Contains",
                "value": substring.into()
            }
        }));
        self
    }

    /// Add a starts-with filter
    pub fn starts_with(mut self, field: impl Into<String>, prefix: impl Into<String>) -> Self {
        self.filters.push(json!({
            "type": "Condition",
            "content": {
                "field": field.into(),
                "operator": "StartsWith",
                "value": prefix.into()
            }
        }));
        self
    }

    /// Add an ends-with filter
    pub fn ends_with(mut self, field: impl Into<String>, suffix: impl Into<String>) -> Self {
        self.filters.push(json!({
            "type": "Condition",
            "content": {
                "field": field.into(),
                "operator": "EndsWith",
                "value": suffix.into()
            }
        }));
        self
    }

    /// Add a regex filter (Note: not directly supported by server, use contains/starts_with/ends_with instead)
    pub fn regex(mut self, field: impl Into<String>, pattern: impl Into<String>) -> Self {
        // Regex is not in the server's FilterOperator enum
        // We'll use Contains as a fallback
        self.filters.push(json!({
            "type": "Condition",
            "content": {
                "field": field.into(),
                "operator": "Contains",
                "value": pattern.into()
            }
        }));
        self
    }

    // Note: Array operators like elem_match and exists are not supported by the server's FilterOperator enum

    // ========================================================================
    // Logical Operators
    // ========================================================================

    /// Combine filters with AND logic
    pub fn and(mut self, conditions: Vec<Value>) -> Self {
        self.filters.push(json!({
            "type": "Logical",
            "content": {
                "operator": "And",
                "expressions": conditions
            }
        }));
        self
    }

    /// Combine filters with OR logic
    pub fn or(mut self, conditions: Vec<Value>) -> Self {
        self.filters.push(json!({
            "type": "Logical",
            "content": {
                "operator": "Or",
                "expressions": conditions
            }
        }));
        self
    }

    /// Negate a filter
    pub fn not(mut self, condition: Value) -> Self {
        self.filters.push(json!({
            "type": "Logical",
            "content": {
                "operator": "Not",
                "expressions": [condition]
            }
        }));
        self
    }

    /// Add a raw filter expression
    pub fn raw_filter(mut self, filter: Value) -> Self {
        self.filters.push(filter);
        self
    }

    // ========================================================================
    // Sorting
    // ========================================================================

    /// Add a sort field in ascending order
    pub fn sort_asc(mut self, field: impl Into<String>) -> Self {
        self.sort_fields.push((field.into(), SortOrder::Asc));
        self
    }

    /// Add a sort field in descending order
    pub fn sort_desc(mut self, field: impl Into<String>) -> Self {
        self.sort_fields.push((field.into(), SortOrder::Desc));
        self
    }

    // ========================================================================
    // Pagination
    // ========================================================================

    /// Set the maximum number of results
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the number of results to skip (for pagination)
    pub fn skip(mut self, skip: usize) -> Self {
        self.skip = Some(skip);
        self
    }

    /// Set page number and page size (convenience method)
    pub fn page(mut self, page: usize, page_size: usize) -> Self {
        self.skip = Some(page * page_size);
        self.limit = Some(page_size);
        self
    }

    // ========================================================================
    // Joins
    // ========================================================================

    /// Add a join configuration
    pub fn join(mut self, join_config: Value) -> Self {
        self.join = Some(join_config);
        self
    }

    // ========================================================================
    // Performance Flags
    // ========================================================================

    /// Bypass cache for this query
    pub fn bypass_cache(mut self, bypass: bool) -> Self {
        self.bypass_cache = bypass;
        self
    }

    /// Bypass ripple propagation for this query
    pub fn bypass_ripple(mut self, bypass: bool) -> Self {
        self.bypass_ripple = bypass;
        self
    }

    // ========================================================================
    // Field Projection
    // ========================================================================

    /// Select specific fields to return (plus 'id' which is always included)
    pub fn select_fields(mut self, fields: Vec<String>) -> Self {
        self.select_fields = Some(fields);
        self
    }

    /// Exclude specific fields from results
    pub fn exclude_fields(mut self, fields: Vec<String>) -> Self {
        self.exclude_fields = Some(fields);
        self
    }

    // ========================================================================
    // Build
    // ========================================================================

    /// Build the final Query object
    pub fn build(self) -> Query {
        let mut query = Query::default();

        // Combine all filters with AND logic if multiple filters exist
        if !self.filters.is_empty() {
            query.filter = if self.filters.len() == 1 {
                Some(self.filters[0].clone())
            } else {
                Some(json!({
                    "type": "Logical",
                    "content": {
                        "operator": "And",
                        "expressions": self.filters
                    }
                }))
            };
        }

        // Build sort expression as array of {field, ascending} objects
        if !self.sort_fields.is_empty() {
            let sort_array: Vec<_> = self
                .sort_fields
                .into_iter()
                .map(|(field, order)| {
                    json!({
                        "field": field,
                        "ascending": matches!(order, SortOrder::Asc)
                    })
                })
                .collect();
            query.sort = Some(json!(sort_array));
        }

        query.limit = self.limit;
        query.skip = self.skip;
        query.join = self.join;
        query.bypass_cache = Some(self.bypass_cache);
        query.bypass_ripple = Some(self.bypass_ripple);

        if let Some(fields) = self.select_fields {
            query.select_fields = Some(fields);
        }

        if let Some(fields) = self.exclude_fields {
            query.exclude_fields = Some(fields);
        }

        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_query() {
        let query = QueryBuilder::new()
            .eq("status", "active")
            .gte("age", 18)
            .build();

        assert!(query.filter.is_some());
    }

    #[test]
    fn test_complex_query() {
        let query = QueryBuilder::new()
            .eq("status", "active")
            .gte("age", 18)
            .lt("age", 65)
            .contains("email", "@example.com")
            .sort_desc("created_at")
            .limit(10)
            .skip(20)
            .build();

        assert!(query.filter.is_some());
        assert!(query.sort.is_some());
        assert_eq!(query.limit, Some(10));
        assert_eq!(query.skip, Some(20));
    }

    #[test]
    fn test_logical_operators() {
        let query = QueryBuilder::new()
            .or(vec![
                json!({"type": "Condition", "content": {"field": "status", "operator": "Eq", "value": "active"}}),
                json!({"type": "Condition", "content": {"field": "status", "operator": "Eq", "value": "pending"}}),
            ])
            .build();

        assert!(query.filter.is_some());
    }

    #[test]
    fn test_ne_operator() {
        let query = QueryBuilder::new().ne("status", "deleted").build();
        assert!(query.filter.is_some());
    }

    #[test]
    fn test_gt_operator() {
        let query = QueryBuilder::new().gt("age", 18).build();
        assert!(query.filter.is_some());
    }

    #[test]
    fn test_lt_operator() {
        let query = QueryBuilder::new().lt("age", 65).build();
        assert!(query.filter.is_some());
    }

    #[test]
    fn test_lte_operator() {
        let query = QueryBuilder::new().lte("score", 100).build();
        assert!(query.filter.is_some());
    }

    #[test]
    fn test_in_operator() {
        let query = QueryBuilder::new()
            .in_array("status", vec![json!("active"), json!("pending")])
            .build();
        assert!(query.filter.is_some());
    }

    #[test]
    fn test_nin_operator() {
        let query = QueryBuilder::new()
            .nin("status", vec![json!("deleted"), json!("archived")])
            .build();
        assert!(query.filter.is_some());
    }

    // test_exists_operator removed - Exists is not supported by server's FilterOperator enum

    #[test]
    fn test_regex_operator() {
        let query = QueryBuilder::new()
            .regex("email", "^.*@example\\.com$")
            .build();
        assert!(query.filter.is_some());
    }

    #[test]
    fn test_starts_with() {
        let query = QueryBuilder::new().starts_with("name", "John").build();
        assert!(query.filter.is_some());
    }

    #[test]
    fn test_ends_with() {
        let query = QueryBuilder::new()
            .ends_with("email", "@example.com")
            .build();
        assert!(query.filter.is_some());
    }

    // test_elem_match removed - ElemMatch is not supported by server's FilterOperator enum

    #[test]
    fn test_and_operator() {
        let query = QueryBuilder::new()
            .and(vec![
                json!({"type": "Condition", "content": {"field": "age", "operator": "Gte", "value": 18}}),
                json!({"type": "Condition", "content": {"field": "status", "operator": "Eq", "value": "active"}}),
            ])
            .build();
        assert!(query.filter.is_some());
    }

    #[test]
    fn test_not_operator() {
        let query = QueryBuilder::new()
            .not(json!({"status": "deleted"}))
            .build();
        assert!(query.filter.is_some());
    }

    #[test]
    fn test_sort_asc() {
        let query = QueryBuilder::new().sort_asc("name").build();
        assert!(query.sort.is_some());
    }

    #[test]
    fn test_sort_desc() {
        let query = QueryBuilder::new().sort_desc("created_at").build();
        assert!(query.sort.is_some());
    }

    #[test]
    fn test_limit() {
        let query = QueryBuilder::new().limit(50).build();
        assert_eq!(query.limit, Some(50));
    }

    #[test]
    fn test_skip() {
        let query = QueryBuilder::new().skip(100).build();
        assert_eq!(query.skip, Some(100));
    }

    #[test]
    fn test_page() {
        let query = QueryBuilder::new().page(2, 20).build();
        assert_eq!(query.limit, Some(20));
        assert_eq!(query.skip, Some(40)); // page * page_size = 2 * 20 = 40
    }

    #[test]
    fn test_bypass_cache() {
        let query = QueryBuilder::new().bypass_cache(true).build();
        assert_eq!(query.bypass_cache, Some(true));
    }

    #[test]
    fn test_bypass_ripple() {
        let query = QueryBuilder::new().bypass_ripple(true).build();
        assert_eq!(query.bypass_ripple, Some(true));
    }

    #[test]
    fn test_join() {
        let join = json!({
            "collections": ["users"],
            "local_field": "user_id",
            "foreign_field": "id",
            "as_field": "user"
        });
        let query = QueryBuilder::new().join(join.clone()).build();
        assert_eq!(query.join, Some(join));
    }

    #[test]
    fn test_chaining_all_methods() {
        let query = QueryBuilder::new()
            .eq("status", "active")
            .gte("age", 18)
            .contains("email", "@example.com")
            .sort_desc("created_at")
            .limit(10)
            .skip(20)
            .bypass_cache(true)
            .build();

        assert!(query.filter.is_some());
        assert!(query.sort.is_some());
        assert_eq!(query.limit, Some(10));
        assert_eq!(query.skip, Some(20));
        assert_eq!(query.bypass_cache, Some(true));
    }
}
