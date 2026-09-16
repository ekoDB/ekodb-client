//! ekoDB Rust Client - KV Document Linking Example
//!
//! Demonstrates linking and unlinking documents to KV keys,
//! building on the basic KV operations from client_kv_operations.rs.
//!
//! Run with: `cargo run --example client_kv_links`

use ekodb_client::{Client, Record};
use serde_json::json;
use std::error::Error;

const COLLECTION: &str = "kv_links_example_rs";
const KV_KEY: &str = "kv_links:rs:user:alice:projects";

fn validate_links(
    links: &serde_json::Value,
    collection: &str,
    expected_ids: &[&str],
) -> Result<(), Box<dyn Error>> {
    let items = links.as_array().ok_or("links response was not an array")?;
    if items.len() != expected_ids.len() {
        return Err(format!(
            "links response contained {} items; expected {}",
            items.len(),
            expected_ids.len()
        )
        .into());
    }
    for expected_id in expected_ids {
        let found = items.iter().any(|item| {
            item.get("collection").and_then(|value| value.as_str()) == Some(collection)
                && item.get("document_id").and_then(|value| value.as_str()) == Some(*expected_id)
        });
        if !found {
            return Err(format!("missing link for document {expected_id}").into());
        }
    }
    Ok(())
}

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

    match client.kv_delete(KV_KEY).await {
        Ok(()) | Err(ekodb_client::Error::Api { code: 404, .. }) => {}
        Err(error) => return Err(error.into()),
    }
    match client.delete_collection(COLLECTION).await {
        Ok(()) | Err(ekodb_client::Error::Api { code: 404, .. }) => {}
        Err(error) => return Err(error.into()),
    }

    let mut document_ids = Vec::new();
    let mut kv_key_created = false;
    let operation_result = async {
        // 1. Insert documents to link against
        println!("--- Inserting documents ---");
        let mut doc1 = Record::new();
        doc1.insert("title", "Project Alpha");
        doc1.insert("status", "active");
        let inserted1 = client.insert(COLLECTION, doc1, None).await?;
        let doc1_id = inserted1
            .get_string("id")
            .filter(|id| !id.is_empty())
            .ok_or("doc1 insert response did not include an id")?
            .to_string();
        document_ids.push(doc1_id.clone());
        println!("Inserted doc1: Project Alpha (id: {doc1_id})");

        let mut doc2 = Record::new();
        doc2.insert("title", "Project Beta");
        doc2.insert("status", "planning");
        let inserted2 = client.insert(COLLECTION, doc2, None).await?;
        let doc2_id = inserted2
            .get_string("id")
            .filter(|id| !id.is_empty())
            .ok_or("doc2 insert response did not include an id")?
            .to_string();
        document_ids.push(doc2_id.clone());
        println!("Inserted doc2: Project Beta (id: {doc2_id})");

        // 2. Set a KV key to link documents to
        println!("\n--- Setting KV key ---");
        client
            .kv_set(KV_KEY, json!({"owner": "alice", "role": "lead"}), None)
            .await?;
        kv_key_created = true;
        println!("Set key: {KV_KEY}");

        // 3. Link documents to the KV key
        println!("\n--- Linking documents ---");
        let link1 = client.kv_link(KV_KEY, COLLECTION, &doc1_id).await?;
        println!("Linked doc1: {}", link1);

        let link2 = client.kv_link(KV_KEY, COLLECTION, &doc2_id).await?;
        println!("Linked doc2: {}", link2);

        // 4. Get all links for the key
        println!("\n--- Getting links ---");
        let links = client.kv_get_links(KV_KEY).await?;
        println!("Links for {KV_KEY}: {links}");
        validate_links(&links, COLLECTION, &[&doc1_id, &doc2_id])?;

        // 5. Unlink one document
        println!("\n--- Unlinking doc1 ---");
        let unlink_result = client.kv_unlink(KV_KEY, COLLECTION, &doc1_id).await?;
        println!("Unlinked doc1: {}", unlink_result);

        // 6. Verify remaining links
        println!("\n--- Verifying remaining links ---");
        let remaining = client.kv_get_links(KV_KEY).await?;
        println!("Remaining links: {}", remaining);
        validate_links(&remaining, COLLECTION, &[&doc2_id])?;
        Ok::<(), Box<dyn Error>>(())
    }
    .await;

    // Cleanup
    println!("\n--- Cleanup ---");
    let mut cleanup_errors = Vec::new();
    if let Some(doc2_id) = document_ids.get(1) {
        match client.kv_unlink(KV_KEY, COLLECTION, doc2_id).await {
            Ok(_) => println!("Unlinked remaining document"),
            Err(error) => cleanup_errors.push(format!("remaining link: {error}")),
        }
    }
    if kv_key_created {
        match client.kv_delete(KV_KEY).await {
            Ok(()) | Err(ekodb_client::Error::Api { code: 404, .. }) => {
                println!("Deleted KV key")
            }
            Err(error) => cleanup_errors.push(format!("KV key {KV_KEY}: {error}")),
        }
    }
    let mut all_documents_deleted = true;
    for document_id in &document_ids {
        if let Err(error) = client.delete(COLLECTION, document_id, None).await {
            all_documents_deleted = false;
            cleanup_errors.push(format!("document {document_id}: {error}"));
        }
    }
    if all_documents_deleted {
        println!("Deleted documents");
    }
    match client.delete_collection(COLLECTION).await {
        Ok(()) | Err(ekodb_client::Error::Api { code: 404, .. }) => {
            println!("Deleted collection")
        }
        Err(error) => cleanup_errors.push(format!("collection {COLLECTION}: {error}")),
    }

    if let Err(operation_error) = operation_result {
        if !cleanup_errors.is_empty() {
            return Err(format!(
                "{operation_error}; cleanup also failed: {}",
                cleanup_errors.join("; ")
            )
            .into());
        }
        return Err(operation_error);
    }
    if !cleanup_errors.is_empty() {
        return Err(format!("cleanup failed: {}", cleanup_errors.join("; ")).into());
    }

    println!("\n=== All KV linking operations completed ===");
    Ok(())
}
