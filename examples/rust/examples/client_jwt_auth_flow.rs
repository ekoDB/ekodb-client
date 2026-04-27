//! JWT auth flow — registration + login as pure stored functions.
//!
//! Demonstrates the canonical password-auth pattern using ekoDB stages
//! end-to-end:
//!
//!   register flow:  Validate → BcryptHash → Insert
//!   login flow:     FindOne  → BcryptVerify → If(ok) { JwtSign + Return }
//!   verify flow:    JwtVerify → If(claims != null) { Return ok } else { Return 401 }
//!
//! Nothing in the client needs to host bcrypt or JWT primitives — the app
//! sends the password through a `{{password}}` placeholder and ekoDB
//! handles hashing, verification, and signing. Operator-owned secrets
//! (the JWT signing key) flow through `{{env.JWT_SECRET}}` so they never
//! enter the LLM context window or the function definition stored on
//! disk.
//!
//! Requires ekoDB >= 0.43.0.

use ekodb_client::{Client, Function, FunctionCondition, ParameterDefinition, UserFunction};
use serde_json::json;
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

    // Stage 1: register — bcrypt-hash, insert.
    let register = UserFunction::new("rs_users_register", "Register user")
        .with_description("Validate, bcrypt-hash, insert.")
        .with_parameter(ParameterDefinition::new("email").required())
        .with_parameter(ParameterDefinition::new("password").required())
        .with_function(Function::BcryptHash {
            plain: "{{password}}".to_string(),
            cost: Some(12),
            output_field: "password_hash".to_string(),
        })
        .with_function(Function::Insert {
            collection: "rs_users".to_string(),
            record: json!({
                "email": "{{email}}",
                "password_hash": "{{password_hash}}",
            }),
            bypass_ripple: None,
            ttl: None,
        });
    let _ = client.save_user_function(register).await;
    println!("✓ rs_users_register saved");

    // Stage 2: login — find the user, verify bcrypt, sign a JWT on success.
    let mut claims = HashMap::new();
    claims.insert(
        "email".to_string(),
        serde_json::Value::String("{{email}}".to_string()),
    );

    let login = UserFunction::new("rs_users_login", "Login user")
        .with_description("Verify password, mint JWT.")
        .with_parameter(ParameterDefinition::new("email").required())
        .with_parameter(ParameterDefinition::new("password").required())
        .with_function(Function::FindOne {
            collection: "rs_users".to_string(),
            key: "email".to_string(),
            value: serde_json::Value::String("{{email}}".to_string()),
        })
        .with_function(Function::BcryptVerify {
            plain: "{{password}}".to_string(),
            hash_field: "password_hash".to_string(),
            output_field: "password_ok".to_string(),
        })
        .with_function(Function::If {
            condition: FunctionCondition::FieldEquals {
                field: "password_ok".to_string(),
                value: serde_json::Value::Bool(true),
            },
            then_functions: vec![
                Box::new(Function::JwtSign {
                    claims,
                    secret: "{{env.JWT_SECRET}}".to_string(),
                    algorithm: Some("HS256".to_string()),
                    expires_in_secs: Some(3600),
                    output_field: "token".to_string(),
                }),
                Box::new(Function::Return {
                    fields: HashMap::from([
                        ("ok".to_string(), serde_json::Value::Bool(true)),
                        (
                            "token".to_string(),
                            serde_json::Value::String("{{token}}".to_string()),
                        ),
                    ]),
                    status_code: Some(200),
                }),
            ],
            else_functions: Some(vec![Box::new(Function::Return {
                fields: HashMap::from([("ok".to_string(), serde_json::Value::Bool(false))]),
                status_code: Some(401),
            })]),
        });
    let _ = client.save_user_function(login).await;
    println!("✓ rs_users_login saved");

    // Stage 3: verify — read the token off the request, decode + validate
    // the signature, branch on null claims.
    let verify = UserFunction::new("rs_users_verify_token", "Verify JWT token")
        .with_description("Decode + validate JWT; null claims = reject.")
        .with_parameter(ParameterDefinition::new("token").required())
        .with_function(Function::Insert {
            // Synthetic record so JwtVerify has a `working_data[0]` to read
            // the token field off. (In a real flow the token would arrive
            // on the request path or an existing record.)
            collection: "_inflight_jwt_check".to_string(),
            record: json!({ "token": "{{token}}" }),
            bypass_ripple: Some(true),
            ttl: Some(serde_json::json!(60)),
        })
        .with_function(Function::JwtVerify {
            token_field: "token".to_string(),
            secret: "{{env.JWT_SECRET}}".to_string(),
            algorithm: Some("HS256".to_string()),
            output_field: "claims".to_string(),
        })
        .with_function(Function::If {
            condition: FunctionCondition::FieldEquals {
                field: "claims".to_string(),
                value: serde_json::Value::Null,
            },
            then_functions: vec![Box::new(Function::Return {
                fields: HashMap::from([("ok".to_string(), serde_json::Value::Bool(false))]),
                status_code: Some(401),
            })],
            else_functions: Some(vec![Box::new(Function::Return {
                fields: HashMap::from([("ok".to_string(), serde_json::Value::Bool(true))]),
                status_code: Some(200),
            })]),
        });
    let _ = client.save_user_function(verify).await;
    println!("✓ rs_users_verify_token saved");

    println!("\n=== Auth flow defined as pure stored functions ===");
    println!("Call them like:");
    println!("  POST /api/functions/rs_users_register {{ \"email\": \"a@b.com\", \"password\": \"s3cret\" }}");
    println!("  POST /api/functions/rs_users_login    {{ \"email\": \"a@b.com\", \"password\": \"s3cret\" }}");
    println!("  POST /api/functions/rs_users_verify_token {{ \"token\": \"<jwt>\" }}");
    println!(
        "\nSet JWT_SECRET in ekoDB's environment_vars whitelist before invoking — \
         the {{{{env.JWT_SECRET}}}} placeholder reads from that whitelist, NEVER from \
         the function definition."
    );

    // Cleanup so re-runs don't conflict with the first registration.
    let _ = client.delete_user_function("rs_users_register").await;
    let _ = client.delete_user_function("rs_users_login").await;
    let _ = client.delete_user_function("rs_users_verify_token").await;
    println!("\n✓ Cleaned up demo functions");

    Ok(())
}
