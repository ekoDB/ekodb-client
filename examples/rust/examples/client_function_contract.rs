// Offline contract smoke test for stored-function stage serialization.

use ekodb_client::{
    Function, FunctionCondition, GroupFunctionConfig, GroupFunctionOp, QueryConditionOperator,
    QueryExpression, TransactionConfig, UserFunction,
};
use serde_json::{json, Value};

fn stage<'a>(document: &'a Value, stage_type: &str) -> &'a Value {
    document["functions"]
        .as_array()
        .and_then(|stages| stages.iter().find(|stage| stage["type"] == stage_type))
        .unwrap_or_else(|| panic!("missing {stage_type} stage"))
}

fn main() {
    let filter = QueryExpression::condition("status", QueryConditionOperator::Eq, json!("active"));
    let function = UserFunction::new("client_function_contract", "Client function contract")
        .with_transaction_config(TransactionConfig {
            enabled: true,
            auto_rollback: true,
            isolation_level: Some("Serializable".to_string()),
        })
        .with_function(Function::Query {
            collection: "items".to_string(),
            filter: Some(filter.clone()),
            sort: None,
            limit: Some(json!(5)),
            skip: None,
        })
        .with_function(Function::Update {
            collection: "items".to_string(),
            filter: filter.clone(),
            updates: json!({"status": "ready"}),
            bypass_ripple: None,
            ttl: None,
        })
        .with_function(Function::Delete {
            collection: "items".to_string(),
            filter,
            bypass_ripple: None,
        })
        .with_function(Function::Group {
            by_fields: vec!["category".to_string()],
            functions: vec![
                GroupFunctionConfig::new("labels", GroupFunctionOp::AddToSet)
                    .with_input_field("label"),
                GroupFunctionConfig::new("score_stddev", GroupFunctionOp::StandardDeviation)
                    .with_input_field("score"),
                GroupFunctionConfig::new("unique_users", GroupFunctionOp::ApproxDistinct)
                    .with_input_field("user_id"),
            ],
        })
        .with_function(Function::If {
            condition: FunctionCondition::FieldGreaterThanOrEqual {
                field: "stock".to_string(),
                value: json!(1),
            },
            then_functions: vec![Box::new(Function::SetField {
                field: "available".to_string(),
                value: json!(true),
            })],
            else_functions: None,
        })
        .with_function(Function::Upsert {
            collection: "items".to_string(),
            key: "sku".to_string(),
            value: json!("sku-1"),
            record: json!({"name": "Widget"}),
            bypass_ripple: None,
            ttl: None,
        })
        .with_function(Function::Increment {
            collection: "items".to_string(),
            record_id: "item-1".to_string(),
            field: "views".to_string(),
            by: Some(json!(2)),
            bypass_ripple: None,
        })
        .with_function(Function::Push {
            collection: "items".to_string(),
            record_id: "item-1".to_string(),
            field: "tags".to_string(),
            value: json!("new"),
            bypass_ripple: None,
        })
        .with_function(Function::SetField {
            field: "source".to_string(),
            value: json!("contract"),
        })
        .with_function(Function::AddFields {
            fields: vec![
                json!({"field_name": "total", "expression": {"type": "FieldReference", "value": "price"}}),
            ],
        })
        .with_function(Function::CurrentDatetime {
            output_field: "processed_at".to_string(),
        })
        .with_function(Function::VectorSearch {
            collection: "items".to_string(),
            query_vector: vec![0.1, 0.2],
            limit: Some(3),
            threshold: Some(0.8),
        })
        .with_function(Function::HybridSearch {
            collection: "items".to_string(),
            query_text: "widget".to_string(),
            query_vector: Some(vec![0.1, 0.2]),
            limit: Some(3),
        })
        .with_function(Function::FindOneAndUpdate {
            collection: "items".to_string(),
            record_id: "item-1".to_string(),
            updates: json!({"status": "claimed"}),
            bypass_ripple: Some(true),
            ttl: Some(json!(60)),
        })
        .with_function(Function::UpdateWithAction {
            collection: "items".to_string(),
            record_id: "item-1".to_string(),
            action: "increment".to_string(),
            field: "views".to_string(),
            value: json!(1),
            bypass_ripple: Some(true),
        })
        .with_function(Function::BatchDelete {
            collection: "items".to_string(),
            record_ids: vec!["item-1".to_string(), "item-2".to_string()],
            bypass_ripple: Some(true),
        })
        .with_function(Function::Embed {
            input_field: "body".to_string(),
            output_field: "embedding".to_string(),
            model: Some("text-embedding-3-small".to_string()),
        })
        .with_function(Function::HttpRequest {
            url: "https://example.com/items".to_string(),
            method: "GET".to_string(),
            headers: None,
            body: None,
            timeout_seconds: Some(10),
            output_field: Some("response".to_string()),
        });

    let encoded = serde_json::to_value(function).expect("function must serialize");
    assert_eq!(
        encoded["transaction_config"]["isolation_level"],
        "Serializable"
    );
    assert_eq!(stage(&encoded, "Query")["filter"]["type"], "Condition");
    assert_eq!(stage(&encoded, "Update")["filter"]["type"], "Condition");
    assert_eq!(stage(&encoded, "Delete")["filter"]["type"], "Condition");
    assert_eq!(
        stage(&encoded, "Group")["functions"][2]["operation"],
        "ApproxDistinct"
    );
    assert_eq!(
        stage(&encoded, "If")["condition"]["type"],
        "FieldGreaterThanOrEqual"
    );
    assert_eq!(stage(&encoded, "BatchDelete")["record_ids"][1], "item-2");
    assert_eq!(
        stage(&encoded, "AddFields")["fields"][0]["expression"],
        json!({"type": "FieldReference", "value": "price"})
    );
    assert_eq!(stage(&encoded, "Embed")["input_field"], "body");
    assert_eq!(stage(&encoded, "HttpRequest")["timeout_seconds"], 10);
    assert_eq!(stage(&encoded, "HttpRequest")["output_field"], "response");
    for expected in [
        "Upsert",
        "Increment",
        "Push",
        "SetField",
        "AddFields",
        "CurrentDatetime",
        "VectorSearch",
        "HybridSearch",
        "FindOneAndUpdate",
        "UpdateWithAction",
    ] {
        stage(&encoded, expected);
    }

    println!("client_function_contract: ok");
}
