// errors.go
package ekodb

import (
	"errors"
	"fmt"
)

// Common errors
var (
	ErrNotFound = errors.New("record not found")
)

type APIError struct {
	Message    string
	StatusCode int
	Response   interface{}
}

func (e *APIError) Error() string {
	// Always include status code in message if it's non-zero or negative
	if e.StatusCode != 0 {
		return fmt.Sprintf("%s (Status: %d)", e.Message, e.StatusCode)
	}
	return e.Message
}
