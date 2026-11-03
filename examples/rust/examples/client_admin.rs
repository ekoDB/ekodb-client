//! Read-only by default; EKODB_ADMIN_LIVE=1 creates and revokes one disposable key.
use ekodb_client::admin::{
    AdminClient, ApiKey, ApiKeyAction, CreateApiKeyRequest, FunctionAction,
    LegacyUpdateApiKeyCollectionPermissionsRequest, PatchApiKeyKvPermissionsRequest,
    PermissionEdit, UpdateApiKeyCollectionPermissionsRequest, UpdateApiKeyKvPermissionsRequest,
};
use std::{collections::HashMap, env, error::Error};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let base = env::var("API_BASE_URL")?;
    let owner = env::var("API_BASE_KEY")?;
    let admin = AdminClient::new(&base, &owner)?;
    let current = admin.get_permissions().await?;
    println!("Admin permissions read successfully");
    if env::var("EKODB_ADMIN_LIVE").as_deref() != Ok("1") {
        return Ok(());
    }
    if !current.is_admin {
        return Err("An admin key is required".into());
    }
    let label = format!("sdk-rust-admin-example-{}", uuid::Uuid::new_v4());
    println!("Disposable cleanup label: {label}");
    let result: Result<(), Box<dyn Error>> = async {
        let created = admin
            .create_api_key(&CreateApiKeyRequest {
                label: Some(label.clone()),
                collection_permissions: HashMap::from([(
                    "sdk_admin_probe".into(),
                    vec![ApiKeyAction::Read],
                )]),
                function_permissions: HashMap::from([(
                    format!("{label}-registered"),
                    vec![FunctionAction::Call],
                )]),
                ..Default::default()
            })
            .await?;
        if created.key == owner || created.is_admin {
            return Err("Unexpected disposable key".into());
        }
        let scoped = AdminClient::new(&base, &created.key)?;
        if scoped.get_permissions().await?.is_admin {
            return Err("Unexpected admin grant".into());
        }
        if admin.uses_key_permission_routes().await? {
            let id = created.id.as_ref().ok_or("Registration omitted key id")?;
            let listed = admin.list_api_keys().await?;
            if !listed
                .iter()
                .any(|key| key.label.as_deref() == Some(&label) && key.id.as_ref() == Some(id))
            {
                return Err("Key listing omitted the registered id".into());
            }
            let prefix = format!("sdk-admin:{label}:");
            let change = admin
                .put_api_key_kv_permissions(&UpdateApiKeyKvPermissionsRequest {
                    key_id: id.clone(),
                    prefix: prefix.clone(),
                    read: true,
                    write: false,
                })
                .await?;
            if !change.after.is_some_and(|grant| grant.read && !grant.write) {
                return Err("KV grant creation failed".into());
            }
            admin
                .patch_api_key_kv_permissions(&PatchApiKeyKvPermissionsRequest {
                    key_id: id.clone(),
                    prefix: prefix.clone(),
                    read: None,
                    write: Some(true),
                })
                .await?;
            if admin
                .delete_api_key_kv_permissions(id, &prefix)
                .await?
                .after
                .is_some()
            {
                return Err("KV grant removal failed".into());
            }
            // Stored-function call grants (needs a server that supports them).
            let registered = format!("{label}-registered");
            let function_pattern = format!("{label}-*");
            let held = admin.list_api_key_function_permissions(id).await?;
            if !held.iter().any(|grant| grant.pattern == registered) {
                return Err("Registered function grant missing".into());
            }
            let granted = admin
                .put_api_key_function_permissions(id, &function_pattern)
                .await?;
            if granted.before.is_some()
                || granted.after.map(|grant| grant.pattern) != Some(function_pattern.clone())
            {
                return Err("Function grant creation failed".into());
            }
            if admin
                .get_api_key_function_permissions(id, &function_pattern)
                .await?
                .pattern
                != function_pattern
            {
                return Err("Function grant readback failed".into());
            }
            if admin
                .delete_api_key_function_permissions(id, &function_pattern)
                .await?
                .after
                .is_some()
            {
                return Err("Function grant removal failed".into());
            }
        }
        let request = LegacyUpdateApiKeyCollectionPermissionsRequest {
            api_key: created.key.clone(),
            collection: "sdk_admin_probe".into(),
            read: true,
            write: true,
            select_fields: PermissionEdit::Set(vec!["public".into()]),
            exclude_fields: PermissionEdit::Set(vec!["secret".into()]),
            writable_fields: PermissionEdit::Set(vec!["public".into()]),
        };
        edit_grant(&admin, &created, &request, true).await?;
        let first = AdminClient::new(&base, &created.key)?
            .get_permissions()
            .await?;
        let initial = first
            .collection_permissions
            .get("sdk_admin_probe")
            .ok_or("Missing disposable grant")?;
        if !initial.read
            || !initial.write
            || initial.select_fields != Some(vec!["public".into()])
            || initial.exclude_fields != Some(vec!["secret".into()])
            || initial.writable_fields != Some(vec!["public".into()])
        {
            return Err("Initial restriction readback failed".into());
        }
        let request = LegacyUpdateApiKeyCollectionPermissionsRequest {
            write: false,
            select_fields: PermissionEdit::Keep,
            exclude_fields: PermissionEdit::Clear,
            writable_fields: PermissionEdit::Set(vec![]),
            ..request
        };
        edit_grant(&admin, &created, &request, false).await?;
        let fresh = AdminClient::new(&base, &created.key)?
            .get_permissions()
            .await?;
        let grant = fresh
            .collection_permissions
            .get("sdk_admin_probe")
            .ok_or("Missing disposable grant")?;
        if !grant.read
            || grant.write
            || grant.select_fields != Some(vec!["public".into()])
            || grant.exclude_fields.is_some()
            || grant.writable_fields != Some(vec![])
        {
            return Err("Restriction readback failed".into());
        }
        Ok(())
    }
    .await;
    // Registered structurally before creation: label lookup also recovers ambiguous success.
    let matches = admin
        .list_api_keys()
        .await?
        .into_iter()
        .filter(|key| key.label.as_deref() == Some(&label))
        .collect::<Vec<_>>();
    if matches.len() > 1 || matches.iter().any(|key| key.key == owner) {
        return Err("Unexpected disposable cleanup match".into());
    }
    for key in matches {
        admin
            .revoke_api_key(key.id.as_deref().unwrap_or(&key.key))
            .await?;
    }
    if admin
        .list_api_keys()
        .await?
        .iter()
        .any(|key| key.label.as_deref() == Some(&label))
    {
        return Err(format!("Disposable cleanup failed for label {label}").into());
    }
    result?;
    println!("Disposable admin lifecycle verified; no matching key remains");
    Ok(())
}

async fn edit_grant(
    admin: &AdminClient,
    key: &ApiKey,
    request: &LegacyUpdateApiKeyCollectionPermissionsRequest,
    replace: bool,
) -> Result<(), Box<dyn Error>> {
    if admin.uses_key_permission_routes().await? {
        let request = UpdateApiKeyCollectionPermissionsRequest {
            key_id: key.id.clone().ok_or("Registration omitted key id")?,
            collection: request.collection.clone(),
            read: request.read,
            write: request.write,
            select_fields: request.select_fields.clone(),
            exclude_fields: request.exclude_fields.clone(),
            writable_fields: request.writable_fields.clone(),
        };
        if replace {
            admin.put_api_key_collection_permissions(&request).await?;
        } else {
            admin
                .update_api_key_collection_permissions(&request)
                .await?;
        }
    } else {
        admin
            .legacy_update_api_key_collection_permissions(request)
            .await?;
    }
    Ok(())
}
