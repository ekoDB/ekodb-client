use ekodb_client::{FieldType, Record};
use serde_json::{Value, json};
use std::collections::HashMap;

#[test]
fn explicit_vector_envelope_matches_complete_insertion_fixture() {
    let golden: Value =
        serde_json::from_str(include_str!("../../test-fixtures/vector-record.json")).unwrap();
    let mut record = Record::new();
    record.insert("label", "vector-golden");
    record.insert("count", 7_i64);
    record.insert("eligible", true);
    record.insert(
        "ordinary",
        FieldType::array(vec![
            FieldType::float(1.0),
            FieldType::float(0.0),
            FieldType::float(0.0),
        ]),
    );
    // Rust's untagged FieldType::Vector currently loses its tag as well.
    // Construct the established envelope explicitly; this test does not claim
    // that Rust's vector helper has been fixed or exercised against the server.
    record.insert(
        "embedding",
        FieldType::Object(HashMap::from([
            ("type".into(), FieldType::string("Vector")),
            ("value".into(), FieldType::vector(vec![1.0, 0.0, 0.0])),
        ])),
    );
    assert_eq!(serde_json::to_value(&record).unwrap(), golden);
    let packed = rmp_serde::to_vec(&record).unwrap();
    assert_eq!(rmp_serde::from_slice::<Value>(&packed).unwrap(), golden);
    assert_eq!(
        serde_json::to_value(FieldType::vector(vec![1.0, 0.0, 0.0])).unwrap(),
        json!([1.0, 0.0, 0.0])
    );
}
