use serde_json::Value;
use std::collections::HashMap;

/// Extract the value from an ekoDB field object.
/// ekoDB returns fields as `{"type": "String", "value": "..."}` objects.
/// This helper safely extracts the value or returns the input if it's not a field object.
///
/// # Examples
///
/// ```
/// use ekodb_client::get_value;
/// use serde_json::json;
///
/// let field = json!({"type": "String", "value": "user@example.com"});
/// let value = get_value(&field);
/// assert_eq!(value, json!("user@example.com"));
///
/// let plain = json!("direct_value");
/// let value = get_value(&plain);
/// assert_eq!(value, plain);
/// ```
pub fn get_value(field: &Value) -> Value {
    if let Value::Object(map) = field {
        if let Some(value) = map.get("value") {
            return value.clone();
        }
    }
    field.clone()
}

/// Extract a string value from an ekoDB field
pub fn get_string_value(field: &Value) -> Option<String> {
    get_value(field).as_str().map(|s| s.to_string())
}

/// Extract an integer value from an ekoDB field
pub fn get_int_value(field: &Value) -> Option<i64> {
    get_value(field).as_i64()
}

/// Extract a float value from an ekoDB field
pub fn get_float_value(field: &Value) -> Option<f64> {
    get_value(field).as_f64()
}

/// Extract a boolean value from an ekoDB field
pub fn get_bool_value(field: &Value) -> Option<bool> {
    get_value(field).as_bool()
}

/// Extract a DateTime string from an ekoDB DateTime field
pub fn get_datetime_value(field: &Value) -> Option<String> {
    get_string_value(field)
}

/// Extract a UUID string from an ekoDB UUID field
pub fn get_uuid_value(field: &Value) -> Option<String> {
    get_string_value(field)
}

/// Extract a decimal value from an ekoDB Decimal field
pub fn get_decimal_value(field: &Value) -> Option<f64> {
    get_float_value(field)
}

/// Extract duration in milliseconds from an ekoDB Duration field
pub fn get_duration_value(field: &Value) -> Option<i64> {
    let val = get_value(field);
    if let Some(i) = val.as_i64() {
        return Some(i);
    }
    if let Some(obj) = val.as_object() {
        let secs = obj.get("secs")?.as_i64().unwrap_or(0);
        let nanos = obj.get("nanos")?.as_i64().unwrap_or(0);
        return Some(secs * 1000 + nanos / 1_000_000);
    }
    None
}

/// Extract bytes from an ekoDB Bytes field
pub fn get_bytes_value(field: &Value) -> Option<Vec<u8>> {
    let val = get_value(field);
    if let Some(arr) = val.as_array() {
        return Some(
            arr.iter()
                .filter_map(|v| v.as_u64().map(|n| n as u8))
                .collect(),
        );
    }
    if let Some(s) = val.as_str() {
        return base64::Engine::decode(&base64::engine::general_purpose::STANDARD, s).ok();
    }
    None
}

/// Extract bytes from an ekoDB Binary field
pub fn get_binary_value(field: &Value) -> Option<Vec<u8>> {
    get_bytes_value(field)
}

/// Extract an array from an ekoDB Array field
pub fn get_array_value(field: &Value) -> Option<Vec<Value>> {
    let val = get_value(field);
    val.as_array().cloned()
}

/// Extract an array from an ekoDB Set field
pub fn get_set_value(field: &Value) -> Option<Vec<Value>> {
    let val = get_value(field);
    val.as_array().cloned()
}

/// Extract a vector of floats from an ekoDB Vector field
pub fn get_vector_value(field: &Value) -> Option<Vec<f64>> {
    let val = get_value(field);
    if let Some(arr) = val.as_array() {
        return Some(arr.iter().filter_map(|v| v.as_f64()).collect());
    }
    None
}

/// Extract an object from an ekoDB Object field
pub fn get_object_value(field: &Value) -> Option<serde_json::Map<String, Value>> {
    let val = get_value(field);
    val.as_object().cloned()
}

/// Extract values from multiple fields in a record.
/// Useful for processing entire records returned from ekoDB.
///
/// # Examples
///
/// ```
/// use ekodb_client::get_values;
/// use serde_json::json;
///
/// let record = json!({
///     "email": {"type": "String", "value": "user@example.com"},
///     "age": {"type": "Integer", "value": 25},
///     "id": "123"
/// });
///
/// let fields = vec!["email".to_string(), "age".to_string()];
/// let values = get_values(&record, &fields);
/// assert_eq!(values.get("email").unwrap(), "user@example.com");
/// ```
pub fn get_values(record: &Value, fields: &[String]) -> HashMap<String, Value> {
    let mut result = HashMap::new();

    if let Value::Object(map) = record {
        for field in fields {
            if let Some(value) = map.get(field) {
                result.insert(field.clone(), get_value(value));
            }
        }
    }

    result
}

/// Common ID field names in ekoDB records, ordered by priority.
/// The primary_key_alias is configurable per collection (default: "id").
const ID_CANDIDATES: &[&str] = &["id", "_id"];

/// Extract the record ID from a JSON record (API response).
///
/// ekoDB's `primary_key_alias` is configurable per collection, so the ID field
/// might be `"id"` (default), `"_id"`, `"user_id"`, etc. This function tries
/// common candidates. For custom aliases, pass them via `extra_candidates`.
///
/// # Examples
///
/// ```
/// use ekodb_client::extract_record_id;
/// use serde_json::json;
///
/// let record = json!({"id": "abc123", "name": "Alice"});
/// assert_eq!(extract_record_id(&record, &[]), Some("abc123".to_string()));
///
/// let record = json!({"_id": "def456", "name": "Bob"});
/// assert_eq!(extract_record_id(&record, &[]), Some("def456".to_string()));
///
/// let record = json!({"user_id": "ghi789", "name": "Charlie"});
/// assert_eq!(extract_record_id(&record, &["user_id"]), Some("ghi789".to_string()));
/// ```
pub fn extract_record_id(record: &Value, extra_candidates: &[&str]) -> Option<String> {
    if let Value::Object(map) = record {
        // Try extra candidates first (collection-specific alias)
        for key in extra_candidates {
            if let Some(val) = map.get(*key) {
                if let Some(s) = get_string_value(val) {
                    return Some(s);
                }
            }
        }
        // Then try common defaults
        for key in ID_CANDIDATES {
            if let Some(val) = map.get(*key) {
                if let Some(s) = get_string_value(val) {
                    return Some(s);
                }
            }
        }
    }
    None
}

/// Transform an entire record by extracting all field values.
/// Preserves known ID fields (`id`, `_id`) as-is and extracts values from all other fields.
///
/// # Examples
///
/// ```
/// use ekodb_client::extract_record;
/// use serde_json::json;
///
/// let record = json!({
///     "id": "123",
///     "email": {"type": "String", "value": "user@example.com"},
///     "first_name": {"type": "String", "value": "John"}
/// });
///
/// let plain = extract_record(&record);
/// assert_eq!(plain["id"], "123");
/// assert_eq!(plain["email"], "user@example.com");
/// assert_eq!(plain["first_name"], "John");
/// ```
pub fn extract_record(record: &Value) -> Value {
    if let Value::Object(map) = record {
        let mut result = serde_json::Map::new();

        for (key, value) in map {
            // Preserve ID fields as-is (don't unwrap typed wrappers)
            if ID_CANDIDATES.contains(&key.as_str()) {
                result.insert(key.clone(), value.clone());
            } else {
                result.insert(key.clone(), get_value(value));
            }
        }

        Value::Object(result)
    } else {
        record.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_value_from_field_object() {
        let field = json!({"type": "String", "value": "test@example.com"});
        let result = get_value(&field);
        assert_eq!(result, json!("test@example.com"));
    }

    #[test]
    fn test_get_value_from_plain_value() {
        let field = json!("direct_value");
        let result = get_value(&field);
        assert_eq!(result, json!("direct_value"));
    }

    #[test]
    fn test_get_string_value() {
        let field = json!({"type": "String", "value": "hello"});
        let result = get_string_value(&field);
        assert_eq!(result, Some("hello".to_string()));
    }

    #[test]
    fn test_get_int_value() {
        let field = json!({"type": "Integer", "value": 42});
        let result = get_int_value(&field);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_extract_record() {
        let record = json!({
            "id": "user123",
            "email": {"type": "String", "value": "user@example.com"},
            "age": {"type": "Integer", "value": 30}
        });

        let result = extract_record(&record);
        assert_eq!(result["id"], "user123");
        assert_eq!(result["email"], "user@example.com");
        assert_eq!(result["age"], 30);
    }

    #[test]
    fn test_extract_record_id_default() {
        let record = json!({"id": "abc123", "name": "Alice"});
        assert_eq!(extract_record_id(&record, &[]), Some("abc123".to_string()));
    }

    #[test]
    fn test_extract_record_id_underscore() {
        let record = json!({"_id": "def456", "name": "Bob"});
        assert_eq!(extract_record_id(&record, &[]), Some("def456".to_string()));
    }

    #[test]
    fn test_extract_record_id_custom_alias() {
        let record = json!({"user_id": "ghi789", "name": "Charlie"});
        assert_eq!(
            extract_record_id(&record, &["user_id"]),
            Some("ghi789".to_string())
        );
    }

    #[test]
    fn test_extract_record_id_custom_takes_priority() {
        // Both "id" and "user_id" exist — custom should win
        let record = json!({"id": "default", "user_id": "custom", "name": "Dave"});
        assert_eq!(
            extract_record_id(&record, &["user_id"]),
            Some("custom".to_string())
        );
    }

    #[test]
    fn test_extract_record_id_typed_wrapper() {
        // ID as typed wrapper ({"type": "String", "value": "..."})
        let record = json!({"id": {"type": "String", "value": "wrapped123"}, "name": "Eve"});
        assert_eq!(
            extract_record_id(&record, &[]),
            Some("wrapped123".to_string())
        );
    }

    #[test]
    fn test_extract_record_id_missing() {
        let record = json!({"name": "NoId"});
        assert_eq!(extract_record_id(&record, &[]), None);
    }

    #[test]
    fn test_extract_record_id_not_object() {
        let record = json!("just a string");
        assert_eq!(extract_record_id(&record, &[]), None);
    }
}
