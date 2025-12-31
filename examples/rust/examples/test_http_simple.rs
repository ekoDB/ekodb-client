use ekodb_client::{Client, FieldType, Function, Script};
use std::collections::HashMap;
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

    println!("=== Testing Basic HttpRequest Function ===\n");

    // Test 1: Simple HTTP request without parameters
    println!("Test 1: Static URL HTTP request");
    let script1 =
        Script::new("test_http_static", "Test Static HTTP").with_function(Function::HttpRequest {
            url: "https://api.github.com/users/torvalds".to_string(),
            method: "GET".to_string(),
            headers: None,
            body: None,
            timeout_seconds: None,
            output_field: None,
        });

    let json1 = serde_json::to_string_pretty(&script1)?;
    println!("Script JSON:\n{}\n", json1);

    let id1 = client.save_script(script1).await?;
    println!("✓ Saved script: {}\n", id1);

    let result1 = client.call_script("test_http_static", None).await?;
    println!("✓ Called successfully");
    println!("Records: {}\n", result1.records.len());

    // Test 2: HTTP request with parameter
    println!("Test 2: Parameterized URL HTTP request");
    let script2 = Script::new("test_http_param", "Test Param HTTP")
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
    println!("Script JSON:\n{}\n", json2);

    let id2 = client.save_script(script2).await?;
    println!("✓ Saved script: {}\n", id2);

    let mut params = HashMap::new();
    params.insert(
        "username".to_string(),
        FieldType::String("torvalds".to_string()),
    );

    println!("Calling with params: {:?}", params);
    let result2 = client.call_script("test_http_param", Some(params)).await?;
    println!("✓ Called successfully");
    println!("Records: {}\n", result2.records.len());

    println!("✅ All HttpRequest tests passed!");

    Ok(())
}
