"""Read-only by default; EKODB_ADMIN_LIVE=1 registers and revokes one disposable key."""

import asyncio
import os
from uuid import uuid4
from dotenv import load_dotenv
from ekodb_client.admin import AdminClient


async def main():
    load_dotenv()
    base, owner = os.environ["API_BASE_URL"], os.environ["API_BASE_KEY"]
    admin = AdminClient(base, owner)
    current = await admin.get_permissions()
    print("Admin permissions read successfully")
    if os.getenv("EKODB_ADMIN_LIVE") != "1":
        return
    if not current["is_admin"]:
        raise RuntimeError("An admin key is required")
    label = f"sdk-python-admin-example-{uuid4()}"
    print(f"Disposable cleanup label: {label}")
    try:
        registered = f"{label}-registered"
        created = await admin.create_api_key(
            {
                "label": label,
                "collection_permissions": {"sdk_admin_probe": ["read"]},
                "function_permissions": {registered: ["call"]},
            }
        )
        if created["key"] == owner or created["is_admin"]:
            raise RuntimeError("Unexpected disposable key")
        scoped = AdminClient(base, created["key"])
        if (await scoped.get_permissions())["is_admin"]:
            raise RuntimeError("Unexpected admin grant")
        modern = await admin.uses_key_permission_routes()
        if modern:
            key_id = created.get("id")
            if not key_id or not any(
                key.get("id") == key_id and key.get("label") == label
                for key in await admin.list_api_keys()
            ):
                raise RuntimeError("Registration/listing omitted key id")
            prefix = f"sdk-admin:{label}:"
            target = {"key_id": key_id, "prefix": prefix}
            change = await admin.put_api_key_kv_permissions(
                {**target, "read": True, "write": False}
            )
            if not change["after"]["read"] or change["after"]["write"]:
                raise RuntimeError("KV grant creation failed")
            await admin.patch_api_key_kv_permissions({**target, "write": True})
            if (await admin.delete_api_key_kv_permissions(key_id, prefix))[
                "after"
            ] is not None:
                raise RuntimeError("KV grant removal failed")
            # Stored-function call grants (needs a server that supports them).
            held = await admin.list_api_key_function_permissions(key_id)
            if not any(grant["pattern"] == registered for grant in held):
                raise RuntimeError("Registered function grant missing")
            function_pattern = f"{label}-*"
            granted = await admin.put_api_key_function_permissions(
                key_id, function_pattern
            )
            if granted["before"] is not None or granted["after"] != {
                "pattern": function_pattern
            }:
                raise RuntimeError("Function grant creation failed")
            if await admin.get_api_key_function_permissions(
                key_id, function_pattern
            ) != {"pattern": function_pattern}:
                raise RuntimeError("Function grant readback failed")
            if (
                await admin.delete_api_key_function_permissions(
                    key_id, function_pattern
                )
            )["after"] is not None:
                raise RuntimeError("Function grant removal failed")
        target = {"key_id": created["id"]} if modern else {"api_key": created["key"]}
        request = {
            **target,
            "collection": "sdk_admin_probe",
            "read": True,
            "write": True,
            "select_fields": ["public"],
            "exclude_fields": ["secret"],
            "writable_fields": ["public"],
        }
        if modern:
            await admin.put_api_key_collection_permissions(request)
        else:
            await admin.legacy_update_api_key_collection_permissions(request)
        initial = (await AdminClient(base, created["key"]).get_permissions())[
            "collection_permissions"
        ]["sdk_admin_probe"]
        expected = {
            "read": True,
            "write": True,
            "select_fields": ["public"],
            "exclude_fields": ["secret"],
            "writable_fields": ["public"],
        }
        if any(initial.get(field) != value for field, value in expected.items()):
            raise RuntimeError("Initial restriction readback failed")
        update = (
            admin.update_api_key_collection_permissions
            if modern
            else admin.legacy_update_api_key_collection_permissions
        )
        await update(
            {
                **target,
                "collection": "sdk_admin_probe",
                "read": True,
                "write": False,
                "exclude_fields": None,
                "writable_fields": [],
            }
        )
        grant = (await AdminClient(base, created["key"]).get_permissions())[
            "collection_permissions"
        ]["sdk_admin_probe"]
        if grant != {
            "read": True,
            "write": False,
            "select_fields": ["public"],
            "exclude_fields": None,
            "writable_fields": [],
        }:
            raise RuntimeError("Restriction readback failed")
    finally:
        matches = [
            key for key in await admin.list_api_keys() if key.get("label") == label
        ]
        if len(matches) > 1 or any(key["key"] == owner for key in matches):
            raise RuntimeError("Unexpected disposable cleanup match")
        for key in matches:
            await admin.revoke_api_key(key.get("id") or key["key"])
        if any(key.get("label") == label for key in await admin.list_api_keys()):
            raise RuntimeError(f"Disposable cleanup failed for label {label}")
    print("Disposable admin lifecycle verified; no matching key remains")


if __name__ == "__main__":
    asyncio.run(main())
