// Read-only by default; EKODB_ADMIN_LIVE=1 creates and revokes one disposable key.
package main

import (
	"crypto/rand"
	"fmt"
	ekodb "github.com/ekoDB/ekodb-client-go"
	"github.com/joho/godotenv"
	"os"
	"reflect"
)

func main() {
	if err := runAdmin(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
func runAdmin() (result error) {
	_ = godotenv.Load()
	base, owner := os.Getenv("API_BASE_URL"), os.Getenv("API_BASE_KEY")
	if base == "" || owner == "" {
		return fmt.Errorf("API_BASE_URL and API_BASE_KEY are required")
	}
	admin, err := ekodb.NewAdminClient(base, owner)
	if err != nil {
		return err
	}
	current, err := admin.GetPermissions()
	if err != nil {
		return err
	}
	fmt.Println("Admin permissions read successfully")
	if os.Getenv("EKODB_ADMIN_LIVE") != "1" {
		return nil
	}
	if !current.IsAdmin {
		return fmt.Errorf("an admin key is required")
	}
	var nonce [16]byte
	if _, err = rand.Read(nonce[:]); err != nil {
		return err
	}
	label := fmt.Sprintf("sdk-go-admin-example-%x", nonce)
	fmt.Printf("Disposable cleanup label: %s\n", label)
	// Registered before creation, including ambiguous creation failures.
	defer func() {
		keys, cleanupErr := admin.ListAPIKeys()
		if cleanupErr == nil {
			matches := []ekodb.APIKey{}
			for _, key := range keys {
				if key.Label != nil && *key.Label == label {
					matches = append(matches, key)
				}
			}
			if len(matches) > 1 || (len(matches) == 1 && matches[0].Key == owner) {
				cleanupErr = fmt.Errorf("unexpected disposable cleanup match")
			}
			if cleanupErr == nil {
				for _, key := range matches {
					if cleanupErr = admin.RevokeAPIKey(key.Key); cleanupErr != nil {
						break
					}
				}
			}
			if cleanupErr == nil {
				keys, cleanupErr = admin.ListAPIKeys()
				for _, key := range keys {
					if key.Label != nil && *key.Label == label {
						cleanupErr = fmt.Errorf("matching key remains")
					}
				}
			}
		}
		if cleanupErr != nil {
			result = fmt.Errorf("disposable cleanup failed for label %s", label)
		}
	}()
	key, err := admin.CreateAPIKey(ekodb.CreateAPIKeyRequest{Label: label, CollectionPermissions: map[string][]ekodb.APIKeyAction{"sdk_admin_probe": {ekodb.APIKeyRead}}})
	if err != nil {
		return err
	}
	if key.Key == owner || key.IsAdmin {
		return fmt.Errorf("unexpected disposable key")
	}
	scoped, err := ekodb.NewAdminClient(base, key.Key)
	if err != nil {
		return err
	}
	permissions, err := scoped.GetPermissions()
	if err != nil {
		return err
	}
	if permissions.IsAdmin {
		return fmt.Errorf("unexpected admin grant")
	}
	selected, excluded, writable := []string{"public"}, []string{"secret"}, []string{"public"}
	request := ekodb.UpdateAPIKeyCollectionPermissionsRequest{APIKey: key.Key, Collection: "sdk_admin_probe", Read: true, Write: true, SelectFields: &selected, ExcludeFields: &excluded, WritableFields: &writable}
	if _, err = admin.UpdateAPIKeyCollectionPermissions(request); err != nil {
		return err
	}
	firstClient, err := ekodb.NewAdminClient(base, key.Key)
	if err != nil {
		return err
	}
	first, err := firstClient.GetPermissions()
	if err != nil {
		return err
	}
	initial := first.CollectionPermissions["sdk_admin_probe"]
	if !initial.Read || !initial.Write || !reflect.DeepEqual(initial.SelectFields, selected) || !reflect.DeepEqual(initial.ExcludeFields, excluded) || !reflect.DeepEqual(initial.WritableFields, writable) {
		return fmt.Errorf("initial restriction readback failed")
	}
	var clear []string
	empty := []string{}
	request.Write = false
	request.SelectFields = nil
	request.ExcludeFields = &clear
	request.WritableFields = &empty
	if _, err = admin.UpdateAPIKeyCollectionPermissions(request); err != nil {
		return err
	}
	fresh, err := ekodb.NewAdminClient(base, key.Key)
	if err != nil {
		return err
	}
	permissions, err = fresh.GetPermissions()
	if err != nil {
		return err
	}
	grant := permissions.CollectionPermissions["sdk_admin_probe"]
	if !grant.Read || grant.Write || len(grant.SelectFields) != 1 || grant.SelectFields[0] != "public" || grant.ExcludeFields != nil || grant.WritableFields == nil || len(grant.WritableFields) != 0 {
		return fmt.Errorf("restriction readback failed")
	}
	fmt.Println("Disposable admin lifecycle verified; cleanup runs before exit")
	return nil
}
