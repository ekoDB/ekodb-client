//! Search Example
//!
//! This example demonstrates text search, vector search, and hybrid search capabilities.
//! It shows how to use search with various options including limit, scoring, and fuzzy matching.
//!
//! Prerequisites:
//! - Run the ekoDB server: `make run`
//!
//! Run with: `cargo run --example client_search`

use ekodb_client::{Client, Record, SearchQuery};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("=== ekoDB Search Example ===\n");

    // Create client
    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    let collection = "search_client_rust";

    // Cleanup any existing test collection
    let _ = client.delete_collection(collection).await;

    // Step 1: Insert sample documents
    println!("=== Inserting Sample Documents ===");

    let docs = vec![
        (
            "Rust Programming",
            "Learn Rust programming language with hands-on examples and best practices.",
            vec!["programming", "rust", "tutorial"],
        ),
        (
            "Python for Data Science",
            "Master Python for data analysis, machine learning, and visualization.",
            vec!["programming", "python", "data-science"],
        ),
        (
            "JavaScript Web Development",
            "Build modern web applications using JavaScript, React, and Node.js.",
            vec!["programming", "javascript", "web"],
        ),
        (
            "Database Design",
            "Learn database design principles, normalization, and query optimization.",
            vec!["database", "design", "sql"],
        ),
        (
            "Machine Learning Basics",
            "Introduction to machine learning algorithms and neural networks.",
            vec!["ai", "machine-learning", "python"],
        ),
    ];

    let doc_count = docs.len();
    for (title, description, tags) in docs {
        let mut doc = Record::new();
        doc.insert("title", title);
        doc.insert("description", description);
        doc.insert("tags", tags.join(",")); // Store as comma-separated string
        doc.insert("views", (rand::random::<u32>() % 1000) as i64);
        client.insert(collection, doc, None).await?;
    }
    println!("✓ Inserted {} sample documents\n", doc_count);

    // Step 2: Basic text search
    println!("=== Basic Text Search ===");
    let search = SearchQuery::new("programming").min_score(0.1).limit(3);

    let results = client.search(collection, search).await?;
    println!(
        "✓ Found {} results for 'programming'",
        results.results.len()
    );
    for (i, result) in results.results.iter().enumerate() {
        println!(
            "  {}. Score: {:.4} - {:?}",
            i + 1,
            result.score,
            result.record.get("title")
        );
    }
    println!();

    // Step 3: Fuzzy search
    println!("=== Fuzzy Search ===");
    let fuzzy_search = SearchQuery::new("progamming") // Intentional typo
        .fuzzy(true)
        .max_edit_distance(2)
        .min_score(0.1)
        .limit(3);

    let fuzzy_results = client.search(collection, fuzzy_search).await?;
    println!(
        "✓ Found {} results for 'progamming' (typo)",
        fuzzy_results.results.len()
    );
    for (i, result) in fuzzy_results.results.iter().enumerate() {
        println!(
            "  {}. Score: {:.4} - {:?}",
            i + 1,
            result.score,
            result.record.get("title")
        );
    }
    println!();

    // Step 4: Field-specific search
    println!("=== Field-Specific Search ===");
    let field_search = SearchQuery::new("machine learning")
        .fields("title,description")
        .min_score(0.2)
        .limit(5);

    let field_results = client.search(collection, field_search).await?;
    println!(
        "✓ Found {} results in title/description",
        field_results.results.len()
    );
    for (i, result) in field_results.results.iter().enumerate() {
        println!("  {}. Score: {:.4}", i + 1, result.score);
        println!("     Title: {:?}", result.record.get("title"));
        println!("     Matched: {:?}", result.matched_fields);
    }
    println!();

    // Step 5: Search with field weights
    println!("=== Weighted Search ===");
    let weighted_search = SearchQuery::new("python")
        .weights("title:2.0,description:1.0,tags:0.5")
        .min_score(0.1)
        .limit(5);

    let weighted_results = client.search(collection, weighted_search).await?;
    println!(
        "✓ Found {} results with field weights",
        weighted_results.results.len()
    );
    for (i, result) in weighted_results.results.iter().enumerate() {
        println!(
            "  {}. Score: {:.4} - {:?}",
            i + 1,
            result.score,
            result.record.get("title")
        );
    }
    println!();

    // Step 6: Search with stemming and exact boost
    println!("=== Advanced Search Options ===");
    let advanced_search = SearchQuery::new("databases")
        .enable_stemming(true)
        .boost_exact(true)
        .case_sensitive(false)
        .min_score(0.1)
        .limit(5);

    let advanced_results = client.search(collection, advanced_search).await?;
    println!(
        "✓ Found {} results with stemming",
        advanced_results.results.len()
    );
    for (i, result) in advanced_results.results.iter().enumerate() {
        println!(
            "  {}. Score: {:.4} - {:?}",
            i + 1,
            result.score,
            result.record.get("title")
        );
    }
    println!();

    // Step 7: Search with limit
    println!("=== Search with Limit ===");
    let limited_search = SearchQuery::new("programming")
        .limit(2) // Only return top 2 results
        .min_score(0.1);

    let limited_results = client.search(collection, limited_search).await?;
    println!(
        "✓ Limited to {} results (requested 2)",
        limited_results.results.len()
    );
    for (i, result) in limited_results.results.iter().enumerate() {
        println!(
            "  {}. Score: {:.4} - {:?}",
            i + 1,
            result.score,
            result.record.get("title")
        );
    }
    println!();

    // Cleanup: Delete the collection
    println!("=== Cleanup ===");
    client.delete_collection(collection).await?;
    println!("✓ Deleted collection\n");

    println!("✓ All search operations completed successfully");
    println!("Execution time: {}ms", results.execution_time_ms);

    Ok(())
}
