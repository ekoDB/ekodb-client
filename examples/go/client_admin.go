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
					if cleanupErr = admin.RevokeAPIKey(func() string {
						if key.ID != "" {
							return key.ID
						}
						return key.Key
					}()); cleanupErr != nil {
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
	modern, err := admin.UsesKeyPermissionRoutes()
	if err != nil {
		return err
	}
	if modern {
		if key.ID == "" {
			return fmt.Errorf("registration omitted key id")
		}
		listed, err := admin.ListAPIKeys()
		if err != nil {
			return err
		}
		found := false
		for _, listedKey := range listed {
			if listedKey.ID == key.ID && listedKey.Label != nil && *listedKey.Label == label {
				found = true
			}
		}
		if !found {
			return fmt.Errorf("key listing omitted the registered id")
		}
		prefix := "sdk-admin:" + label + ":"
		change, err := admin.PutAPIKeyKVPermissions(ekodb.UpdateAPIKeyKVPermissionsRequest{KeyID: key.ID, Prefix: prefix, Read: true, Write: false})
		if err != nil {
			return err
		}
		if change.After == nil || !change.After.Read || change.After.Write {
			return fmt.Errorf("KV grant creation failed")
		}
		write := true
		if _, err = admin.PatchAPIKeyKVPermissions(ekodb.PatchAPIKeyKVPermissionsRequest{KeyID: key.ID, Prefix: prefix, Write: &write}); err != nil {
			return err
		}
		change, err = admin.DeleteAPIKeyKVPermissions(key.ID, prefix)
		if err != nil {
			return err
		}
		if change.After != nil {
			return fmt.Errorf("KV grant removal failed")
		}
	}
	edit := func(request ekodb.LegacyUpdateAPIKeyCollectionPermissionsRequest, replace bool) error {
		if modern {
			input := ekodb.UpdateAPIKeyCollectionPermissionsRequest{KeyID: key.ID, Collection: request.Collection, Read: request.Read, Write: request.Write, SelectFields: request.SelectFields, ExcludeFields: request.ExcludeFields, WritableFields: request.WritableFields}
			if replace {
				_, err := admin.PutAPIKeyCollectionPermissions(input)
				return err
			}
			_, err := admin.UpdateAPIKeyCollectionPermissions(input)
			return err
		}
		_, err := admin.LegacyUpdateAPIKeyCollectionPermissions(request)
		return err
	}
	selected, excluded, writable := []string{"public"}, []string{"secret"}, []string{"public"}
	request := ekodb.LegacyUpdateAPIKeyCollectionPermissionsRequest{APIKey: key.Key, Collection: "sdk_admin_probe", Read: true, Write: true, SelectFields: &selected, ExcludeFields: &excluded, WritableFields: &writable}
	if err = edit(request, true); err != nil {
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
	if err = edit(request, false); err != nil {
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
