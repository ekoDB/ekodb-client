// types.go
package ekodb

import (
	"encoding/json"
	"fmt"
)

// ValueType represents the type of a field value
type ValueType string

const (
	StringType  ValueType = "String"
	IntegerType ValueType = "Integer"
	FloatType   ValueType = "Float"
	BooleanType ValueType = "Boolean"
	DateType    ValueType = "Date"
)

// TypedValue represents a value with its type information
type TypedValue struct {
	Type  ValueType   `json:"type"`
	Value interface{} `json:"value"`
}

// FilterOperator represents comparison operators for queries
type FilterOperator string

const (
	EqOp       FilterOperator = "Eq"
	GtOp       FilterOperator = "Gt"
	LtOp       FilterOperator = "Lt"
	GteOp      FilterOperator = "Gte"
	LteOp      FilterOperator = "Lte"
	ContainsOp FilterOperator = "Contains"
)

// LogicalOperator represents logical operators for combining filters
type LogicalOperator string

const (
	AndOp LogicalOperator = "And"
	OrOp  LogicalOperator = "Or"
)

// Record represents a document with typed fields
type Record map[string]TypedValue

// Filter interface for all filter types
type Filter interface {
	GetType() string
	isFilter()
}

// ConditionFilter represents a single condition in a query
type ConditionFilter struct {
	Type    string           `json:"type"`
	Content ConditionContent `json:"content"`
}

func (f ConditionFilter) GetType() string { return f.Type }
func (f ConditionFilter) isFilter()       {}

// ConditionContent represents the content of a condition filter
type ConditionContent struct {
	Field    string         `json:"field"`
	Operator FilterOperator `json:"operator"`
	Value    TypedValue     `json:"value"`
}

// LogicalFilter represents a logical combination of filters
type LogicalFilter struct {
	Type    string         `json:"type"`
	Content LogicalContent `json:"content"`
}

func (f LogicalFilter) GetType() string { return f.Type }
func (f LogicalFilter) isFilter()       {}

// LogicalContent represents the content of a logical filter
type LogicalContent struct {
	Operator    LogicalOperator `json:"operator"`
	Expressions []Filter        `json:"expressions"`
}

// Custom JSON marshaling/unmarshaling for LogicalContent to handle Filter interface
func (c *LogicalContent) UnmarshalJSON(data []byte) error {
	var raw struct {
		Operator    LogicalOperator   `json:"operator"`
		Expressions []json.RawMessage `json:"expressions"`
	}
	if err := json.Unmarshal(data, &raw); err != nil {
		return err
	}

	c.Operator = raw.Operator
	c.Expressions = make([]Filter, len(raw.Expressions))
	for i, exprData := range raw.Expressions {
		filter, err := unmarshalFilter(exprData)
		if err != nil {
			return err
		}
		c.Expressions[i] = filter
	}
	return nil
}

func (c LogicalContent) MarshalJSON() ([]byte, error) {
	var raw struct {
		Operator    LogicalOperator `json:"operator"`
		Expressions []Filter        `json:"expressions"`
	}
	raw.Operator = c.Operator
	raw.Expressions = c.Expressions
	return json.Marshal(raw)
}

// Helper function to unmarshal Filter interface
func unmarshalFilter(data []byte) (Filter, error) {
	var raw struct {
		Type string `json:"type"`
	}
	if err := json.Unmarshal(data, &raw); err != nil {
		return nil, err
	}

	switch raw.Type {
	case "Condition":
		var cf ConditionFilter
		if err := json.Unmarshal(data, &cf); err != nil {
			return nil, err
		}
		return &cf, nil
	case "Logical":
		var lf LogicalFilter
		if err := json.Unmarshal(data, &lf); err != nil {
			return nil, err
		}
		return &lf, nil
	default:
		return nil, fmt.Errorf("unknown filter type: %s", raw.Type)
	}
}

// SortOption represents sorting criteria
type SortOption struct {
	Field     string `json:"field"`
	Ascending bool   `json:"ascending"`
}

// FindOptions represents query options
type FindOptions struct {
	Filter *Filter      `json:"filter,omitempty"`
	Sort   []SortOption `json:"sort,omitempty"`
	Limit  int          `json:"limit,omitempty"`
	Skip   int          `json:"skip,omitempty"`
}

// UnmarshalJSON for FindOptions to handle Filter interface
func (o *FindOptions) UnmarshalJSON(data []byte) error {
	var raw struct {
		Filter json.RawMessage `json:"filter"`
		Sort   []SortOption    `json:"sort"`
		Limit  int             `json:"limit"`
		Skip   int             `json:"skip"`
	}
	if err := json.Unmarshal(data, &raw); err != nil {
		return err
	}

	o.Sort = raw.Sort
	o.Limit = raw.Limit
	o.Skip = raw.Skip

	if raw.Filter != nil {
		filter, err := unmarshalFilter(raw.Filter)
		if err != nil {
			return err
		}
		o.Filter = &filter
	}
	return nil
}

// Batch operation response types
type BatchInsertResponse struct {
	InsertedIDs []string `json:"insertedIds"`
	Success     bool     `json:"success"`
}

// BatchUpdateResponse represents the response from a batch update operation
type BatchUpdateResponse struct {
	UpdatedCount int  `json:"updatedCount"`
	Success      bool `json:"success"`
}

// BatchDeleteResponse represents the response from a batch delete operation
type BatchDeleteResponse struct {
	DeletedCount int  `json:"deletedCount"`
	Success      bool `json:"success"`
}

// SchemaField represents a field definition in a collection schema
type SchemaField struct {
	Type     ValueType   `json:"type"`
	Required bool        `json:"required,omitempty"`
	Default  interface{} `json:"default,omitempty"`
}

// CollectionSchema represents the schema of a collection
type CollectionSchema struct {
	Fields map[string]SchemaField `json:"fields"`
}
