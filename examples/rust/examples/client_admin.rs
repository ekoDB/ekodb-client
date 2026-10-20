//! Read-only by default; EKODB_ADMIN_LIVE=1 creates and revokes one disposable key.
use ekodb_client::admin::{
    AdminClient, ApiKeyAction, CreateApiKeyRequest, PermissionEdit,
    UpdateApiKeyCollectionPermissionsRequest,
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
        let request = UpdateApiKeyCollectionPermissionsRequest {
            api_key: created.key.clone(),
            collection: "sdk_admin_probe".into(),
            read: true,
            write: true,
            select_fields: PermissionEdit::Set(vec!["public".into()]),
            exclude_fields: PermissionEdit::Set(vec!["secret".into()]),
            writable_fields: PermissionEdit::Set(vec!["public".into()]),
        };
        admin
            .update_api_key_collection_permissions(&request)
            .await?;
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
        let request = UpdateApiKeyCollectionPermissionsRequest {
            write: false,
            select_fields: PermissionEdit::Keep,
            exclude_fields: PermissionEdit::Clear,
            writable_fields: PermissionEdit::Set(vec![]),
            ..request
        };
        admin
            .update_api_key_collection_permissions(&request)
            .await?;
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
        admin.revoke_api_key(&key.key).await?;
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
