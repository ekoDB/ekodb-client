//! Structured, degraded-tolerant health for ekoDB's `/api/health`.
//!
//! Mirrors the `ekodb-client-go` health contract exactly.

use serde::Serialize;
use serde_json::Value;

/// A health status value. Like ekodb-client-go's `HealthState` (a string type),
/// an off-contract status reported by the server is preserved verbatim.
pub type HealthState = String;

/// Canonical [`HealthStatus::status`] values.
pub const HEALTH_OK: &str = "ok";
pub const HEALTH_DEGRADED: &str = "degraded";
pub const HEALTH_UNKNOWN: &str = "unknown";

/// A snapshot of an ekoDB `/api/health` probe.
///
/// Degraded-tolerant: a reachable server that reports `degraded` is a
/// successful snapshot (`reachable: true`, `status: "degraded"`), NOT an error.
/// An unreachable/unparseable probe yields `{ reachable: false, status:
/// "unknown" }`. A missing/odd `status` on a reachable body fails safe to
/// `"degraded"`; a non-empty string status is preserved verbatim.
///
/// [`detail`](HealthStatus::detail) (the full admin body, which includes
/// internal metrics and collection names) is skipped during serialization
/// (`#[serde(skip)]`) so surfacing the snapshot cannot leak internals.
#[derive(Debug, Clone, Serialize)]
pub struct HealthStatus {
    pub reachable: bool,
    pub status: HealthState,
    pub integrity_ok: bool,
    #[serde(skip)]
    pub detail: Option<Value>,
}

impl HealthStatus {
    /// The snapshot returned when the server is unreachable or its response is
    /// unparseable.
    pub(crate) fn unreachable() -> Self {
        HealthStatus {
            reachable: false,
            status: HEALTH_UNKNOWN.to_string(),
            integrity_ok: false,
            detail: None,
        }
    }
}

/// Interprets a parsed `/api/health` body per the shared health contract.
///
/// A missing/odd `status` on a reachable body fails safe to `"degraded"`; a
/// non-empty string status is preserved verbatim (matching ekodb-client-go).
/// `integrity_ok` is read from the top-level field (public response) or nested
/// `integrity.healthy` (admin response). A body that is not a JSON object yields
/// an unreachable `"unknown"` snapshot.
pub fn parse_health_status(body: &Value) -> HealthStatus {
    let obj = match body.as_object() {
        Some(o) => o,
        None => return HealthStatus::unreachable(),
    };

    // Default to degraded; a non-empty string status is kept verbatim.
    let mut status = HEALTH_DEGRADED.to_string();
    if let Some(s) = obj.get("status").and_then(Value::as_str) {
        if !s.is_empty() {
            status = s.to_string();
        }
    }

    let integrity_ok = obj
        .get("integrity_ok")
        .and_then(Value::as_bool)
        .or_else(|| {
            obj.get("integrity")
                .and_then(|i| i.get("healthy"))
                .and_then(Value::as_bool)
        })
        .unwrap_or(false);

    HealthStatus {
        reachable: true,
        status,
        integrity_ok,
        detail: Some(body.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn degraded_is_reachable() {
        let hs = parse_health_status(&json!({"status": "degraded", "integrity_ok": false}));
        assert!(hs.reachable);
        assert_eq!(hs.status.as_str(), HEALTH_DEGRADED);
        assert!(!hs.integrity_ok);
    }

    #[test]
    fn missing_status_fails_safe_to_degraded() {
        let hs = parse_health_status(&json!({"integrity_ok": true}));
        assert!(hs.reachable);
        assert_eq!(hs.status.as_str(), HEALTH_DEGRADED);
    }

    #[test]
    fn off_contract_string_status_is_preserved_verbatim() {
        // Matches ekodb-client-go: a non-empty string status is kept as-is.
        let hs = parse_health_status(&json!({"status": "healthy"}));
        assert_eq!(hs.status.as_str(), "healthy");
    }

    #[test]
    fn non_string_status_fails_safe_to_degraded() {
        let hs = parse_health_status(&json!({"status": {"nested": 1}}));
        assert_eq!(hs.status.as_str(), HEALTH_DEGRADED);
    }

    #[test]
    fn reads_integrity_from_admin_nested_shape() {
        let hs = parse_health_status(
            &json!({"status": "ok", "integrity": {"healthy": true, "manifest_load_failed": []}}),
        );
        assert_eq!(hs.status.as_str(), HEALTH_OK);
        assert!(hs.integrity_ok);
    }

    #[test]
    fn non_object_body_is_unreachable_unknown() {
        let hs = parse_health_status(&json!("not an object"));
        assert!(!hs.reachable);
        assert_eq!(hs.status.as_str(), HEALTH_UNKNOWN);
    }

    #[test]
    fn serialization_excludes_detail() {
        let hs = parse_health_status(
            &json!({"status": "degraded", "integrity": {"manifest_load_failed": ["secret_collection"]}}),
        );
        let out = serde_json::to_string(&hs).unwrap();
        assert!(!out.contains("detail"), "detail must not serialize: {out}");
        assert!(
            !out.contains("secret_collection"),
            "internal body leaked: {out}"
        );
        assert!(out.contains("\"status\":\"degraded\""));
        assert!(out.contains("\"integrity_ok\":false"));
        assert!(hs.detail.is_some(), "detail still readable in-process");
    }
}
