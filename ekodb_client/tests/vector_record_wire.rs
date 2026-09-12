use ekodb_client::{FieldType, Record};
use serde_json::{Value, json};

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
    record.insert("embedding", FieldType::vector(vec![1.0, 0.0, 0.0]));
    assert_eq!(serde_json::to_value(&record).unwrap(), golden);
    let packed = rmp_serde::to_vec(&record).unwrap();
    assert_eq!(rmp_serde::from_slice::<Value>(&packed).unwrap(), golden);
    let vector = FieldType::vector(vec![1.0, 0.0, 0.0]);
    assert_eq!(
        serde_json::to_value(&vector).unwrap(),
        json!({"type": "Vector", "value": [1.0, 0.0, 0.0]})
    );
    assert_eq!(
        serde_json::from_value::<FieldType>(serde_json::to_value(&vector).unwrap()).unwrap(),
        vector
    );
}
