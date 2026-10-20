"""API-key administration, separate from data CRUD. Returned records contain secrets."""

from typing import Dict, List, Literal, Optional, TypedDict
from ._ekodb_client import AdminClient, MIN_SERVER_VERSION_FOR_PERMISSION_EDITS


class CreateApiKeyRequest(TypedDict, total=False):
    label: str
    description: str
    is_admin: bool
    collection_permissions: Dict[str, List[Literal["read", "write"]]]


class _PermissionTarget(TypedDict):
    api_key: str
    collection: str
    read: bool
    write: bool


class UpdateApiKeyCollectionPermissionsRequest(_PermissionTarget, total=False):
    select_fields: Optional[List[str]]
    exclude_fields: Optional[List[str]]
    writable_fields: Optional[List[str]]


__all__ = [
    "AdminClient",
    "CreateApiKeyRequest",
    "UpdateApiKeyCollectionPermissionsRequest",
    "MIN_SERVER_VERSION_FOR_PERMISSION_EDITS",
]
