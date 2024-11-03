// types_test.go
package ekodb

import (
	"encoding/json"
	"reflect"
	"testing"
)

func TestValueType(t *testing.T) {
	tests := []struct {
		name     string
		valType  ValueType
		expected string
	}{
		{"string type", StringType, "String"},
		{"integer type", IntegerType, "Integer"},
		{"float type", FloatType, "Float"},
		{"boolean type", BooleanType, "Boolean"},
		{"date type", DateType, "Date"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if string(tt.valType) != tt.expected {
				t.Errorf("ValueType = %v, want %v", tt.valType, tt.expected)
			}
		})
	}
}

func TestTypedValue(t *testing.T) {
	tests := []struct {
		name      string
		value     TypedValue
		wantJSON  string
		wantValue interface{}
	}{
		{
			name:      "string value",
			value:     TypedValue{Type: StringType, Value: "test"},
			wantJSON:  `{"type":"String","value":"test"}`,
			wantValue: "test",
		},
		{
			name:      "integer value",
			value:     TypedValue{Type: IntegerType, Value: float64(42)},
			wantJSON:  `{"type":"Integer","value":42}`,
			wantValue: float64(42),
		},
		{
			name:      "boolean value",
			value:     TypedValue{Type: BooleanType, Value: true},
			wantJSON:  `{"type":"Boolean","value":true}`,
			wantValue: true,
		},
		{
			name:      "float value",
			value:     TypedValue{Type: FloatType, Value: 3.14},
			wantJSON:  `{"type":"Float","value":3.14}`,
			wantValue: 3.14,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			data, err := json.Marshal(tt.value)
			if err != nil {
				t.Fatalf("Failed to marshal TypedValue: %v", err)
			}

			var decoded TypedValue
			if err := json.Unmarshal(data, &decoded); err != nil {
				t.Fatalf("Failed to unmarshal TypedValue: %v", err)
			}

			if !reflect.DeepEqual(decoded.Value, tt.wantValue) {
				t.Errorf("TypedValue.Value = %v (%T), want %v (%T)",
					decoded.Value, decoded.Value,
					tt.wantValue, tt.wantValue)
			}
		})
	}
}

func TestFilterOperators(t *testing.T) {
	tests := []struct {
		name     string
		operator FilterOperator
		expected string
	}{
		{"equals operator", EqOp, "Eq"},
		{"greater than operator", GtOp, "Gt"},
		{"less than operator", LtOp, "Lt"},
		{"greater than or equal operator", GteOp, "Gte"},
		{"less than or equal operator", LteOp, "Lte"},
		{"contains operator", ContainsOp, "Contains"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if string(tt.operator) != tt.expected {
				t.Errorf("FilterOperator = %v, want %v", tt.operator, tt.expected)
			}
		})
	}
}

func TestLogicalOperators(t *testing.T) {
	tests := []struct {
		name     string
		operator LogicalOperator
		expected string
	}{
		{"AND operator", AndOp, "And"},
		{"OR operator", OrOp, "Or"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if string(tt.operator) != tt.expected {
				t.Errorf("LogicalOperator = %v, want %v", tt.operator, tt.expected)
			}
		})
	}
}

func TestRecord(t *testing.T) {
	record := Record{
		"name": TypedValue{Type: StringType, Value: "John"},
		"age":  TypedValue{Type: IntegerType, Value: float64(30)},
	}

	data, err := json.Marshal(record)
	if err != nil {
		t.Fatalf("Failed to marshal Record: %v", err)
	}

	var decoded Record
	if err := json.Unmarshal(data, &decoded); err != nil {
		t.Fatalf("Failed to unmarshal Record: %v", err)
	}

	if len(decoded) != 2 {
		t.Errorf("Record length = %v, want 2", len(decoded))
	}

	name, ok := decoded["name"]
	if !ok {
		t.Error("Record missing 'name' field")
	} else {
		if name.Type != StringType {
			t.Errorf("name field type = %v, want %v", name.Type, StringType)
		}
		if name.Value != "John" {
			t.Errorf("name field value = %v, want John", name.Value)
		}
	}
}

func TestConditionFilter(t *testing.T) {
	filter := ConditionFilter{
		Type: "Condition",
		Content: ConditionContent{
			Field:    "age",
			Operator: GtOp,
			Value:    TypedValue{Type: IntegerType, Value: float64(25)},
		},
	}

	data, err := json.Marshal(filter)
	if err != nil {
		t.Fatalf("Failed to marshal ConditionFilter: %v", err)
	}

	var decoded ConditionFilter
	if err := json.Unmarshal(data, &decoded); err != nil {
		t.Fatalf("Failed to unmarshal ConditionFilter: %v", err)
	}

	if decoded.Type != "Condition" {
		t.Errorf("ConditionFilter.Type = %v, want Condition", decoded.Type)
	}
	if decoded.Content.Field != "age" {
		t.Errorf("ConditionFilter.Content.Field = %v, want age", decoded.Content.Field)
	}
	if decoded.Content.Operator != GtOp {
		t.Errorf("ConditionFilter.Content.Operator = %v, want %v", decoded.Content.Operator, GtOp)
	}
}

func TestLogicalFilter(t *testing.T) {
	filter := LogicalFilter{
		Type: "Logical",
		Content: LogicalContent{
			Operator: AndOp,
			Expressions: []Filter{
				&ConditionFilter{
					Type: "Condition",
					Content: ConditionContent{
						Field:    "age",
						Operator: GtOp,
						Value:    TypedValue{Type: IntegerType, Value: float64(25)},
					},
				},
				&ConditionFilter{
					Type: "Condition",
					Content: ConditionContent{
						Field:    "name",
						Operator: ContainsOp,
						Value:    TypedValue{Type: StringType, Value: "John"},
					},
				},
			},
		},
	}

	data, err := json.Marshal(filter)
	if err != nil {
		t.Fatalf("Failed to marshal LogicalFilter: %v", err)
	}

	var decoded LogicalFilter
	if err := json.Unmarshal(data, &decoded); err != nil {
		t.Fatalf("Failed to unmarshal LogicalFilter: %v", err)
	}

	if decoded.Type != "Logical" {
		t.Errorf("LogicalFilter.Type = %v, want Logical", decoded.Type)
	}
	if decoded.Content.Operator != AndOp {
		t.Errorf("LogicalFilter.Content.Operator = %v, want %v", decoded.Content.Operator, AndOp)
	}
	if len(decoded.Content.Expressions) != 2 {
		t.Errorf("LogicalFilter.Content.Expressions length = %v, want 2", len(decoded.Content.Expressions))
	}
}

func TestFindOptions(t *testing.T) {
	filter := Filter(&ConditionFilter{
		Type: "Condition",
		Content: ConditionContent{
			Field:    "age",
			Operator: GtOp,
			Value:    TypedValue{Type: IntegerType, Value: float64(25)},
		},
	})

	opts := FindOptions{
		Filter: &filter,
		Sort: []SortOption{
			{Field: "age", Ascending: true},
		},
		Limit: 10,
		Skip:  0,
	}

	data, err := json.Marshal(opts)
	if err != nil {
		t.Fatalf("Failed to marshal FindOptions: %v", err)
	}

	var decoded FindOptions
	if err := json.Unmarshal(data, &decoded); err != nil {
		t.Fatalf("Failed to unmarshal FindOptions: %v", err)
	}

	if decoded.Limit != 10 {
		t.Errorf("FindOptions.Limit = %v, want 10", decoded.Limit)
	}
	if len(decoded.Sort) != 1 {
		t.Errorf("FindOptions.Sort length = %v, want 1", len(decoded.Sort))
	}
	if decoded.Filter == nil {
		t.Error("FindOptions.Filter is nil, want non-nil")
	}
}

func TestBatchResponses(t *testing.T) {
	t.Run("BatchInsertResponse", func(t *testing.T) {
		resp := BatchInsertResponse{
			InsertedIDs: []string{"id1", "id2"},
			Success:     true,
		}

		data, err := json.Marshal(resp)
		if err != nil {
			t.Fatalf("Failed to marshal BatchInsertResponse: %v", err)
		}

		var decoded BatchInsertResponse
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("Failed to unmarshal BatchInsertResponse: %v", err)
		}

		if !reflect.DeepEqual(decoded.InsertedIDs, resp.InsertedIDs) {
			t.Errorf("BatchInsertResponse.InsertedIDs = %v, want %v", decoded.InsertedIDs, resp.InsertedIDs)
		}
		if decoded.Success != resp.Success {
			t.Errorf("BatchInsertResponse.Success = %v, want %v", decoded.Success, resp.Success)
		}
	})

	t.Run("BatchUpdateResponse", func(t *testing.T) {
		resp := BatchUpdateResponse{
			UpdatedCount: 2,
			Success:      true,
		}

		data, err := json.Marshal(resp)
		if err != nil {
			t.Fatalf("Failed to marshal BatchUpdateResponse: %v", err)
		}

		var decoded BatchUpdateResponse
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("Failed to unmarshal BatchUpdateResponse: %v", err)
		}

		if decoded.UpdatedCount != resp.UpdatedCount {
			t.Errorf("BatchUpdateResponse.UpdatedCount = %v, want %v", decoded.UpdatedCount, resp.UpdatedCount)
		}
		if decoded.Success != resp.Success {
			t.Errorf("BatchUpdateResponse.Success = %v, want %v", decoded.Success, resp.Success)
		}
	})

	t.Run("BatchDeleteResponse", func(t *testing.T) {
		resp := BatchDeleteResponse{
			DeletedCount: 2,
			Success:      true,
		}

		data, err := json.Marshal(resp)
		if err != nil {
			t.Fatalf("Failed to marshal BatchDeleteResponse: %v", err)
		}

		var decoded BatchDeleteResponse
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("Failed to unmarshal BatchDeleteResponse: %v", err)
		}

		if decoded.DeletedCount != resp.DeletedCount {
			t.Errorf("BatchDeleteResponse.DeletedCount = %v, want %v", decoded.DeletedCount, resp.DeletedCount)
		}
		if decoded.Success != resp.Success {
			t.Errorf("BatchDeleteResponse.Success = %v, want %v", decoded.Success, resp.Success)
		}
	})
}

func TestSchemaTypes(t *testing.T) {
	t.Run("SchemaField", func(t *testing.T) {
		field := SchemaField{
			Type:     StringType,
			Required: true,
			Default:  "default value",
		}

		data, err := json.Marshal(field)
		if err != nil {
			t.Fatalf("Failed to marshal SchemaField: %v", err)
		}

		var decoded SchemaField
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("Failed to unmarshal SchemaField: %v", err)
		}

		if decoded.Type != StringType {
			t.Errorf("SchemaField.Type = %v, want %v", decoded.Type, StringType)
		}
		if !decoded.Required {
			t.Errorf("SchemaField.Required = %v, want true", decoded.Required)
		}
		if decoded.Default != "default value" {
			t.Errorf("SchemaField.Default = %v, want 'default value'", decoded.Default)
		}
	})

	t.Run("CollectionSchema", func(t *testing.T) {
		schema := CollectionSchema{
			Fields: map[string]SchemaField{
				"name": {
					Type:     StringType,
					Required: true,
				},
				"age": {
					Type:     IntegerType,
					Required: false,
					Default:  float64(18),
				},
			},
		}

		data, err := json.Marshal(schema)
		if err != nil {
			t.Fatalf("Failed to marshal CollectionSchema: %v", err)
		}

		var decoded CollectionSchema
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("Failed to unmarshal CollectionSchema: %v", err)
		}

		if len(decoded.Fields) != 2 {
			t.Errorf("CollectionSchema.Fields length = %v, want 2", len(decoded.Fields))
		}

		nameField, ok := decoded.Fields["name"]
		if !ok {
			t.Error("CollectionSchema.Fields missing 'name' field")
		} else {
			if nameField.Type != StringType {
				t.Errorf("name field type = %v, want %v", nameField.Type, StringType)
			}
			if !nameField.Required {
				t.Error("name field should be required")
			}
		}

		ageField, ok := decoded.Fields["age"]
		if !ok {
			t.Error("CollectionSchema.Fields missing 'age' field")
		} else {
			if ageField.Type != IntegerType {
				t.Errorf("age field type = %v, want %v", ageField.Type, IntegerType)
			}
			if ageField.Required {
				t.Error("age field should not be required")
			}
			if ageField.Default != float64(18) {
				t.Errorf("age field default = %v, want 18", ageField.Default)
			}
		}
	})

	t.Run("SchemaField with nil default", func(t *testing.T) {
		field := SchemaField{
			Type:     StringType,
			Required: false,
			Default:  nil,
		}

		data, err := json.Marshal(field)
		if err != nil {
			t.Fatalf("Failed to marshal SchemaField: %v", err)
		}

		var decoded SchemaField
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("Failed to unmarshal SchemaField: %v", err)
		}

		if decoded.Default != nil {
			t.Errorf("SchemaField.Default = %v, want nil", decoded.Default)
		}
	})
}

func TestSortOption(t *testing.T) {
	tests := []struct {
		name     string
		option   SortOption
		wantJSON string
	}{
		{
			name: "ascending sort",
			option: SortOption{
				Field:     "age",
				Ascending: true,
			},
			wantJSON: `{"field":"age","ascending":true}`,
		},
		{
			name: "descending sort",
			option: SortOption{
				Field:     "name",
				Ascending: false,
			},
			wantJSON: `{"field":"name","ascending":false}`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			data, err := json.Marshal(tt.option)
			if err != nil {
				t.Fatalf("Failed to marshal SortOption: %v", err)
			}

			if string(data) != tt.wantJSON {
				t.Errorf("SortOption JSON = %v, want %v", string(data), tt.wantJSON)
			}

			var decoded SortOption
			if err := json.Unmarshal(data, &decoded); err != nil {
				t.Fatalf("Failed to unmarshal SortOption: %v", err)
			}

			if decoded.Field != tt.option.Field {
				t.Errorf("SortOption.Field = %v, want %v", decoded.Field, tt.option.Field)
			}
			if decoded.Ascending != tt.option.Ascending {
				t.Errorf("SortOption.Ascending = %v, want %v", decoded.Ascending, tt.option.Ascending)
			}
		})
	}
}

func TestNestedTypedValue(t *testing.T) {
	t.Run("nested object", func(t *testing.T) {
		value := TypedValue{
			Type: "Object",
			Value: map[string]interface{}{
				"name": "John",
				"age":  30,
			},
		}

		data, err := json.Marshal(value)
		if err != nil {
			t.Fatalf("Failed to marshal nested TypedValue: %v", err)
		}

		var decoded TypedValue
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("Failed to unmarshal nested TypedValue: %v", err)
		}

		decodedMap, ok := decoded.Value.(map[string]interface{})
		if !ok {
			t.Fatalf("Expected map[string]interface{}, got %T", decoded.Value)
		}

		if decodedMap["name"] != "John" {
			t.Errorf("nested name = %v, want John", decodedMap["name"])
		}

		// JSON numbers are float64 by default
		if decodedMap["age"] != float64(30) {
			t.Errorf("nested age = %v, want 30", decodedMap["age"])
		}
	})

	t.Run("nested array", func(t *testing.T) {
		value := TypedValue{
			Type:  "Array",
			Value: []interface{}{1, "two", true},
		}

		data, err := json.Marshal(value)
		if err != nil {
			t.Fatalf("Failed to marshal array TypedValue: %v", err)
		}

		var decoded TypedValue
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("Failed to unmarshal array TypedValue: %v", err)
		}

		decodedArray, ok := decoded.Value.([]interface{})
		if !ok {
			t.Fatalf("Expected []interface{}, got %T", decoded.Value)
		}

		if len(decodedArray) != 3 {
			t.Errorf("array length = %v, want 3", len(decodedArray))
		}

		// JSON numbers are float64 by default
		if decodedArray[0] != float64(1) {
			t.Errorf("array[0] = %v, want 1", decodedArray[0])
		}
		if decodedArray[1] != "two" {
			t.Errorf("array[1] = %v, want 'two'", decodedArray[1])
		}
		if decodedArray[2] != true {
			t.Errorf("array[2] = %v, want true", decodedArray[2])
		}
	})
}
