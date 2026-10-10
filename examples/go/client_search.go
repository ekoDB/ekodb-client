package main

import (
	"errors"
	"fmt"
	"log"
	"math/rand"
	"os"
	"time"

	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
)

func run() (runErr error) {
	// Load environment variables
	if err := godotenv.Load(); err != nil {
		log.Printf("Warning: .env file not found")
	}

	baseURL := os.Getenv("API_BASE_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}

	apiKey := os.Getenv("API_BASE_KEY")
	if apiKey == "" {
		apiKey = "a-test-api-key-from-ekodb"
	}

	// Create client
	client, err := ekodb.NewClient(baseURL, apiKey)
	if err != nil {
		return fmt.Errorf("create client: %w", err)
	}

	fmt.Print("=== Search Examples ===\n\n")

	// Use unique collection names
	usersCollection := "search_users_client_go"
	documentsCollection := "search_documents_client_go"
	pairedCollection := fmt.Sprintf("search_paired_client_go_%d", time.Now().UnixNano())
	usersOwned, documentsOwned := false, false
	pairedAttempted := false
	defer func() {
		fmt.Println("=== Cleanup ===")
		if usersOwned {
			runErr = errors.Join(runErr, client.DeleteCollection(usersCollection))
		}
		if documentsOwned {
			runErr = errors.Join(runErr, client.DeleteCollection(documentsCollection))
		}
		if pairedAttempted {
			// A lost create response can still leave a collection on the server.
			collections, err := client.ListCollections()
			if err != nil {
				runErr = errors.Join(runErr, fmt.Errorf("list collections for cleanup: %w", err))
			} else {
				for _, name := range collections {
					if name == pairedCollection {
						runErr = errors.Join(runErr, client.DeleteCollection(pairedCollection))
						break
					}
				}
			}
		}
		if runErr == nil {
			fmt.Println("✅ Deleted test collections")
		}
	}()

	// Cleanup any existing test collections
	_ = client.DeleteCollection(usersCollection)
	_ = client.DeleteCollection(documentsCollection)

	// Setup: Insert test data
	fmt.Println("Setting up test data...")

	// Insert users for searching
	users := []ekodb.Record{
		{
			"name":   "John Doe",
			"email":  "john@example.com",
			"bio":    "Software engineer specializing in JavaScript and TypeScript",
			"title":  "Senior Developer Engineer",
			"skills": []string{"JavaScript", "TypeScript", "React", "Node.js"},
		},
		{
			"name":   "Jane Smith",
			"email":  "jane@example.com",
			"bio":    "Product manager with experience in running agile teams",
			"title":  "Product Manager",
			"skills": []string{"Product Management", "Agile", "Scrum"},
		},
		{
			"name":   "Bob Johnson",
			"email":  "bob@example.com",
			"bio":    "DevOps engineer running cloud infrastructure",
			"title":  "DevOps Engineer",
			"skills": []string{"AWS", "Docker", "Kubernetes", "Terraform"},
		},
		{
			"name":   "Alice Williams",
			"email":  "alice@example.com",
			"bio":    "Senior software engineer with expertise in distributed systems",
			"title":  "Staff Engineer",
			"skills": []string{"Go", "Rust", "Distributed Systems", "Microservices"},
		},
		{
			"name":   "Charlie Davis",
			"email":  "charlie@example.com",
			"bio":    "Machine learning engineer working on AI models",
			"title":  "ML Engineer",
			"skills": []string{"Python", "TensorFlow", "PyTorch", "Machine Learning"},
		},
	}

	for _, user := range users {
		if _, err := client.Insert(usersCollection, user); err != nil {
			return fmt.Errorf("insert search user: %w", err)
		}
		usersOwned = true
	}

	// Insert documents with vector embeddings
	generateVector := func() []float64 {
		vec := make([]float64, 384)
		for i := range vec {
			vec[i] = rand.Float64()
		}
		return vec
	}

	// The "category" field is used to demonstrate metadata pre-filtering.
	documents := []ekodb.Record{
		{
			"title":    "Introduction to Machine Learning",
			"content":  "Machine learning is a subset of artificial intelligence...",
			"category": "ml",
			"embedding": map[string]interface{}{
				"type":  "Vector",
				"value": generateVector(),
			},
		},
		{
			"title":    "Deep Learning Fundamentals",
			"content":  "Deep learning uses neural networks with multiple layers...",
			"category": "ml",
			"embedding": map[string]interface{}{
				"type":  "Vector",
				"value": generateVector(),
			},
		},
		{
			"title":    "Natural Language Processing",
			"content":  "NLP enables computers to understand human language...",
			"category": "nlp",
			"embedding": map[string]interface{}{
				"type":  "Vector",
				"value": generateVector(),
			},
		},
	}

	for _, doc := range documents {
		if _, err := client.Insert(documentsCollection, doc); err != nil {
			return fmt.Errorf("insert search document: %w", err)
		}
		documentsOwned = true
	}

	fmt.Print("✅ Test data created\n\n")

	// Example 1: Basic full-text search
	fmt.Println("1. Basic full-text search:")
	fields1 := "name,email,bio"
	limit1 := 10
	search1 := ekodb.SearchQuery{
		Query:  "john",
		Fields: &fields1,
		Limit:  &limit1,
	}

	results1, err := client.Search(usersCollection, search1)
	if err != nil {
		return fmt.Errorf("basic full-text search: %w", err)
	} else {
		fmt.Printf("Found %d results\n", results1.Total)
		for i, result := range results1.Results {
			fmt.Printf("  %d. Score: %.3f\n", i+1, result.Score)
		}
	}
	fmt.Println()

	// Example 2: Fuzzy search with typo tolerance
	fmt.Println("2. Fuzzy search (typo tolerance):")
	fields2 := "title,bio"
	fuzzy := true
	maxEdit := 2
	search2 := ekodb.SearchQuery{
		Query:           "enginer", // Typo: should match "engineer"
		Fields:          &fields2,
		Fuzzy:           &fuzzy,
		MaxEditDistance: &maxEdit,
		Limit:           &limit1,
	}

	results2, err := client.Search(usersCollection, search2)
	if err != nil {
		return fmt.Errorf("fuzzy search: %w", err)
	} else {
		fmt.Printf("Found %d results with fuzzy matching\n", results2.Total)
		for i, result := range results2.Results {
			fmt.Printf("  %d. Score: %.3f\n", i+1, result.Score)
		}
	}
	fmt.Println()

	// Example 3: Search with field weights
	fmt.Println("3. Search with field weights:")
	fields3 := "title,bio"
	weights3 := "title:3.0,bio:1.0"
	search3 := ekodb.SearchQuery{
		Query:   "engineer",
		Fields:  &fields3,
		Weights: &weights3,
		Limit:   &limit1,
	}

	results3, err := client.Search(usersCollection, search3)
	if err != nil {
		return fmt.Errorf("weighted search: %w", err)
	} else {
		fmt.Printf("Found %d results with weighted fields\n", results3.Total)
		for i, result := range results3.Results {
			fmt.Printf("  %d. Score: %.3f\n", i+1, result.Score)
		}
	}
	fmt.Println()

	// Example 4: Search with minimum score threshold
	fmt.Println("4. Search with minimum score threshold:")
	minScore := 0.3
	search4 := ekodb.SearchQuery{
		Query:    "software",
		Fields:   &fields3,
		MinScore: &minScore,
		Limit:    &limit1,
	}

	results4, err := client.Search(usersCollection, search4)
	if err != nil {
		return fmt.Errorf("minimum-score search: %w", err)
	} else {
		fmt.Printf("Found %d results with score >= 0.3\n", results4.Total)
		for i, result := range results4.Results {
			fmt.Printf("  %d. Score: %.3f\n", i+1, result.Score)
		}
	}
	fmt.Println()

	// Example 5: Search with stemming
	fmt.Println("5. Search with stemming and exact match boosting:")
	bioField := "bio"
	stemming := true
	boostExact := true
	language := "english"
	search5 := ekodb.SearchQuery{
		Query:          "work",
		Fields:         &bioField,
		EnableStemming: &stemming,
		BoostExact:     &boostExact,
		Language:       &language,
		Limit:          &limit1,
	}

	results5, err := client.Search(usersCollection, search5)
	if err != nil {
		return fmt.Errorf("stemming search: %w", err)
	} else {
		fmt.Printf("Found %d results (matches: work, working, worked)\n", results5.Total)
		for i, result := range results5.Results {
			fmt.Printf("  %d. Score: %.3f\n", i+1, result.Score)
		}
	}
	fmt.Println()

	// Example 6: Vector search (semantic search)
	fmt.Println("6. Vector search (semantic search):")
	queryVector := generateVector()
	vectorField := "embedding"
	vectorMetric := "cosine"
	vectorK := 10
	vectorThreshold := 0.0
	search6 := ekodb.SearchQuery{
		Query:           "",
		Vector:          queryVector,
		VectorField:     &vectorField,
		VectorMetric:    &vectorMetric,
		VectorK:         &vectorK,
		VectorThreshold: &vectorThreshold,
	}

	results6, err := client.Search(documentsCollection, search6)
	if err != nil {
		return fmt.Errorf("vector search: %w", err)
	} else {
		fmt.Printf("Found %d semantically similar documents\n", results6.Total)
		for i, result := range results6.Results {
			fmt.Printf("  %d. Score: %.3f\n", i+1, result.Score)
		}
	}
	fmt.Println()

	// Example 7: Hybrid search (text + vector)
	fmt.Println("7. Hybrid search (text + vector):")
	contentFields := "title,content"
	textWeight := 0.6
	vectorWeight := 0.4
	limit7 := 3
	search7 := ekodb.SearchQuery{
		Query:        "machine learning",
		Fields:       &contentFields,
		Vector:       queryVector,
		VectorField:  &vectorField,
		TextWeight:   &textWeight,
		VectorWeight: &vectorWeight,
		Limit:        &limit7,
	}

	results7, err := client.Search(documentsCollection, search7)
	if err != nil {
		return fmt.Errorf("hybrid search: %w", err)
	} else {
		fmt.Printf("Found %d results using hybrid search (text + vector)\n", results7.Total)
		for i, result := range results7.Results {
			fmt.Printf("  %d. Score: %.3f\n", i+1, result.Score)
		}
	}
	fmt.Println()

	// Example 8: Case-sensitive search
	fmt.Println("8. Case-sensitive search:")
	skillsField := "skills,bio"
	caseSensitive := true
	search8 := ekodb.SearchQuery{
		Query:         "JavaScript",
		Fields:        &skillsField,
		CaseSensitive: &caseSensitive,
		Limit:         &limit1,
	}

	results8, err := client.Search(usersCollection, search8)
	if err != nil {
		return fmt.Errorf("case-sensitive search: %w", err)
	} else {
		fmt.Printf("Found %d results (case-sensitive)\n", results8.Total)
		for i, result := range results8.Results {
			fmt.Printf("  %d. Score: %.3f\n", i+1, result.Score)
		}
	}
	fmt.Println()

	// Example 9: Vector search with a metadata pre-filter
	fmt.Println("9. Vector search with a metadata pre-filter (category = ml):")
	mlFilter := map[string]interface{}{
		"type": "Condition",
		"content": map[string]interface{}{
			"field":    "category",
			"operator": "Eq",
			"value":    "ml",
		},
	}
	search9 := ekodb.SearchQuery{
		Query:           "",
		Vector:          queryVector,
		VectorField:     &vectorField,
		VectorMetric:    &vectorMetric,
		VectorK:         &vectorK,
		VectorThreshold: &vectorThreshold,
		Filters:         mlFilter, // only "ml" documents are candidates
	}

	results9, err := client.Search(documentsCollection, search9)
	if err != nil {
		return fmt.Errorf("filtered vector search: %w", err)
	} else {
		fmt.Printf("Found %d documents in category \"ml\" (NLP excluded)\n", results9.Total)
		for i, result := range results9.Results {
			title := ekodb.GetStringValue(result.Record["title"])
			category := ekodb.GetStringValue(result.Record["category"])
			fmt.Printf("  %d. %s (category: %s)\n", i+1, title, category)
		}
	}
	fmt.Println()

	// Orthogonal toy vectors show field selection. Real query/document vectors
	// need compatible dimensions and a shared dual-encoder model space.
	pairedSchema := ekodb.NewSchemaBuilder().
		AddField("title", ekodb.NewFieldTypeSchemaBuilder("String").Required().Build()).
		AddField("category", ekodb.NewFieldTypeSchemaBuilder("String").Required().Build()).
		AddField("query_embedding", ekodb.NewFieldTypeSchemaBuilder("Vector").Required().
			VectorIndex(ekodb.VectorIndexFlat, ekodb.DistanceMetricCosine, 16, 200).Dimension(3).Build()).
		AddField("document_embedding", ekodb.NewFieldTypeSchemaBuilder("Vector").Required().
			VectorIndex(ekodb.VectorIndexFlat, ekodb.DistanceMetricCosine, 16, 200).Dimension(3).Build()).
		Build()
	pairedAttempted = true
	if err := client.CreateCollection(pairedCollection, pairedSchema); err != nil {
		return fmt.Errorf("create paired collection: %w", err)
	}
	pairedDocuments := []struct {
		title    string
		category string
		query    []float64
		document []float64
	}{
		{"Rust Programming Question", "programming", []float64{1, 0, 0}, []float64{0, 1, 0}},
		{"Rust Programming Answer", "programming", []float64{0, 1, 0}, []float64{1, 0, 0}},
		{"Database Design Answer", "database", []float64{0, 0, 1}, []float64{0, 0, 1}},
	}
	for _, item := range pairedDocuments {
		record := ekodb.Record{
			"title":              item.title,
			"category":           item.category,
			"query_embedding":    map[string]interface{}{"type": "Vector", "value": item.query},
			"document_embedding": map[string]interface{}{"type": "Vector", "value": item.document},
		}
		if _, err := client.Insert(pairedCollection, record); err != nil {
			return fmt.Errorf("insert paired document: %w", err)
		}
	}

	sourceQuery := pairedDocuments[0].query
	documentField := "document_embedding"
	queryField := "query_embedding"
	pairedK, pairedLimit, bypassCache := 3, 3, true
	documentMatches, err := client.Search(pairedCollection, ekodb.SearchQuery{
		Query: "", Vector: sourceQuery, VectorField: &documentField,
		VectorMetric: &vectorMetric, VectorK: &pairedK, Limit: &pairedLimit,
		BypassCache: &bypassCache,
	})
	if err != nil {
		return fmt.Errorf("paired document search: %w", err)
	}
	queryMatches, err := client.Search(pairedCollection, ekodb.SearchQuery{
		Query: "", Vector: sourceQuery, VectorField: &queryField,
		VectorMetric: &vectorMetric, VectorK: &pairedK, Limit: &pairedLimit,
		BypassCache: &bypassCache,
	})
	if err != nil {
		return fmt.Errorf("paired query search: %w", err)
	}
	documentTop, queryTop := "", ""
	if len(documentMatches.Results) > 0 {
		documentTop = ekodb.GetStringValue(documentMatches.Results[0].Record["title"])
	}
	if len(queryMatches.Results) > 0 {
		queryTop = ekodb.GetStringValue(queryMatches.Results[0].Record["title"])
	}
	if documentTop != "Rust Programming Answer" || queryTop != "Rust Programming Question" {
		return fmt.Errorf("unexpected paired-vector tops: document=%q, query=%q", documentTop, queryTop)
	}
	fmt.Printf("Paired vector search: document=%s, query=%s\n", documentTop, queryTop)
	if documentMatches.ExecutionPath != nil {
		fmt.Printf("Document search execution path: %s\n", *documentMatches.ExecutionPath)
	}
	if queryMatches.ExecutionPath != nil {
		fmt.Printf("Query search execution path: %s\n", *queryMatches.ExecutionPath)
	}

	fmt.Println("\n✅ Search examples completed!")
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
