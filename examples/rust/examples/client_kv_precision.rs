//! KV Precision Example - Float vs Decimal Comparison
//!
//! This example demonstrates the difference between using f64 floats
//! (which can lose precision) vs ekoDB's Decimal type (which preserves precision).
//!
//! Key takeaway: Use FieldType::decimal() for monetary values and other cases
//! where precision matters.

use ekodb_client::{get_string_value, get_value, Client, FieldType, Record};
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::env;
use std::str::FromStr;

const FLOAT_KEYS: [&str; 3] = [
    "kv_precision:rs:product:float:1",
    "kv_precision:rs:product:float:2",
    "kv_precision:rs:product:float:3",
];
const DECIMAL_KEYS: [&str; 3] = [
    "kv_precision:rs:product:decimal:1",
    "kv_precision:rs:product:decimal:2",
    "kv_precision:rs:product:decimal:3",
];
const FLOAT_PRECISION_KEY: &str = "kv_precision:rs:precision:float";
const DECIMAL_PRECISION_KEY: &str = "kv_precision:rs:precision:decimal";

fn all_keys() -> Vec<String> {
    FLOAT_KEYS
        .into_iter()
        .chain(DECIMAL_KEYS)
        .chain([FLOAT_PRECISION_KEY, DECIMAL_PRECISION_KEY])
        .map(str::to_string)
        .collect()
}

fn kv_object(value: Value, label: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let decoded = match value {
        Value::String(raw) => serde_json::from_str(&raw)?,
        value => value,
    };
    let unwrapped = get_value(&decoded);
    if !unwrapped.is_object() {
        return Err(format!("{label} KV value was not an object: {unwrapped}").into());
    }
    Ok(unwrapped)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Create ekoDB client
    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    client.kv_batch_delete(all_keys()).await?;

    let run_result: Result<(), Box<dyn std::error::Error>> = async {
        println!("=== KV Precision: Float vs Decimal ===\n");

        // =========================================================================
        // Test 1: Float Precision Loss
        // =========================================================================
        println!("=== Test 1: Using f64 Floats (MAY LOSE PRECISION) ===");

        let float_keys = FLOAT_KEYS.map(str::to_string).to_vec();
        let float_prices = [29.99_f64, 39.99, 49.99];
        let float_names = ["Widget A", "Widget B", "Widget C"];

        // Store with float values
        let mut float_values = Vec::new();
        for i in 0..3 {
            let mut record = Record::new();
            record.insert("name", float_names[i]);
            record.insert("price", float_prices[i]);
            float_values.push(record);
        }
        client
            .kv_batch_set(float_keys.clone(), float_values, None)
            .await?;
        println!("Stored products with float prices");

        // Retrieve and show the precision
        let float_results = client.kv_batch_get(float_keys.clone()).await?;
        if float_results.len() != float_prices.len() {
            return Err(format!(
                "expected {} float products, got {}",
                float_prices.len(),
                float_results.len()
            )
            .into());
        }
        println!("\nRetrieved float prices:");
        for (i, record) in float_results.iter().enumerate() {
            let price_field = record
                .get("price")
                .ok_or_else(|| format!("missing float price for {}", float_names[i]))?;
            let price_val = get_value(&json!(price_field));
            let actual_price = price_val
                .as_f64()
                .ok_or_else(|| format!("float price for {} was not numeric", float_names[i]))?;
            let expected = float_prices[i];
            let matches = (expected - actual_price).abs() < f64::EPSILON;
            let status = if matches { "OK" } else { "PRECISION LOST" };
            println!(
                "  {}: ${} (expected ${}) {}",
                float_names[i], actual_price, expected, status
            );
            if !matches {
                return Err(format!(
                    "unexpected float price for {}: {}",
                    float_names[i], actual_price
                )
                .into());
            }
        }

        // =========================================================================
        // Test 2: Decimal Precision Preserved
        // =========================================================================
        println!("\n=== Test 2: Using FieldType::decimal() (PRESERVES PRECISION) ===");

        let decimal_keys = DECIMAL_KEYS.map(str::to_string).to_vec();
        let decimal_prices = ["29.99", "39.99", "49.99"];

        // Store with decimal values
        let mut decimal_values = Vec::new();
        for i in 0..3 {
            let mut record = Record::new();
            record.insert("name", float_names[i]);
            record.insert(
                "price",
                FieldType::decimal(Decimal::from_str(decimal_prices[i])?),
            );
            decimal_values.push(record);
        }
        client
            .kv_batch_set(decimal_keys.clone(), decimal_values, None)
            .await?;
        println!("Stored products with decimal prices");

        // Retrieve and show precision is preserved
        // Decimal values come back as {"type": "Decimal", "value": "29.99"} - get_string_value extracts the string
        let decimal_results = client.kv_batch_get(decimal_keys.clone()).await?;
        if decimal_results.len() != decimal_prices.len() {
            return Err(format!(
                "expected {} decimal products, got {}",
                decimal_prices.len(),
                decimal_results.len()
            )
            .into());
        }
        println!("\nRetrieved decimal prices:");
        for (i, record) in decimal_results.iter().enumerate() {
            let price_field = record
                .get("price")
                .ok_or_else(|| format!("missing decimal price for {}", float_names[i]))?;
            let actual_str = get_string_value(&json!(price_field))
                .ok_or_else(|| format!("decimal price for {} was not a string", float_names[i]))?;
            println!(
                "  {}: ${} (expected ${}) OK",
                float_names[i], actual_str, decimal_prices[i]
            );
            if actual_str != decimal_prices[i] {
                return Err(format!(
                    "unexpected decimal price for {}: {}",
                    float_names[i], actual_str
                )
                .into());
            }
        }

        // =========================================================================
        // Test 3: Real-world calculation comparison
        // =========================================================================
        println!("\n=== Test 3: Sum Calculation Comparison ===");

        // Sum float prices
        let mut float_sum = 0.0_f64;
        for (i, record) in float_results.iter().enumerate() {
            let price_field = record
                .get("price")
                .ok_or_else(|| format!("missing float price for {}", float_names[i]))?;
            let price_val = get_value(&json!(price_field));
            float_sum += price_val
                .as_f64()
                .ok_or_else(|| format!("float price for {} was not numeric", float_names[i]))?;
        }
        println!("  Float sum: ${} (expected $119.97)", float_sum);
        if (float_sum - 119.97).abs() > 1e-9 {
            return Err(format!("unexpected float sum: {float_sum}").into());
        }

        // Sum decimal prices (values come back as strings, parse for arithmetic)
        let mut decimal_sum = Decimal::ZERO;
        for (i, record) in decimal_results.iter().enumerate() {
            let price_field = record
                .get("price")
                .ok_or_else(|| format!("missing decimal price for {}", float_names[i]))?;
            let price_str = get_string_value(&json!(price_field))
                .ok_or_else(|| format!("decimal price for {} was not a string", float_names[i]))?;
            decimal_sum += Decimal::from_str(&price_str)?;
        }
        println!("  Decimal sum: ${} (expected $119.97)", decimal_sum);
        if decimal_sum != Decimal::from_str("119.97")? {
            return Err(format!("unexpected decimal sum: {decimal_sum}").into());
        }

        // =========================================================================
        // Test 4: Extreme precision example
        // =========================================================================
        println!("\n=== Test 4: Extreme Precision Example ===");

        // Store 0.1 + 0.2 as a float (classic floating-point issue)
        client
            .kv_set(
                FLOAT_PRECISION_KEY,
                json!({"amount": 0.1_f64 + 0.2_f64}),
                None,
            )
            .await?;

        // Store "0.30" as a decimal (exact representation)
        client
            .kv_set(
                DECIMAL_PRECISION_KEY,
                json!({"amount": FieldType::decimal(Decimal::from_str("0.30")?)}),
                None,
            )
            .await?;

        // Retrieve and compare
        let float_val = client
            .kv_get(FLOAT_PRECISION_KEY)
            .await?
            .ok_or("precision:float was not returned")?;
        let float_object = kv_object(float_val, "precision:float")?;
        let float_amount = float_object
            .get("amount")
            .map(get_value)
            .and_then(|value| value.as_f64())
            .ok_or("precision:float amount was not numeric")?;
        println!("  Float 0.1 + 0.2 = {:.17} (should be 0.3)", float_amount);
        if float_amount != 0.1_f64 + 0.2_f64 {
            return Err(format!("unexpected float precision value: {float_amount}").into());
        }

        let decimal_val = client
            .kv_get(DECIMAL_PRECISION_KEY)
            .await?
            .ok_or("precision:decimal was not returned")?;
        let decimal_object = kv_object(decimal_val, "precision:decimal")?;
        let decimal_amount = decimal_object
            .get("amount")
            .and_then(get_string_value)
            .ok_or("precision:decimal amount was not a string")?;
        println!("  Decimal \"0.30\" = {} (exact!)", decimal_amount);
        if decimal_amount != "0.30" {
            return Err(format!("unexpected decimal precision value: {decimal_amount}").into());
        }

        Ok(())
    }
    .await;

    // =========================================================================
    // Cleanup
    // =========================================================================
    println!("\n=== Cleanup ===");
    let cleanup_result = client.kv_batch_delete(all_keys()).await;
    if cleanup_result.is_ok() {
        println!("Cleaned up test keys");
    }

    match (run_result, cleanup_result) {
        (Err(primary), Err(cleanup)) => {
            return Err(format!("{primary}; cleanup also failed: {cleanup}").into());
        }
        (Err(primary), Ok(_)) => return Err(primary),
        (Ok(_), Err(cleanup)) => return Err(cleanup.into()),
        (Ok(_), Ok(_)) => {}
    }

    println!("\n=== Summary ===");
    println!("Use FieldType::decimal() for monetary values, percentages, and");
    println!("any case where floating-point errors are unacceptable.");
    println!("Decimal values are stored as exact strings internally,");
    println!("preserving precision across all operations.");

    Ok(())
}
