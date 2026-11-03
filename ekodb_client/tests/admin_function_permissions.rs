//! Stored-function call grants: registration, key reads, per-pattern grant
//! routes, and the function-definition fields that accompany them.
use ekodb_client::admin::{
    AdminClient, CreateApiKeyRequest, FunctionAction, FunctionPermissions,
    MAX_FUNCTION_PATTERN_BYTES,
};
use ekodb_client::{Client, Error, Principal, RunAs, UserFunction};
use mockito::{Matcher, Server, ServerGuard};
use serde_json::{Value, json};

const ADMIN_KEY: &str = "synthetic-admin-key-0123456789-abcdef";

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../test-fixtures/admin-function-permissions-contract.json"
    ))
    .unwrap()
}

fn key_id() -> String {
    fixture()["key_id"].as_str().unwrap().to_owned()
}

async fn authed_server() -> ServerGuard {
    let mut server = Server::new_async().await;
    server
        .mock("POST", "/api/auth/token")
        .with_body(r#"{"token":"jwt-synthetic"}"#)
        .create_async()
        .await;
    server
}

async fn modern(server: &mut ServerGuard) {
    server
        .mock("GET", "/api/auth/keys/me/permissions")
        .with_body(json!({"admin": true, "collections": {}, "kv": [], "functions": []}).to_string())
        .create_async()
        .await;
}

fn patterns(grants: &[FunctionPermissions]) -> Vec<&str> {
    grants.iter().map(|grant| grant.pattern.as_str()).collect()
}

#[tokio::test]
async fn registration_sends_function_permissions_and_reads_the_echoed_map() {
    let fixture = fixture();
    let mut server = authed_server().await;
    let register = server
        .mock("POST", "/api/auth/register")
        .match_body(Matcher::Json(fixture["registration_request"].clone()))
        .with_body(fixture["registration_response"].to_string())
        .expect(1)
        .create_async()
        .await;
    let request: CreateApiKeyRequest = serde_json::from_value(json!({
        "label": "report-runner",
        "function_permissions": {
            "report_*": ["call"],
            "nightly_close": ["call"],
            "audit_only": []
        }
    }))
    .unwrap();
    assert_eq!(
        request.function_permissions["report_*"],
        [FunctionAction::Call]
    );
    let client = AdminClient::new(&server.url(), ADMIN_KEY).unwrap();
    let key = client.create_api_key(&request).await.unwrap();
    register.assert_async().await;
    let granted = key.function_permissions.expect("the echoed grants");
    let expected: Vec<&str> = fixture["expected_registered_patterns"]
        .as_array()
        .unwrap()
        .iter()
        .map(|pattern| pattern.as_str().unwrap())
        .collect();
    assert_eq!(patterns(&granted), expected, "empty rights grant nothing");
}

#[tokio::test]
async fn registration_without_function_permissions_keeps_the_old_body_and_reads_none() {
    let fixture = fixture();
    let mut server = authed_server().await;
    let mut old_body = fixture["registration_request"].clone();
    old_body
        .as_object_mut()
        .unwrap()
        .remove("function_permissions");
    let register = server
        .mock("POST", "/api/auth/register")
        .match_body(Matcher::Json(old_body))
        .with_body(fixture["registration_response_without_function_permissions"].to_string())
        .expect(1)
        .create_async()
        .await;
    let request: CreateApiKeyRequest =
        serde_json::from_value(json!({"label": "report-runner"})).unwrap();
    let client = AdminClient::new(&server.url(), ADMIN_KEY).unwrap();
    let key = client.create_api_key(&request).await.unwrap();
    register.assert_async().await;
    assert!(key.function_permissions.is_none());
}

#[tokio::test]
async fn registration_refuses_an_invalid_pattern_before_any_request() {
    let mut server = Server::new_async().await;
    let token = server
        .mock("POST", "/api/auth/token")
        .expect(0)
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), ADMIN_KEY).unwrap();
    for pattern in ["", "re*port", "report_**", "*report"] {
        let request: CreateApiKeyRequest =
            serde_json::from_value(json!({"function_permissions": {pattern: ["call"]}})).unwrap();
        assert!(
            matches!(client.create_api_key(&request).await, Err(Error::Validation(m)) if m == "Invalid function pattern"),
            "{pattern:?}"
        );
    }
    token.assert_async().await;
    // Only the one right exists: any other spelling is not a request.
    assert!(
        serde_json::from_value::<CreateApiKeyRequest>(
            json!({"function_permissions": {"report_*": ["run"]}})
        )
        .is_err()
    );
}

#[tokio::test]
async fn key_listings_and_permission_reads_carry_function_grants() {
    let fixture = fixture();
    let mut server = authed_server().await;
    server
        .mock("GET", "/api/auth/keys")
        .with_body(json!([fixture["key_list_record"]]).to_string())
        .create_async()
        .await;
    let probe = server
        .mock("GET", "/api/auth/keys/me/permissions")
        .with_body(fixture["permissions"].to_string())
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), ADMIN_KEY).unwrap();
    let keys = client.list_api_keys().await.unwrap();
    assert_eq!(
        patterns(keys[0].function_permissions.as_ref().unwrap()),
        ["nightly_close", "report_*"]
    );
    let permissions = client.get_permissions().await.unwrap();
    assert_eq!(
        patterns(&permissions.function_permissions),
        ["nightly_close", "report_*"]
    );
    probe.remove_async().await;

    // A server whose key routes predate function grants omits the field.
    server
        .mock("GET", "/api/auth/keys/me/permissions")
        .with_body(fixture["permissions_without_functions"].to_string())
        .create_async()
        .await;
    let older = AdminClient::new(&server.url(), ADMIN_KEY).unwrap();
    assert!(
        older
            .get_permissions()
            .await
            .unwrap()
            .function_permissions
            .is_empty()
    );
}

#[tokio::test]
async fn list_and_get_read_the_function_grant_routes() {
    let fixture = fixture();
    let id = key_id();
    let mut server = authed_server().await;
    modern(&mut server).await;
    let list = server
        .mock(
            "GET",
            format!("/api/auth/keys/{id}/permissions/functions").as_str(),
        )
        .with_body(fixture["function_grants"].to_string())
        .expect(1)
        .create_async()
        .await;
    let one = server
        .mock(
            "GET",
            format!("/api/auth/keys/{id}/permissions/functions/report_%2A").as_str(),
        )
        .with_body(json!({"pattern": "report_*"}).to_string())
        .expect(1)
        .create_async()
        .await;
    let missing = server
        .mock(
            "GET",
            format!("/api/auth/keys/{id}/permissions/functions/nope").as_str(),
        )
        .with_status(404)
        .with_body(r#"{"error":"synthetic-secret"}"#)
        .expect(1)
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), ADMIN_KEY).unwrap();
    let grants = client.list_api_key_function_permissions(&id).await.unwrap();
    assert_eq!(patterns(&grants), ["nightly_close", "report_*"]);
    let grant = client
        .get_api_key_function_permissions(&id, "report_*")
        .await
        .unwrap();
    assert_eq!(grant.pattern, "report_*");
    let error = client
        .get_api_key_function_permissions(&id, "nope")
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Api { code: 404, .. }));
    assert!(!error.to_string().contains("synthetic-secret"));
    list.assert_async().await;
    one.assert_async().await;
    missing.assert_async().await;
}

#[tokio::test]
async fn grant_changes_follow_the_recorded_routes_bodies_and_replies() {
    let fixture = fixture();
    let id = key_id();
    for case in fixture["function_grant_changes"].as_array().unwrap() {
        let mut server = authed_server().await;
        modern(&mut server).await;
        let method = case["method"].as_str().unwrap();
        let mut route = server
            .mock(method, case["path"].as_str().unwrap())
            .with_status(case["status"].as_u64().unwrap() as usize)
            .with_body(case["response"].to_string())
            .expect(1);
        route = if case["body"].is_null() {
            route.match_body(Matcher::Missing)
        } else {
            route.match_body(Matcher::Json(case["body"].clone()))
        };
        let route = route.create_async().await;
        let client = AdminClient::new(&server.url(), ADMIN_KEY).unwrap();
        let pattern = case["pattern"].as_str().unwrap();
        let change = if method == "PUT" {
            client.put_api_key_function_permissions(&id, pattern).await
        } else {
            client
                .delete_api_key_function_permissions(&id, pattern)
                .await
        }
        .unwrap_or_else(|error| panic!("{}: {error}", case["name"]));
        route.assert_async().await;
        let wire = &case["response"];
        assert_eq!(
            change.before.as_ref().map(|grant| &grant.pattern),
            wire["before"]["pattern"]
                .as_str()
                .map(str::to_owned)
                .as_ref(),
            "{}",
            case["name"]
        );
        assert_eq!(
            change.after.as_ref().map(|grant| &grant.pattern),
            wire["after"]["pattern"]
                .as_str()
                .map(str::to_owned)
                .as_ref(),
            "{}",
            case["name"]
        );
        assert_eq!(
            change.metadata.get("ripple_warning"),
            wire.get("ripple_warning"),
            "{}",
            case["name"]
        );
    }
}

#[tokio::test]
async fn refused_grant_changes_surface_the_status_without_the_body() {
    let id = key_id();
    for case in fixture()["refusals"].as_array().unwrap() {
        let mut server = authed_server().await;
        modern(&mut server).await;
        server
            .mock(
                case["method"].as_str().unwrap(),
                format!("/api/auth/keys/{id}/permissions/functions/report_%2A").as_str(),
            )
            .with_status(case["status"].as_u64().unwrap() as usize)
            .with_body(r#"{"error":"synthetic-secret"}"#)
            .create_async()
            .await;
        let client = AdminClient::new(&server.url(), ADMIN_KEY).unwrap();
        let result = if case["method"] == "PUT" {
            client
                .put_api_key_function_permissions(&id, "report_*")
                .await
        } else {
            client
                .delete_api_key_function_permissions(&id, "report_*")
                .await
        };
        let error = result.unwrap_err();
        assert!(
            matches!(&error, Error::Api { code, .. } if u64::from(*code) == case["status"].as_u64().unwrap()),
            "{}: {error}",
            case["name"]
        );
        assert!(!error.to_string().contains("synthetic-secret"));
    }
}

#[tokio::test]
async fn invalid_patterns_and_targets_fail_before_any_request() {
    let fixture = fixture();
    let mut server = Server::new_async().await;
    let token = server
        .mock("POST", "/api/auth/token")
        .expect(0)
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), ADMIN_KEY).unwrap();
    let id = key_id();
    for case in fixture["rejected_patterns"].as_array().unwrap() {
        let pattern = match case["value"].as_str() {
            Some(value) => value.to_owned(),
            None => "a".repeat(case["length"].as_u64().unwrap() as usize),
        };
        assert!(
            matches!(client.get_api_key_function_permissions(&id, &pattern).await, Err(Error::Validation(m)) if m == "Invalid function pattern"),
            "{}",
            case["name"]
        );
        assert!(
            client
                .put_api_key_function_permissions(&id, &pattern)
                .await
                .is_err(),
            "{}",
            case["name"]
        );
        assert!(
            client
                .delete_api_key_function_permissions(&id, &pattern)
                .await
                .is_err(),
            "{}",
            case["name"]
        );
    }
    // A key value is never a target.
    assert!(matches!(
        client
            .put_api_key_function_permissions("synthetic-target-key-0123456789-abcdef", "x")
            .await,
        Err(Error::Validation(_))
    ));
    token.assert_async().await;
}

#[tokio::test]
async fn the_longest_pattern_is_accepted() {
    let id = key_id();
    let pattern = format!("{}*", "a".repeat(MAX_FUNCTION_PATTERN_BYTES - 1));
    assert_eq!(pattern.len(), MAX_FUNCTION_PATTERN_BYTES);
    let mut server = authed_server().await;
    modern(&mut server).await;
    let route = server
        .mock(
            "PUT",
            Matcher::Regex("/permissions/functions/a+%2A$".into()),
        )
        .with_body(json!({"before": null, "after": {"pattern": pattern}}).to_string())
        .expect(1)
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), ADMIN_KEY).unwrap();
    client
        .put_api_key_function_permissions(&id, &pattern)
        .await
        .unwrap();
    route.assert_async().await;
}

#[tokio::test]
async fn function_grant_routes_need_key_addressed_servers() {
    let mut server = authed_server().await;
    server
        .mock("GET", "/api/auth/keys/me/permissions")
        .with_status(404)
        .create_async()
        .await;
    let client = AdminClient::new(&server.url(), ADMIN_KEY).unwrap();
    assert!(matches!(
        client
            .put_api_key_function_permissions("me", "report_*")
            .await,
        Err(Error::Validation(_))
    ));
}

#[test]
fn function_definitions_carry_run_as_and_created_by() {
    let fixture = fixture();
    let function: UserFunction =
        serde_json::from_value(fixture["function_definition"].clone()).unwrap();
    assert_eq!(function.run_as, Some(RunAs::Author));
    assert_eq!(
        function.created_by,
        Some(Principal {
            key_id: key_id(),
            key_label: Some("report-runner".into()),
            subject: Some("alice".into()),
        })
    );
    let unrecorded: UserFunction =
        serde_json::from_value(fixture["function_definition_unrecorded"].clone()).unwrap();
    assert_eq!(unrecorded.run_as, None);
    assert_eq!(unrecorded.created_by, None);

    // A save names run_as; an unset one is omitted so the server applies its own default.
    let saved = UserFunction::new("nightly_close", "Nightly close").with_run_as(RunAs::Caller);
    let body = serde_json::to_value(&saved).unwrap();
    assert_eq!(body["run_as"], "caller");
    assert!(body.get("created_by").is_none());
    let plain = serde_json::to_value(UserFunction::new("f", "F")).unwrap();
    assert!(plain.get("run_as").is_none());

    // Only the two names exist.
    let mut invalid = fixture["function_save_request"].clone();
    invalid["run_as"] = json!("root");
    assert!(serde_json::from_value::<UserFunction>(invalid).is_err());
    let request: UserFunction =
        serde_json::from_value(fixture["function_save_request"].clone()).unwrap();
    assert_eq!(request.run_as, Some(RunAs::Caller));
}

#[tokio::test]
async fn a_refused_function_call_is_the_clients_normal_permission_error() {
    let fixture = fixture();
    let mut server = authed_server().await;
    let call = server
        .mock("POST", "/api/functions/nightly_close")
        .with_status(fixture["function_call_refusal"]["status"].as_u64().unwrap() as usize)
        .with_body(fixture["function_call_refusal"]["body"].to_string())
        .expect(1)
        .create_async()
        .await;
    let client = Client::builder()
        .base_url(server.url())
        .api_key(ADMIN_KEY)
        .should_retry(false)
        .build()
        .unwrap();
    let error = client
        .call_function("nightly_close", None)
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Api { code: 403, .. }), "{error}");
    assert!(!error.is_retryable());
    call.assert_async().await;
}

#[tokio::test]
async fn schedules_return_the_creator_the_server_recorded() {
    let fixture = fixture();
    let record = fixture["schedule_record"].clone();
    let mut server = authed_server().await;
    let id = record["id"].as_str().unwrap();
    server
        .mock("GET", format!("/api/schedules/{id}").as_str())
        .with_body(record.to_string())
        .create_async()
        .await;
    let client = Client::builder()
        .base_url(server.url())
        .api_key(ADMIN_KEY)
        .should_retry(false)
        .build()
        .unwrap();
    let schedule = client.get_schedule(id).await.unwrap();
    assert_eq!(schedule["created_by"], record["created_by"]);
    let principal: Principal = serde_json::from_value(schedule["created_by"].clone()).unwrap();
    assert_eq!(principal.key_label.as_deref(), Some("report-runner"));
    assert_eq!(principal.subject, None);
}

#[tokio::test]
async fn malformed_grant_replies_are_rejected() {
    let id = key_id();
    let url = format!("/api/auth/keys/{id}/permissions/functions/report_%2A");
    for reply in [
        json!({"pattern": ""}),
        json!({}),
        json!({"before": null, "after": {"pattern": ""}}),
        json!({"before": {"pattern": ""}, "after": null}),
        json!({"before": null, "after": {}}),
        json!({"before": null}),
        json!([]),
    ] {
        let mut server = authed_server().await;
        modern(&mut server).await;
        server
            .mock("GET", url.as_str())
            .with_body(reply.to_string())
            .create_async()
            .await;
        server
            .mock("PUT", url.as_str())
            .with_body(reply.to_string())
            .create_async()
            .await;
        let client = AdminClient::new(&server.url(), ADMIN_KEY).unwrap();
        if reply.get("before").is_none() && reply.get("after").is_none() {
            assert!(
                client
                    .get_api_key_function_permissions(&id, "report_*")
                    .await
                    .is_err(),
                "get accepted {reply}"
            );
        }
        if reply.get("pattern").is_none() || reply.get("before").is_some() {
            assert!(
                client
                    .put_api_key_function_permissions(&id, "report_*")
                    .await
                    .is_err(),
                "put accepted {reply}"
            );
        }
    }
}
