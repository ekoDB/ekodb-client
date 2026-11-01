use ekodb_client::admin::{
    AdminClient, CreateApiKeyRequest, LegacyUpdateApiKeyCollectionPermissionsRequest,
    PatchApiKeyCollectionPermissionsRequest, PatchApiKeyKvPermissionsRequest,
    UpdateApiKeyCollectionPermissionsRequest, UpdateApiKeyKvPermissionsRequest,
};
use mockito::Matcher;
use serde_json::{Value, json};

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../test-fixtures/admin-contract-0.80.0.json"
    ))
    .unwrap()
}

fn permission_names() -> Value {
    serde_json::from_str(include_str!(
        "../../test-fixtures/admin-permission-names.json"
    ))
    .unwrap()
}

fn decoded_facet(path: &str, prefix: &str) -> Option<String> {
    let encoded = path.strip_prefix(prefix)?;
    if encoded.contains('/') || encoded.contains('?') || encoded.contains('#') {
        return None;
    }
    let query = format!("name={encoded}");
    let mut pairs = url::form_urlencoded::parse(query.as_bytes());
    let (key, value) = pairs.next()?;
    if key != "name" || pairs.next().is_some() {
        return None;
    }
    Some(value.into_owned())
}

#[tokio::test]
async fn accepted_facet_names_cross_the_public_http_boundary() {
    let names = permission_names();
    let mut failures = Vec::new();
    for case in names["accepted"].as_array().unwrap() {
        let name = case["value"].as_str().unwrap();
        let mut server = mockito::Server::new_async().await;
        let auth = server
            .mock("POST", "/api/auth/token")
            .with_body(r#"{"token":"jwt-synthetic"}"#)
            .expect(1)
            .create_async()
            .await;
        let probe = server
            .mock("GET", "/api/auth/keys/me/permissions")
            .with_body(fixture()["permissions"].to_string())
            .expect(1)
            .create_async()
            .await;
        let expected = name.to_owned();
        let collection = server
            .mock(
                "GET",
                Matcher::Regex("^/api/auth/keys/me/permissions/collections/".into()),
            )
            .match_request(move |request| {
                decoded_facet(request.path(), "/api/auth/keys/me/permissions/collections/")
                    == Some(expected.clone())
            })
            .with_body(r#"{"read":true,"write":false}"#)
            .expect(1)
            .create_async()
            .await;
        let expected = name.to_owned();
        let kv = server
            .mock(
                "GET",
                Matcher::Regex("^/api/auth/keys/me/permissions/kv/".into()),
            )
            .match_request(move |request| {
                decoded_facet(request.path(), "/api/auth/keys/me/permissions/kv/")
                    == Some(expected.clone())
            })
            .with_body(json!({"prefix":name,"read":true,"write":false}).to_string())
            .expect(1)
            .create_async()
            .await;
        let client =
            AdminClient::new(&server.url(), "synthetic-admin-key-0123456789-abcdef").unwrap();
        let collection_result = client.get_api_key_collection_permissions("me", name).await;
        let kv_result = client.get_api_key_kv_permissions("me", name).await;
        if collection_result.is_err() || kv_result.is_err() {
            failures.push(case["name"].as_str().unwrap().to_owned());
            continue;
        }
        assert!(collection_result.unwrap().read);
        assert_eq!(kv_result.unwrap().prefix.as_deref(), Some(name));
        auth.assert_async().await;
        probe.assert_async().await;
        collection.assert_async().await;
        kv.assert_async().await;
    }
    assert!(failures.is_empty(), "accepted names refused: {failures:?}");
}

#[tokio::test]
async fn rejected_facet_names_fail_before_any_http_request() {
    let names = permission_names();
    let mut server = mockito::Server::new_async().await;
    let auth = server
        .mock("POST", "/api/auth/token")
        .expect(0)
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), "synthetic-admin-key-0123456789-abcdef").unwrap();
    let mut failures = Vec::new();
    for case in names["rejected"].as_array().unwrap() {
        let name = case["value"].as_str().unwrap();
        for facet in ["collections", "kv"] {
            let rejected = if facet == "collections" {
                matches!(client.get_api_key_collection_permissions("me", name).await, Err(ekodb_client::Error::Validation(message)) if message == "Invalid permission facet")
            } else {
                matches!(client.get_api_key_kv_permissions("me", name).await, Err(ekodb_client::Error::Validation(message)) if message == "Invalid permission facet")
            };
            if !rejected {
                failures.push(format!("{}:{facet}", case["name"].as_str().unwrap()));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "rejected names reached transport: {failures:?}"
    );
    auth.assert_async().await;
}

#[tokio::test]
async fn failed_legacy_health_reads_are_retried_then_success_is_cached() {
    for (status, body, error_fragment) in [
        (503, r#"{"error":"synthetic-secret"}"#, "503"),
        (
            200,
            "synthetic-secret invalid JSON",
            "Invalid admin response",
        ),
        (200, "{}", "earlier servers remove"),
        (200, r#"{"version":9}"#, "earlier servers remove"),
        (200, r#"{"version":"invalid"}"#, "earlier servers remove"),
        (
            200,
            r#"{"version":"0.80.0-rc.1"}"#,
            "earlier servers remove",
        ),
    ] {
        let mut server = mockito::Server::new_async().await;
        let auth = server
            .mock("POST", "/api/auth/token")
            .with_body(r#"{"token":"jwt-synthetic"}"#)
            .expect(1)
            .create_async()
            .await;
        let probe = server
            .mock("GET", "/api/auth/keys/me/permissions")
            .with_status(404)
            .expect(1)
            .create_async()
            .await;
        let failed_health = server
            .mock("GET", "/api/health")
            .with_status(status)
            .with_body(body)
            .expect(1)
            .create_async()
            .await;
        let update = server
            .mock("PUT", "/api/auth/permissions")
            .with_body(fixture()["key"].to_string())
            .expect(2)
            .create_async()
            .await;
        let client =
            AdminClient::new(&server.url(), "synthetic-admin-key-0123456789-abcdef").unwrap();
        let request: LegacyUpdateApiKeyCollectionPermissionsRequest = serde_json::from_value(json!({"api_key":"synthetic-target-key-0123456789-abcdef","collection":"users","read":true,"write":false})).unwrap();
        let first_error = client
            .legacy_update_api_key_collection_permissions(&request)
            .await
            .err()
            .unwrap()
            .to_string();
        assert!(!first_error.contains("synthetic-secret"));
        failed_health.assert_async().await;
        failed_health.remove_async().await;
        let successful_health = server
            .mock("GET", "/api/health")
            .with_body(r#"{"version":"0.79.2"}"#)
            .expect(1)
            .create_async()
            .await;
        assert!(
            client
                .legacy_update_api_key_collection_permissions(&request)
                .await
                .is_ok(),
            "failed health read poisoned the cache"
        );
        assert!(
            first_error.contains(error_fragment),
            "unexpected safe error: {first_error}"
        );
        assert!(
            client
                .legacy_update_api_key_collection_permissions(&request)
                .await
                .is_ok()
        );
        auth.assert_async().await;
        probe.assert_async().await;
        successful_health.assert_async().await;
        update.assert_async().await;
    }
}

#[tokio::test]
async fn modern_permission_facets_preserve_wire_bodies_and_change_responses() {
    let fixture = fixture();
    for case in fixture["permission_changes"].as_array().unwrap() {
        let mut server = mockito::Server::new_async().await;
        let auth = server
            .mock("POST", "/api/auth/token")
            .with_body(r#"{"token":"jwt-synthetic"}"#)
            .expect(1)
            .create_async()
            .await;
        let probe = server
            .mock("GET", "/api/auth/keys/me/permissions")
            .with_body(fixture["permissions"].to_string())
            .expect(1)
            .create_async()
            .await;
        let method = case["method"].as_str().unwrap();
        let mut change = server
            .mock(method, case["path"].as_str().unwrap())
            .match_header("authorization", "Bearer jwt-synthetic");
        change = if case["body"].is_null() {
            change.match_body("")
        } else {
            change.match_body(Matcher::Json(case["body"].clone()))
        };
        let change = change
            .with_body(case["response"].to_string())
            .expect(1)
            .create_async()
            .await;
        let client =
            AdminClient::new(&server.url(), "synthetic-admin-key-0123456789-abcdef").unwrap();
        assert!(client.uses_key_permission_routes().await.unwrap());
        assert!(client.uses_key_permission_routes().await.unwrap());
        let mut request = case["body"].as_object().cloned().unwrap_or_default();
        request.insert("key_id".into(), fixture["key_id"].clone());
        let facet = case["facet"].as_str().unwrap();
        request.insert(
            if facet == "collections" {
                "collection"
            } else {
                "prefix"
            }
            .into(),
            case["name"].clone(),
        );
        let input = Value::Object(request);
        let id = fixture["key_id"].as_str().unwrap();
        let name = case["name"].as_str().unwrap();
        let actual = match (facet, method) {
            ("collections", "PUT") => serde_json::to_value(
                client
                    .put_api_key_collection_permissions(
                        &serde_json::from_value::<UpdateApiKeyCollectionPermissionsRequest>(input)
                            .unwrap(),
                    )
                    .await
                    .unwrap(),
            )
            .unwrap(),
            ("collections", "PATCH") => serde_json::to_value(
                client
                    .patch_api_key_collection_permissions(
                        &serde_json::from_value::<PatchApiKeyCollectionPermissionsRequest>(input)
                            .unwrap(),
                    )
                    .await
                    .unwrap(),
            )
            .unwrap(),
            ("collections", "DELETE") => serde_json::to_value(
                client
                    .delete_api_key_collection_permissions(id, name)
                    .await
                    .unwrap(),
            )
            .unwrap(),
            ("kv", "PUT") => serde_json::to_value(
                client
                    .put_api_key_kv_permissions(
                        &serde_json::from_value::<UpdateApiKeyKvPermissionsRequest>(input).unwrap(),
                    )
                    .await
                    .unwrap(),
            )
            .unwrap(),
            ("kv", "PATCH") => serde_json::to_value(
                client
                    .patch_api_key_kv_permissions(
                        &serde_json::from_value::<PatchApiKeyKvPermissionsRequest>(input).unwrap(),
                    )
                    .await
                    .unwrap(),
            )
            .unwrap(),
            ("kv", "DELETE") => serde_json::to_value(
                client
                    .delete_api_key_kv_permissions(id, name)
                    .await
                    .unwrap(),
            )
            .unwrap(),
            _ => unreachable!(),
        };
        assert_eq!(actual, case["response"], "{}", case["name"]);
        auth.assert_async().await;
        probe.assert_async().await;
        change.assert_async().await;
    }
}

#[tokio::test]
async fn modern_capability_overrides_reported_version_and_preserves_ids_and_grants() {
    let fixture = fixture();
    let mut server = mockito::Server::new_async().await;
    let _auth = server
        .mock("POST", "/api/auth/token")
        .with_body(r#"{"token":"jwt-synthetic"}"#)
        .create_async()
        .await;
    let _probe = server
        .mock("GET", "/api/auth/keys/me/permissions")
        .with_body(fixture["permissions"].to_string())
        .create_async()
        .await;
    let _health = server
        .mock("GET", "/api/health")
        .expect(0)
        .create_async()
        .await;
    let _register = server
        .mock("POST", "/api/auth/register")
        .with_body(fixture["key"].to_string())
        .create_async()
        .await;
    let _list = server
        .mock("GET", "/api/auth/keys")
        .with_body(json!([fixture["key"]]).to_string())
        .create_async()
        .await;
    let id = fixture["key_id"].as_str().unwrap();
    let _own = server
        .mock("GET", format!("/api/auth/keys/{id}/permissions").as_str())
        .with_body(fixture["permissions"].to_string())
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), "synthetic-admin-key-0123456789-abcdef").unwrap();
    assert!(client.uses_key_permission_routes().await.unwrap());
    let created = client
        .create_api_key(&CreateApiKeyRequest::default())
        .await
        .unwrap();
    assert_eq!(created.id.as_deref(), Some(id));
    assert_eq!(
        client.list_api_keys().await.unwrap()[0].id.as_deref(),
        Some(id)
    );
    let permissions = client.get_api_key_permissions(id).await.unwrap();
    assert!(!permissions.is_admin);
    assert_eq!(
        permissions.kv_permissions[0].prefix.as_deref(),
        Some("session:")
    );
    assert_eq!(
        permissions.collection_permissions["tenant:a/"].select_fields,
        Some(vec!["name".into()])
    );
    let legacy: LegacyUpdateApiKeyCollectionPermissionsRequest = serde_json::from_value(json!({"api_key":"synthetic-target-key-0123456789-abcdef","collection":"users","read":true,"write":false})).unwrap();
    assert!(
        client
            .legacy_update_api_key_collection_permissions(&legacy)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn only_missing_route_statuses_select_legacy_and_failures_are_not_cached() {
    for status in [403, 429, 500, 503] {
        let mut server = mockito::Server::new_async().await;
        let _auth = server
            .mock("POST", "/api/auth/token")
            .with_body(r#"{"token":"jwt-synthetic"}"#)
            .create_async()
            .await;
        let probe = server
            .mock("GET", "/api/auth/keys/me/permissions")
            .with_status(status)
            .expect(2)
            .create_async()
            .await;
        let old = server
            .mock("GET", "/api/auth/permissions")
            .expect(0)
            .create_async()
            .await;
        let client =
            AdminClient::new(&server.url(), "synthetic-admin-key-0123456789-abcdef").unwrap();
        assert!(client.get_permissions().await.is_err());
        assert!(client.get_permissions().await.is_err());
        probe.assert_async().await;
        old.assert_async().await;
    }
    for status in [404, 405] {
        let mut server = mockito::Server::new_async().await;
        let _auth = server
            .mock("POST", "/api/auth/token")
            .with_body(r#"{"token":"jwt-synthetic"}"#)
            .create_async()
            .await;
        let probe = server
            .mock("GET", "/api/auth/keys/me/permissions")
            .with_status(status)
            .expect(1)
            .create_async()
            .await;
        let old = server
            .mock("GET", "/api/auth/permissions")
            .with_body(r#"{"is_admin":false,"collection_permissions":{},"kv_permissions":null}"#)
            .expect(2)
            .create_async()
            .await;
        let client =
            AdminClient::new(&server.url(), "synthetic-admin-key-0123456789-abcdef").unwrap();
        assert!(!client.get_permissions().await.unwrap().is_admin);
        assert!(!client.get_permissions().await.unwrap().is_admin);
        probe.assert_async().await;
        old.assert_async().await;
    }
}

#[tokio::test]
async fn dot_segment_facets_fail_before_http() {
    let mut server = mockito::Server::new_async().await;
    let auth = server
        .mock("POST", "/api/auth/token")
        .expect(0)
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), "synthetic-admin-key-0123456789-abcdef").unwrap();
    macro_rules! rejected {
        ($operation:expr) => {
            assert!(matches!($operation.await, Err(ekodb_client::Error::Validation(message)) if message.contains("Invalid permission facet")));
        };
    }
    for name in [".", ".."] {
        rejected!(client.get_api_key_collection_permissions("me", name));
        rejected!(client.delete_api_key_collection_permissions("me", name));
        rejected!(client.get_api_key_kv_permissions("me", name));
        rejected!(client.delete_api_key_kv_permissions("me", name));
        rejected!(client.put_api_key_collection_permissions(
            &UpdateApiKeyCollectionPermissionsRequest {
                key_id: "me".into(),
                collection: name.into(),
                read: true,
                write: false,
                select_fields: Default::default(),
                exclude_fields: Default::default(),
                writable_fields: Default::default()
            }
        ));
        rejected!(client.patch_api_key_collection_permissions(
            &PatchApiKeyCollectionPermissionsRequest {
                key_id: "me".into(),
                collection: name.into(),
                read: Some(true),
                ..Default::default()
            }
        ));
        rejected!(
            client.put_api_key_kv_permissions(&UpdateApiKeyKvPermissionsRequest {
                key_id: "me".into(),
                prefix: name.into(),
                read: true,
                write: false
            })
        );
        rejected!(
            client.patch_api_key_kv_permissions(&PatchApiKeyKvPermissionsRequest {
                key_id: "me".into(),
                prefix: name.into(),
                read: Some(true),
                write: None
            })
        );
    }
    auth.assert_async().await;
}

#[tokio::test]
async fn key_values_cannot_be_used_as_permission_targets() {
    let client = AdminClient::new(
        "http://127.0.0.1:1",
        "synthetic-admin-key-0123456789-abcdef",
    )
    .unwrap();
    let empty = UpdateApiKeyKvPermissionsRequest {
        key_id: "me".into(),
        prefix: "session:".into(),
        read: false,
        write: false,
    };
    assert!(
        client
            .put_api_key_kv_permissions(&empty)
            .await
            .err()
            .unwrap()
            .to_string()
            .contains("A KV grant")
    );
    assert!(
        client
            .get_api_key_permissions("synthetic-target-key-0123456789-abcdef")
            .await
            .err()
            .unwrap()
            .to_string()
            .contains("key id or me")
    );
    assert!(
        client
            .delete_api_key_kv_permissions("synthetic-target-key-0123456789-abcdef", "session:")
            .await
            .err()
            .unwrap()
            .to_string()
            .contains("key id or me")
    );
    assert!(
        client
            .delete_api_key_collection_permissions("me", "users\u{200b}")
            .await
            .err()
            .unwrap()
            .to_string()
            .contains("Invalid permission facet")
    );
}

#[tokio::test]
async fn nullable_registration_kv_grants_are_preserved() {
    let mut fixture = fixture();
    fixture["key"]["kv_permissions"] = Value::Null;
    let mut server = mockito::Server::new_async().await;
    let _auth = server
        .mock("POST", "/api/auth/token")
        .with_body(r#"{"token":"jwt-synthetic"}"#)
        .create_async()
        .await;
    let _register = server
        .mock("POST", "/api/auth/register")
        .with_body(fixture["key"].to_string())
        .create_async()
        .await;
    let _list = server
        .mock("GET", "/api/auth/keys")
        .with_body(json!([fixture["key"]]).to_string())
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), "synthetic-admin-key-0123456789-abcdef").unwrap();
    assert!(
        client
            .create_api_key(&CreateApiKeyRequest::default())
            .await
            .unwrap()
            .kv_permissions
            .is_none()
    );
    assert!(
        client.list_api_keys().await.unwrap()[0]
            .kv_permissions
            .is_none()
    );
}

#[tokio::test]
async fn facet_reads_and_update_alias_use_public_http_routes() {
    let fixture = fixture();
    let mut server = mockito::Server::new_async().await;
    let _auth = server
        .mock("POST", "/api/auth/token")
        .with_body(r#"{"token":"jwt-synthetic"}"#)
        .create_async()
        .await;
    let _probe = server
        .mock("GET", "/api/auth/keys/me/permissions")
        .with_body(fixture["permissions"].to_string())
        .create_async()
        .await;
    let id = fixture["key_id"].as_str().unwrap();
    let root = format!("/api/auth/keys/{id}/permissions");
    let collections = server
        .mock("GET", format!("{root}/collections").as_str())
        .with_body(fixture["permissions"]["collections"].to_string())
        .expect(1)
        .create_async()
        .await;
    let collection = server
        .mock("GET", format!("{root}/collections/tenant%3Aa%2F").as_str())
        .with_body(fixture["permissions"]["collections"]["tenant:a/"].to_string())
        .expect(1)
        .create_async()
        .await;
    let kvs = server
        .mock("GET", format!("{root}/kv").as_str())
        .match_query(Matcher::Exact("prefix=tenant%3Aa%2F%20%2B".into()))
        .with_body(fixture["permissions"]["kv"].to_string())
        .expect(1)
        .create_async()
        .await;
    let kv = server
        .mock("GET", format!("{root}/kv/session%3A").as_str())
        .with_body(fixture["permissions"]["kv"][0].to_string())
        .expect(1)
        .create_async()
        .await;
    let update = server
        .mock("PATCH", format!("{root}/collections/users").as_str())
        .match_body(Matcher::Json(
            json!({"read":true,"write":false,"select_fields":null}),
        ))
        .with_body(
            json!({"before":null,"after":{"read":true,"write":false},"future_revision":3})
                .to_string(),
        )
        .expect(1)
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), "synthetic-admin-key-0123456789-abcdef").unwrap();
    assert!(
        client
            .list_api_key_collection_permissions(id)
            .await
            .unwrap()["tenant:a/"]
            .read
    );
    assert!(
        client
            .get_api_key_collection_permissions(id, "tenant:a/")
            .await
            .unwrap()
            .read
    );
    assert_eq!(
        client
            .list_api_key_kv_permissions(id, Some("tenant:a/ +"))
            .await
            .unwrap()[0]
            .prefix
            .as_deref(),
        Some("session:")
    );
    assert!(
        client
            .get_api_key_kv_permissions(id, "session:")
            .await
            .unwrap()
            .read
    );
    let request: UpdateApiKeyCollectionPermissionsRequest = serde_json::from_value(
        json!({"key_id":id,"collection":"users","read":true,"write":false,"select_fields":null}),
    )
    .unwrap();
    assert_eq!(
        client
            .update_api_key_collection_permissions(&request)
            .await
            .unwrap()
            .metadata["future_revision"],
        3
    );
    collections.assert_async().await;
    collection.assert_async().await;
    kvs.assert_async().await;
    kv.assert_async().await;
    update.assert_async().await;
}
