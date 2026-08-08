//! KV Precision Example - Float vs Decimal Comparison
//!
//! This example demonstrates the difference between using f64 floats
//! (which can lose precision) vs ekoDB's Decimal type (which preserves precision).
//!
//! Key takeaway: Use FieldType::decimal() for monetary values and other cases
//! where precision matters.

use ekodb_client::{get_string_value, get_value, Client, FieldType, Record};
use rust_decimal::Decimal;
use serde_json::json;
use std::env;
use std::str::FromStr;

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

    println!("=== KV Precision: Float vs Decimal ===\n");

    // =========================================================================
    // Test 1: Float Precision Loss
    // =========================================================================
    println!("=== Test 1: Using f64 Floats (MAY LOSE PRECISION) ===");

    let float_keys = vec![
        "product:float:1".to_string(),
        "product:float:2".to_string(),
        "product:float:3".to_string(),
    ];
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
    println!("\nRetrieved float prices:");
    for (i, record) in float_results.iter().enumerate() {
        if let Some(price_field) = record.get("price") {
            let price_val = get_value(&json!(price_field));
            let actual_price = price_val.as_f64().unwrap_or(0.0);
            let expected = float_prices[i];
            let status = if (expected - actual_price).abs() < f64::EPSILON {
                "OK"
            } else {
                "PRECISION LOST"
            };
            println!(
                "  {}: ${} (expected ${}) {}",
                float_names[i], actual_price, expected, status
            );
        }
    }

    // =========================================================================
    // Test 2: Decimal Precision Preserved
    // =========================================================================
    println!("\n=== Test 2: Using FieldType::decimal() (PRESERVES PRECISION) ===");

    let decimal_keys = vec![
        "product:decimal:1".to_string(),
        "product:decimal:2".to_string(),
        "product:decimal:3".to_string(),
    ];
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
    println!("\nRetrieved decimal prices:");
    for (i, record) in decimal_results.iter().enumerate() {
        if let Some(price_field) = record.get("price") {
            let actual_str = get_string_value(&json!(price_field))
                .unwrap_or_else(|| format!("{:?}", get_value(&json!(price_field))));
            println!(
                "  {}: ${} (expected ${}) OK",
                float_names[i], actual_str, decimal_prices[i]
            );
        }
    }

    // =========================================================================
    // Test 3: Real-world calculation comparison
    // =========================================================================
    println!("\n=== Test 3: Sum Calculation Comparison ===");

    // Sum float prices
    let mut float_sum = 0.0_f64;
    for record in &float_results {
        if let Some(price_field) = record.get("price") {
            let price_val = get_value(&json!(price_field));
            float_sum += price_val.as_f64().unwrap_or(0.0);
        }
    }
    println!("  Float sum: ${} (expected $119.97)", float_sum);

    // Sum decimal prices (values come back as strings, parse for arithmetic)
    let mut decimal_sum = Decimal::ZERO;
    for record in &decimal_results {
        if let Some(price_field) = record.get("price") {
            let price_str = get_string_value(&json!(price_field)).unwrap_or_default();
            if let Ok(d) = Decimal::from_str(&price_str) {
                decimal_sum += d;
            }
        }
    }
    println!("  Decimal sum: ${} (expected $119.97)", decimal_sum);

    // =========================================================================
    // Test 4: Extreme precision example
    // =========================================================================
    println!("\n=== Test 4: Extreme Precision Example ===");

    // Store 0.1 + 0.2 as a float (classic floating-point issue)
    client
        .kv_set(
            "precision:float",
            json!({"amount": 0.1_f64 + 0.2_f64}),
            None,
        )
        .await?;

    // Store "0.30" as a decimal (exact representation)
    client
        .kv_set(
            "precision:decimal",
            json!({"amount": FieldType::decimal(Decimal::from_str("0.30")?)}),
            None,
        )
        .await?;

    // Retrieve and compare
    if let Some(float_val) = client.kv_get("precision:float").await? {
        // kv_get returns the raw JSON value; parse to access fields
        if let Some(amount) = float_val.get("amount") {
            let extracted = get_value(amount);
            println!("  Float 0.1 + 0.2 = {} (should be 0.3)", extracted);
        }
    }

    if let Some(decimal_val) = client.kv_get("precision:decimal").await? {
        if let Some(amount) = decimal_val.get("amount") {
            let extracted = get_value(amount);
            println!("  Decimal \"0.30\" = {} (exact!)", extracted);
        }
    }

    // =========================================================================
    // Cleanup
    // =========================================================================
    println!("\n=== Cleanup ===");
    let mut all_keys = float_keys;
    all_keys.extend(decimal_keys);
    all_keys.push("precision:float".to_string());
    all_keys.push("precision:decimal".to_string());
    client.kv_batch_delete(all_keys).await?;
    println!("Cleaned up test keys");

    println!("\n=== Summary ===");
    println!("Use FieldType::decimal() for monetary values, percentages, and");
    println!("any case where floating-point errors are unacceptable.");
    println!("Decimal values are stored as exact strings internally,");
    println!("preserving precision across all operations.");

    Ok(())
}
