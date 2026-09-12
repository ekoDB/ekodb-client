"""Offline contract smoke test for stored-function stage serialization."""

import json

from ekodb_client import Stage


def main() -> None:
    query_filter = {
        "type": "Condition",
        "content": {"field": "status", "operator": "Eq", "value": "active"},
    }
    stages = [
        Stage.query("items", query_filter, limit=5),
        Stage.update("items", query_filter, {"status": "ready"}),
        Stage.delete("items", query_filter),
        Stage.group(
            ["category"],
            [
                {
                    "output_field": "labels",
                    "operation": "AddToSet",
                    "input_field": "label",
                },
                {
                    "output_field": "score_stddev",
                    "operation": "StandardDeviation",
                    "input_field": "score",
                },
                {
                    "output_field": "unique_users",
                    "operation": "ApproxDistinct",
                    "input_field": "user_id",
                },
            ],
        ),
        Stage.if_condition(
            {
                "type": "FieldGreaterThanOrEqual",
                "value": {"field": "stock", "value": 1},
            },
            [Stage.set_field("available", True)],
        ),
        Stage.upsert("items", "sku", "sku-1", {"name": "Widget"}),
        Stage.increment("items", "item-1", "views", by=2),
        Stage.push("items", "item-1", "tags", "new"),
        Stage.set_field("source", "contract"),
        Stage.add_fields(
            [
                {
                    "field_name": "total",
                    "expression": {"type": "FieldReference", "value": "price"},
                }
            ]
        ),
        Stage.current_datetime("processed_at"),
        Stage.vector_search("items", [0.1, 0.2], limit=3, threshold=0.8),
        Stage.hybrid_search("items", "widget", [0.1, 0.2], limit=3),
        Stage.find_one_and_update(
            "items", "item-1", {"status": "claimed"}, bypass_ripple=True, ttl=60
        ),
        Stage.update_with_action(
            "items", "item-1", "increment", "views", 1, bypass_ripple=True
        ),
        Stage.batch_delete("items", ["item-1", "item-2"], bypass_ripple=True),
        Stage.embed("body", "embedding", "text-embedding-3-small"),
        Stage.http_request(
            "https://example.com/items",
            timeout_seconds=10,
            output_field="response",
        ),
    ]
    function = {
        "label": "client_function_contract",
        "name": "Client function contract",
        "parameters": {},
        "functions": stages,
        "transaction_config": {
            "enabled": True,
            "auto_rollback": True,
            "isolation_level": "Serializable",
        },
    }

    encoded = json.loads(json.dumps(function))
    by_type = {stage["type"]: stage for stage in encoded["functions"]}
    assert encoded["transaction_config"]["isolation_level"] == "Serializable"
    for stage_type in ("Query", "Update", "Delete"):
        assert by_type[stage_type]["filter"]["type"] == "Condition"
    assert by_type["Group"]["functions"][2]["operation"] == "ApproxDistinct"
    assert by_type["If"]["condition"]["type"] == "FieldGreaterThanOrEqual"
    assert by_type["BatchDelete"]["record_ids"] == ["item-1", "item-2"]
    assert by_type["AddFields"]["fields"][0]["expression"] == {
        "type": "FieldReference",
        "value": "price",
    }
    assert by_type["Embed"]["input_field"] == "body"
    assert by_type["HttpRequest"]["timeout_seconds"] == 10
    assert by_type["HttpRequest"]["output_field"] == "response"
    for stage_type in (
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
    ):
        assert stage_type in by_type

    print("client_function_contract: ok")


if __name__ == "__main__":
    main()
