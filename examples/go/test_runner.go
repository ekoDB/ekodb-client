package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"time"

	"github.com/joho/godotenv"
)

// Colors
const (
	colorReset  = "\033[0m"
	colorRed    = "\033[31m"
	colorGreen  = "\033[32m"
	colorYellow = "\033[33m"
	colorBlue   = "\033[34m"
)

func log(message, color string) {
	fmt.Printf("%s%s%s\n", color, message, colorReset)
}

func checkServer() bool {
	var baseURL = os.Getenv("API_BASE_URL")
	log("\n=== Checking Server Connection ===", colorBlue)

	for i := 0; i < 30; i++ {
		resp, err := http.Get(baseURL + "/api/health")
		if err == nil && resp.StatusCode == 200 {
			resp.Body.Close()
			log("✓ Server is ready", colorGreen)
			return true
		}
		fmt.Print(".")
		time.Sleep(1 * time.Second)
	}

	log("\n✗ Server not responding", colorRed)
	return false
}

func getAuthToken() (string, error) {
	var baseURL = os.Getenv("API_BASE_URL")
	var apiKey = os.Getenv("API_BASE_KEY")
	log("\n=== Getting Authentication Token ===", colorBlue)

	authReq := map[string]string{"api_key": apiKey}
	body, _ := json.Marshal(authReq)

	resp, err := http.Post(baseURL+"/api/auth/token", "application/json", bytes.NewBuffer(body))
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	var result map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&result)

	log("✓ Authentication successful", colorGreen)
	return result["token"].(string), nil
}

func runExample(file string) bool {
	log(fmt.Sprintf("\n=== Running %s ===", filepath.Base(file)), colorBlue)

	// Get the directory containing the file
	dir := filepath.Dir(file)

	cmd := exec.Command("go", "run", filepath.Base(file))
	cmd.Dir = dir
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	err := cmd.Run()
	if err != nil {
		log(fmt.Sprintf("✗ %s failed", filepath.Base(file)), colorRed)
		return false
	}

	log(fmt.Sprintf("✓ %s completed successfully", filepath.Base(file)), colorGreen)
	return true
}

func main() {
	// Load environment variables from the .env file
	err := godotenv.Load()
	if err != nil {
		log(fmt.Sprintf("✗ Failed to load environment variables: %v", err), colorRed)
		os.Exit(1)
	}

	log("\n╔════════════════════════════════════════╗", colorBlue)
	log("║     ekoDB Go Examples Test Suite      ║", colorBlue)
	log("╚════════════════════════════════════════╝", colorBlue)

	// Check server
	if !checkServer() {
		os.Exit(1)
	}

	// List of examples to run
	examples := []string{
		"ttl-caching/simple_crud.go",
		"ttl-caching/simple_websocket.go",
		"ttl-caching/batch_operations.go",
		"ttl-caching/kv_operations.go",
		"ttl-caching/collection_management.go",
		"ttl-caching/document_ttl.go",
		"ttl-caching/websocket_ttl.go",
	}

	log(fmt.Sprintf("\n=== Running %d Examples ===", len(examples)), colorYellow)

	passed := 0
	failed := 0

	for _, example := range examples {
		if runExample(example) {
			passed++
		} else {
			failed++
			log("Continuing with next example...", colorYellow)
		}
		time.Sleep(2 * time.Second)
	}

	// Summary
	log("\n╔════════════════════════════════════════╗", colorBlue)
	log("║           Test Summary                 ║", colorBlue)
	log("╚════════════════════════════════════════╝", colorBlue)
	log(fmt.Sprintf("Total: %d", len(examples)), colorBlue)
	log(fmt.Sprintf("Passed: %d", passed), colorGreen)
	if failed > 0 {
		log(fmt.Sprintf("Failed: %d", failed), colorRed)
		os.Exit(1)
	} else {
		log(fmt.Sprintf("Failed: %d", failed), colorGreen)
	}
}
