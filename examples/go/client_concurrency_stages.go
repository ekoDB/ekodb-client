// Concurrency stages — IdempotencyClaim, RateLimit, LockAcquire/Release.
//
// These three primitives wrap ekoDB's atomic KV layer with the right
// semantics for common server-side patterns. Requires ekoDB >= 0.42.0.

package main

import (
	"errors"
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

var concurrencyLabels = []string{
	"conc_demo_pay_go", "conc_demo_rl_fail_go",
	"conc_demo_rl_skip_go", "conc_demo_lock_go",
}

func run() (runErr error) {
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found")
	}
	baseURL := getenv("API_BASE_URL", "http://localhost:8080")
	apiKey := getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return err
	}
	fmt.Println("✓ Client created")
	ownedLabels := make([]string, 0, len(concurrencyLabels))
	defer func() {
		for _, label := range ownedLabels {
			if err := client.DeleteUserFunction(label); err != nil {
				runErr = errors.Join(runErr, fmt.Errorf("cleanup function %s: %w", label, err))
			}
		}
		if runErr == nil {
			fmt.Println("\n✓ Cleaned up demo functions")
		}
	}()

	chargeTtl := int64(60)

	// 1. Idempotent payment — claim → branch on replay → do work.
	pay := ekodb.UserFunction{
		Label: concurrencyLabels[0],
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
					ekodb.StageInsert("concurrency_charges_go", map[string]interface{}{
						"amount":          "{{amount}}",
						"idempotency_key": "{{idempotency_key}}",
					}, false, nil),
					ekodb.StageReturn(map[string]interface{}{"status": "charged"}, 201),
				},
			),
		},
	}
	if err := saveFn(client, pay); err != nil {
		return err
	}
	ownedLabels = append(ownedLabels, pay.Label)

	// 2. Rate-limited endpoint (fail mode).
	rlFail := ekodb.UserFunction{
		Label: concurrencyLabels[1],
		Name:  "Rate-limit (fail mode)",
		Parameters: map[string]ekodb.ParameterDefinition{
			"user_id": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageRateLimit("user-{{user_id}}", 10, 60, "rl", ""),
			ekodb.StageReturn(map[string]interface{}{"ok": true}, 200),
		},
	}
	if err := saveFn(client, rlFail); err != nil {
		return err
	}
	ownedLabels = append(ownedLabels, rlFail.Label)

	// 3. Rate-limited endpoint (skip mode).
	rlSkip := ekodb.UserFunction{
		Label: concurrencyLabels[2],
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
	if err := saveFn(client, rlSkip); err != nil {
		return err
	}
	ownedLabels = append(ownedLabels, rlSkip.Label)

	// 4. Distributed lock — acquire + critical section + release (token-fenced).
	lock := ekodb.UserFunction{
		Label: concurrencyLabels[3],
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
					ekodb.StageInsert("concurrency_lock_audit_go", map[string]interface{}{
						"resource": "{{resource}}",
					}, false, &chargeTtl),
					ekodb.StageLockRelease("{{resource}}", "{{lock.token}}", "release"),
					ekodb.StageReturn(map[string]interface{}{"status": "done"}, 200),
				},
			),
		},
	}
	if err := saveFn(client, lock); err != nil {
		return err
	}
	ownedLabels = append(ownedLabels, lock.Label)

	fmt.Println("\nInvoke them like:")
	fmt.Printf("  POST /api/functions/%s        { \"idempotency_key\": \"...\", \"amount\": 100 }\n", concurrencyLabels[0])
	fmt.Printf("  POST /api/functions/%s    { \"user_id\": 42 }\n", concurrencyLabels[1])
	fmt.Printf("  POST /api/functions/%s    { \"user_id\": 42 }\n", concurrencyLabels[2])
	fmt.Printf("  POST /api/functions/%s       { \"resource\": \"queue:drain\" }\n", concurrencyLabels[3])

	return nil
}

func saveFn(client *ekodb.Client, f ekodb.UserFunction) error {
	_, err := client.SaveUserFunction(f)
	if err == nil {
		fmt.Printf("✓ %s saved\n", f.Label)
		return nil
	}
	var httpErr *ekodb.HTTPError
	if errors.As(err, &httpErr) && httpErr.StatusCode == 409 {
		if uerr := client.UpdateUserFunction(f.Label, f); uerr != nil {
			return fmt.Errorf("update function %s: %w", f.Label, uerr)
		}
		fmt.Printf("✓ %s already existed — updated instead\n", f.Label)
		return nil
	}
	return fmt.Errorf("save function %s: %w", f.Label, err)
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}

func getenv(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}
