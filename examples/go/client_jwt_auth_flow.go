// JWT auth flow — registration + login as pure stored functions.
//
// Demonstrates the canonical password-auth pattern using ekoDB stages:
//
//	register flow:  BcryptHash → Insert
//	login flow:     FindOne  → BcryptVerify → If(ok) { JwtSign + Return }
//	verify flow:    JwtVerify → If(claims != null) { 200 } else { 401 }
//
// Operator-owned secrets (the JWT signing key) flow through
// {{env.JWT_SECRET}} so they never enter the function definition or
// the LLM context window.
//
// Requires ekoDB >= 0.42.0.

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

	cost := 12
	expires := int64(3600)

	// 1. Register: bcrypt-hash, insert.
	register := ekodb.UserFunction{
		Label:       "go_users_register",
		Name:        "Register user",
		Description: strPtr("Validate, bcrypt-hash, insert."),
		Parameters: map[string]ekodb.ParameterDefinition{
			"email":    {Required: true},
			"password": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageBcryptHash("{{password}}", "password_hash", &cost),
			ekodb.StageInsert("go_users", map[string]interface{}{
				"email":         "{{email}}",
				"password_hash": "{{password_hash}}",
			}, false, nil),
		},
	}
	_save(client, register)

	// 2. Login: find user, verify bcrypt, sign JWT on success.
	login := ekodb.UserFunction{
		Label:       "go_users_login",
		Name:        "Login user",
		Description: strPtr("Verify password, mint JWT."),
		Parameters: map[string]ekodb.ParameterDefinition{
			"email":    {Required: true},
			"password": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageFindOne("go_users", "email", "{{email}}"),
			ekodb.StageBcryptVerify("{{password}}", "password_hash", "password_ok"),
			ekodb.StageIf(
				ekodb.FunctionCondition{
					Type:       "FieldEquals",
					Field:      "password_ok",
					FieldValue: true,
				},
				[]ekodb.FunctionStageConfig{
					ekodb.StageJwtSign(
						map[string]interface{}{"email": "{{email}}"},
						"{{env.JWT_SECRET}}",
						"token",
						&expires,
						"HS256",
					),
					ekodb.StageReturn(
						map[string]interface{}{"ok": true, "token": "{{token}}"},
						200,
					),
				},
				[]ekodb.FunctionStageConfig{
					ekodb.StageReturn(map[string]interface{}{"ok": false}, 401),
				},
			),
		},
	}
	_save(client, login)

	// 3. Verify a JWT — fail-closed when claims is null.
	verifyTtl := int64(60)
	verify := ekodb.UserFunction{
		Label: "go_users_verify_token",
		Name:  "Verify JWT token",
		Parameters: map[string]ekodb.ParameterDefinition{
			"token": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			// Synthetic record so JwtVerify has working_data[0] to read off.
			ekodb.StageInsert(
				"_inflight_jwt_check",
				map[string]interface{}{"token": "{{token}}"},
				true,
				&verifyTtl,
			),
			ekodb.StageJwtVerify("token", "{{env.JWT_SECRET}}", "claims", "HS256"),
			ekodb.StageIf(
				ekodb.FunctionCondition{
					Type:       "FieldEquals",
					Field:      "claims",
					FieldValue: nil,
				},
				[]ekodb.FunctionStageConfig{
					ekodb.StageReturn(map[string]interface{}{"ok": false}, 401),
				},
				[]ekodb.FunctionStageConfig{
					ekodb.StageReturn(map[string]interface{}{"ok": true}, 200),
				},
			),
		},
	}
	_save(client, verify)

	fmt.Println("\n=== Auth flow defined as pure stored functions ===")
	fmt.Println("Call them like:")
	fmt.Println(`  POST /api/functions/go_users_register { "email": "a@b.com", "password": "s3cret" }`)
	fmt.Println(`  POST /api/functions/go_users_login    { "email": "a@b.com", "password": "s3cret" }`)
	fmt.Println(`  POST /api/functions/go_users_verify_token { "token": "<jwt>" }`)

	for _, label := range []string{
		"go_users_register",
		"go_users_login",
		"go_users_verify_token",
	} {
		_ = client.DeleteUserFunction(label)
	}
	fmt.Println("\n✓ Cleaned up demo functions")
}

func _save(client *ekodb.Client, f ekodb.UserFunction) {
	if _, err := client.SaveUserFunction(f); err != nil {
		fmt.Printf("SaveUserFunction(%s) error: %v\n", f.Label, err)
		return
	}
	fmt.Printf("✓ %s saved\n", f.Label)
}

func strPtr(s string) *string { return &s }

func getenv(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}
