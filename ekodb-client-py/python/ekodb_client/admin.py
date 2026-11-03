"""API-key administration, separate from data CRUD. Returned records contain secrets."""

from typing import Dict, List, Literal, Optional, TypedDict
from ._ekodb_client import (
    AdminClient,
    MAX_FUNCTION_PATTERN_BYTES,
    MIN_SERVER_VERSION_FOR_PERMISSION_EDITS,
    MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS,
)


class CreateApiKeyRequest(TypedDict, total=False):
    label: str
    description: str
    is_admin: bool
    collection_permissions: Dict[str, List[Literal["read", "write"]]]
    # Stored-function call grants: a function label, a label prefix ending in
    # one "*", or "*", each with ["call"]. A pattern given [] grants nothing.
    function_permissions: Dict[str, List[Literal["call"]]]


class _CollectionTarget(TypedDict):
    key_id: str
    collection: str


class _PermissionTarget(_CollectionTarget):
    read: bool
    write: bool


class UpdateApiKeyCollectionPermissionsRequest(_PermissionTarget, total=False):
    select_fields: Optional[List[str]]
    exclude_fields: Optional[List[str]]
    writable_fields: Optional[List[str]]


__all__ = [
    "AdminClient",
    "ApiKey",
    "ApiKeyPermissions",
    "KvPermissions",
    "FunctionPermissions",
    "FunctionPermissionChange",
    "MAX_FUNCTION_PATTERN_BYTES",
    "CreateApiKeyRequest",
    "UpdateApiKeyCollectionPermissionsRequest",
    "MIN_SERVER_VERSION_FOR_PERMISSION_EDITS",
    "MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS",
    "LegacyUpdateApiKeyCollectionPermissionsRequest",
    "PatchApiKeyCollectionPermissionsRequest",
    "UpdateApiKeyKvPermissionsRequest",
    "PatchApiKeyKvPermissionsRequest",
]


class _LegacyPermissionTarget(TypedDict):
    api_key: str
    collection: str
    read: bool
    write: bool


class LegacyUpdateApiKeyCollectionPermissionsRequest(
    _LegacyPermissionTarget, total=False
):
    select_fields: Optional[List[str]]
    exclude_fields: Optional[List[str]]
    writable_fields: Optional[List[str]]


class PatchApiKeyCollectionPermissionsRequest(_CollectionTarget, total=False):
    read: bool
    write: bool
    select_fields: Optional[List[str]]
    exclude_fields: Optional[List[str]]
    writable_fields: Optional[List[str]]


class _KvTarget(TypedDict):
    key_id: str
    prefix: str


class UpdateApiKeyKvPermissionsRequest(_KvTarget):
    read: bool
    write: bool


class PatchApiKeyKvPermissionsRequest(_KvTarget, total=False):
    read: bool
    write: bool


class KvPermissions(TypedDict):
    prefix: str
    read: bool
    write: bool


class FunctionPermissions(TypedDict):
    pattern: str


class FunctionPermissionChange(TypedDict):
    # None before means the grant was created; None after means it was removed.
    before: Optional[FunctionPermissions]
    after: Optional[FunctionPermissions]


class ApiKeyPermissions(TypedDict):
    is_admin: bool
    collection_permissions: Dict[str, Dict[str, object]]
    kv_permissions: List[KvPermissions]
    function_permissions: List[FunctionPermissions]


class ApiKey(TypedDict, total=False):
    id: str
    key: str
    is_admin: bool
    label: Optional[str]
    description: Optional[str]
    collection_permissions: Dict[str, Dict[str, object]]
    kv_permissions: Optional[List[KvPermissions]]
    # Grants sorted by pattern when the response carries them, otherwise None.
    function_permissions: Optional[List[FunctionPermissions]]
