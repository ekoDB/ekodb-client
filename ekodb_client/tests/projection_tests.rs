//! Unit tests for field projection support in QueryBuilder

use ekodb_client::QueryBuilder;

#[test]
fn test_select_fields() {
    let query = QueryBuilder::new()
        .eq("status", "active")
        .select_fields(vec![
            "name".to_string(),
            "email".to_string(),
            "created_at".to_string(),
        ])
        .build();

    assert!(query.select_fields.is_some());
    let fields = query.select_fields.unwrap();
    assert_eq!(fields.len(), 3);
    assert!(fields.contains(&"name".to_string()));
    assert!(fields.contains(&"email".to_string()));
    assert!(fields.contains(&"created_at".to_string()));
}

#[test]
fn test_exclude_fields() {
    let query = QueryBuilder::new()
        .eq("user_role", "admin")
        .exclude_fields(vec![
            "password".to_string(),
            "api_key".to_string(),
            "secret_token".to_string(),
        ])
        .build();

    assert!(query.exclude_fields.is_some());
    let fields = query.exclude_fields.unwrap();
    assert_eq!(fields.len(), 3);
    assert!(fields.contains(&"password".to_string()));
    assert!(fields.contains(&"api_key".to_string()));
    assert!(fields.contains(&"secret_token".to_string()));
}

#[test]
fn test_both_projections() {
    let query = QueryBuilder::new()
        .eq("status", "active")
        .select_fields(vec!["name".to_string(), "email".to_string()])
        .exclude_fields(vec!["metadata.internal".to_string()])
        .build();

    assert!(query.select_fields.is_some());
    assert!(query.exclude_fields.is_some());

    let select = query.select_fields.unwrap();
    let exclude = query.exclude_fields.unwrap();

    assert_eq!(select.len(), 2);
    assert_eq!(exclude.len(), 1);
}

#[test]
fn test_projection_with_complex_query() {
    let query = QueryBuilder::new()
        .eq("status", "active")
        .gte("age", 18)
        .lt("age", 65)
        .select_fields(vec![
            "id".to_string(),
            "name".to_string(),
            "email".to_string(),
        ])
        .sort_desc("created_at")
        .limit(10)
        .build();

    assert!(query.filter.is_some());
    assert!(query.select_fields.is_some());
    assert!(query.sort.is_some());
    assert_eq!(query.limit, Some(10));

    let fields = query.select_fields.unwrap();
    assert_eq!(fields.len(), 3);
}

#[test]
fn test_projection_preserves_other_fields() {
    let query = QueryBuilder::new()
        .eq("type", "user")
        .select_fields(vec!["username".to_string(), "email".to_string()])
        .bypass_cache(true)
        .bypass_ripple(true)
        .skip(20)
        .build();

    assert!(query.filter.is_some());
    assert!(query.select_fields.is_some());
    assert_eq!(query.bypass_cache, Some(true));
    assert_eq!(query.bypass_ripple, Some(true));
    assert_eq!(query.skip, Some(20));
}

#[test]
fn test_empty_select_fields() {
    let query = QueryBuilder::new()
        .eq("status", "active")
        .select_fields(vec![])
        .build();

    assert!(query.select_fields.is_some());
    let fields = query.select_fields.unwrap();
    assert_eq!(fields.len(), 0);
}

#[test]
fn test_projection_with_pagination() {
    let query = QueryBuilder::new()
        .eq("category", "tech")
        .select_fields(vec!["id".to_string(), "title".to_string()])
        .page(2, 20)
        .build();

    assert!(query.select_fields.is_some());
    assert_eq!(query.limit, Some(20));
    assert_eq!(query.skip, Some(40)); // page 2 * pageSize 20
}

#[test]
fn test_chaining_projection_methods() {
    let builder = QueryBuilder::new();
    let result = builder
        .eq("status", "active")
        .select_fields(vec!["id".to_string(), "name".to_string()])
        .limit(10);

    // Verify fluent interface works
    let query = result.build();
    assert!(query.filter.is_some());
    assert!(query.select_fields.is_some());
    assert_eq!(query.limit, Some(10));
}
