// client.go
package ekodb

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
)

// Client represents an EkoDB API client
type Client struct {
	baseURL    string
	apiKey     string
	httpClient *http.Client
	token      string
}

// ClientConfig holds configuration for creating a new client
type ClientConfig struct {
	BaseURL    string
	APIKey     string
	HTTPClient *http.Client
}

// NewClient creates a new EkoDB client
func NewClient(cfg *ClientConfig) *Client {
	if cfg.HTTPClient == nil {
		cfg.HTTPClient = http.DefaultClient
	}
	if cfg.BaseURL == "" {
		cfg.BaseURL = "http://localhost:8080"
	}
	return &Client{
		baseURL:    cfg.BaseURL,
		apiKey:     cfg.APIKey,
		httpClient: cfg.HTTPClient,
	}
}

// authenticate performs API authentication and stores the token
func (c *Client) authenticate(ctx context.Context) error {
	payload := map[string]string{"api_key": c.apiKey}
	resp := struct {
		Token string `json:"token"`
	}{}

	err := c.doRequest(ctx, http.MethodPost, "/api/auth/token", payload, &resp)
	if err != nil {
		return fmt.Errorf("authentication failed: %w", err)
	}

	c.token = resp.Token
	return nil
}

// Insert adds a new record to the specified collection
func (c *Client) Insert(ctx context.Context, collection string, record Record) (string, error) {
	resp := struct {
		ID string `json:"id"`
	}{}

	err := c.doRequest(ctx, http.MethodPost, fmt.Sprintf("/api/insert/%s", collection), record, &resp)
	if err != nil {
		return "", fmt.Errorf("insert failed: %w", err)
	}

	return resp.ID, nil
}

// BatchInsert adds multiple records to the specified collection
func (c *Client) BatchInsert(ctx context.Context, collection string, records []Record) (*BatchInsertResponse, error) {
	payload := struct {
		Records []Record `json:"records"`
	}{
		Records: records,
	}

	var resp BatchInsertResponse
	err := c.doRequest(ctx, http.MethodPost, fmt.Sprintf("/api/batch/insert/%s", collection), payload, &resp)
	if err != nil {
		return nil, fmt.Errorf("batch insert failed: %w", err)
	}

	return &resp, nil
}

// Find retrieves records from the specified collection
func (c *Client) Find(ctx context.Context, collection string, opts *FindOptions) ([]Record, error) {
	params := url.Values{}
	if opts != nil {
		if opts.Filter != nil {
			filterJSON, err := json.Marshal(opts.Filter)
			if err != nil {
				return nil, fmt.Errorf("invalid filter: %w", err)
			}
			params.Set("filter", string(filterJSON))
		}
		if opts.Sort != nil {
			sortJSON, err := json.Marshal(opts.Sort)
			if err != nil {
				return nil, fmt.Errorf("invalid sort: %w", err)
			}
			params.Set("sort", string(sortJSON))
		}
		if opts.Limit > 0 {
			params.Set("limit", fmt.Sprintf("%d", opts.Limit))
		}
		if opts.Skip > 0 {
			params.Set("skip", fmt.Sprintf("%d", opts.Skip))
		}
	}

	var records []Record
	err := c.doRequest(ctx, http.MethodGet, fmt.Sprintf("/api/find/%s?%s", collection, params.Encode()), nil, &records)
	if err != nil {
		return nil, fmt.Errorf("find failed: %w", err)
	}

	return records, nil
}

// FindByID retrieves a single record by ID
func (c *Client) FindByID(ctx context.Context, collection, id string) (*Record, error) {
	var record Record
	err := c.doRequest(ctx, http.MethodGet, fmt.Sprintf("/api/find/%s/%s", collection, id), nil, &record)
	if err != nil {
		if err == ErrNotFound {
			return nil, nil
		}
		return nil, fmt.Errorf("find by ID failed: %w", err)
	}

	return &record, nil
}

// Update modifies an existing record
func (c *Client) Update(ctx context.Context, collection, id string, updates Record) (bool, error) {
	resp := struct {
		Success bool `json:"success"`
	}{}

	err := c.doRequest(ctx, http.MethodPut, fmt.Sprintf("/api/update/%s/%s", collection, id), updates, &resp)
	if err != nil {
		return false, fmt.Errorf("update failed: %w", err)
	}

	return resp.Success, nil
}

// BatchUpdate modifies multiple records
func (c *Client) BatchUpdate(ctx context.Context, collection string, updates map[string]Record) (*BatchUpdateResponse, error) {
	payload := struct {
		Updates map[string]Record `json:"updates"`
	}{
		Updates: updates,
	}

	var resp BatchUpdateResponse
	err := c.doRequest(ctx, http.MethodPut, fmt.Sprintf("/api/batch/update/%s", collection), payload, &resp)
	if err != nil {
		return nil, fmt.Errorf("batch update failed: %w", err)
	}

	return &resp, nil
}

// Delete removes a record
func (c *Client) Delete(ctx context.Context, collection, id string) (bool, error) {
	resp := struct {
		Success bool `json:"success"`
	}{}

	err := c.doRequest(ctx, http.MethodDelete, fmt.Sprintf("/api/delete/%s/%s", collection, id), nil, &resp)
	if err != nil {
		return false, fmt.Errorf("delete failed: %w", err)
	}

	return resp.Success, nil
}

// BatchDelete removes multiple records
func (c *Client) BatchDelete(ctx context.Context, collection string, ids []string) (*BatchDeleteResponse, error) {
	payload := struct {
		IDs []string `json:"ids"`
	}{
		IDs: ids,
	}

	var resp BatchDeleteResponse
	err := c.doRequest(ctx, http.MethodDelete, fmt.Sprintf("/api/batch/delete/%s", collection), payload, &resp)
	if err != nil {
		return nil, fmt.Errorf("batch delete failed: %w", err)
	}

	return &resp, nil
}

// GetSchema retrieves the schema for a collection
func (c *Client) GetSchema(ctx context.Context, collection string) (*CollectionSchema, error) {
	var schema CollectionSchema
	err := c.doRequest(ctx, http.MethodGet, fmt.Sprintf("/api/schemas/%s", collection), nil, &schema)
	if err != nil {
		return nil, fmt.Errorf("get schema failed: %w", err)
	}

	return &schema, nil
}

// doRequest performs an HTTP request with authentication
func (c *Client) doRequest(ctx context.Context, method, path string, body interface{}, result interface{}) error {
	if c.token == "" && path != "/api/auth/token" {
		if err := c.authenticate(ctx); err != nil {
			return err
		}
	}

	var reqBody []byte
	var err error
	if body != nil {
		reqBody, err = json.Marshal(body)
		if err != nil {
			return fmt.Errorf("failed to marshal request body: %w", err)
		}
	}

	req, err := http.NewRequestWithContext(ctx, method, c.baseURL+path, bytes.NewReader(reqBody))
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")
	if c.token != "" {
		req.Header.Set("Authorization", "Bearer "+c.token)
	}

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return fmt.Errorf("request failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 400 {
		var apiError APIError
		if err := json.NewDecoder(resp.Body).Decode(&apiError); err != nil {
			return fmt.Errorf("failed to decode error response: %w", err)
		}
		if resp.StatusCode == http.StatusNotFound {
			return ErrNotFound
		}
		return &apiError
	}

	if result != nil {
		if err := json.NewDecoder(resp.Body).Decode(result); err != nil {
			return fmt.Errorf("failed to decode response: %w", err)
		}
	}

	return nil
}
