// Concurrency stages — IdempotencyClaim, RateLimit, LockAcquire/Release.
//
// These three primitives wrap ekoDB's atomic KV layer with the right
// semantics for common server-side patterns. Requires ekoDB >= 0.43.0.

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found")
	}
	baseURL := getenv("API_BASE_URL", "http://localhost:8080")
	apiKey := getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("✓ Client created")

	chargeTtl := int64(60)

	// 1. Idempotent payment — claim → branch on replay → do work.
	pay := ekodb.UserFunction{
		Label: "conc_demo_pay",
		Name:  "Idempotent payment",
		Parameters: map[string]ekodb.ParameterDefinition{
			"idempotency_key": {Required: true},
			"amount":          {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageIdempotencyClaim("{{idempotency_key}}", 86400, "claim"),
			ekodb.StageIf(
				ekodb.FunctionCondition{
					Type:       "FieldEquals",
					Field:      "claim.claimed",
					FieldValue: false,
				},
				[]ekodb.FunctionStageConfig{
					ekodb.StageReturn(
						map[string]interface{}{
							"status":          "replay",
							"idempotency_key": "{{idempotency_key}}",
						},
						200,
					),
				},
				[]ekodb.FunctionStageConfig{
					ekodb.StageInsert("charges", map[string]interface{}{
						"amount":          "{{amount}}",
						"idempotency_key": "{{idempotency_key}}",
					}, false, nil),
					ekodb.StageReturn(map[string]interface{}{"status": "charged"}, 201),
				},
			),
		},
	}
	saveFn(client, pay)

	// 2. Rate-limited endpoint (fail mode).
	rlFail := ekodb.UserFunction{
		Label: "conc_demo_rl_fail",
		Name:  "Rate-limit (fail mode)",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_id": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageRateLimit("user-{{user_id}}", 10, 60, "rl", ""),
			ekodb.StageReturn(map[string]interface{}{"ok": true}, 200),
		},
	}
	saveFn(client, rlFail)

	// 3. Rate-limited endpoint (skip mode).
	rlSkip := ekodb.UserFunction{
		Label: "conc_demo_rl_skip",
		Name:  "Rate-limit (skip mode)",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_id": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageRateLimit("user-{{user_id}}", 10, 60, "rl", "skip"),
			ekodb.StageIf(
				ekodb.FunctionCondition{
					Type:       "FieldEquals",
					Field:      "rl.allowed",
					FieldValue: false,
				},
				[]ekodb.FunctionStageConfig{
					ekodb.StageReturn(map[string]interface{}{"status": "rate_limited"}, 429),
				},
				[]ekodb.FunctionStageConfig{
					ekodb.StageReturn(map[string]interface{}{"status": "ok"}, 200),
				},
			),
		},
	}
	saveFn(client, rlSkip)

	// 4. Distributed lock — acquire + critical section + release (token-fenced).
	lock := ekodb.UserFunction{
		Label: "conc_demo_lock",
		Name:  "Critical section under lock",
		Parameters: map[string]ekodb.ParameterDefinition{
			"resource": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageLockAcquire("{{resource}}", 30, "lock"),
			ekodb.StageIf(
				ekodb.FunctionCondition{
					Type:       "FieldEquals",
					Field:      "lock.acquired",
					FieldValue: false,
				},
				[]ekodb.FunctionStageConfig{
					ekodb.StageReturn(map[string]interface{}{"status": "busy"}, 409),
				},
				[]ekodb.FunctionStageConfig{
					ekodb.StageInsert("lock_demo_audit", map[string]interface{}{
						"resource": "{{resource}}",
					}, false, &chargeTtl),
					ekodb.StageLockRelease("{{resource}}", "{{lock.token}}", "release"),
					ekodb.StageReturn(map[string]interface{}{"status": "done"}, 200),
				},
			),
		},
	}
	saveFn(client, lock)

	fmt.Println("\nInvoke them like:")
	fmt.Println(`  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }`)
	fmt.Println(`  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }`)
	fmt.Println(`  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }`)
	fmt.Println(`  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }`)

	for _, label := range []string{
		"conc_demo_pay", "conc_demo_rl_fail",
		"conc_demo_rl_skip", "conc_demo_lock",
	} {
		_ = client.DeleteUserFunction(label)
	}
	fmt.Println("\n✓ Cleaned up demo functions")
}

func saveFn(client *ekodb.Client, f ekodb.UserFunction) {
	if _, err := client.SaveUserFunction(f); err != nil {
		fmt.Printf("SaveUserFunction(%s) error: %v\n", f.Label, err)
		return
	}
	fmt.Printf("✓ %s saved\n", f.Label)
}

func getenv(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}
