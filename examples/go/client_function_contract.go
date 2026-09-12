// Offline contract smoke test for stored-function stage serialization.
package main

import (
	"encoding/json"
	"fmt"

	ekodb "github.com/ekoDB/ekodb-client-go"
)

func requireStage(stages []map[string]interface{}, stageType string) map[string]interface{} {
	for _, stage := range stages {
		if stage["type"] == stageType {
			return stage
		}
	}
	panic("missing " + stageType + " stage")
}

func require(ok bool, message string) {
	if !ok {
		panic(message)
	}
}

func rawStage(stageType string, data map[string]interface{}) ekodb.FunctionStageConfig {
	return ekodb.FunctionStageConfig{Stage: stageType, Data: data}
}

func main() {
	filter := map[string]interface{}{
		"type": "Condition",
		"content": map[string]interface{}{
			"field": "status", "operator": "Eq", "value": "active",
		},
	}
	limit, threshold := 3, 0.8
	timeout := uint64(10)
	model := "text-embedding-3-small"
	isolation := "Serializable"
	stages := []ekodb.FunctionStageConfig{
		ekodb.StageQuery("items", filter, nil, &limit, nil),
		ekodb.StageUpdate("items", filter, map[string]interface{}{"status": "ready"}, false, nil),
		ekodb.StageDelete("items", filter, false),
		ekodb.StageGroup([]string{"category"}, []ekodb.GroupFunctionConfig{
			{OutputField: "labels", Operation: ekodb.GroupFunctionOp("AddToSet"), InputField: stringPtr("label")},
			{OutputField: "score_stddev", Operation: ekodb.GroupFunctionOp("StandardDeviation"), InputField: stringPtr("score")},
			{OutputField: "unique_users", Operation: ekodb.GroupFunctionOp("ApproxDistinct"), InputField: stringPtr("user_id")},
		}),
		ekodb.StageIf(
			ekodb.ConditionFieldGreaterThanOrEqual("stock", 1),
			[]ekodb.FunctionStageConfig{rawStage("SetField", map[string]interface{}{"field": "available", "value": true})},
			nil,
		),
		rawStage("Upsert", map[string]interface{}{
			"collection": "items", "key": "sku", "value": "sku-1",
			"record": map[string]interface{}{"name": "Widget"},
		}),
		rawStage("Increment", map[string]interface{}{
			"collection": "items", "record_id": "item-1", "field": "views", "by": 2,
		}),
		rawStage("Push", map[string]interface{}{
			"collection": "items", "record_id": "item-1", "field": "tags", "value": "new",
		}),
		rawStage("SetField", map[string]interface{}{"field": "source", "value": "contract"}),
		rawStage("AddFields", map[string]interface{}{
			"fields": []map[string]interface{}{{
				"field_name": "total",
				"expression": map[string]interface{}{"type": "FieldReference", "value": "price"},
			}},
		}),
		rawStage("CurrentDatetime", map[string]interface{}{"output_field": "processed_at"}),
		ekodb.StageVectorSearch("items", []float64{0.1, 0.2}, &limit, &threshold),
		ekodb.StageHybridSearch("items", "widget", []float64{0.1, 0.2}, map[string]interface{}{"limit": 3}),
		ekodb.StageFindOneAndUpdate("items", "item-1", map[string]interface{}{"status": "claimed"}, true, int64Ptr(60)),
		ekodb.StageUpdateWithAction("items", "item-1", string(ekodb.UpdateActionIncrement), "views", 1, true),
		ekodb.StageBatchDelete("items", []string{"item-1", "item-2"}, true),
		ekodb.StageEmbed("body", "embedding", &model),
		ekodb.StageHttpRequestWithOptions(
			"https://example.com/items",
			"GET",
			nil,
			nil,
			&ekodb.HttpRequestOptions{TimeoutSeconds: &timeout, OutputField: "response"},
		),
	}
	function := ekodb.UserFunction{
		Label:      "client_function_contract",
		Name:       "Client function contract",
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions:  stages,
		TransactionConfig: &ekodb.TransactionConfig{
			Enabled: true, AutoRollback: true, IsolationLevel: &isolation,
		},
	}

	payload, err := json.Marshal(function)
	if err != nil {
		panic(err)
	}
	var encoded struct {
		TransactionConfig map[string]interface{}   `json:"transaction_config"`
		Functions         []map[string]interface{} `json:"functions"`
	}
	if err := json.Unmarshal(payload, &encoded); err != nil {
		panic(err)
	}
	require(encoded.TransactionConfig["isolation_level"] == "Serializable", "transaction_config mismatch")
	for _, stageType := range []string{"Query", "Update", "Delete"} {
		stage := requireStage(encoded.Functions, stageType)
		stageFilter, ok := stage["filter"].(map[string]interface{})
		require(ok && stageFilter["type"] == "Condition", stageType+" filter mismatch")
	}
	group := requireStage(encoded.Functions, "Group")
	aggregates := group["functions"].([]interface{})
	require(aggregates[2].(map[string]interface{})["operation"] == "ApproxDistinct", "group operation mismatch")
	condition := requireStage(encoded.Functions, "If")["condition"].(map[string]interface{})
	require(condition["type"] == "FieldGreaterThanOrEqual", "function condition mismatch")
	recordIDs := requireStage(encoded.Functions, "BatchDelete")["record_ids"].([]interface{})
	require(recordIDs[1] == "item-2", "batch delete record_ids mismatch")
	fields := requireStage(encoded.Functions, "AddFields")["fields"].([]interface{})
	expression := fields[0].(map[string]interface{})["expression"].(map[string]interface{})
	require(expression["type"] == "FieldReference" && expression["value"] == "price", "add fields expression mismatch")
	require(requireStage(encoded.Functions, "Embed")["input_field"] == "body", "embed field mismatch")
	require(requireStage(encoded.Functions, "HttpRequest")["timeout_seconds"] == float64(10), "http timeout mismatch")
	require(requireStage(encoded.Functions, "HttpRequest")["output_field"] == "response", "http output field mismatch")
	for _, stageType := range []string{
		"Upsert", "Increment", "Push", "SetField", "AddFields", "CurrentDatetime",
		"VectorSearch", "HybridSearch", "FindOneAndUpdate", "UpdateWithAction",
	} {
		requireStage(encoded.Functions, stageType)
	}

	fmt.Println("client_function_contract: ok")
}

func stringPtr(value string) *string { return &value }
func int64Ptr(value int64) *int64    { return &value }
