use ekodb_client::{QueryBuilder, SearchQuery};
use serde_json::{Value, json};

fn base(query: &str) -> SearchQuery {
    SearchQuery::new(query)
        .bypass_ripple(false)
        .bypass_cache(false)
        .limit(10)
}

#[test]
fn cross_client_search_wire_cases() {
    let golden: Value =
        serde_json::from_str(include_str!("../../test-fixtures/search-requests.json")).unwrap();
    let vector = vec![0.1, 0.2, 0.3];
    let filter = serde_json::to_value(
        QueryBuilder::new()
            .eq("category", "ml")
            .build()
            .filter
            .unwrap(),
    )
    .unwrap();
    let cases = [
        (
            "filtered_vector",
            base("")
                .vector(vector.clone())
                .vector_metric("cosine")
                .vector_k(10)
                .vector_threshold(0.25)
                .filters(filter),
        ),
        (
            "hybrid_custom_weights",
            base("machine learning")
                .vector(vector.clone())
                .text_weight(0.7)
                .vector_weight(0.3),
        ),
        (
            "named_vector_field",
            base("")
                .vector(vector)
                .vector_field("title_embedding")
                .vector_metric("dotproduct")
                .vector_k(10),
        ),
        (
            "full_text",
            base("enginer")
                .fields("title,body")
                .weights("title:2.5,body:1.5")
                .fuzzy(true)
                .max_edit_distance(2),
        ),
    ];
    for (name, query) in cases {
        assert_eq!(serde_json::to_value(query).unwrap(), golden[name], "{name}");
    }
}

#[test]
fn rust_unset_options_are_explicitly_distinct_from_typescript_and_kotlin() {
    assert_eq!(
        serde_json::to_value(SearchQuery::new("")).unwrap(),
        json!({
            "query": "", "bypass_ripple": null, "bypass_cache": null, "limit": null
        })
    );
}
