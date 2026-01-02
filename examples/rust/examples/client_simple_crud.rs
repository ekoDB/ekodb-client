//! Simple CRUD operations using ekodb_client library
//!
//! This example demonstrates the same operations as simple_crud.rs
//! but using the ekodb_client library for cleaner, type-safe code.
//!
//! Compare with simple_crud.rs to see the difference!

use ekodb_client::{Client, FieldType, Record};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_key =
        env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());

    // Create ekoDB client (automatically handles token exchange)
    let client = Client::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    println!("✓ Client created (token exchange happens automatically)");

    let collection = "client_simple_crud_rust";

    // Example 1: Insert a document with various field types
    println!("\n=== Insert Document ===");
    let mut record = Record::new();
    record.insert("name", "Test Record");
    record.insert("value", 42);
    record.insert("active", true);
    record.insert("price", 99.99);
    record.insert("created_at", chrono::Utc::now().to_rfc3339());
    record.insert("user_id", "550e8400-e29b-41d4-a716-446655440000");
    record.insert(
        "tags",
        FieldType::array(vec![
            FieldType::string("tag1"),
            FieldType::string("tag2"),
            FieldType::string("tag3"),
        ]),
    );
    record.insert(
        "metadata",
        serde_json::json!({"key": "value", "nested": {"deep": true}}),
    );
    record.insert(
        "embedding",
        FieldType::vector(vec![0.1, 0.2, 0.3, 0.4, 0.5]),
    );
    record.insert(
        "categories",
        FieldType::set(vec![
            FieldType::string("electronics"),
            FieldType::string("computers"),
        ]),
    );
    record.insert(
        "data",
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, b"hello world"),
    );

    let inserted = client.insert(collection, record, None).await?;
    println!("Inserted: {:?}", inserted);

    // Extract the ID
    let doc_id = if let Some(id_field) = inserted.get("id") {
        match id_field {
            FieldType::String(id) => id.clone(),
            _ => return Err("Expected string ID".into()),
        }
    } else {
        return Err("No ID returned".into());
    };

    // Example 2: Find by ID
    println!("\n=== Find by ID ===");
    let found_doc = client.find_by_id(collection, &doc_id, None).await?;
    println!("Found: {:?}", found_doc);

    // Example 2b: Extract field values using type-specific getValue utilities
    println!("\n=== Extract Field Values (All Types) ===");
    // ekoDB returns fields as FieldType enum
    // For direct extraction, we can pattern match on FieldType
    let name_value = match found_doc.get("name") {
        Some(FieldType::Object(map)) => match map.get("value") {
            Some(FieldType::String(s)) => Some(s.clone()),
            _ => None,
        },
        Some(FieldType::String(s)) => Some(s.clone()),
        _ => None,
    };
    let value_value = match found_doc.get("value") {
        Some(FieldType::Object(map)) => match map.get("value") {
            Some(FieldType::Integer(i)) => Some(*i),
            _ => None,
        },
        Some(FieldType::Integer(i)) => Some(*i),
        _ => None,
    };
    let active_value = match found_doc.get("active") {
        Some(FieldType::Object(map)) => match map.get("value") {
            Some(FieldType::Boolean(b)) => Some(*b),
            _ => None,
        },
        Some(FieldType::Boolean(b)) => Some(*b),
        _ => None,
    };
    let price_value = match found_doc.get("price") {
        Some(FieldType::Object(map)) => match map.get("value") {
            Some(FieldType::Float(f)) => Some(*f),
            _ => None,
        },
        Some(FieldType::Float(f)) => Some(*f),
        _ => None,
    };
    let created_at_value = match found_doc.get("created_at") {
        Some(FieldType::Object(map)) => match map.get("value") {
            Some(FieldType::String(s)) => Some(s.clone()),
            Some(FieldType::DateTime(dt)) => Some(dt.to_rfc3339()),
            _ => None,
        },
        Some(FieldType::String(s)) => Some(s.clone()),
        Some(FieldType::DateTime(dt)) => Some(dt.to_rfc3339()),
        _ => None,
    };
    let user_id_value = match found_doc.get("user_id") {
        Some(FieldType::Object(map)) => match map.get("value") {
            Some(FieldType::String(s)) => Some(s.clone()),
            _ => None,
        },
        Some(FieldType::String(s)) => Some(s.clone()),
        _ => None,
    };
    let tags_value = match found_doc.get("tags") {
        Some(FieldType::Object(map)) => match map.get("value") {
            Some(FieldType::Array(arr)) => Some(arr.clone()),
            _ => None,
        },
        Some(FieldType::Array(arr)) => Some(arr.clone()),
        _ => None,
    };
    let metadata_value = match found_doc.get("metadata") {
        Some(FieldType::Object(map)) => match map.get("value") {
            Some(FieldType::Object(obj)) => Some(obj.clone()),
            _ => Some(map.clone()),
        },
        _ => None,
    };
    let embedding_value = match found_doc.get("embedding") {
        Some(FieldType::Object(map)) => match map.get("value") {
            Some(FieldType::Array(arr)) | Some(FieldType::Vector(arr)) => Some(arr.clone()),
            _ => None,
        },
        Some(FieldType::Vector(vec)) => Some(vec.clone()),
        _ => None,
    };
    let categories_value = match found_doc.get("categories") {
        Some(FieldType::Object(map)) => match map.get("value") {
            Some(FieldType::Array(arr)) | Some(FieldType::Set(arr)) => Some(arr.clone()),
            _ => None,
        },
        Some(FieldType::Set(set)) => Some(set.clone()),
        _ => None,
    };
    let data_value = match found_doc.get("data") {
        Some(FieldType::Object(map)) => match map.get("value") {
            Some(FieldType::String(s)) => {
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, s).ok()
            }
            Some(FieldType::Bytes(b)) => Some(b.clone()),
            _ => None,
        },
        Some(FieldType::String(s)) => {
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, s).ok()
        }
        Some(FieldType::Bytes(b)) => Some(b.clone()),
        _ => None,
    };

    println!("Extracted values:");
    println!("  name (String): {:?}", name_value);
    println!("  value (Integer): {:?}", value_value);
    println!("  active (Boolean): {:?}", active_value);
    println!("  price (Decimal): {:?}", price_value);
    println!("  created_at (DateTime): {:?}", created_at_value);
    println!("  user_id (UUID): {:?}", user_id_value);
    println!(
        "  tags (Array): {} items",
        tags_value.as_ref().map(|v| v.len()).unwrap_or(0)
    );
    println!(
        "  metadata (Object): {} keys",
        metadata_value.as_ref().map(|v| v.len()).unwrap_or(0)
    );
    println!(
        "  embedding (Vector): {} dims",
        embedding_value.as_ref().map(|v| v.len()).unwrap_or(0)
    );
    println!(
        "  categories (Set): {} items",
        categories_value.as_ref().map(|v| v.len()).unwrap_or(0)
    );
    println!(
        "  data (Bytes): {} bytes",
        data_value.as_ref().map(|v| v.len()).unwrap_or(0)
    );

    // Example 3: Find with query
    println!("\n=== Find with Query ===");
    use ekodb_client::Query;

    let query = Query::new()
        .filter(serde_json::json!({
            "type": "Condition",
            "content": {
                "field": "active",
                "operator": "Eq",
                "value": true
            }
        }))
        .limit(10);

    let docs = client.find(collection, query, None).await?;
    println!("Found documents: {:?}", docs);

    // Example 4: Update document
    println!("\n=== Update Document ===");
    let mut updates = Record::new();
    updates.insert("name", "Updated Record");
    updates.insert("value", 100);

    let updated = client.update(collection, &doc_id, updates, None).await?;
    println!("Updated: {:?}", updated);

    // Example 5: Delete document
    println!("\n=== Delete Document ===");
    client.delete(collection, &doc_id, None).await?;
    println!("Deleted document");

    // Cleanup: Delete the collection
    println!("\n=== Cleanup ===");
    client.delete_collection(collection).await?;
    println!("✓ Deleted collection");

    println!("\n✓ All CRUD operations completed successfully");

    Ok(())
}
