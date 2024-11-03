from enum import Enum
from typing import TypeVar, Generic, Dict, List, Optional, Union, Any
from pydantic import BaseModel, ConfigDict
import json

T = TypeVar('T')

class ValueType(str, Enum):
    STRING = "String"
    INTEGER = "Integer"
    FLOAT = "Float"
    BOOLEAN = "Boolean"
    DATE = "Date"

class TypedValue(BaseModel, Generic[T]):
    type: ValueType
    value: T

    def dict(self, *args, **kwargs) -> Dict[str, Any]:
        return {
            "type": self.type.value if isinstance(self.type, ValueType) else self.type,
            "value": self.value
        }

    def model_dump(self, *args, **kwargs) -> Dict[str, Any]:
        return self.dict()

    def json(self, *args, **kwargs) -> str:
        return json.dumps(self.dict())

    def __json__(self):
        return self.dict()

    model_config = ConfigDict(
        json_encoders={
            ValueType: lambda v: v.value if isinstance(v, ValueType) else v
        },
        arbitrary_types_allowed=True
    )

class FilterOperator(str, Enum):
    EQ = "Eq"
    GT = "Gt"
    LT = "Lt"
    GTE = "Gte"
    LTE = "Lte"
    CONTAINS = "Contains"

class LogicalOperator(str, Enum):
    AND = "And"
    OR = "Or"

class ConditionFilter(BaseModel):
    type: str = "Condition"
    content: Dict[str, Any]

    model_config = ConfigDict(
        json_encoders={
            ValueType: lambda v: v.value if isinstance(v, ValueType) else v,
            TypedValue: lambda v: v.dict()
        }
    )

class LogicalFilter(BaseModel):
    type: str = "Logical"
    content: Dict[str, Any]

    model_config = ConfigDict(
        json_encoders={
            ValueType: lambda v: v.value if isinstance(v, ValueType) else v,
            TypedValue: lambda v: v.dict()
        }
    )

Filter = Union[ConditionFilter, LogicalFilter]

class SortOption(BaseModel):
    field: str
    ascending: bool

class FindOptions(BaseModel):
    filter: Optional[Filter] = None
    sort: Optional[List[SortOption]] = None
    limit: Optional[int] = None
    skip: Optional[int] = None

    def dict(self, *args, **kwargs) -> Dict[str, Any]:
        data = super().dict(*args, **kwargs)
        if self.filter:
            data['filter'] = json.dumps(self.filter.dict())
        if self.sort:
            data['sort'] = json.dumps([s.dict() for s in self.sort])
        return data

class ClientConfig(BaseModel):
    base_url: str = "http://localhost:8080"
    api_key: str

class BatchResponse(BaseModel):
    success: bool
    
    model_config = ConfigDict(
        populate_by_name=True,
        alias_generator=None,
        extra="ignore"
    )

class BatchInsertResponse(BatchResponse):
    inserted_ids: List[str]
    insertedIds: Optional[List[str]] = None

    def __init__(self, **data):
        if 'insertedIds' in data and 'inserted_ids' not in data:
            data['inserted_ids'] = data['insertedIds']
        super().__init__(**data)

    model_config = ConfigDict(
        populate_by_name=True,
        alias_generator=None,
        extra="ignore"
    )

class BatchUpdateResponse(BatchResponse):
    updated_count: int
    updatedCount: Optional[int] = None

    def __init__(self, **data):
        if 'updatedCount' in data and 'updated_count' not in data:
            data['updated_count'] = data['updatedCount']
        super().__init__(**data)

    model_config = ConfigDict(
        populate_by_name=True,
        alias_generator=None,
        extra="ignore"
    )

class BatchDeleteResponse(BatchResponse):
    deleted_count: int
    deletedCount: Optional[int] = None

    def __init__(self, **data):
        if 'deletedCount' in data and 'deleted_count' not in data:
            data['deleted_count'] = data['deletedCount']
        super().__init__(**data)

    model_config = ConfigDict(
        populate_by_name=True,
        alias_generator=None,
        extra="ignore"
    )

class TokenResponse(BaseModel):
    token: str

class SchemaField(BaseModel):
    type: ValueType
    required: Optional[bool] = None
    default: Optional[Any] = None

class CollectionSchema(BaseModel):
    fields: Dict[str, SchemaField]