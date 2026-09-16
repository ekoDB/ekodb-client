use ekodb_client::{Client, FieldType, Function, UserFunction};
use std::collections::HashMap;
use std::env;

const STATIC_LABEL: &str = "test_http_static_rs";
const PARAM_LABEL: &str = "test_http_param_rs";

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

    cleanup_functions(
        &client,
        &[STATIC_LABEL.to_string(), PARAM_LABEL.to_string()],
    )
    .await?;

    let mut function_ids = Vec::new();
    let operation_result: Result<(), Box<dyn std::error::Error>> = async {
        println!("=== Testing Basic HttpRequest Function ===\n");

        // Test 1: Simple HTTP request without parameters
        println!("Test 1: Static URL HTTP request");
        let script1 = UserFunction::new(STATIC_LABEL, "Test Static HTTP").with_function(
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
        function_ids.push(id1);

        let result1 = client.call_function(STATIC_LABEL, None).await?;
        println!("✓ Called successfully");
        println!("Records: {}\n", result1.records.len());

        // Test 2: HTTP request with parameter
        println!("Test 2: Parameterized URL HTTP request");
        let script2 = UserFunction::new(PARAM_LABEL, "Test Param HTTP")
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
        function_ids.push(id2);

        let mut params = HashMap::new();
        params.insert(
            "username".to_string(),
            FieldType::String("torvalds".to_string()),
        );

        println!("Calling with params: {:?}", params);
        let result2 = client.call_function(PARAM_LABEL, Some(params)).await?;
        println!("✓ Called successfully");
        println!("Records: {}\n", result2.records.len());

        println!("✅ All HttpRequest tests passed!");

        Ok(())
    }
    .await;

    let cleanup_result = cleanup_functions(&client, &function_ids).await;
    match (operation_result, cleanup_result) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(operation_error), Ok(())) => Err(operation_error),
        (Ok(()), Err(cleanup_error)) => Err(cleanup_error),
        (Err(operation_error), Err(cleanup_error)) => Err(std::io::Error::other(format!(
            "{operation_error}; cleanup also failed: {cleanup_error}"
        ))
        .into()),
    }
}

async fn cleanup_functions(
    client: &Client,
    identifiers: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut errors = Vec::new();
    for identifier in identifiers {
        match client.delete_function(identifier).await {
            Ok(()) => {}
            Err(error)
                if matches!(error, ekodb_client::Error::Api { code: 404, .. })
                    || error.to_string().to_ascii_lowercase().contains("not found") => {}
            Err(error) => errors.push(format!("{identifier}: {error}")),
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "failed to clean owned functions: {}",
            errors.join("; ")
        ))
        .into())
    }
}
