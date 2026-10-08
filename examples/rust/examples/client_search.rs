//! Search Example
//!
//! This example demonstrates text search, vector search, and hybrid search capabilities.
//! It shows how to use search with various options including limit, scoring, and fuzzy matching.
//!
//! Prerequisites:
//! - Run the ekoDB server: `make run`
//!
//! Run with: `cargo run --example client_search`

use ekodb_client::{
    Client, DistanceMetric, Error as EkoError, FieldType, FieldTypeSchema, IndexConfig, Record,
    Schema, SearchQuery, VectorIndexAlgorithm,
};
use std::error::Error;

const COLLECTION: &str = "search_client_rust";

fn is_not_found(error: &EkoError) -> bool {
    matches!(error, EkoError::NotFound | EkoError::Api { code: 404, .. })
}

async fn cleanup(client: &Client) -> Result<(), Box<dyn Error>> {
    match client.delete_collection(COLLECTION).await {
        Err(error) if is_not_found(&error) => Ok(()),
        Err(error) => Err(format!("collection {COLLECTION}: {error}").into()),
        Ok(()) => Ok(()),
    }
}

async fn paired_vector_search(client: &Client) -> Result<(), Box<dyn Error>> {
    let collection = format!("search_paired_client_rust_{:016x}", rand::random::<u64>());
    let operation_result: Result<(), Box<dyn Error>> = async {
        let vector_field = || {
            FieldTypeSchema::new("Vector")
                .required()
                .with_index(IndexConfig::Vector {
                    algorithm: VectorIndexAlgorithm::Flat,
                    metric: DistanceMetric::Cosine,
                    m: 16,
                    ef_construction: 200,
                    ef_search: None,
                    dimension: Some(3),
                })
        };
        let schema = Schema::new()
            .add_field("title", FieldTypeSchema::new("String").required())
            .add_field("category", FieldTypeSchema::new("String").required())
            .add_field("query_embedding", vector_field())
            .add_field("document_embedding", vector_field());
        client.create_collection(&collection, schema).await?;

        // Orthogonal toy vectors show field selection; production embeddings need
        // compatible dimensions and a shared dual-encoder model space.
        let documents = [
            (
                "Rust Programming Question",
                "programming",
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
            ),
            (
                "Rust Programming Answer",
                "programming",
                [0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0],
            ),
            (
                "Database Design Answer",
                "database",
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ),
        ];
        for (title, category, query, document) in documents {
            let record = Record::new()
                .field("title", title)
                .field("category", category)
                .field("query_embedding", FieldType::vector(query.to_vec()))
                .field("document_embedding", FieldType::vector(document.to_vec()));
            client.insert(&collection, record, None).await?;
        }

        let source_query = vec![1.0, 0.0, 0.0];
        let document_matches = client
            .search(
                &collection,
                SearchQuery::new("")
                    .vector(source_query.clone())
                    .vector_field("document_embedding")
                    .vector_metric("cosine")
                    .vector_k(3)
                    .limit(3)
                    .bypass_cache(true),
            )
            .await?;
        let query_matches = client
            .search(
                &collection,
                SearchQuery::new("")
                    .vector(source_query)
                    .vector_field("query_embedding")
                    .vector_metric("cosine")
                    .vector_k(3)
                    .limit(3)
                    .bypass_cache(true),
            )
            .await?;
        let document_top = document_matches
            .results
            .first()
            .and_then(|hit| hit.record.get("title"))
            .and_then(|field| field.get("value"))
            .and_then(|value| value.as_str());
        let query_top = query_matches
            .results
            .first()
            .and_then(|hit| hit.record.get("title"))
            .and_then(|field| field.get("value"))
            .and_then(|value| value.as_str());
        if document_top != Some("Rust Programming Answer")
            || query_top != Some("Rust Programming Question")
        {
            return Err(format!(
                "unexpected paired-vector tops: document={document_top:?}, query={query_top:?}"
            )
            .into());
        }
        println!("Paired vector search: document={document_top:?}, query={query_top:?}");
        Ok(())
    }
    .await;

    // A create response can be lost after server success, so always attempt cleanup.
    let cleanup_result = match client.delete_collection(&collection).await {
        Err(error) if is_not_found(&error) => Ok(()),
        Err(error) => Err(format!("collection {collection}: {error}").into()),
        Ok(()) => Ok(()),
    };
    match (operation_result, cleanup_result) {
        (Err(primary), Err(cleanup)) => {
            Err(format!("{primary}; cleanup also failed: {cleanup}").into())
        }
        (Err(primary), Ok(())) => Err(primary),
        (Ok(()), Err(cleanup)) => Err(cleanup),
        (Ok(()), Ok(())) => Ok(()),
    }
}

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

    cleanup(&client).await?;

    let operation_result: Result<(), Box<dyn Error>> = async {
        let collection = COLLECTION;

        // Step 1: Insert sample documents
        println!("=== Inserting Sample Documents ===");

        // The `category` field is used to demonstrate metadata pre-filtering.
        let docs = vec![
            (
                "Rust Programming",
                "Learn Rust programming language with hands-on examples and best practices.",
                vec!["programming", "rust", "tutorial"],
                "programming",
            ),
            (
                "Python for Data Science",
                "Master Python for data analysis, machine learning, and visualization.",
                vec!["programming", "python", "data-science"],
                "programming",
            ),
            (
                "JavaScript Web Development",
                "Build modern web applications using JavaScript, React, and Node.js.",
                vec!["programming", "javascript", "web"],
                "programming",
            ),
            (
                "Database Design",
                "Learn database design principles, normalization, and query optimization.",
                vec!["database", "design", "sql"],
                "database",
            ),
            (
                "Machine Learning Basics",
                "Introduction to machine learning algorithms and neural networks.",
                vec!["ai", "machine-learning", "python"],
                "ai",
            ),
        ];

        let doc_count = docs.len();
        for (title, description, tags, category) in docs {
            let mut doc = Record::new();
            doc.insert("title", title);
            doc.insert("description", description);
            doc.insert("tags", tags.join(",")); // Store as comma-separated string
            doc.insert("category", category);
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

        // Search with a metadata pre-filter (works on text, vector, and hybrid).
        // The same query is restricted to documents in the "programming" category.
        println!("=== Search with a metadata pre-filter (category = programming) ===");
        let filtered_search = SearchQuery::new("learn")
            .min_score(0.1)
            .filters(serde_json::json!({
                "type": "Condition",
                "content": {
                    "field": "category",
                    "operator": "Eq",
                    "value": "programming"
                }
            }));

        let filtered_results = client.search(collection, filtered_search).await?;
        println!(
            "✓ Found {} results in category 'programming' (database/ai excluded)",
            filtered_results.results.len()
        );
        for (i, result) in filtered_results.results.iter().enumerate() {
            println!(
                "  {}. {:?} (category: {:?})",
                i + 1,
                result.record.get("title"),
                result.record.get("category")
            );
        }
        println!();

        println!("Execution time: {}ms", results.execution_time_ms);
        paired_vector_search(&client).await?;
        Ok(())
    }
    .await;

    println!("=== Cleanup ===");
    let cleanup_result = cleanup(&client).await;
    if cleanup_result.is_ok() {
        println!("✓ Deleted collection\n");
    }
    match (operation_result, cleanup_result) {
        (Err(primary), Err(cleanup)) => {
            eprintln!("Cleanup also failed: {cleanup}");
            return Err(format!("{primary}; cleanup also failed: {cleanup}").into());
        }
        (Err(primary), Ok(())) => return Err(primary),
        (Ok(()), Err(cleanup)) => return Err(cleanup),
        (Ok(()), Ok(())) => {}
    }

    println!("✓ All search operations completed successfully");
    Ok(())
}
