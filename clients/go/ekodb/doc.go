// Package ekodb provides a type-safe client for interacting with EkoDB databases.
//
// EkoDB is a strongly-typed document database that provides runtime type safety
// and schema validation. This client package offers a Go interface that maintains
// these type guarantees while providing an idiomatic Go API.
//
// The package provides comprehensive support for:
//   - CRUD operations with type safety
//   - Batch operations for bulk data handling
//   - Query filtering with complex conditions
//   - Collection schema management
//   - Automatic authentication and token refresh
//
// Error Handling:
//
// The package uses custom error types for detailed error information through
// the APIError type and predefined errors like ErrNotFound.
//
// Query Filters:
//
// The package supports complex query filters through ConditionFilter and
// LogicalFilter types, allowing for sophisticated data querying.
//
// Batch Operations:
//
// The client provides efficient batch operations for inserting, updating,
// and deleting multiple records in a single request.
//
// Schema Management:
//
// Collections can be managed through the GetSchema method, providing
// information about field types and constraints.
//
// Context Support:
//
// All operations support context.Context for timeout and cancellation control.
//
// Thread Safety:
//
// The client is safe for concurrent use by multiple goroutines.
// The underlying HTTP client handles connection pooling and reuse.
package ekodb
