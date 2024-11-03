import pytest
import json
from ekodb.types import (
    ValueType, TypedValue, FilterOperator, LogicalOperator,
    ConditionFilter, LogicalFilter, FindOptions, BatchInsertResponse,
    BatchUpdateResponse, BatchDeleteResponse
)

def test_typed_value_serialization():
    value = TypedValue(type=ValueType.STRING, value="test")
    assert value.dict() == {"type": "String", "value": "test"}
    
    # Test JSON serialization
    json_str = value.json()
    parsed = json.loads(json_str)
    assert parsed == {"type": "String", "value": "test"}

def test_typed_value_custom_json():
    value = TypedValue(type=ValueType.INTEGER, value=42)
    assert value.__json__() == {"type": "Integer", "value": 42}

def test_condition_filter():
    filter = ConditionFilter(
        content={
            "field": "age",
            "operator": FilterOperator.GT,
            "value": TypedValue(type=ValueType.INTEGER, value=25)
        }
    )
    
    assert filter.type == "Condition"
    serialized = filter.dict()
    assert serialized["content"]["value"] == {"type": "Integer", "value": 25}

def test_logical_filter():
    filter = LogicalFilter(
        content={
            "operator": LogicalOperator.AND,
            "expressions": [
                ConditionFilter(
                    content={
                        "field": "age",
                        "operator": FilterOperator.GT,
                        "value": TypedValue(type=ValueType.INTEGER, value=25)
                    }
                ),
                ConditionFilter(
                    content={
                        "field": "name",
                        "operator": FilterOperator.EQ,
                        "value": TypedValue(type=ValueType.STRING, value="John")
                    }
                )
            ]
        }
    )
    
    assert filter.type == "Logical"
    serialized = filter.dict()
    assert serialized["content"]["operator"] == "And"
    assert len(serialized["content"]["expressions"]) == 2

def test_find_options():
    options = FindOptions(
        filter=ConditionFilter(
            content={
                "field": "age",
                "operator": FilterOperator.GT,
                "value": TypedValue(type=ValueType.INTEGER, value=25)
            }
        ),
        limit=10,
        skip=0
    )
    
    params = options.dict(exclude_none=True)
    assert "filter" in params
    assert params["limit"] == 10
    assert params["skip"] == 0

def test_batch_responses():
    # Test BatchInsertResponse
    insert_response = BatchInsertResponse(insertedIds=["id1", "id2"], success=True)
    assert insert_response.inserted_ids == ["id1", "id2"]
    assert insert_response.success is True
    
    # Test BatchUpdateResponse
    update_response = BatchUpdateResponse(updatedCount=2, success=True)
    assert update_response.updated_count == 2
    assert update_response.success is True
    
    # Test BatchDeleteResponse
    delete_response = BatchDeleteResponse(deletedCount=2, success=True)
    assert delete_response.deleted_count == 2
    assert delete_response.success is True

def test_batch_response_aliases():
    # Test that both camelCase and snake_case work
    insert_response = BatchInsertResponse(
        inserted_ids=["id1"],
        success=True
    )
    assert insert_response.inserted_ids == ["id1"]
    
    insert_response = BatchInsertResponse(
        insertedIds=["id1"],
        success=True
    )
    assert insert_response.inserted_ids == ["id1"]

def test_value_type_enum():
    assert ValueType.STRING == "String"
    assert ValueType.INTEGER == "Integer"
    assert ValueType.FLOAT == "Float"
    assert ValueType.BOOLEAN == "Boolean"
    assert ValueType.DATE == "Date"

def test_filter_operator_enum():
    assert FilterOperator.EQ == "Eq"
    assert FilterOperator.GT == "Gt"
    assert FilterOperator.LT == "Lt"
    assert FilterOperator.GTE == "Gte"
    assert FilterOperator.LTE == "Lte"
    assert FilterOperator.CONTAINS == "Contains"

def test_logical_operator_enum():
    assert LogicalOperator.AND == "And"
    assert LogicalOperator.OR == "Or"