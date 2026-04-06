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

    val funcIds = mutableListOf<String>()

    try {
        // Wrapped Types Examples
        wrappedTypesInsert(client)
        funcIds.add(wrappedTypesInScript(client))

        // KV Store Examples
        kvBasicOperations(client)
        funcIds.add(kvScriptOperations(client))

        // Combined Example
        funcIds.add(combinedExample(client))

        // Cleanup
        cleanup(client, funcIds)

        println("✅ All KV & Wrapped Types examples completed!")
        println("\n💡 Key takeaways:")
        println("   ✅ Use field* helpers for type-safe wrapped values")
        println("   ✅ fieldDecimal() preserves precision (no floating point errors)")
        println("   ✅ KV store is great for caching and quick lookups")
        println("   ✅ FunctionStageConfig.Kv* classes work within functions")

    } catch (e: Exception) {
        println("❌ Error: ${e.message}")
        e.printStackTrace()
    }
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

    val result = client.insert("orders_example", order)
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

    client.insert("products_example", product1)
    client.insert("products_example", product2)
    println("✅ Inserted 2 products\n")
}

suspend fun wrappedTypesInScript(client: EkoDBClient): String {
    println("📝 Example 2: function with Wrapped Type Parameters\n")

    val func = UserFunction(
        label = "create_order_with_types_kt",
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
                collection = "function_orders",
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

    val id = client.saveFunction(func)
    println("✅ Function saved: $id")

    val result = client.callFunction("create_order_with_types_kt", mapOf(
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
    client.kvSet("user:session:123", sessionData)
    println("✅ Set session data")

    // Get the value back
    val session = client.kvGet("user:session:123")
    println("📊 Retrieved session: $session")

    // Set with TTL (1 hour)
    val cacheData = buildJsonObject {
        put("name", "Cached Product")
        put("price", 99.99)
    }
    client.kvSetWithTtl("cache:product:456", cacheData, "3600")
    println("✅ Set cached data with 1 hour TTL")

    // Delete a key
    client.kvDelete("user:session:123")
    println("🗑️  Deleted session\n")
}

suspend fun kvScriptOperations(client: EkoDBClient): String {
    println("📝 Example 4: KV Operations in Functions\n")

    val func = UserFunction(
        label = "cached_product_lookup_kt",
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

    val id = client.saveFunction(func)
    println("✅ Function saved: $id")

    val result = client.callFunction("cached_product_lookup_kt", mapOf(
        "product_key" to JsonPrimitive("product:cache:789"),
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
        label = "process_order_with_cache_kt",
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
                key = "order:status:{{order_id}}",
                value = buildJsonObject {
                    put("status", "processing")
                    put("updated_at", "{{timestamp}}")
                },
                ttl = 86400
            ),
            FunctionStageConfig.Insert(
                collection = "processed_orders",
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
            FunctionStageConfig.KvGet(key = "order:status:{{order_id}}")
        ),
        tags = listOf("orders", "kv", "wrapped-types")
    )

    val id = client.saveFunction(func)
    println("✅ Function saved: $id")

    val result = client.callFunction("process_order_with_cache_kt", mapOf(
        "order_id" to JsonPrimitive("c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6"),
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

suspend fun cleanup(client: EkoDBClient, funcIds: List<String>) {
    println("🧹 Cleaning up...")

    try {
        for (id in funcIds) {
            client.deleteFunction(id)
        }

        client.deleteCollection("orders_example")
        client.deleteCollection("products_example")
        client.deleteCollection("function_orders")
        client.deleteCollection("processed_orders")

        client.kvDelete("cache:product:456")
        client.kvDelete("product:cache:789")
        client.kvDelete("order:status:c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6")

        println("✅ Cleanup complete\n")
    } catch (e: Exception) {
        println("⚠️  Cleanup had some errors (may be expected)\n")
    }
}
