use ekodb_client::{Client, FieldType, Function, UserFunction};
use std::collections::HashMap;
use std::env;

/// Save a function idempotently: if the label already exists (HTTP 409),
/// update the existing definition instead, then return its id.
async fn save_or_update(
    client: &Client,
    function: UserFunction,
) -> Result<String, Box<dyn std::error::Error>> {
    let label = function.label.clone();
    match client.save_function(function.clone()).await {
        Ok(id) => Ok(id),
        Err(ekodb_client::Error::Api { code: 409, .. }) => {
            client.update_function(&label, function).await?;
            println!("ℹ️  Function '{}' already existed — updated instead", label);
            let existing = client.get_function(&label).await?;
            Ok(existing.id.unwrap_or(label))
        }
        Err(e) => Err(Box::new(e)),
    }
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

    println!("=== Testing Basic HttpRequest Function ===\n");

    // Test 1: Simple HTTP request without parameters
    println!("Test 1: Static URL HTTP request");
    let script1 = UserFunction::new("test_http_static", "Test Static HTTP").with_function(
        Function::HttpRequest {
            url: "https://api.github.com/users/torvalds".to_string(),
            method: "GET".to_string(),
            headers: None,
            body: None,
            timeout_seconds: None,
            output_field: None,
        },
    );

    let json1 = serde_json::to_string_pretty(&script1)?;
    println!("Function JSON:\n{}\n", json1);

    let id1 = save_or_update(&client, script1).await?;
    println!("✓ Saved function: {}\n", id1);

    let result1 = client.call_function("test_http_static", None).await?;
    println!("✓ Called successfully");
    println!("Records: {}\n", result1.records.len());

    // Test 2: HTTP request with parameter
    println!("Test 2: Parameterized URL HTTP request");
    let script2 = UserFunction::new("test_http_param", "Test Param HTTP")
        .with_parameter(
            ekodb_client::ParameterDefinition::new("username")
                .required()
                .with_description("GitHub username"),
        )
        .with_function(Function::HttpRequest {
            url: "https://api.github.com/users/{{username}}".to_string(),
            method: "GET".to_string(),
            headers: None,
            body: None,
            timeout_seconds: None,
            output_field: None,
        });

    let json2 = serde_json::to_string_pretty(&script2)?;
    println!("Function JSON:\n{}\n", json2);

    let id2 = save_or_update(&client, script2).await?;
    println!("✓ Saved function: {}\n", id2);

    let mut params = HashMap::new();
    params.insert(
        "username".to_string(),
        FieldType::String("torvalds".to_string()),
    );

    println!("Calling with params: {:?}", params);
    let result2 = client
        .call_function("test_http_param", Some(params))
        .await?;
    println!("✓ Called successfully");
    println!("Records: {}\n", result2.records.len());

    println!("✅ All HttpRequest tests passed!");

    Ok(())
}
