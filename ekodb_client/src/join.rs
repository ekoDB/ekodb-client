//! Join support for multi-collection queries
//!
//! This module provides support for joining data across multiple collections,
//! similar to SQL joins but with document-oriented semantics.

use serde::{Deserialize, Serialize};

/// Configuration for joining collections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinConfig {
    /// Target collections to join with
    pub collections: Vec<String>,

    /// Field in the current collection
    pub local_field: String,

    /// Field in the target collection(s)
    pub foreign_field: String,

    /// Name of the field to store joined data
    pub as_field: String,
}

impl JoinConfig {
    /// Create a new join configuration
    pub fn new(
        collections: Vec<String>,
        local_field: impl Into<String>,
        foreign_field: impl Into<String>,
        as_field: impl Into<String>,
    ) -> Self {
        Self {
            collections,
            local_field: local_field.into(),
            foreign_field: foreign_field.into(),
            as_field: as_field.into(),
        }
    }

    /// Create a join with a single collection
    pub fn single(
        collection: impl Into<String>,
        local_field: impl Into<String>,
        foreign_field: impl Into<String>,
        as_field: impl Into<String>,
    ) -> Self {
        Self::new(
            vec![collection.into()],
            local_field,
            foreign_field,
            as_field,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_config_new() {
        let join = JoinConfig::new(
            vec!["orders".to_string(), "products".to_string()],
            "user_id",
            "id",
            "user_data",
        );

        assert_eq!(join.collections.len(), 2);
        assert_eq!(join.collections[0], "orders");
        assert_eq!(join.collections[1], "products");
        assert_eq!(join.local_field, "user_id");
        assert_eq!(join.foreign_field, "id");
        assert_eq!(join.as_field, "user_data");
    }

    #[test]
    fn test_join_config_single() {
        let join = JoinConfig::single("users", "user_id", "id", "user");

        assert_eq!(join.collections.len(), 1);
        assert_eq!(join.collections[0], "users");
        assert_eq!(join.local_field, "user_id");
        assert_eq!(join.foreign_field, "id");
        assert_eq!(join.as_field, "user");
    }

    #[test]
    fn test_join_config_serialization() {
        let join = JoinConfig::single("users", "user_id", "id", "user");

        let json = serde_json::to_value(&join).unwrap();
        assert_eq!(json["collections"][0], "users");
        assert_eq!(json["local_field"], "user_id");
        assert_eq!(json["foreign_field"], "id");
        assert_eq!(json["as_field"], "user");
    }

    #[test]
    fn test_join_config_deserialization() {
        let json = serde_json::json!({
            "collections": ["users"],
            "local_field": "user_id",
            "foreign_field": "id",
            "as_field": "user"
        });

        let join: JoinConfig = serde_json::from_value(json).unwrap();
        assert_eq!(join.collections.len(), 1);
        assert_eq!(join.collections[0], "users");
        assert_eq!(join.local_field, "user_id");
        assert_eq!(join.foreign_field, "id");
        assert_eq!(join.as_field, "user");
    }

    #[test]
    fn test_multi_collection_join() {
        let join = JoinConfig::new(
            vec![
                "users".to_string(),
                "profiles".to_string(),
                "settings".to_string(),
            ],
            "user_id",
            "id",
            "user_info",
        );

        assert_eq!(join.collections.len(), 3);
        assert!(join.collections.contains(&"users".to_string()));
        assert!(join.collections.contains(&"profiles".to_string()));
        assert!(join.collections.contains(&"settings".to_string()));
    }
}
