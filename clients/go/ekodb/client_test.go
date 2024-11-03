// client_test.go
package ekodb

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

func TestNewClient(t *testing.T) {
	tests := []struct {
		name   string
		config *ClientConfig
		want   string
	}{
		{
			name:   "default configuration",
			config: &ClientConfig{APIKey: "test-key"},
			want:   "http://localhost:8080",
		},
		{
			name: "custom configuration",
			config: &ClientConfig{
				BaseURL: "http://custom.api",
				APIKey:  "test-key",
			},
			want: "http://custom.api",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			client := NewClient(tt.config)
			if client.baseURL != tt.want {
				t.Errorf("NewClient() baseURL = %v, want %v", client.baseURL, tt.want)
			}
			if client.apiKey != tt.config.APIKey {
				t.Errorf("NewClient() apiKey = %v, want %v", client.apiKey, tt.config.APIKey)
			}
		})
	}
}

func TestAuthentication(t *testing.T) {
	tests := []struct {
		name       string
		apiKey     string
		statusCode int
		response   string
		wantErr    bool
	}{
		{
			name:       "successful authentication",
			apiKey:     "valid-key",
			statusCode: http.StatusOK,
			response:   `{"token": "test-token"}`,
			wantErr:    false,
		},
		{
			name:       "invalid api key",
			apiKey:     "invalid-key",
			statusCode: http.StatusUnauthorized,
			response:   `{"message": "Invalid API key"}`,
			wantErr:    true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				if r.Method != http.MethodPost {
					t.Errorf("Expected POST request, got %s", r.Method)
				}
				if r.URL.Path != "/api/auth/token" {
					t.Errorf("Expected /api/auth/token path, got %s", r.URL.Path)
				}
				w.WriteHeader(tt.statusCode)
				w.Write([]byte(tt.response))
			}))
			defer server.Close()

			client := NewClient(&ClientConfig{
				BaseURL: server.URL,
				APIKey:  tt.apiKey,
			})

			err := client.authenticate(context.Background())
			if (err != nil) != tt.wantErr {
				t.Errorf("authenticate() error = %v, wantErr %v", err, tt.wantErr)
			}
			if !tt.wantErr && client.token != "test-token" {
				t.Errorf("authenticate() token = %v, want %v", client.token, "test-token")
			}
		})
	}
}

func TestCRUDOperations(t *testing.T) {
	ctx := context.Background()
	record := Record{
		"name": TypedValue{Type: StringType, Value: "John Doe"},
		"age":  TypedValue{Type: IntegerType, Value: 30},
	}

	t.Run("Insert", func(t *testing.T) {
		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			if r.Method == http.MethodPost && r.URL.Path == "/api/auth/token" {
				json.NewEncoder(w).Encode(map[string]string{"token": "test-token"})
				return
			}
			if r.Method != http.MethodPost {
				t.Errorf("Expected POST request, got %s", r.Method)
			}
			if r.URL.Path != "/api/insert/users" {
				t.Errorf("Expected /api/insert/users path, got %s", r.URL.Path)
			}
			json.NewEncoder(w).Encode(map[string]string{"id": "test-id"})
		}))
		defer server.Close()

		client := NewClient(&ClientConfig{
			BaseURL: server.URL,
			APIKey:  "test-key",
		})

		id, err := client.Insert(ctx, "users", record)
		if err != nil {
			t.Errorf("Insert() error = %v", err)
		}
		if id != "test-id" {
			t.Errorf("Insert() = %v, want %v", id, "test-id")
		}
	})

	t.Run("Find", func(t *testing.T) {
		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			if r.Method == http.MethodPost && r.URL.Path == "/api/auth/token" {
				json.NewEncoder(w).Encode(map[string]string{"token": "test-token"})
				return
			}
			if r.Method != http.MethodGet {
				t.Errorf("Expected GET request, got %s", r.Method)
			}
			json.NewEncoder(w).Encode([]Record{record})
		}))
		defer server.Close()

		client := NewClient(&ClientConfig{
			BaseURL: server.URL,
			APIKey:  "test-key",
		})

		records, err := client.Find(ctx, "users", nil)
		if err != nil {
			t.Errorf("Find() error = %v", err)
		}
		if len(records) != 1 {
			t.Errorf("Find() = %v records, want 1", len(records))
		}
	})

	t.Run("FindByID", func(t *testing.T) {
		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			if r.Method == http.MethodPost && r.URL.Path == "/api/auth/token" {
				json.NewEncoder(w).Encode(map[string]string{"token": "test-token"})
				return
			}
			if r.Method != http.MethodGet {
				t.Errorf("Expected GET request, got %s", r.Method)
			}
			if r.URL.Path != "/api/find/users/test-id" {
				t.Errorf("Expected /api/find/users/test-id path, got %s", r.URL.Path)
			}
			json.NewEncoder(w).Encode(record)
		}))
		defer server.Close()

		client := NewClient(&ClientConfig{
			BaseURL: server.URL,
			APIKey:  "test-key",
		})

		found, err := client.FindByID(ctx, "users", "test-id")
		if err != nil {
			t.Errorf("FindByID() error = %v", err)
		}
		if found == nil {
			t.Error("FindByID() = nil, want record")
		}
	})
}

func TestContextCancellation(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		time.Sleep(100 * time.Millisecond)
		json.NewEncoder(w).Encode(map[string]string{"token": "test-token"})
	}))
	defer server.Close()

	client := NewClient(&ClientConfig{
		BaseURL: server.URL,
		APIKey:  "test-key",
	})

	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel()

	_, err := client.Find(ctx, "users", nil)
	if err == nil {
		t.Error("Expected context deadline exceeded error, got nil")
	}
}
