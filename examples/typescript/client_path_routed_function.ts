/**
 * Path-routed function — expose a stored function under a REST URL.
 *
 * Sets `http_method` + `http_path` on a UserFunction so ekoDB's
 * path-routed dispatcher (`/api/route/{path}`) routes incoming HTTP
 * requests to it instead of forcing every caller through
 * `POST /api/functions/:label`. Path placeholders (`:id`) are
 * extracted into the function's `params` map at call time.
 *
 * Routing rules:
 *   - method match required (GET/POST/PUT/PATCH/DELETE all dispatch)
 *   - segment count must match the pattern
 *   - lowest "specificity score" (fewest `:placeholder`) wins on ties
 *   - body params merge OVER path params on collision
 *
 * Requires ekoDB >= 0.42.0.
 */

import { EkoDBClient, Stage, type UserFunction } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log("✓ Client created");

  // Most-specific literal — `/users/admin` answers admin lookups
  // separately from the `:id` route.
  const admin: UserFunction = {
    label: "ts_route_admin",
    name: "Get admin user",
    http_method: "GET",
    http_path: "/users/admin",
    parameters: {},
    functions: [Stage.returnResponse({ role: "admin" }, 200)],
  };
  await client.saveUserFunction(admin).catch(() => {});
  console.log("✓ ts_route_admin → GET /api/route/users/admin");

  // Single-placeholder route.
  const byId: UserFunction = {
    label: "ts_route_user_by_id",
    name: "Get user by id",
    http_method: "GET",
    http_path: "/users/:id",
    parameters: { id: { required: true } },
    functions: [Stage.returnResponse({ requested_id: "{{id}}" }, 200)],
  };
  await client.saveUserFunction(byId).catch(() => {});
  console.log("✓ ts_route_user_by_id → GET /api/route/users/:id");

  // Two-placeholder nested route.
  const posts: UserFunction = {
    label: "ts_route_user_posts",
    name: "Get user's post",
    http_method: "GET",
    http_path: "/users/:id/posts/:post_id",
    parameters: { id: { required: true }, post_id: { required: true } },
    functions: [
      Stage.returnResponse({ user_id: "{{id}}", post_id: "{{post_id}}" }, 200),
    ],
  };
  await client.saveUserFunction(posts).catch(() => {});
  console.log(
    "✓ ts_route_user_posts → GET /api/route/users/:id/posts/:post_id",
  );

  // POST with body merge: path provides `:org`, body provides `name`.
  const create: UserFunction = {
    label: "ts_route_org_create_member",
    name: "Add member to org",
    http_method: "POST",
    http_path: "/orgs/:org/members",
    parameters: { org: { required: true }, name: { required: true } },
    functions: [
      Stage.returnResponse(
        { org: "{{org}}", name: "{{name}}", created: true },
        201,
      ),
    ],
  };
  await client.saveUserFunction(create).catch(() => {});
  console.log(
    "✓ ts_route_org_create_member → POST /api/route/orgs/:org/members",
  );

  console.log("\nTry them with curl:");
  console.log("  curl http://localhost:8080/api/route/users/admin");
  console.log("  curl http://localhost:8080/api/route/users/42");
  console.log("  curl http://localhost:8080/api/route/users/42/posts/7");
  console.log(
    "  curl -X POST http://localhost:8080/api/route/orgs/acme/members \\",
  );
  console.log(
    "       -H 'Content-Type: application/json' -d '{\"name\":\"alice\"}'",
  );

  for (const label of [
    "ts_route_admin",
    "ts_route_user_by_id",
    "ts_route_user_posts",
    "ts_route_org_create_member",
  ]) {
    await client.deleteUserFunction(label).catch(() => {});
  }
  console.log("\n✓ Cleaned up demo functions");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
