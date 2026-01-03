//! Transaction Example using ekoDB Rust Client
//!
//! Demonstrates:
//! - Begin transaction with isolation level
//! - Operations within a transaction
//! - Transaction status checking
//! - Commit transaction
//! - Rollback transaction

use ekodb_client::{Client, FieldType, Record};
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
    println!("✓ Client created\n");

    // Setup test data
    println!("=== Setup: Creating Test Accounts ===");
    let mut alice_record = Record::new();
    alice_record.insert("account_id", "ACC001");
    alice_record.insert("name", "Alice");
    alice_record.insert("balance", 1000);

    let alice = client.insert("test_accounts", alice_record, None).await?;
    let alice_id = match alice.get("id") {
        Some(FieldType::String(id)) => id.clone(),
        _ => return Err("No ID returned for Alice".into()),
    };
    println!("Created Alice: $1000 - ID: {}", alice_id);

    let mut bob_record = Record::new();
    bob_record.insert("account_id", "ACC002");
    bob_record.insert("name", "Bob");
    bob_record.insert("balance", 500);

    let bob = client.insert("test_accounts", bob_record, None).await?;
    let bob_id = match bob.get("id") {
        Some(FieldType::String(id)) => id.clone(),
        _ => return Err("No ID returned for Bob".into()),
    };
    println!("Created Bob: $500 - ID: {}\n", bob_id);

    // Example 1: Begin Transaction
    println!("=== Example 1: Begin Transaction ===");
    let tx_id = client.begin_transaction("ReadCommitted").await?;
    println!("Transaction ID: {}\n", tx_id);

    // Example 2: Operations within Transaction
    println!("=== Example 2: Operations within Transaction ===");
    println!("Updated Alice: $1000 → $800 (simulated)");
    println!("Updated Bob: $500 → $700 (simulated)\n");

    // Example 3: Transaction Status
    println!("=== Example 3: Transaction Status ===");
    let status = client.get_transaction_status(&tx_id).await?;
    println!("Status: {}", status["state"]);
    println!("Operations: {}\n", status["operations_count"]);

    // Example 4: Commit Transaction
    println!("=== Example 4: Commit Transaction ===");
    client.commit_transaction(&tx_id).await?;
    println!("✓ Transaction committed\n");

    // Example 5: Rollback Demo
    println!("=== Example 5: Rollback Demo ===");
    let tx_id2 = client.begin_transaction("ReadCommitted").await?;
    println!("New transaction: {}", tx_id2);

    let status2 = client.get_transaction_status(&tx_id2).await?;
    println!("Status before rollback: {}", status2["state"]);

    client.rollback_transaction(&tx_id2).await?;
    println!("✓ Transaction rolled back\n");

    // Cleanup
    println!("=== Cleanup ===");
    client.delete("test_accounts", &alice_id, None).await?;
    client.delete("test_accounts", &bob_id, None).await?;
    println!("✓ Deleted test accounts\n");

    println!("✓ All client transaction examples completed");
    Ok(())
}
