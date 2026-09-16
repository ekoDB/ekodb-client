/**
 * KV Store & Wrapped Types Example for ekoDB Kotlin Client
 *
 * Demonstrates: KV operations in functions, wrapped type field builders
 */

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.UserFunction
import io.ekodb.client.functions.ParameterDefinition
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import java.util.UUID
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

private const val ORDERS_COLLECTION = "kv_wrapped_orders_kt"
private const val PRODUCTS_COLLECTION = "kv_wrapped_products_kt"
private const val FUNCTION_ORDERS_COLLECTION = "kv_wrapped_function_orders_kt"
private const val PROCESSED_ORDERS_COLLECTION = "kv_wrapped_processed_orders_kt"
private const val CREATE_ORDER_LABEL = "kv_wrapped_create_order_kt"
private const val CACHED_PRODUCT_LABEL = "kv_wrapped_cached_product_kt"
private const val PROCESS_ORDER_LABEL = "kv_wrapped_process_order_kt"
private const val SESSION_KEY = "kv_wrapped_kt:user:session:123"
private const val CACHE_KEY = "kv_wrapped_kt:cache:product:456"
private const val PRODUCT_KEY = "kv_wrapped_kt:product:cache:789"
private const val ORDER_ID = "c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6"
private const val ORDER_STATUS_KEY = "kv_wrapped_kt:order:status:$ORDER_ID"

private fun isAlreadyExistsError(e: Exception): Boolean {
    val msg = e.message ?: return false
    return msg.contains("status 409") || msg.contains("already exists")
}

private suspend fun saveOrUpdate(client: EkoDBClient, func: UserFunction): String {
    return try {
        client.saveFunction(func)
    } catch (e: Exception) {
        if (isAlreadyExistsError(e)) {
            client.updateFunction(func.label, func)
            println("ℹ️  Function '${func.label}' already existed — updated instead")
            client.getFunction(func.label).id
                ?: throw IllegalStateException("No ID returned for function '${func.label}'")
        } else {
            throw e
        }
    }
}

fun main() = runBlocking {
    println("🚀 ekoDB Kotlin KV Store & Wrapped Types Example\n")
    println("📋 Demonstrates:")
    println("   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)")
    println("   • KV store operations (get, set, delete, exists, query)")
    println("   • KV operations within functions")
    println("   • Combined wrapped types + KV workflows\n")

    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("✅ Client initialized\n")

    var failure: Throwable? = null

    try {
        // Wrapped Types Examples
        wrappedTypesInsert(client)
        wrappedTypesInScript(client)

        // KV Store Examples
        kvBasicOperations(client)
        kvScriptOperations(client)

        // Combined Example
        combinedExample(client)

    } catch (e: Throwable) {
        println("❌ Error: ${e.message}")
        e.printStackTrace()
        failure = e
    } finally {
        try {
            cleanup(client)
        } catch (cleanupError: Throwable) {
            failure = failure?.also { it.addSuppressed(cleanupError) } ?: cleanupError
        }
        try {
            client.close()
        } catch (cleanupError: Throwable) {
            failure = failure?.also { it.addSuppressed(cleanupError) } ?: cleanupError
        }
    }
    failure?.let { throw it }
    println("✅ All KV & Wrapped Types examples completed!")
    println("\n💡 Key takeaways:")
    println("   ✅ Use field* helpers for type-safe wrapped values")
    println("   ✅ fieldDecimal() preserves precision (no floating point errors)")
    println("   ✅ KV store is great for caching and quick lookups")
    println("   ✅ FunctionStageConfig.Kv* classes work within functions")
}

// =============================================================================
// Wrapped Types Examples
// =============================================================================

suspend fun wrappedTypesInsert(client: EkoDBClient) {
    println("📝 Example 1: Inserting Records with Wrapped Types\n")

    // Insert an order with wrapped types for precise data handling
    // Note: The field* helpers return Map<String, Any> which can be used with Record
    val order = Record.new()
        .insert("order_id", "550e8400-e29b-41d4-a716-446655440000")
        .insert("total", "1234.56")  // Store as string for precision
        .insert("quantity", 42)
        .insert("status", "pending")

    val result = client.insert(ORDERS_COLLECTION, order)
    println("✅ Inserted order: ${result.get("id")}")

    // Insert products with proper UUID types
    val product1 = Record.new()
        .insert("sku", FieldType.uuid(UUID.fromString("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11")))
        .insert("name", "Laptop Pro")
        .insert("price", "1299.99")
        .insert("stock", 15)
        .insert("available", true)

    val product2 = Record.new()
        .insert("sku", FieldType.uuid(UUID.fromString("b1ffcd00-0d1c-5f09-cc7e-7cc0ce491b22")))
        .insert("name", "Wireless Mouse")
        .insert("price", "29.99")
        .insert("stock", 150)
        .insert("available", true)

    client.insert(PRODUCTS_COLLECTION, product1)
    client.insert(PRODUCTS_COLLECTION, product2)
    println("✅ Inserted 2 products\n")
}

suspend fun wrappedTypesInScript(client: EkoDBClient): String {
    println("📝 Example 2: function with Wrapped Type Parameters\n")

    val func = UserFunction(
        label = CREATE_ORDER_LABEL,
        name = "Create Order with Wrapped Types (Kotlin)",
        description = "Demonstrates wrapped types in script insert operations",
        version = "1.0",
        parameters = mapOf(
            "order_total" to ParameterDefinition(
                required = true,
                default = JsonPrimitive("0.00"),
                description = "Total amount for the order"
            ),
            "order_id" to ParameterDefinition(
                required = true,
                description = "Unique order ID"
            ),
            "timestamp" to ParameterDefinition(
                paramType = "DateTime",
                required = true,
                description = "Current UTC timestamp (ISO 8601)"
            )
        ),
        functions = listOf(
            FunctionStageConfig.Insert(
                collection = FUNCTION_ORDERS_COLLECTION,
                record = buildJsonObject {
                    put("order_id", "{{order_id}}")
                    put("total", buildJsonObject {
                        put("type", "Decimal")
                        put("value", "{{order_total}}")
                    })
                    put("created_at", "{{timestamp}}")
                    put("status", "pending")
                }
            )
        ),
        tags = listOf("orders", "wrapped-types")
    )

    val id = saveOrUpdate(client, func)
    println("✅ Function saved: $id")

    val result = client.callFunction(CREATE_ORDER_LABEL, mapOf(
        "order_total" to JsonPrimitive("599.99"),
        "order_id" to JsonPrimitive("order_${System.currentTimeMillis()}"),
        "timestamp" to JsonPrimitive(java.time.Instant.now().toString())
    ))
    println("📊 Created order via function")
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")

    return id
}

// =============================================================================
// KV Store Examples
// =============================================================================

suspend fun kvBasicOperations(client: EkoDBClient) {
    println("📝 Example 3: Basic KV Store Operations\n")

    // Set a simple value
    val sessionData = buildJsonObject {
        put("userId", "user_abc")
        put("role", "admin")
    }
    client.kvSet(SESSION_KEY, sessionData)
    println("✅ Set session data")

    // Get the value back
    val session = client.kvGet(SESSION_KEY)
    println("📊 Retrieved session: $session")

    // Set with TTL (1 hour)
    val cacheData = buildJsonObject {
        put("name", "Cached Product")
        put("price", 99.99)
    }
    client.kvSetWithTtl(CACHE_KEY, cacheData, "3600")
    println("✅ Set cached data with 1 hour TTL")

    // Delete a key
    client.kvDelete(SESSION_KEY)
    println("🗑️  Deleted session\n")
}

suspend fun kvScriptOperations(client: EkoDBClient): String {
    println("📝 Example 4: KV Operations in Functions\n")

    val func = UserFunction(
        label = CACHED_PRODUCT_LABEL,
        name = "Cached Product Lookup (Kotlin)",
        description = "Uses KV store for caching within a script",
        version = "1.0",
        parameters = mapOf(
            "product_key" to ParameterDefinition(
                required = true,
                default = JsonPrimitive(""),
                description = "Cache key"
            ),
            "product_data" to ParameterDefinition(
                required = true,
                default = JsonPrimitive(""),
                description = "Data to cache"
            )
        ),
        functions = listOf(
            FunctionStageConfig.KvSet(
                key = "{{product_key}}",
                value = JsonPrimitive("{{product_data}}"),
                ttl = 3600
            ),
            FunctionStageConfig.KvGet(
                key = "{{product_key}}"
            )
        ),
        tags = listOf("kv", "caching")
    )

    val id = saveOrUpdate(client, func)
    println("✅ Function saved: $id")

    val result = client.callFunction(CACHED_PRODUCT_LABEL, mapOf(
        "product_key" to JsonPrimitive(PRODUCT_KEY),
        "product_data" to JsonPrimitive("{\"name\":\"Test Product\",\"price\":49.99}")
    ))
    println("📊 Cached and retrieved product data")
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")

    return id
}

// =============================================================================
// Combined Example
// =============================================================================

suspend fun combinedExample(client: EkoDBClient): String {
    println("📝 Example 5: Combined Wrapped Types + KV Function\n")

    val func = UserFunction(
        label = PROCESS_ORDER_LABEL,
        name = "Process Order with Cache (Kotlin)",
        description = "Demonstrates combined KV and wrapped type usage",
        version = "1.0",
        parameters = mapOf(
            "order_id" to ParameterDefinition(
                required = true,
                default = JsonPrimitive(""),
                description = "Order ID"
            ),
            "total" to ParameterDefinition(
                required = true,
                default = JsonPrimitive("0.00"),
                description = "Order total"
            ),
            "timestamp" to ParameterDefinition(
                paramType = "DateTime",
                required = true,
                description = "Current UTC timestamp (ISO 8601)"
            )
        ),
        functions = listOf(
            FunctionStageConfig.KvSet(
                key = "kv_wrapped_kt:order:status:{{order_id}}",
                value = buildJsonObject {
                    put("status", "processing")
                    put("updated_at", "{{timestamp}}")
                },
                ttl = 86400
            ),
            FunctionStageConfig.Insert(
                collection = PROCESSED_ORDERS_COLLECTION,
                record = buildJsonObject {
                    put("order_id", "{{order_id}}")
                    put("total", buildJsonObject {
                        put("type", "Decimal")
                        put("value", "{{total}}")
                    })
                    put("created_at", "{{timestamp}}")
                    put("status", "processing")
                }
            ),
            FunctionStageConfig.KvGet(key = "kv_wrapped_kt:order:status:{{order_id}}")
        ),
        tags = listOf("orders", "kv", "wrapped-types")
    )

    val id = saveOrUpdate(client, func)
    println("✅ Function saved: $id")

    val result = client.callFunction(PROCESS_ORDER_LABEL, mapOf(
        "order_id" to JsonPrimitive(ORDER_ID),
        "total" to JsonPrimitive("299.99"),
        "timestamp" to JsonPrimitive(java.time.Instant.now().toString())
    ))
    println("📊 Processed order with caching")
    println("⏱️  Stages executed: ${result.stats.stages_executed}")
    println("⏱️  Execution time: ${result.stats.execution_time_ms}ms\n")

    return id
}

// =============================================================================
// Cleanup
// =============================================================================

suspend fun cleanup(client: EkoDBClient) {
    println("🧹 Cleaning up...")
    var failure: Exception? = null
    suspend fun attempt(operation: suspend () -> Unit) {
        try {
            operation()
        } catch (error: Exception) {
            val notFound = error.message?.let {
                it.contains("status 404") || it.contains("not found", ignoreCase = true)
            } == true
            if (!notFound) {
                failure = failure?.also { it.addSuppressed(error) } ?: error
            }
        }
    }

    for (label in listOf(CREATE_ORDER_LABEL, CACHED_PRODUCT_LABEL, PROCESS_ORDER_LABEL)) {
        attempt { client.deleteFunction(label) }
    }
    for (collection in listOf(ORDERS_COLLECTION, PRODUCTS_COLLECTION, FUNCTION_ORDERS_COLLECTION, PROCESSED_ORDERS_COLLECTION)) {
        attempt { client.deleteCollection(collection) }
    }
    for (key in listOf(
        SESSION_KEY,
        CACHE_KEY,
        PRODUCT_KEY,
        ORDER_STATUS_KEY
    )) {
        attempt { client.kvDelete(key) }
    }

    failure?.let {
        println("⚠️  Cleanup had some errors (may be expected)\n")
        throw it
    }
    println("✅ Cleanup complete\n")
}
