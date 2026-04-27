//! Crypto stages — comprehensive demo of every primitive.
//!
//! Builds and saves five small stored functions covering the full
//! crypto-stage surface added in ekoDB 0.42.0:
//!
//!   - `crypto_demo_hmac`     HmacSign + HmacVerify (round trip)
//!   - `crypto_demo_aes`      AesEncrypt + AesDecrypt (round trip)
//!   - `crypto_demo_uuid`     UuidGenerate
//!   - `crypto_demo_totp`     TotpGenerate (RFC 6238 reference secret)
//!   - `crypto_demo_encoding` Base64 + Hex + Slugify (single function chain)
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

    // 1. HMAC sign + verify round trip — single pipeline so the verify
    //    can read the signed value off the same working record.
    let hmac = UserFunction::new("crypto_demo_hmac", "HMAC sign + verify")
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
    let _ = client.save_user_function(hmac).await;
    println!("✓ crypto_demo_hmac saved");

    // 2. AES-256-GCM encrypt + decrypt round trip. Decrypt reads the
    //    {ciphertext, nonce} envelope off `working_data[0]` written by
    //    encrypt. Wrong-key decrypt is fail-closed (writes null).
    let aes = UserFunction::new("crypto_demo_aes", "AES encrypt + decrypt")
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
    let _ = client.save_user_function(aes).await;
    println!("✓ crypto_demo_aes saved");

    // 3. UuidGenerate — single-shot ID minting inside a pipeline.
    let uuid_fn = UserFunction::new("crypto_demo_uuid", "Generate v4 UUID").with_function(
        Function::UuidGenerate {
            output_field: "id".to_string(),
        },
    );
    let _ = client.save_user_function(uuid_fn).await;
    println!("✓ crypto_demo_uuid saved");

    // 4. TotpGenerate — RFC 6238 with the canonical reference secret.
    //    Pair with TotpVerify on the way back in if you wire this into
    //    a 2FA flow (skip default = ±1 step / ±30 s window).
    let totp = UserFunction::new("crypto_demo_totp", "Generate TOTP code").with_function(
        Function::TotpGenerate {
            secret: "{{env.TOTP_SECRET}}".to_string(),
            digits: Some(6),
            period: Some(30),
            algorithm: Some("sha1".to_string()),
            output_field: "code".to_string(),
        },
    );
    let _ = client.save_user_function(totp).await;
    println!("✓ crypto_demo_totp saved");

    // 5. Encoding bag — base64 + hex + slugify chained on one call.
    //    Each stage writes a separate output_field so the caller can
    //    see all three outcomes at once.
    let encoding = UserFunction::new("crypto_demo_encoding", "Base64 / Hex / Slugify")
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
    let _ = client.save_user_function(encoding).await;
    println!("✓ crypto_demo_encoding saved");

    println!("\nAll crypto-stage demos defined. Invoke any of them with:");
    println!("  POST /api/functions/crypto_demo_hmac     {{ \"payload\": \"hi\" }}");
    println!("  POST /api/functions/crypto_demo_aes      {{ \"plaintext\": \"secret\" }}");
    println!("  POST /api/functions/crypto_demo_uuid");
    println!("  POST /api/functions/crypto_demo_totp");
    println!("  POST /api/functions/crypto_demo_encoding {{ \"title\": \"Héllo World\" }}");

    // Cleanup so re-runs don't hit duplicate-label errors.
    for label in [
        "crypto_demo_hmac",
        "crypto_demo_aes",
        "crypto_demo_uuid",
        "crypto_demo_totp",
        "crypto_demo_encoding",
    ] {
        let _ = client.delete_user_function(label).await;
    }
    println!("\n✓ Cleaned up demo functions");

    Ok(())
}
