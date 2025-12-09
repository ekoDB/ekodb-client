import io.github.cdimascio.dotenv.dotenv
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import org.json.JSONArray
import org.json.JSONObject

val client = OkHttpClient()
val JSON = "application/json; charset=utf-8".toMediaType()

fun getAuthToken(): String {
    val dotenv = dotenv()
    val baseURL = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"
    
    val json = JSONObject().put("api_key", apiKey).toString()
    val body = json.toRequestBody(JSON)
    val request = Request.Builder()
        .url("$baseURL/api/auth/token")
        .post(body)
        .build()

    client.newCall(request).execute().use { response ->
        val jsonResponse = JSONObject(response.body!!.string())
        return jsonResponse.getString("token")
    }
}

fun makeRequest(method: String, path: String, token: String, data: JSONObject? = null): JSONObject {
    val dotenv = dotenv()
    val baseURL = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    
    val requestBuilder = Request.Builder()
        .url("$baseURL$path")
        .header("Authorization", "Bearer $token")
        .header("Content-Type", "application/json")

    when (method) {
        "POST" -> requestBuilder.post(data?.toString()?.toRequestBody(JSON) ?: "{}".toRequestBody(JSON))
        "PUT" -> requestBuilder.put(data?.toString()?.toRequestBody(JSON) ?: "{}".toRequestBody(JSON))
        "DELETE" -> requestBuilder.delete()
        else -> requestBuilder.get()
    }

    val request = requestBuilder.build()
    client.newCall(request).execute().use { response ->
        val responseBody = response.body!!.string()
        return if (responseBody.isNotEmpty()) JSONObject(responseBody) else JSONObject()
    }
}

fun main() {
    val token = getAuthToken()
    println("✓ Authentication successful\n")

    // Setup test data
    println("=== Setup: Creating Test Accounts ===")
    val alice = makeRequest("POST", "/api/insert/test_accounts", token, JSONObject().apply {
        put("account_id", "ACC001")
        put("name", "Alice")
        put("balance", 1000)
    })
    val aliceId = alice.getString("id")
    println("Created Alice: \$1000 - ID: $aliceId")

    val bob = makeRequest("POST", "/api/insert/test_accounts", token, JSONObject().apply {
        put("account_id", "ACC002")
        put("name", "Bob")
        put("balance", 500)
    })
    val bobId = bob.getString("id")
    println("Created Bob: \$500 - ID: $bobId\n")

    // Example 1: Begin Transaction
    println("=== Example 1: Begin Transaction ===")
    val txData = makeRequest("POST", "/api/transactions", token, JSONObject().apply {
        put("isolation_level", "ReadCommitted")
    })
    val txId = txData.getString("transaction_id")
    println("Transaction ID: $txId\n")

    // Example 2: Operations with transaction_id
    println("=== Example 2: Operations with transaction_id ===")
    makeRequest("PUT", "/api/batch/update/test_accounts?transaction_id=$txId", token, JSONObject().apply {
        put("updates", JSONArray().apply {
            put(JSONObject().apply {
                put("id", aliceId)
                put("data", JSONObject().put("balance", 800))
            })
            put(JSONObject().apply {
                put("id", bobId)
                put("data", JSONObject().put("balance", 700))
            })
        })
    })
    println("Updated Alice: \$1000 → \$800")
    println("Updated Bob: \$500 → \$700\n")

    // Example 3: Transaction Status
    println("=== Example 3: Transaction Status ===")
    val status = makeRequest("GET", "/api/transactions/$txId", token)
    println("Status: ${status.getString("state")}")
    println("Operations: ${status.getInt("operations_count")}\n")

    // Example 4: Commit Transaction
    println("=== Example 4: Commit Transaction ===")
    makeRequest("POST", "/api/transactions/$txId/commit", token)
    println("✓ Transaction committed\n")

    // Verification
    println("=== Verification ===")
    val aliceFinal = makeRequest("GET", "/api/find/test_accounts/$aliceId", token)
    println("Alice: ${aliceFinal.get("balance")}")

    val bobFinal = makeRequest("GET", "/api/find/test_accounts/$bobId", token)
    println("Bob: ${bobFinal.get("balance")}\n")

    // Example 5: Rollback
    println("=== Example 5: Rollback ===")
    val txData2 = makeRequest("POST", "/api/transactions", token, JSONObject().apply {
        put("isolation_level", "ReadCommitted")
    })
    val txId2 = txData2.getString("transaction_id")
    println("New transaction: $txId2")

    makeRequest("PUT", "/api/batch/update/test_accounts?transaction_id=$txId2", token, JSONObject().apply {
        put("updates", JSONArray().apply {
            put(JSONObject().apply {
                put("id", bobId)
                put("data", JSONObject().put("balance", 600))
            })
        })
    })
    println("Updated Bob: \$700 → \$600 (in transaction)")

    makeRequest("POST", "/api/transactions/$txId2/rollback", token)
    println("✓ Transaction rolled back")

    val bobAfter = makeRequest("GET", "/api/find/test_accounts/$bobId", token)
    println("Bob after rollback: ${bobAfter.get("balance")}\n")

    // Cleanup
    println("=== Cleanup ===")
    makeRequest("DELETE", "/api/delete/test_accounts/$aliceId", token)
    makeRequest("DELETE", "/api/delete/test_accounts/$bobId", token)
    println("✓ Deleted test accounts\n")

    println("✓ All transaction examples completed")
}
