//! Crypto stages — comprehensive demo of every primitive.
//!
//! Builds and saves five small stored functions covering the full
//! crypto-stage surface added in ekoDB 0.42.0:
//!
//!   - `crypto_demo_hmac_rs`     HmacSign + HmacVerify (round trip)
//!   - `crypto_demo_aes_rs`      AesEncrypt + AesDecrypt (round trip)
//!   - `crypto_demo_uuid_rs`     UuidGenerate
//!   - `crypto_demo_totp_rs`     TotpGenerate (RFC 6238 reference secret)
//!   - `crypto_demo_encoding_rs` Base64 + Hex + Slugify (single function chain)
//!
//! Operator-owned secrets (HMAC keys, AES data keys, TOTP shared keys)
//! flow through `{{env.NAME}}` so the function definition stored on
//! disk never embeds plaintext credentials. Sensitive call-time inputs
//! (the value being signed, the data being encrypted, etc.) flow
//! through ordinary `{{param}}` placeholders.
//!
//! Requires ekoDB >= 0.42.0.

use ekodb_client::{Client, Function, ParameterDefinition, UserFunction};
use std::env;

const FUNCTION_LABELS: [&str; 5] = [
    "crypto_demo_hmac_rs",
    "crypto_demo_aes_rs",
    "crypto_demo_uuid_rs",
    "crypto_demo_totp_rs",
    "crypto_demo_encoding_rs",
];

fn is_missing_function(error: &ekodb_client::Error) -> bool {
    match error {
        ekodb_client::Error::Api { code: 404, .. } => true,
        _ => error.to_string().to_ascii_lowercase().contains("not found"),
    }
}

/// Save a user function idempotently: if the label already exists (HTTP 409),
/// update the existing definition instead.
async fn save_or_update_user(
    client: &Client,
    function: UserFunction,
) -> Result<(), Box<dyn std::error::Error>> {
    let label = function.label.clone();
    match client.save_user_function(function.clone()).await {
        Ok(_) => Ok(()),
        Err(ekodb_client::Error::Api { code: 409, .. }) => {
            client.update_user_function(&label, function).await?;
            println!("ℹ️  Function '{}' already existed — updated instead", label);
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}

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

    let operation_result: Result<(), Box<dyn std::error::Error>> = async {
        println!("✓ Client created");

        // 1. HMAC sign + verify round trip — single pipeline so the verify
        //    can read the signed value off the same working record.
        let hmac = UserFunction::new(FUNCTION_LABELS[0], "HMAC sign + verify")
            .with_parameter(ParameterDefinition::new("payload").required())
            .with_function(Function::HmacSign {
                input: "{{payload}}".to_string(),
                secret: "{{env.HMAC_KEY}}".to_string(),
                algorithm: Some("sha256".to_string()),
                output_field: "mac".to_string(),
                encoding: Some("hex".to_string()),
            })
            .with_function(Function::HmacVerify {
                input: "{{payload}}".to_string(),
                provided_mac: "{{mac}}".to_string(),
                secret: "{{env.HMAC_KEY}}".to_string(),
                algorithm: Some("sha256".to_string()),
                encoding: Some("hex".to_string()),
                output_field: "verified".to_string(),
            });
        save_or_update_user(&client, hmac).await?;
        println!("✓ {} saved", FUNCTION_LABELS[0]);

        // 2. AES-256-GCM encrypt + decrypt round trip. Decrypt reads the
        //    {ciphertext, nonce} envelope off `working_data[0]` written by
        //    encrypt. Wrong-key decrypt is fail-closed (writes null).
        let aes = UserFunction::new(FUNCTION_LABELS[1], "AES encrypt + decrypt")
            .with_parameter(ParameterDefinition::new("plaintext").required())
            .with_function(Function::AesEncrypt {
                plaintext: "{{plaintext}}".to_string(),
                key: "{{env.DATA_KEY}}".to_string(),
                key_encoding: Some("hex".to_string()),
                output_field: "envelope".to_string(),
            })
            .with_function(Function::AesDecrypt {
                ciphertext_field: "envelope".to_string(),
                key: "{{env.DATA_KEY}}".to_string(),
                key_encoding: Some("hex".to_string()),
                output_field: "recovered".to_string(),
            });
        save_or_update_user(&client, aes).await?;
        println!("✓ {} saved", FUNCTION_LABELS[1]);

        // 3. UuidGenerate — single-shot ID minting inside a pipeline.
        let uuid_fn = UserFunction::new(FUNCTION_LABELS[2], "Generate v4 UUID").with_function(
            Function::UuidGenerate {
                output_field: "id".to_string(),
            },
        );
        save_or_update_user(&client, uuid_fn).await?;
        println!("✓ {} saved", FUNCTION_LABELS[2]);

        // 4. TotpGenerate — RFC 6238 with the canonical reference secret.
        //    Pair with TotpVerify on the way back in if you wire this into
        //    a 2FA flow (skip default = ±1 step / ±30 s window).
        let totp = UserFunction::new(FUNCTION_LABELS[3], "Generate TOTP code").with_function(
            Function::TotpGenerate {
                secret: "{{env.TOTP_SECRET}}".to_string(),
                digits: Some(6),
                period: Some(30),
                algorithm: Some("sha1".to_string()),
                output_field: "code".to_string(),
            },
        );
        save_or_update_user(&client, totp).await?;
        println!("✓ {} saved", FUNCTION_LABELS[3]);

        // 5. Encoding bag — base64 + hex + slugify chained on one call.
        //    Each stage writes a separate output_field so the caller can
        //    see all three outcomes at once.
        let encoding = UserFunction::new(FUNCTION_LABELS[4], "Base64 / Hex / Slugify")
            .with_parameter(ParameterDefinition::new("title").required())
            .with_function(Function::Base64Encode {
                input: "{{title}}".to_string(),
                url_safe: Some(false),
                output_field: "title_b64".to_string(),
            })
            .with_function(Function::HexEncode {
                input: "{{title}}".to_string(),
                output_field: "title_hex".to_string(),
            })
            .with_function(Function::Slugify {
                input: "{{title}}".to_string(),
                output_field: "title_slug".to_string(),
            });
        save_or_update_user(&client, encoding).await?;
        println!("✓ {} saved", FUNCTION_LABELS[4]);

        println!("\nAll crypto-stage demos defined. Invoke any of them with:");
        println!(
            "  POST /api/functions/{} {{ \"payload\": \"hi\" }}",
            FUNCTION_LABELS[0]
        );
        println!(
            "  POST /api/functions/{} {{ \"plaintext\": \"secret\" }}",
            FUNCTION_LABELS[1]
        );
        println!("  POST /api/functions/{}", FUNCTION_LABELS[2]);
        println!("  POST /api/functions/{}", FUNCTION_LABELS[3]);
        println!(
            "  POST /api/functions/{} {{ \"title\": \"Héllo World\" }}",
            FUNCTION_LABELS[4]
        );

        Ok(())
    }
    .await;

    let mut cleanup_errors = Vec::new();
    for label in FUNCTION_LABELS {
        match client.delete_user_function(label).await {
            Ok(()) => {}
            Err(error) if is_missing_function(&error) => {}
            Err(error) => cleanup_errors.push(format!("{label}: {error}")),
        }
    }

    match (operation_result, cleanup_errors.is_empty()) {
        (Ok(()), true) => {
            println!("\n✓ Cleaned up demo functions");
            Ok(())
        }
        (Err(error), true) => Err(error),
        (Ok(()), false) => Err(std::io::Error::other(format!(
            "failed to clean owned functions: {}",
            cleanup_errors.join("; ")
        ))
        .into()),
        (Err(error), false) => Err(std::io::Error::other(format!(
            "{error}; cleanup also failed: {}",
            cleanup_errors.join("; ")
        ))
        .into()),
    }
}
