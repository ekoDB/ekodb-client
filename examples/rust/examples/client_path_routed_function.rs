//! Path-routed function — expose a stored function under a REST URL.
//!
//! Sets `http_method` + `http_path` on a UserFunction so ekoDB's
//! path-routed dispatcher (`/api/route/{path}`) routes incoming HTTP
//! requests to it instead of forcing every caller through
//! `POST /api/functions/:label`. Path placeholders (`:id`) are
//! extracted into the function's `params` map at call time.
//!
//! Routing rules (from the dispatcher):
//!   - method match required (GET, POST, PUT, PATCH, DELETE all dispatch)
//!   - segment count must match the pattern
//!   - lowest "specificity score" (fewest `:placeholder` segments) wins
//!     when multiple patterns match — `/users/admin` beats `/users/:id`
//!   - body params merge OVER path params on collision (so a POST body
//!     can augment URL placeholders)
//!
//! Requires ekoDB >= 0.43.0.

use ekodb_client::{Client, Function, ParameterDefinition, UserFunction};
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("✓ Client created");

    // Most-specific literal — `/users/admin` answers admin lookups
    // separately from the `:id` route. Specificity score 0 beats the
    // `:id` route's score of 1.
    let admin = UserFunction::new("rs_route_admin", "Get admin user")
        .with_http_route("GET", "/users/admin")
        .with_function(Function::Return {
            fields: HashMap::from([(
                "role".to_string(),
                serde_json::Value::String("admin".to_string()),
            )]),
            status_code: Some(200),
        });
    let _ = client.save_user_function(admin).await;
    println!("✓ rs_route_admin → GET /api/route/users/admin");

    // Single-placeholder route — `:id` lands in params.id at call time.
    let by_id = UserFunction::new("rs_route_user_by_id", "Get user by id")
        .with_http_route("GET", "/users/:id")
        .with_parameter(ParameterDefinition::new("id").required())
        .with_function(Function::Return {
            fields: HashMap::from([(
                "requested_id".to_string(),
                serde_json::Value::String("{{id}}".to_string()),
            )]),
            status_code: Some(200),
        });
    let _ = client.save_user_function(by_id).await;
    println!("✓ rs_route_user_by_id → GET /api/route/users/:id");

    // Two-placeholder nested route — both segments extracted.
    let posts = UserFunction::new("rs_route_user_posts", "List user's posts")
        .with_http_route("GET", "/users/:id/posts/:post_id")
        .with_parameter(ParameterDefinition::new("id").required())
        .with_parameter(ParameterDefinition::new("post_id").required())
        .with_function(Function::Return {
            fields: HashMap::from([
                (
                    "user_id".to_string(),
                    serde_json::Value::String("{{id}}".to_string()),
                ),
                (
                    "post_id".to_string(),
                    serde_json::Value::String("{{post_id}}".to_string()),
                ),
            ]),
            status_code: Some(200),
        });
    let _ = client.save_user_function(posts).await;
    println!("✓ rs_route_user_posts → GET /api/route/users/:id/posts/:post_id");

    // POST route demonstrating the body-over-path-params merge.
    // Path provides `:org`; the body can additionally provide
    // arbitrary fields (a `name`, etc.) — body keys win on collision.
    let create = UserFunction::new("rs_route_org_create_member", "Add member to org")
        .with_http_route("POST", "/orgs/:org/members")
        .with_parameter(ParameterDefinition::new("org").required())
        .with_parameter(ParameterDefinition::new("name").required())
        .with_function(Function::Return {
            fields: HashMap::from([
                (
                    "org".to_string(),
                    serde_json::Value::String("{{org}}".to_string()),
                ),
                (
                    "name".to_string(),
                    serde_json::Value::String("{{name}}".to_string()),
                ),
                ("created".to_string(), serde_json::Value::Bool(true)),
            ]),
            status_code: Some(201),
        });
    let _ = client.save_user_function(create).await;
    println!("✓ rs_route_org_create_member → POST /api/route/orgs/:org/members");

    println!("\nTry them with curl:");
    println!("  curl http://localhost:8080/api/route/users/admin");
    println!("  curl http://localhost:8080/api/route/users/42");
    println!("  curl http://localhost:8080/api/route/users/42/posts/7");
    println!("  curl -X POST http://localhost:8080/api/route/orgs/acme/members \\");
    println!("       -H 'Content-Type: application/json' -d '{{\"name\":\"alice\"}}'");

    for label in [
        "rs_route_admin",
        "rs_route_user_by_id",
        "rs_route_user_posts",
        "rs_route_org_create_member",
    ] {
        let _ = client.delete_user_function(label).await;
    }
    println!("\n✓ Cleaned up demo functions");

    Ok(())
}
