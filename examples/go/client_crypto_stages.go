// Crypto stages — comprehensive demo of every primitive added in
// ekoDB 0.42.0 (HMAC, AES-GCM, UUID, TOTP, Base64, Hex, Slugify).
//
//	crypto_demo_hmac      HmacSign + HmacVerify (round trip)
//	crypto_demo_aes       AesEncrypt + AesDecrypt (round trip)
//	crypto_demo_uuid      UuidGenerate
//	crypto_demo_totp      TotpGenerate (RFC 6238)
//	crypto_demo_encoding  Base64 + Hex + Slugify
//
// Operator-owned secrets flow through {{env.NAME}} so they never land
// in the function definition stored on disk.
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

	digits := 6
	period := uint64(30)

	// 1. HMAC sign + verify round trip.
	hmac := ekodb.UserFunction{
		Label: "crypto_demo_hmac",
		Name:  "HMAC sign + verify",
		Parameters: map[string]ekodb.ParameterDefinition{
			"payload": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageHmacSign("{{payload}}", "{{env.HMAC_KEY}}", "mac", "sha256", "hex"),
			ekodb.StageHmacVerify("{{payload}}", "{{mac}}", "{{env.HMAC_KEY}}", "verified", "sha256", "hex"),
		},
	}
	saveFn(client, hmac)

	// 2. AES-256-GCM encrypt + decrypt round trip.
	aes := ekodb.UserFunction{
		Label: "crypto_demo_aes",
		Name:  "AES encrypt + decrypt",
		Parameters: map[string]ekodb.ParameterDefinition{
			"plaintext": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageAesEncrypt("{{plaintext}}", "{{env.DATA_KEY}}", "envelope", "hex"),
			ekodb.StageAesDecrypt("envelope", "{{env.DATA_KEY}}", "recovered", "hex"),
		},
	}
	saveFn(client, aes)

	// 3. UuidGenerate.
	uuidFn := ekodb.UserFunction{
		Label:      "crypto_demo_uuid",
		Name:       "Generate v4 UUID",
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageUuidGenerate("id"),
		},
	}
	saveFn(client, uuidFn)

	// 4. TotpGenerate (RFC 6238 with SHA1).
	totp := ekodb.UserFunction{
		Label:      "crypto_demo_totp",
		Name:       "Generate TOTP code",
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageTotpGenerate(
				"{{env.TOTP_SECRET}}",
				"code",
				&ekodb.TotpOptions{Digits: &digits, Period: &period, Algorithm: "sha1"},
			),
		},
	}
	saveFn(client, totp)

	// 5. Base64 + Hex + Slugify chained.
	encoding := ekodb.UserFunction{
		Label: "crypto_demo_encoding",
		Name:  "Base64 / Hex / Slugify",
		Parameters: map[string]ekodb.ParameterDefinition{
			"title": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageBase64Encode("{{title}}", "title_b64", nil),
			ekodb.StageHexEncode("{{title}}", "title_hex"),
			ekodb.StageSlugify("{{title}}", "title_slug"),
		},
	}
	saveFn(client, encoding)

	fmt.Println("\nInvoke them with:")
	fmt.Println(`  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }`)
	fmt.Println(`  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }`)
	fmt.Println("  POST /api/functions/crypto_demo_uuid")
	fmt.Println("  POST /api/functions/crypto_demo_totp")
	fmt.Println(`  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }`)

	for _, label := range []string{
		"crypto_demo_hmac", "crypto_demo_aes", "crypto_demo_uuid",
		"crypto_demo_totp", "crypto_demo_encoding",
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
