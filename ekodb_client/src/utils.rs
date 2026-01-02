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
/// assert_eq!(value, &json!("user@example.com"));
///
/// let plain = json!("direct_value");
/// let value = get_value(&plain);
/// assert_eq!(value, &plain);
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

/// Transform an entire record by extracting all field values.
/// Preserves the 'id' field and extracts values from all other fields.
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
            if key == "id" {
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
}
