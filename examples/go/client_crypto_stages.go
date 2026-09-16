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
	"errors"
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

var cryptoLabels = []string{
	"crypto_demo_hmac_go", "crypto_demo_aes_go", "crypto_demo_uuid_go",
	"crypto_demo_totp_go", "crypto_demo_encoding_go",
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
	ownedLabels := make([]string, 0, len(cryptoLabels))
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

	digits := 6
	period := uint64(30)

	// 1. HMAC sign + verify round trip.
	hmac := ekodb.UserFunction{
		Label: cryptoLabels[0],
		Name:  "HMAC sign + verify",
		Parameters: map[string]ekodb.ParameterDefinition{
			"payload": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageHmacSign("{{payload}}", "{{env.HMAC_KEY}}", "mac", "sha256", "hex"),
			ekodb.StageHmacVerify("{{payload}}", "{{mac}}", "{{env.HMAC_KEY}}", "verified", "sha256", "hex"),
		},
	}
	if err := saveFn(client, hmac); err != nil {
		return err
	}
	ownedLabels = append(ownedLabels, hmac.Label)

	// 2. AES-256-GCM encrypt + decrypt round trip.
	aes := ekodb.UserFunction{
		Label: cryptoLabels[1],
		Name:  "AES encrypt + decrypt",
		Parameters: map[string]ekodb.ParameterDefinition{
			"plaintext": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageAesEncrypt("{{plaintext}}", "{{env.DATA_KEY}}", "envelope", "hex"),
			ekodb.StageAesDecrypt("envelope", "{{env.DATA_KEY}}", "recovered", "hex"),
		},
	}
	if err := saveFn(client, aes); err != nil {
		return err
	}
	ownedLabels = append(ownedLabels, aes.Label)

	// 3. UuidGenerate.
	uuidFn := ekodb.UserFunction{
		Label:      cryptoLabels[2],
		Name:       "Generate v4 UUID",
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageUuidGenerate("id"),
		},
	}
	if err := saveFn(client, uuidFn); err != nil {
		return err
	}
	ownedLabels = append(ownedLabels, uuidFn.Label)

	// 4. TotpGenerate (RFC 6238 with SHA1).
	totp := ekodb.UserFunction{
		Label:      cryptoLabels[3],
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
	if err := saveFn(client, totp); err != nil {
		return err
	}
	ownedLabels = append(ownedLabels, totp.Label)

	// 5. Base64 + Hex + Slugify chained.
	encoding := ekodb.UserFunction{
		Label: cryptoLabels[4],
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
	if err := saveFn(client, encoding); err != nil {
		return err
	}
	ownedLabels = append(ownedLabels, encoding.Label)

	fmt.Println("\nInvoke them with:")
	fmt.Printf("  POST /api/functions/%s     { \"payload\": \"hi\" }\n", cryptoLabels[0])
	fmt.Printf("  POST /api/functions/%s      { \"plaintext\": \"secret\" }\n", cryptoLabels[1])
	fmt.Printf("  POST /api/functions/%s\n", cryptoLabels[2])
	fmt.Printf("  POST /api/functions/%s\n", cryptoLabels[3])
	fmt.Printf("  POST /api/functions/%s { \"title\": \"Héllo World\" }\n", cryptoLabels[4])
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
