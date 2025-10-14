//! Batch operations for efficient bulk inserts, updates, and deletes
//!
//! This module will provide batch operation support.
//! Implementation coming soon.

use crate::types::Record;

/// Builder for batch operations
#[derive(Default)]
pub struct BatchBuilder {
    records: Vec<Record>,
}

impl BatchBuilder {
    /// Create a new batch builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a record to the batch
    pub fn add(mut self, record: Record) -> Self {
        self.records.push(record);
        self
    }

    /// Get the number of records in the batch
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Check if the batch is empty
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Build the batch
    pub fn build(self) -> Vec<Record> {
        self.records
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_builder_new() {
        let builder = BatchBuilder::new();
        assert!(builder.is_empty());
        assert_eq!(builder.len(), 0);
    }

    #[test]
    fn test_batch_builder_default() {
        let builder = BatchBuilder::default();
        assert!(builder.is_empty());
    }

    #[test]
    fn test_batch_builder_add() {
        let mut record = Record::new();
        record.insert("name", "test");

        let builder = BatchBuilder::new().add(record);
        assert_eq!(builder.len(), 1);
        assert!(!builder.is_empty());
    }

    #[test]
    fn test_batch_builder_add_multiple() {
        let mut record1 = Record::new();
        record1.insert("name", "test1");

        let mut record2 = Record::new();
        record2.insert("name", "test2");

        let builder = BatchBuilder::new().add(record1).add(record2);
        assert_eq!(builder.len(), 2);
    }

    #[test]
    fn test_batch_builder_build() {
        let mut record = Record::new();
        record.insert("name", "test");

        let builder = BatchBuilder::new().add(record);
        let records = builder.build();

        assert_eq!(records.len(), 1);
    }

    #[test]
    fn test_batch_builder_chaining() {
        let records: Vec<Record> = (0..5)
            .map(|i| {
                let mut record = Record::new();
                record.insert("id", i);
                record
            })
            .fold(BatchBuilder::new(), |builder, record| builder.add(record))
            .build();

        assert_eq!(records.len(), 5);
    }
}
