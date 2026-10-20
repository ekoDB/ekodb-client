use ekodb_client::admin::{
    AdminClient, CreateApiKeyRequest, MIN_SERVER_VERSION_FOR_PERMISSION_EDITS, PermissionEdit,
    UpdateApiKeyCollectionPermissionsRequest,
};
use mockito::Matcher;
use serde_json::{Value, json};
fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../test-fixtures/admin-contract-0.79.2.json"
    ))
    .unwrap()
}
fn edit() -> UpdateApiKeyCollectionPermissionsRequest {
    serde_json::from_value(
        json!({"api_key":"target-secret","collection":"users","read":true,"write":false}),
    )
    .unwrap()
}

#[tokio::test]
async fn recorded_three_state_bodies_and_responses_with_cached_health() {
    for case in fixture()["permission_edits"].as_array().unwrap() {
        let mut server = mockito::Server::new_async().await;
        let auth = server
            .mock("POST", "/api/auth/token")
            .match_body(Matcher::Json(json!({"api_key":"admin-secret"})))
            .with_body(r#"{"token":"jwt-secret"}"#)
            .expect(1)
            .create_async()
            .await;
        let health = server
            .mock("GET", "/api/health")
            .match_header("authorization", "Bearer jwt-secret")
            .with_body(r#"{"version":"0.79.2"}"#)
            .expect(1)
            .create_async()
            .await;
        let update = server
            .mock("PUT", "/api/auth/permissions")
            .match_header("authorization", "Bearer jwt-secret")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(case["request"].clone()))
            .with_body(case["response"].to_string())
            .expect(2)
            .create_async()
            .await;
        let client = AdminClient::new(&server.url(), "admin-secret").unwrap();
        let request: UpdateApiKeyCollectionPermissionsRequest =
            serde_json::from_value(case["request"].clone()).unwrap();
        for _ in 0..2 {
            let result = client
                .update_api_key_collection_permissions(&request)
                .await
                .unwrap();
            assert_eq!(
                serde_json::to_value(&result.collection_permissions["users"]).unwrap(),
                case["fresh_token_grant"]
            );
        }
        auth.assert_async().await;
        health.assert_async().await;
        update.assert_async().await;
    }
}
#[tokio::test]
async fn all_five_operations_registration_defaults_and_metadata() {
    let mut server = mockito::Server::new_async().await;
    let record = json!({"key":"new-secret","is_admin":false,"collection_permissions":{"users":["read"]},"label":"disposable","description":null,"ripple_warning":"Incomplete propagation","created_at":1,"kv_permissions":[]});
    let auth = server
        .mock("POST", "/api/auth/token")
        .with_body(r#"{"token":"jwt-secret"}"#)
        .expect(1)
        .create_async()
        .await;
    let create = server
        .mock("POST", "/api/auth/register")
        .match_header("authorization", "Bearer jwt-secret")
        .match_body(Matcher::Json(
            json!({"api_key":"admin-secret","is_admin":false,"collection_permissions":{}}),
        ))
        .with_body(record.to_string())
        .expect(1)
        .create_async()
        .await;
    let list = server
        .mock("GET", "/api/auth/keys")
        .with_body(json!([record.clone()]).to_string())
        .create_async()
        .await;
    let read = server
        .mock("GET", "/api/auth/permissions")
        .with_body(record.to_string())
        .create_async()
        .await;
    let revoke = server
        .mock(
            "DELETE",
            "/api/auth/keys/secret%2F%20%3F%23%25%2B%26%3D%E9%9B%AA",
        )
        .with_status(204)
        .expect(1)
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), "admin-secret").unwrap();
    let created = client
        .create_api_key(&CreateApiKeyRequest::default())
        .await
        .unwrap();
    assert!(!created.is_admin);
    assert!(created.collection_permissions["users"].read);
    assert!(!created.collection_permissions["users"].write);
    assert_eq!(
        created.ripple_warning.as_deref(),
        Some("Incomplete propagation")
    );
    assert_eq!(created.metadata["created_at"], 1);
    assert_eq!(client.list_api_keys().await.unwrap().len(), 1);
    assert!(
        client
            .get_permissions()
            .await
            .unwrap()
            .collection_permissions["users"]
            .read
    );
    client.revoke_api_key("secret/ ?#%+&=雪").await.unwrap();
    auth.assert_async().await;
    create.assert_async().await;
    list.assert_async().await;
    read.assert_async().await;
    revoke.assert_async().await;
}
#[tokio::test]
async fn gate_rejects_unreadable_old_and_prerelease_without_put() {
    assert_eq!(MIN_SERVER_VERSION_FOR_PERMISSION_EDITS, "0.79.2");
    for version in [
        Value::Null,
        json!(9),
        json!("0.79.0"),
        json!("0.79.1"),
        json!("0.79.2-rc.1"),
        json!("09.79.2"),
        json!("0.79.2+"),
        json!("999999999999999999999999.1.1"),
    ] {
        let mut server = mockito::Server::new_async().await;
        let auth = server
            .mock("POST", "/api/auth/token")
            .with_body(r#"{"token":"jwt-secret"}"#)
            .expect(1)
            .create_async()
            .await;
        let health = server
            .mock("GET", "/api/health")
            .with_body(json!({"version":version}).to_string())
            .expect(1)
            .create_async()
            .await;
        let put = server
            .mock("PUT", "/api/auth/permissions")
            .expect(0)
            .create_async()
            .await;
        let client = AdminClient::new(&server.url(), "admin-secret").unwrap();
        for _ in 0..2 {
            assert!(
                client
                    .update_api_key_collection_permissions(&edit())
                    .await
                    .err()
                    .unwrap()
                    .to_string()
                    .contains("earlier servers remove")
            );
        }
        auth.assert_async().await;
        health.assert_async().await;
        put.assert_async().await;
    }
}
#[tokio::test]
async fn errors_are_safe_creation_is_not_replayed_and_401_refresh_is_bounded() {
    for status in [400, 401, 403, 429, 500, 503, 307] {
        let mut server = mockito::Server::new_async().await;
        let expected = if status == 401 { 2 } else { 1 };
        let auth = server
            .mock("POST", "/api/auth/token")
            .with_body(r#"{"token":"jwt-secret"}"#)
            .expect(expected)
            .create_async()
            .await;
        let create = server
            .mock("POST", "/api/auth/register")
            .with_status(status)
            .with_body("admin-secret jwt-secret target-secret")
            .expect(expected)
            .create_async()
            .await;
        let client = AdminClient::new(&server.url(), "admin-secret").unwrap();
        let error = client
            .create_api_key(&CreateApiKeyRequest::default())
            .await
            .err()
            .unwrap()
            .to_string();
        assert!(!error.contains("secret"));
        assert!(error.contains(&status.to_string()));
        auth.assert_async().await;
        create.assert_async().await;
    }
}
#[tokio::test]
async fn validation_precedes_http_and_parser_errors_redact_payloads() {
    let client = AdminClient::new("http://127.0.0.1:1", "admin-secret").unwrap();
    let mut bad = edit();
    bad.select_fields = PermissionEdit::Set(vec![" ".into()]);
    assert!(
        client
            .update_api_key_collection_permissions(&bad)
            .await
            .err()
            .unwrap()
            .to_string()
            .contains("Invalid collection")
    );
    assert!(serde_json::from_value::<UpdateApiKeyCollectionPermissionsRequest>(json!({"api_key":"x","collection":"users","read":true,"write":false,"select_fields":[null]})).is_err());
    let mut server = mockito::Server::new_async().await;
    let auth = server
        .mock("POST", "/api/auth/token")
        .with_body(r#"{"token":"jwt-secret"}"#)
        .create_async()
        .await;
    let bad = server
        .mock("GET", "/api/auth/keys")
        .with_body("target-secret not JSON")
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), "admin-secret").unwrap();
    assert!(
        !client
            .list_api_keys()
            .await
            .err()
            .unwrap()
            .to_string()
            .contains("secret")
    );
    auth.assert_async().await;
    bad.assert_async().await;
}
