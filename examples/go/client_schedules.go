// ekoDB Go Client - Schedules Integration Example
//
// Exercises the full schedule lifecycle:
//   CreateSchedule -> ListSchedules -> GetSchedule -> UpdateSchedule
//   -> PauseSchedule -> ResumeSchedule -> DeleteSchedule

package main

import (
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func main() {
	fmt.Println("=== ekoDB Schedules Integration Example (Go) ===\n")

	_ = godotenv.Load()

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		log.Fatal("API_BASE_KEY environment variable is required")
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}

	// 1. Create a schedule
	fmt.Println("--- Creating schedule ---")
	schedule, err := client.CreateSchedule(map[string]interface{}{
		"name":        "nightly-backup",
		"description": "Runs a database backup every night at midnight",
		"cron":        "0 0 * * *",
		"action": map[string]interface{}{
			"type":       "script",
			"script_id":  "backup_script_1",
			"parameters": map[string]interface{}{"target": "production"},
		},
	})
	if err != nil {
		log.Fatalf("CreateSchedule failed: %v", err)
	}
	scheduleID := schedule["id"].(string)
	fmt.Printf("Created schedule: %s (id: %s)\n", schedule["name"], scheduleID)

	// 2. List all schedules
	fmt.Println("\n--- Listing schedules ---")
	list, err := client.ListSchedules()
	if err != nil {
		log.Fatalf("ListSchedules failed: %v", err)
	}
	fmt.Printf("Schedules: %v\n", list)

	// 3. Get schedule by ID
	fmt.Println("\n--- Getting schedule ---")
	fetched, err := client.GetSchedule(scheduleID)
	if err != nil {
		log.Fatalf("GetSchedule failed: %v", err)
	}
	fmt.Printf("Fetched schedule: %s (cron: %s)\n", fetched["name"], fetched["cron"])

	// 4. Update schedule
	fmt.Println("\n--- Updating schedule ---")
	updated, err := client.UpdateSchedule(scheduleID, map[string]interface{}{
		"cron":        "0 2 * * *",
		"description": "Runs backup at 2 AM instead of midnight",
	})
	if err != nil {
		log.Fatalf("UpdateSchedule failed: %v", err)
	}
	fmt.Printf("Updated cron: %v\n", updated["cron"])

	// 5. Pause schedule
	fmt.Println("\n--- Pausing schedule ---")
	paused, err := client.PauseSchedule(scheduleID)
	if err != nil {
		log.Fatalf("PauseSchedule failed: %v", err)
	}
	fmt.Printf("Schedule status: %s\n", paused["status"])

	// 6. Resume schedule
	fmt.Println("\n--- Resuming schedule ---")
	resumed, err := client.ResumeSchedule(scheduleID)
	if err != nil {
		log.Fatalf("ResumeSchedule failed: %v", err)
	}
	fmt.Printf("Schedule status: %s\n", resumed["status"])

	// 7. Delete schedule
	fmt.Println("\n--- Deleting schedule ---")
	if err := client.DeleteSchedule(scheduleID); err != nil {
		log.Fatalf("DeleteSchedule failed: %v", err)
	}
	fmt.Println("Schedule deleted")

	fmt.Println("\n=== All schedule operations completed successfully ===")
}
