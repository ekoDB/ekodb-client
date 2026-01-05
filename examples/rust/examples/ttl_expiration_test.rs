use ekodb_client::{Client, FieldType, Record};
use std::env;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║     TTL EXPIRATION VERIFICATION TEST                   ║");
    println!("╚════════════════════════════════════════════════════════╝");
    println!();
    println!("This test verifies that document TTL expiration works correctly.");
    println!("We will insert documents with short TTL and verify they expire.");
    println!();

    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;
    println!("✓ Client connected");

    let collection = "ttl_expiration_test_rs";
    let ttl_seconds = 3;

    println!();
    println!("═══════════════════════════════════════════════════════════");
    println!("TEST 1: Document TTL Expiration");
    println!("═══════════════════════════════════════════════════════════");

    println!("\n[Step 1] Insert document with {} second TTL", ttl_seconds);
    println!("  Input: {{name: 'TTL Test', value: 'should expire'}}");
    println!("  TTL: {}s", ttl_seconds);

    let mut record = Record::new();
    record.insert("name", "TTL Test");
    record.insert("value", "should expire");
    let record = record.with_ttl(&format!("{}s", ttl_seconds));

    let doc = client.insert(collection, record, None).await?;
    let doc_id = match doc.get("id") {
        Some(FieldType::String(s)) => s.clone(),
        _ => return Err("No ID returned".into()),
    };
    println!("  Output: Document ID = {}", doc_id);
    println!("  ✓ PASS: Document inserted");

    println!("\n[Step 2] Verify document exists immediately");
    println!("  Input: find_by_id({})", doc_id);

    let found = client.find_by_id(collection, &doc_id, None).await?;
    let name_val = match found.fields.get("name") {
        Some(FieldType::String(s)) => s.as_str(),
        _ => "unknown",
    };
    println!("  Output: Found document with name = {}", name_val);
    println!("  ✓ PASS: Document exists");

    let wait_time = ttl_seconds + 2;
    println!("\n[Step 3] Wait for TTL to expire ({}s)", wait_time);
    print!("  Waiting");
    for _ in 0..wait_time {
        thread::sleep(Duration::from_secs(1));
        print!(".");
        std::io::Write::flush(&mut std::io::stdout())?;
    }
    println!(" done");
    println!("  ✓ PASS: Wait complete");

    println!("\n[Step 4] Verify document has expired");
    println!("  Input: find_by_id({})", doc_id);

    match client.find_by_id(collection, &doc_id, None).await {
        Ok(_) => {
            println!("  Output: Document still exists!");
            return Err("❌ FAILED: Document should have expired but still exists!".into());
        }
        Err(e) => {
            println!("  Output: Error (expected) - {}", e);
            println!("  ✓ PASS: Document expired (not found error)");
        }
    }

    println!("\n═══════════════════════════════════════════════════════════");
    println!("CLEANUP");
    println!("═══════════════════════════════════════════════════════════");

    if let Err(e) = client.delete_collection(collection).await {
        println!("Warning: cleanup failed: {}", e);
    } else {
        println!("✓ Deleted test collection");
    }

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║              ALL TESTS PASSED ✓                        ║");
    println!("╚════════════════════════════════════════════════════════╝");

    Ok(())
}
