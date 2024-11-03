// errors_test.go
package ekodb

import (
	"errors"
	"testing"
)

func TestAPIError(t *testing.T) {
	tests := []struct {
		name       string
		err        *APIError
		wantString string
	}{
		{
			name: "error with status code",
			err: &APIError{
				Message:    "Invalid request",
				StatusCode: 400,
			},
			wantString: "Invalid request (Status: 400)",
		},
		{
			name: "error without status code",
			err: &APIError{
				Message: "Operation failed",
			},
			wantString: "Operation failed",
		},
		{
			name: "error with response data",
			err: &APIError{
				Message:    "Validation failed",
				StatusCode: 422,
				Response:   map[string]interface{}{"field": "name", "error": "required"},
			},
			wantString: "Validation failed (Status: 422)",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.err.Error() != tt.wantString {
				t.Errorf("APIError.Error() = %v, want %v", tt.err.Error(), tt.wantString)
			}
		})
	}
}

func TestErrorComparison(t *testing.T) {
	t.Run("ErrNotFound comparison", func(t *testing.T) {
		err := ErrNotFound
		if !errors.Is(err, ErrNotFound) {
			t.Error("Expected error to be ErrNotFound")
		}
	})

	t.Run("APIError is not ErrNotFound", func(t *testing.T) {
		err := &APIError{
			Message:    "Some other error",
			StatusCode: 500,
		}
		if errors.Is(err, ErrNotFound) {
			t.Error("APIError should not be ErrNotFound")
		}
	})
}

func TestAPIErrorWrapping(t *testing.T) {
	originalErr := errors.New("original error")
	wrappedErr := &APIError{
		Message:    "Wrapped error",
		StatusCode: 500,
		Response:   originalErr.Error(),
	}

	t.Run("error message formatting", func(t *testing.T) {
		expected := "Wrapped error (Status: 500)"
		if wrappedErr.Error() != expected {
			t.Errorf("APIError.Error() = %v, want %v", wrappedErr.Error(), expected)
		}
	})

	t.Run("response preservation", func(t *testing.T) {
		if wrappedErr.Response.(string) != originalErr.Error() {
			t.Errorf("APIError.Response = %v, want %v", wrappedErr.Response, originalErr.Error())
		}
	})
}

func TestErrorTypes(t *testing.T) {
	t.Run("not found error", func(t *testing.T) {
		var err error = ErrNotFound
		if err.Error() != "record not found" {
			t.Errorf("ErrNotFound.Error() = %v, want 'record not found'", err.Error())
		}
	})

	t.Run("api error implementation", func(t *testing.T) {
		var err error = &APIError{
			Message:    "Test error",
			StatusCode: 400,
		}
		if _, ok := err.(error); !ok {
			t.Error("APIError should implement error interface")
		}
	})
}

func TestAPIErrorNilHandling(t *testing.T) {
	t.Run("nil response handling", func(t *testing.T) {
		err := &APIError{
			Message:    "Error with nil response",
			StatusCode: 500,
			Response:   nil,
		}
		if err.Error() != "Error with nil response (Status: 500)" {
			t.Errorf("APIError.Error() with nil response = %v, want 'Error with nil response (Status: 500)'", err.Error())
		}
	})

	t.Run("zero value handling", func(t *testing.T) {
		var err APIError
		if err.Error() != "" {
			t.Errorf("Zero value APIError.Error() = %v, want ''", err.Error())
		}
	})
}

func TestAPIErrorStatusCodeHandling(t *testing.T) {
	tests := []struct {
		name           string
		statusCode     int
		expectedInMsg  bool
		expectedFormat string
	}{
		{
			name:           "positive status code",
			statusCode:     404,
			expectedInMsg:  true,
			expectedFormat: "Test message (Status: 404)",
		},
		{
			name:           "zero status code",
			statusCode:     0,
			expectedInMsg:  false,
			expectedFormat: "Test message",
		},
		{
			name:           "negative status code",
			statusCode:     -1,
			expectedInMsg:  true,
			expectedFormat: "Test message (Status: -1)",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := &APIError{
				Message:    "Test message",
				StatusCode: tt.statusCode,
			}
			result := err.Error()
			if result != tt.expectedFormat {
				t.Errorf("APIError.Error() = %v, want %v", result, tt.expectedFormat)
			}
		})
	}
}
