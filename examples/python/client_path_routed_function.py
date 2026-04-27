#!/usr/bin/env python3
"""Path-routed function — expose a stored function under a REST URL.

Sets ``http_method`` + ``http_path`` on a UserFunction so ekoDB's
path-routed dispatcher (``/api/route/{path}``) routes incoming HTTP
requests to it.

Routing rules:
  - method match required (GET/POST/PUT/PATCH/DELETE all dispatch)
  - segment count must match the pattern
  - lowest "specificity score" (fewest ``:placeholder``) wins on ties
  - body params merge OVER path params on collision

Requires ekoDB >= 0.42.0.
"""

import asyncio
import os
from pathlib import Path
from dotenv import load_dotenv

from ekodb_client import Client, Stage

env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def main() -> None:
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")

    # Most-specific literal — `/users/admin` answers admin lookups
    # separately from the `:id` route.
    admin = {
        "label": "py_route_admin",
        "name": "Get admin user",
        "http_method": "GET",
        "http_path": "/users/admin",
        "parameters": {},
        "functions": [Stage.return_response({"role": "admin"}, 200)],
    }
    await _save(client, admin)

    # Single-placeholder route.
    by_id = {
        "label": "py_route_user_by_id",
        "name": "Get user by id",
        "http_method": "GET",
        "http_path": "/users/:id",
        "parameters": {"id": {"required": True}},
        "functions": [Stage.return_response({"requested_id": "{{id}}"}, 200)],
    }
    await _save(client, by_id)

    # Two-placeholder nested route.
    posts = {
        "label": "py_route_user_posts",
        "name": "Get user's post",
        "http_method": "GET",
        "http_path": "/users/:id/posts/:post_id",
        "parameters": {"id": {"required": True}, "post_id": {"required": True}},
        "functions": [
            Stage.return_response({"user_id": "{{id}}", "post_id": "{{post_id}}"}, 200)
        ],
    }
    await _save(client, posts)

    # POST with body merge: path provides `:org`, body provides `name`.
    create = {
        "label": "py_route_org_create_member",
        "name": "Add member to org",
        "http_method": "POST",
        "http_path": "/orgs/:org/members",
        "parameters": {"org": {"required": True}, "name": {"required": True}},
        "functions": [
            Stage.return_response(
                {"org": "{{org}}", "name": "{{name}}", "created": True}, 201
            )
        ],
    }
    await _save(client, create)

    print("\nTry them with curl:")
    print("  curl http://localhost:8080/api/route/users/admin")
    print("  curl http://localhost:8080/api/route/users/42")
    print("  curl http://localhost:8080/api/route/users/42/posts/7")
    print(
        "  curl -X POST http://localhost:8080/api/route/orgs/acme/members \\\n"
        "       -H 'Content-Type: application/json' -d '{\"name\":\"alice\"}'"
    )

    for label in (
        "py_route_admin",
        "py_route_user_by_id",
        "py_route_user_posts",
        "py_route_org_create_member",
    ):
        try:
            await client.delete_user_function(label)
        except Exception:
            pass
    print("\n✓ Cleaned up demo functions")


async def _save(client, func):
    try:
        await client.save_user_function(func)
        print(f"✓ {func['label']} saved")
    except Exception as e:
        print(f"SaveUserFunction({func['label']}) error: {e}")


if __name__ == "__main__":
    asyncio.run(main())
