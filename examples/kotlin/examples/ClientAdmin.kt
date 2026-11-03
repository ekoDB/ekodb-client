package io.ekodb.client.examples

// Read-only by default; EKODB_ADMIN_LIVE=1 creates and revokes one disposable key.
import io.ekodb.client.admin.*
import io.github.cdimascio.dotenv.dotenv
import java.util.UUID
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val env = dotenv { ignoreIfMissing = true }
    val base = System.getenv("API_BASE_URL") ?: env["API_BASE_URL"] ?: error("API_BASE_URL is required")
    val owner = System.getenv("API_BASE_KEY") ?: env["API_BASE_KEY"] ?: error("API_BASE_KEY is required")
    EkoDBAdminClient(base, owner).use { admin ->
        val current = admin.getPermissions()
        println("Admin permissions read successfully")
        if (System.getenv("EKODB_ADMIN_LIVE") != "1") return@use
        check(current.isAdmin) { "An admin key is required" }
        val label = "sdk-kotlin-admin-example-${UUID.randomUUID()}"
        println("Disposable cleanup label: $label")
        try {
            val registered = "$label-registered"
            val key = admin.createApiKey(CreateApiKeyRequest(label = label, collectionPermissions = mapOf("sdk_admin_probe" to listOf(ApiKeyAction.Read)), functionPermissions = mapOf(registered to listOf(FunctionAction.Call))))
            check(key.key != owner && !key.isAdmin) { "Unexpected disposable key" }
            EkoDBAdminClient(base, key.key).use { check(!it.getPermissions().isAdmin) { "Unexpected admin grant" } }
            val modern = admin.usesKeyPermissionRoutes()
            if (modern) {
                val id = key.id ?: error("Registration omitted key id")
                check(admin.listApiKeys().any { it.id == id && it.label == label }) { "Key listing omitted the registered id" }
                val prefix = "sdk-admin:$label:"
                val change = admin.putApiKeyKvPermissions(UpdateApiKeyKvPermissionsRequest(id, prefix, true, false))
                check(change.after?.read == true && change.after?.write == false) { "KV grant creation failed" }
                admin.patchApiKeyKvPermissions(PatchApiKeyKvPermissionsRequest(id, prefix, write = true))
                check(admin.deleteApiKeyKvPermissions(id, prefix).after == null) { "KV grant removal failed" }
                // Stored-function call grants (needs a server that supports them).
                check(admin.listApiKeyFunctionPermissions(id).any { it.pattern == registered }) { "Registered function grant missing" }
                val functionPattern = "$label-*"
                val granted = admin.putApiKeyFunctionPermissions(id, functionPattern)
                check(granted.before == null && granted.after?.pattern == functionPattern) { "Function grant creation failed" }
                check(admin.getApiKeyFunctionPermissions(id, functionPattern).pattern == functionPattern) { "Function grant readback failed" }
                check(admin.deleteApiKeyFunctionPermissions(id, functionPattern).after == null) { "Function grant removal failed" }
            }
            suspend fun edit(request: LegacyUpdateApiKeyCollectionPermissionsRequest, replace: Boolean = false) {
                if (modern) {
                    val input = UpdateApiKeyCollectionPermissionsRequest(key.id ?: error("Registration omitted key id"), request.collection, request.read, request.write, request.selectFields, request.excludeFields, request.writableFields)
                    if (replace) admin.putApiKeyCollectionPermissions(input) else admin.updateApiKeyCollectionPermissions(input)
                } else admin.legacyUpdateApiKeyCollectionPermissions(request)
            }
            val request = LegacyUpdateApiKeyCollectionPermissionsRequest(key.key, "sdk_admin_probe", true, true, PermissionEdit.Set(listOf("public")), PermissionEdit.Set(listOf("secret")), PermissionEdit.Set(listOf("public")))
            edit(request, replace = true)
            EkoDBAdminClient(base, key.key).use { fresh ->
                val initial = fresh.getPermissions().collectionPermissions.getValue("sdk_admin_probe")
                check(initial.read && initial.write && initial.selectFields == listOf("public") && initial.excludeFields == listOf("secret") && initial.writableFields == listOf("public")) { "Initial restriction readback failed" }
            }
            edit(request.copy(write = false, selectFields = PermissionEdit.Keep, excludeFields = PermissionEdit.Clear, writableFields = PermissionEdit.Set(emptyList())))
            EkoDBAdminClient(base, key.key).use { fresh ->
                val grant = fresh.getPermissions().collectionPermissions.getValue("sdk_admin_probe")
                check(grant.read && !grant.write && grant.selectFields == listOf("public") && grant.excludeFields == null && grant.writableFields == emptyList<String>()) { "Restriction readback failed" }
            }
        } finally {
            val matches = admin.listApiKeys().filter { it.label == label }
            check(matches.size <= 1 && matches.none { it.key == owner }) { "Unexpected disposable cleanup match" }
            matches.forEach { admin.revokeApiKey(it.id ?: it.key) }
            check(admin.listApiKeys().none { it.label == label }) { "Disposable cleanup failed for label $label" }
        }
        println("Disposable admin lifecycle verified; no matching key remains")
    }
}
