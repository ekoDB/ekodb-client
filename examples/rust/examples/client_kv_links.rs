//! ekoDB Rust Client - KV Document Linking Example
//!
//! Demonstrates linking and unlinking documents to KV keys,
//! building on the basic KV operations from client_kv_operations.rs.
//!
//! Run with: `cargo run --example client_kv_links`

use ekodb_client::{Client, Record};
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    println!("=== ekoDB KV Document Linking Example (Rust) ===\n");

    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    let collection = "kv_links_example_rust";

    // 1. Insert documents to link against
    println!("--- Inserting documents ---");
    let mut doc1 = Record::new();
    doc1.insert("title", "Project Alpha");
    doc1.insert("status", "active");
    let inserted1 = client.insert(collection, doc1, None).await?;
    let doc1_id = inserted1.get_string("id").unwrap_or_default().to_string();
    println!(
        "Inserted doc1: {:?} (id: {})",
        inserted1.get_string("title"),
        doc1_id
    );

    let mut doc2 = Record::new();
    doc2.insert("title", "Project Beta");
    doc2.insert("status", "planning");
    let inserted2 = client.insert(collection, doc2, None).await?;
    let doc2_id = inserted2.get_string("id").unwrap_or_default().to_string();
    println!(
        "Inserted doc2: {:?} (id: {})",
        inserted2.get_string("title"),
        doc2_id
    );

    // 2. Set a KV key to link documents to
    println!("\n--- Setting KV key ---");
    client
        .kv_set(
            "user:alice:projects",
            json!({"owner": "alice", "role": "lead"}),
            None,
        )
        .await?;
    println!("Set key: user:alice:projects");

    // 3. Link documents to the KV key
    println!("\n--- Linking documents ---");
    let link1 = client
        .kv_link("user:alice:projects", collection, &doc1_id)
        .await?;
    println!("Linked doc1: {}", link1);

    let link2 = client
        .kv_link("user:alice:projects", collection, &doc2_id)
        .await?;
    println!("Linked doc2: {}", link2);

    // 4. Get all links for the key
    println!("\n--- Getting links ---");
    let links = client.kv_get_links("user:alice:projects").await?;
    println!("Links for user:alice:projects: {}", links);

    // 5. Unlink one document
    println!("\n--- Unlinking doc1 ---");
    let unlink_result = client
        .kv_unlink("user:alice:projects", collection, &doc1_id)
        .await?;
    println!("Unlinked doc1: {}", unlink_result);

    // 6. Verify remaining links
    println!("\n--- Verifying remaining links ---");
    let remaining = client.kv_get_links("user:alice:projects").await?;
    println!("Remaining links: {}", remaining);

    // Cleanup
    println!("\n--- Cleanup ---");
    client.kv_delete("user:alice:projects").await?;
    println!("Deleted KV key");
    client.delete(collection, &doc1_id, None).await?;
    client.delete(collection, &doc2_id, None).await?;
    println!("Deleted documents");
    client.delete_collection(collection).await?;
    println!("Deleted collection");

    println!("\n=== All KV linking operations completed ===");
    Ok(())
}
