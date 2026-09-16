//! Transaction Example using ekoDB Rust Client
//!
//! Demonstrates:
//! - Begin transaction with the server default or an explicit isolation level
//! - Operations within a transaction
//! - Transaction status checking
//! - Commit transaction
//! - Rollback transaction

use ekodb_client::{options::UpdateOptions, Client, FieldType, Record};
use std::env;

const COLLECTION: &str = "client_transactions_accounts_rs";

fn balance(record: &Record) -> Result<i64, Box<dyn std::error::Error>> {
    match record.get("balance") {
        Some(FieldType::Integer(value)) => Ok(*value),
        Some(FieldType::Object(wrapper)) => match wrapper.get("value") {
            Some(FieldType::Integer(value)) => Ok(*value),
            _ => Err("balance wrapper did not contain an integer value".into()),
        },
        _ => Err("record did not contain an integer balance".into()),
    }
}

fn require_equal<T>(actual: T, expected: T, context: &str) -> Result<(), Box<dyn std::error::Error>>
where
    T: std::fmt::Debug + PartialEq,
{
    if actual != expected {
        return Err(format!("{context}: got {actual:?}, expected {expected:?}").into());
    }
    Ok(())
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
    println!("✓ Client created\n");

    let mut active_transactions = Vec::new();
    let operation_result = async {
        // Setup test data
        println!("=== Setup: Creating Test Accounts ===");
        let mut alice_record = Record::new();
        alice_record.insert("account_id", "ACC001");
        alice_record.insert("name", "Alice");
        alice_record.insert("balance", 1000);

        let alice = client.insert(COLLECTION, alice_record, None).await?;
        let alice_id = match alice.get("id") {
            Some(FieldType::String(id)) => id.clone(),
            _ => return Err("No ID returned for Alice".into()),
        };
        println!("Created Alice: $1000 - ID: {}", alice_id);

        let mut bob_record = Record::new();
        bob_record.insert("account_id", "ACC002");
        bob_record.insert("name", "Bob");
        bob_record.insert("balance", 500);

        let bob = client.insert(COLLECTION, bob_record, None).await?;
        let bob_id = match bob.get("id") {
            Some(FieldType::String(id)) => id.clone(),
            _ => return Err("No ID returned for Bob".into()),
        };
        println!("Created Bob: $500 - ID: {}\n", bob_id);

        // Example 1: Begin Transaction
        println!("=== Example 1: Begin Transaction ===");
        let tx_id = client.begin_transaction(None).await?;
        active_transactions.push(tx_id.clone());
        println!("Transaction ID (server-default isolation): {}\n", tx_id);

        // Example 2: Operations within Transaction
        println!("=== Example 2: Operations within Transaction ===");
        let mut alice_update = Record::new();
        alice_update.insert("balance", 800);
        client
            .update(
                COLLECTION,
                &alice_id,
                alice_update,
                Some(UpdateOptions::new().transaction_id(tx_id.clone())),
            )
            .await?;
        let mut bob_update = Record::new();
        bob_update.insert("balance", 700);
        client
            .update(
                COLLECTION,
                &bob_id,
                bob_update,
                Some(UpdateOptions::new().transaction_id(tx_id.clone())),
            )
            .await?;
        let alice_in_tx = client
            .find_by_id_in_transaction(COLLECTION, &alice_id, &tx_id, None)
            .await?;
        let bob_in_tx = client
            .find_by_id_in_transaction(COLLECTION, &bob_id, &tx_id, None)
            .await?;
        require_equal(balance(&alice_in_tx)?, 800, "Alice transactional balance")?;
        require_equal(balance(&bob_in_tx)?, 700, "Bob transactional balance")?;
        println!("Updated Alice: $1000 → $800");
        println!("Updated Bob: $500 → $700\n");

        // Example 3: Transaction Status
        println!("=== Example 3: Transaction Status ===");
        let status = client.get_transaction_status(&tx_id).await?;
        println!("Status: {}", status["state"]);
        println!("Operations: {}\n", status["operations_count"]);
        require_equal(
            status["state"].as_str(),
            Some("Active"),
            "transaction state",
        )?;
        require_equal(
            status["operations_count"].as_u64(),
            Some(2),
            "transaction operation count",
        )?;

        // Example 4: Commit Transaction
        println!("=== Example 4: Commit Transaction ===");
        client.commit_transaction(&tx_id).await?;
        active_transactions.retain(|id| id != &tx_id);
        println!("✓ Transaction committed\n");

        let committed_alice = client.find_by_id(COLLECTION, &alice_id, None).await?;
        let committed_bob = client.find_by_id(COLLECTION, &bob_id, None).await?;
        require_equal(balance(&committed_alice)?, 800, "committed Alice balance")?;
        require_equal(balance(&committed_bob)?, 700, "committed Bob balance")?;
        println!("✓ Verified committed balances: Alice=$800, Bob=$700\n");

        // Example 5: Rollback Demo
        println!("=== Example 5: Rollback Demo ===");
        let tx_id2 = client.begin_transaction(Some("ReadCommitted")).await?;
        active_transactions.push(tx_id2.clone());
        println!("New transaction: {}", tx_id2);

        let mut rollback_update = Record::new();
        rollback_update.insert("balance", 600);
        client
            .update(
                COLLECTION,
                &bob_id,
                rollback_update,
                Some(UpdateOptions::new().transaction_id(tx_id2.clone())),
            )
            .await?;
        let bob_in_rollback_tx = client
            .find_by_id_in_transaction(COLLECTION, &bob_id, &tx_id2, None)
            .await?;
        require_equal(
            balance(&bob_in_rollback_tx)?,
            600,
            "Bob balance before rollback",
        )?;
        println!("Updated Bob: $700 → $600 (in transaction)");

        let status2 = client.get_transaction_status(&tx_id2).await?;
        println!("Status before rollback: {}", status2["state"]);
        require_equal(
            status2["state"].as_str(),
            Some("Active"),
            "rollback transaction state",
        )?;
        require_equal(
            status2["operations_count"].as_u64(),
            Some(1),
            "rollback transaction operation count",
        )?;

        client.rollback_transaction(&tx_id2).await?;
        active_transactions.retain(|id| id != &tx_id2);
        println!("✓ Transaction rolled back\n");

        let bob_after_rollback = client.find_by_id(COLLECTION, &bob_id, None).await?;
        require_equal(
            balance(&bob_after_rollback)?,
            700,
            "Bob balance after rollback",
        )?;
        println!("✓ Verified Bob remains $700 after rollback\n");

        Ok::<(), Box<dyn std::error::Error>>(())
    }
    .await;

    // Cleanup
    println!("=== Cleanup ===");
    let mut cleanup_errors = Vec::new();
    for transaction_id in active_transactions.iter().rev() {
        if let Err(error) = client.rollback_transaction(transaction_id).await {
            cleanup_errors.push(format!("transaction {transaction_id}: {error}"));
        }
    }
    if let Err(error) = client.delete_collection(COLLECTION).await {
        cleanup_errors.push(format!("collection {COLLECTION}: {error}"));
    }

    match (operation_result, cleanup_errors.is_empty()) {
        (Err(operation_error), false) => {
            return Err(std::io::Error::other(format!(
                "operation failed: {operation_error}; cleanup also failed: {}",
                cleanup_errors.join("; ")
            ))
            .into());
        }
        (Err(operation_error), true) => return Err(operation_error),
        (Ok(()), false) => {
            return Err(std::io::Error::other(format!(
                "cleanup failed: {}",
                cleanup_errors.join("; ")
            ))
            .into());
        }
        (Ok(()), true) => {}
    }
    println!("✓ Deleted test account collection\n");

    println!("✓ All client transaction examples completed");
    Ok(())
}
