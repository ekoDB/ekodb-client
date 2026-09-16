// ekoDB Go Client - Schedules Integration Example
//
// Exercises the full schedule lifecycle:
//   CreateSchedule -> ListSchedules -> GetSchedule -> UpdateSchedule
//   -> TriggerSchedule -> PauseSchedule -> ResumeSchedule -> DeleteSchedule
//
// A temporary no-op function makes the trigger step self-contained.

package main

import (
	"errors"
	"fmt"
	"log"
	"os"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func run() (runErr error) {
	fmt.Print("=== ekoDB Schedules Integration Example (Go) ===\n\n")

	_ = godotenv.Load()

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		return errors.New("API_BASE_KEY environment variable is required")
	}

	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return fmt.Errorf("create client: %w", err)
	}

	functionLabel := fmt.Sprintf("schedule_noop_go_%d", os.Getpid())
	functionID, err := client.SaveFunction(ekodb.UserFunction{
		Label:      functionLabel,
		Name:       "Schedule example no-op",
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageReturn(map[string]interface{}{"triggered": true}, 200),
		},
	})
	if err != nil {
		return fmt.Errorf("save function: %w", err)
	}
	defer func() {
		if functionID != "" {
			if cleanupErr := client.DeleteFunction(functionID); cleanupErr != nil {
				runErr = errors.Join(runErr, fmt.Errorf("delete function %s: %w", functionID, cleanupErr))
			}
		}
	}()
	var scheduleID string
	defer func() {
		if scheduleID != "" {
			if cleanupErr := client.DeleteSchedule(scheduleID); cleanupErr != nil {
				runErr = errors.Join(runErr, fmt.Errorf("delete schedule %s: %w", scheduleID, cleanupErr))
			}
		}
	}()

	// 1. Create a schedule
	fmt.Println("--- Creating schedule ---")
	schedule, err := client.CreateSchedule(map[string]interface{}{
		"name":            "nightly-backup",
		"description":     "Runs a database backup every night at midnight",
		"function_label":  functionLabel,
		"cron_expression": "0 0 0 * * *",
		"enabled":         true,
	})
	if err != nil {
		return fmt.Errorf("create schedule: %w", err)
	}
	scheduleID = ekodb.GetStringValue(schedule["id"])
	if scheduleID == "" {
		return errors.New("create schedule response contained no ID")
	}
	fmt.Printf("Created schedule: %s (id: %s)\n", schedule["name"], scheduleID)

	// 2. List all schedules
	fmt.Println("\n--- Listing schedules ---")
	list, err := client.ListSchedules()
	if err != nil {
		return fmt.Errorf("list schedules: %w", err)
	}
	fmt.Printf("Schedules: %v\n", list)

	// 3. Get schedule by ID
	fmt.Println("\n--- Getting schedule ---")
	fetched, err := client.GetSchedule(scheduleID)
	if err != nil {
		return fmt.Errorf("get schedule: %w", err)
	}
	if ekodb.GetStringValue(fetched["name"]) != "nightly-backup" || ekodb.GetStringValue(fetched["cron_expression"]) != "0 0 0 * * *" {
		return fmt.Errorf("fetched schedule had unexpected name/cron: %v", fetched)
	}
	fmt.Printf("Fetched schedule: %s (cron: %s)\n", fetched["name"], fetched["cron_expression"])

	// 4. Update schedule
	fmt.Println("\n--- Updating schedule ---")
	updated, err := client.UpdateSchedule(scheduleID, map[string]interface{}{
		"cron_expression": "0 0 2 * * *",
		"description":     "Runs backup at 2 AM instead of midnight",
	})
	if err != nil {
		return fmt.Errorf("update schedule: %w", err)
	}
	if cron := ekodb.GetStringValue(updated["cron_expression"]); cron != "0 0 2 * * *" {
		return fmt.Errorf("updated schedule cron was %q", cron)
	}
	fmt.Printf("Updated cron: %v\n", updated["cron_expression"])

	// 5. Trigger immediately
	fmt.Println("\n--- Triggering schedule ---")
	triggered, err := client.TriggerSchedule(scheduleID)
	if err != nil {
		return fmt.Errorf("trigger schedule: %w", err)
	}
	if status := ekodb.GetStringValue(triggered["status"]); status != "triggered" {
		return fmt.Errorf("trigger schedule status was %q", status)
	}
	fmt.Printf("Trigger response: %v\n", triggered)

	// 6. Pause schedule
	fmt.Println("\n--- Pausing schedule ---")
	paused, err := client.PauseSchedule(scheduleID)
	if err != nil {
		return fmt.Errorf("pause schedule: %w", err)
	}
	if enabled, ok := paused["enabled"].(bool); !ok || enabled {
		return fmt.Errorf("paused schedule enabled value was %v", paused["enabled"])
	}
	fmt.Printf("Schedule enabled: %v\n", paused["enabled"])

	// 7. Resume schedule
	fmt.Println("\n--- Resuming schedule ---")
	resumed, err := client.ResumeSchedule(scheduleID)
	if err != nil {
		return fmt.Errorf("resume schedule: %w", err)
	}
	if enabled, ok := resumed["enabled"].(bool); !ok || !enabled {
		return fmt.Errorf("resumed schedule enabled value was %v", resumed["enabled"])
	}
	fmt.Printf("Schedule enabled: %v\n", resumed["enabled"])

	// 8. Delete schedule
	fmt.Println("\n--- Deleting schedule ---")
	if err := client.DeleteSchedule(scheduleID); err != nil {
		return fmt.Errorf("delete schedule: %w", err)
	}
	scheduleID = ""
	fmt.Println("Schedule deleted")
	if err := client.DeleteFunction(functionID); err != nil {
		return fmt.Errorf("delete function: %w", err)
	}
	functionID = ""

	fmt.Println("\n=== All schedule operations completed successfully ===")
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
