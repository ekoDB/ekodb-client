// Path-routed function — expose a stored function under a REST URL.
//
// Sets HTTPMethod + HTTPPath on a UserFunction so ekoDB's path-routed
// dispatcher (/api/route/{path}) routes incoming HTTP requests to it.
// Path placeholders (:id) are extracted into the function's params map.
//
// Routing rules:
//   - method match required (GET/POST/PUT/PATCH/DELETE all dispatch)
//   - segment count must match the pattern
//   - lowest "specificity score" (fewest :placeholder) wins on ties
//   - body params merge OVER path params on collision
//
// Requires ekoDB >= 0.43.0.

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

	getMethod := "GET"
	postMethod := "POST"
	adminPath := "/users/admin"
	idPath := "/users/:id"
	postsPath := "/users/:id/posts/:post_id"
	orgPath := "/orgs/:org/members"

	// Most-specific literal — `/users/admin` answers admin lookups
	// separately from the `:id` route.
	admin := ekodb.UserFunction{
		Label:      "go_route_admin",
		Name:       "Get admin user",
		HTTPMethod: &getMethod,
		HTTPPath:   &adminPath,
		Parameters: map[string]ekodb.ParameterDefinition{},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageReturn(map[string]interface{}{"role": "admin"}, 200),
		},
	}
	saveFn(client, admin)

	// Single-placeholder route.
	byId := ekodb.UserFunction{
		Label:      "go_route_user_by_id",
		Name:       "Get user by id",
		HTTPMethod: &getMethod,
		HTTPPath:   &idPath,
		Parameters: map[string]ekodb.ParameterDefinition{
			"id": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageReturn(map[string]interface{}{"requested_id": "{{id}}"}, 200),
		},
	}
	saveFn(client, byId)

	// Two-placeholder nested route.
	posts := ekodb.UserFunction{
		Label:      "go_route_user_posts",
		Name:       "Get user's post",
		HTTPMethod: &getMethod,
		HTTPPath:   &postsPath,
		Parameters: map[string]ekodb.ParameterDefinition{
			"id":      {Required: true},
			"post_id": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageReturn(map[string]interface{}{
				"user_id": "{{id}}",
				"post_id": "{{post_id}}",
			}, 200),
		},
	}
	saveFn(client, posts)

	// POST with body merge: path provides `:org`, body provides `name`.
	create := ekodb.UserFunction{
		Label:      "go_route_org_create_member",
		Name:       "Add member to org",
		HTTPMethod: &postMethod,
		HTTPPath:   &orgPath,
		Parameters: map[string]ekodb.ParameterDefinition{
			"org":  {Required: true},
			"name": {Required: true},
		},
		Functions: []ekodb.FunctionStageConfig{
			ekodb.StageReturn(map[string]interface{}{
				"org":     "{{org}}",
				"name":    "{{name}}",
				"created": true,
			}, 201),
		},
	}
	saveFn(client, create)

	fmt.Println("\nTry them with curl:")
	fmt.Println("  curl http://localhost:8080/api/route/users/admin")
	fmt.Println("  curl http://localhost:8080/api/route/users/42")
	fmt.Println("  curl http://localhost:8080/api/route/users/42/posts/7")
	fmt.Println(`  curl -X POST http://localhost:8080/api/route/orgs/acme/members \`)
	fmt.Println(`       -H 'Content-Type: application/json' -d '{"name":"alice"}'`)

	for _, label := range []string{
		"go_route_admin", "go_route_user_by_id",
		"go_route_user_posts", "go_route_org_create_member",
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
