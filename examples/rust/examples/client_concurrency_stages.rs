//! Concurrency stages — IdempotencyClaim, RateLimit, LockAcquire/Release.
//!
//! These three primitives wrap ekoDB's atomic KV layer (`kv_set_if_absent`,
//! `kv_increment`, `kv_delete_if_token_matches`) with the right semantics
//! for common server-side patterns:
//!
//!   - **IdempotencyClaim** — atomic SETNX with TTL. First call within
//!     the TTL writes `{claimed: true, key}` so downstream stages
//!     execute. Replays write `{claimed: false, key, response}` and the
//!     caller short-circuits with an `If` branch.
//!   - **RateLimit** — fixed-window counter. `on_exceed = "fail"`
//!     (default) errors and stops the pipeline; `on_exceed = "skip"`
//!     writes `{allowed: false, count, limit}` so the caller can
//!     branch.
//!   - **LockAcquire / LockRelease** — token-fenced distributed lock.
//!     Acquire returns a random token; Release only deletes the key
//!     when the token matches (prevents foreign release after a lease
//!     expired).
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

    // 1. Idempotent payment — claim → branch on replay → do work.
    //
    //    Real-world pattern: client sends an `Idempotency-Key` header
    //    on `POST /charges`. On first call we claim the key and run
    //    the charge. Replay (network retry, double-tap) sees
    //    `claimed: false` and returns the previously-recorded response
    //    instead of charging again.
    let pay = UserFunction::new("conc_demo_pay", "Idempotent payment")
        .with_parameter(ParameterDefinition::new("idempotency_key").required())
        .with_parameter(ParameterDefinition::new("amount").required())
        .with_function(Function::IdempotencyClaim {
            key: "{{idempotency_key}}".to_string(),
            ttl_secs: 86_400,
            output_field: "claim".to_string(),
        })
        .with_function(Function::If {
            condition: FunctionCondition::FieldEquals {
                field: "claim.claimed".to_string(),
                value: serde_json::Value::Bool(false),
            },
            then_functions: vec![Box::new(Function::Return {
                fields: HashMap::from([
                    (
                        "status".to_string(),
                        serde_json::Value::String("replay".to_string()),
                    ),
                    (
                        "idempotency_key".to_string(),
                        serde_json::Value::String("{{idempotency_key}}".to_string()),
                    ),
                ]),
                status_code: Some(200),
            })],
            else_functions: Some(vec![
                // First-time path: insert the charge.
                Box::new(Function::Insert {
                    collection: "charges".to_string(),
                    record: json!({
                        "amount": "{{amount}}",
                        "idempotency_key": "{{idempotency_key}}",
                    }),
                    bypass_ripple: None,
                    ttl: None,
                }),
                Box::new(Function::Return {
                    fields: HashMap::from([(
                        "status".to_string(),
                        serde_json::Value::String("charged".to_string()),
                    )]),
                    status_code: Some(201),
                }),
            ]),
        });
    let _ = client.save_user_function(pay).await;
    println!("✓ conc_demo_pay saved");

    // 2. Rate-limited public endpoint — fail mode (RateLimit errors out).
    //    Use this when you want the gate to be authoritative: over-limit
    //    requests fail the pipeline outright with a `RateLimit:` error.
    let public_fail = UserFunction::new("conc_demo_rl_fail", "Rate-limit (fail mode)")
        .with_parameter(ParameterDefinition::new("user_id").required())
        .with_function(Function::RateLimit {
            key: "user-{{user_id}}".to_string(),
            limit: 10,
            window_secs: 60,
            on_exceed: None, // default = fail
            output_field: "rl".to_string(),
        })
        .with_function(Function::Return {
            fields: HashMap::from([("ok".to_string(), serde_json::Value::Bool(true))]),
            status_code: Some(200),
        });
    let _ = client.save_user_function(public_fail).await;
    println!("✓ conc_demo_rl_fail saved");

    // 3. Rate-limited public endpoint — skip mode + If branch.
    //    Use this when you want the pipeline to keep running on
    //    over-limit but flag the request (e.g. for soft throttling
    //    where you still want to log the attempt).
    let public_skip = UserFunction::new("conc_demo_rl_skip", "Rate-limit (skip mode)")
        .with_parameter(ParameterDefinition::new("user_id").required())
        .with_function(Function::RateLimit {
            key: "user-{{user_id}}".to_string(),
            limit: 10,
            window_secs: 60,
            on_exceed: Some("skip".to_string()),
            output_field: "rl".to_string(),
        })
        .with_function(Function::If {
            condition: FunctionCondition::FieldEquals {
                field: "rl.allowed".to_string(),
                value: serde_json::Value::Bool(false),
            },
            then_functions: vec![Box::new(Function::Return {
                fields: HashMap::from([(
                    "status".to_string(),
                    serde_json::Value::String("rate_limited".to_string()),
                )]),
                status_code: Some(429),
            })],
            else_functions: Some(vec![Box::new(Function::Return {
                fields: HashMap::from([(
                    "status".to_string(),
                    serde_json::Value::String("ok".to_string()),
                )]),
                status_code: Some(200),
            })]),
        });
    let _ = client.save_user_function(public_skip).await;
    println!("✓ conc_demo_rl_skip saved");

    // 4. Distributed lock — acquire + critical section + release.
    //    Real-world pattern: two workers should never run the same
    //    long-running job (cron, migration, queue drain) concurrently.
    //    Acquire returns a token; Release verifies that token before
    //    deleting the key, so a worker whose lease expired can't
    //    accidentally release the new holder's lock.
    let lock = UserFunction::new("conc_demo_lock", "Critical section under lock")
        .with_parameter(ParameterDefinition::new("resource").required())
        .with_function(Function::LockAcquire {
            key: "{{resource}}".to_string(),
            ttl_secs: 30,
            output_field: "lock".to_string(),
        })
        .with_function(Function::If {
            condition: FunctionCondition::FieldEquals {
                field: "lock.acquired".to_string(),
                value: serde_json::Value::Bool(false),
            },
            then_functions: vec![Box::new(Function::Return {
                fields: HashMap::from([(
                    "status".to_string(),
                    serde_json::Value::String("busy".to_string()),
                )]),
                status_code: Some(409),
            })],
            else_functions: Some(vec![
                // === critical section here ===
                // (in a real workflow this would be the long-running
                // task; we leave it as a marker insert for the demo)
                Box::new(Function::Insert {
                    collection: "lock_demo_audit".to_string(),
                    record: json!({ "resource": "{{resource}}" }),
                    bypass_ripple: None,
                    ttl: Some(serde_json::json!(60)),
                }),
                Box::new(Function::LockRelease {
                    key: "{{resource}}".to_string(),
                    token: "{{lock.token}}".to_string(),
                    output_field: "release".to_string(),
                }),
                Box::new(Function::Return {
                    fields: HashMap::from([(
                        "status".to_string(),
                        serde_json::Value::String("done".to_string()),
                    )]),
                    status_code: Some(200),
                }),
            ]),
        });
    let _ = client.save_user_function(lock).await;
    println!("✓ conc_demo_lock saved");

    println!("\nInvoke them like:");
    println!("  POST /api/functions/conc_demo_pay        {{ \"idempotency_key\": \"...\", \"amount\": 100 }}");
    println!("  POST /api/functions/conc_demo_rl_fail    {{ \"user_id\": 42 }}");
    println!("  POST /api/functions/conc_demo_rl_skip    {{ \"user_id\": 42 }}");
    println!("  POST /api/functions/conc_demo_lock       {{ \"resource\": \"queue:drain\" }}");

    for label in [
        "conc_demo_pay",
        "conc_demo_rl_fail",
        "conc_demo_rl_skip",
        "conc_demo_lock",
    ] {
        let _ = client.delete_user_function(label).await;
    }
    println!("\n✓ Cleaned up demo functions");

    Ok(())
}
