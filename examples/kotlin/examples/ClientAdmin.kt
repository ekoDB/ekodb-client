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
            val key = admin.createApiKey(CreateApiKeyRequest(label = label, collectionPermissions = mapOf("sdk_admin_probe" to listOf(ApiKeyAction.Read))))
            check(key.key != owner && !key.isAdmin) { "Unexpected disposable key" }
            EkoDBAdminClient(base, key.key).use { check(!it.getPermissions().isAdmin) { "Unexpected admin grant" } }
            val request = UpdateApiKeyCollectionPermissionsRequest(key.key, "sdk_admin_probe", true, true, PermissionEdit.Set(listOf("public")), PermissionEdit.Set(listOf("secret")), PermissionEdit.Set(listOf("public")))
            admin.updateApiKeyCollectionPermissions(request)
            EkoDBAdminClient(base, key.key).use { fresh ->
                val initial = fresh.getPermissions().collectionPermissions.getValue("sdk_admin_probe")
                check(initial.read && initial.write && initial.selectFields == listOf("public") && initial.excludeFields == listOf("secret") && initial.writableFields == listOf("public")) { "Initial restriction readback failed" }
            }
            admin.updateApiKeyCollectionPermissions(request.copy(write = false, selectFields = PermissionEdit.Keep, excludeFields = PermissionEdit.Clear, writableFields = PermissionEdit.Set(emptyList())))
            EkoDBAdminClient(base, key.key).use { fresh ->
                val grant = fresh.getPermissions().collectionPermissions.getValue("sdk_admin_probe")
                check(grant.read && !grant.write && grant.selectFields == listOf("public") && grant.excludeFields == null && grant.writableFields == emptyList<String>()) { "Restriction readback failed" }
            }
        } finally {
            val matches = admin.listApiKeys().filter { it.label == label }
            check(matches.size <= 1 && matches.none { it.key == owner }) { "Unexpected disposable cleanup match" }
            matches.forEach { admin.revokeApiKey(it.key) }
            check(admin.listApiKeys().none { it.label == label }) { "Disposable cleanup failed for label $label" }
        }
        println("Disposable admin lifecycle verified; no matching key remains")
    }
}
