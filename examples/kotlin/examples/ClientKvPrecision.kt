package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.fieldDecimal
import io.ekodb.client.getValue
import io.ekodb.client.getDecimalValue
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*

/**
 * KV Precision example - Float vs Decimal Comparison
 *
 * Demonstrates the difference between using Kotlin Doubles (which lose precision)
 * vs ekoDB's Decimal type (which preserves precision).
 *
 * Key takeaway: Use fieldDecimal() for monetary values and other cases
 * where precision matters.
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== ekoDB Kotlin Client - KV Precision: Float vs Decimal ===\n")

    // Track all keys for cleanup
    val allKeys = mutableListOf<String>()

    try {
        // =====================================================================
        // Test 1: Float Precision Loss
        // =====================================================================
        println("=== Test 1: Using Kotlin Doubles (LOSES PRECISION) ===")

        val floatProducts = listOf(
            Triple("product:float:1", buildJsonObject { put("name", "Widget A"); put("price", 29.99) }, null),
            Triple("product:float:2", buildJsonObject { put("name", "Widget B"); put("price", 39.99) }, null),
            Triple("product:float:3", buildJsonObject { put("name", "Widget C"); put("price", 49.99) }, null)
        )
        val floatKeys = floatProducts.map { it.first }
        allKeys.addAll(floatKeys)

        // Store with float values
        client.kvBatchSet(floatProducts)
        println("Stored products with float prices")

        // Retrieve and show the precision loss
        val floatResults = client.kvBatchGet(floatKeys)
        println("\nRetrieved float prices:")
        val expectedFloatPrices = listOf(29.99, 39.99, 49.99)
        val productNames = listOf("Widget A", "Widget B", "Widget C")
        for (i in floatResults.indices) {
            val obj = floatResults[i].jsonObject
            val actualPrice = getValue<Double>(obj["price"])
            val expectedPrice = expectedFloatPrices[i]
            val match = if (expectedPrice == actualPrice) "MATCH" else "PRECISION LOST"
            println("  ${productNames[i]}: \$$actualPrice (expected \$$expectedPrice) $match")
        }

        // =====================================================================
        // Test 2: Decimal Precision Preserved
        // =====================================================================
        println("\n=== Test 2: Using fieldDecimal() (PRESERVES PRECISION) ===")

        // fieldDecimal() returns Map<String, Any>, but kvBatchSet needs JsonElement values.
        // Build the JSON objects with decimal type wrappers manually.
        val decimalProducts = listOf(
            Triple("product:decimal:1", buildJsonObject {
                put("name", "Widget A")
                putJsonObject("price") { put("type", "Decimal"); put("value", "29.99") }
            }, null),
            Triple("product:decimal:2", buildJsonObject {
                put("name", "Widget B")
                putJsonObject("price") { put("type", "Decimal"); put("value", "39.99") }
            }, null),
            Triple("product:decimal:3", buildJsonObject {
                put("name", "Widget C")
                putJsonObject("price") { put("type", "Decimal"); put("value", "49.99") }
            }, null)
        )
        val decimalKeys = decimalProducts.map { it.first }
        allKeys.addAll(decimalKeys)

        // Store with decimal values
        client.kvBatchSet(decimalProducts)
        println("Stored products with decimal prices")

        // Retrieve and show precision is preserved
        val decimalResults = client.kvBatchGet(decimalKeys)
        val expectedDecimalPrices = listOf("29.99", "39.99", "49.99")
        println("\nRetrieved decimal prices:")
        for (i in decimalResults.indices) {
            val obj = decimalResults[i].jsonObject
            val actualPrice = getDecimalValue(obj["price"])
            val expectedPrice = expectedDecimalPrices[i]
            println("  ${productNames[i]}: \$$actualPrice (expected \$$expectedPrice)")
        }

        // =====================================================================
        // Test 3: Real-world calculation comparison
        // =====================================================================
        println("\n=== Test 3: Sum Calculation Comparison ===")

        // Sum up float prices
        var floatSum = 0.0
        for (result in floatResults) {
            val price = getValue<Double>(result.jsonObject["price"])
            if (price != null) floatSum += price
        }
        println("  Float sum: \$$floatSum (expected \$119.97)")

        // Sum up decimal prices (come back as strings, convert for display)
        var decimalSum = 0.0
        for (result in decimalResults) {
            val price = getDecimalValue(result.jsonObject["price"])
            if (price != null) decimalSum += price
        }
        println("  Decimal sum: \$${"%.2f".format(decimalSum)} (expected \$119.97)")

        // =====================================================================
        // Test 4: More extreme precision example
        // =====================================================================
        println("\n=== Test 4: Extreme Precision Example ===")

        // Store a value that floats can't represent exactly: 0.1 + 0.2
        val floatPrecisionKey = "precision:float"
        val decimalPrecisionKey = "precision:decimal"
        allKeys.add(floatPrecisionKey)
        allKeys.add(decimalPrecisionKey)

        client.kvSet(floatPrecisionKey, buildJsonObject {
            put("amount", 0.1 + 0.2) // Classic floating-point imprecision
        })
        client.kvSet(decimalPrecisionKey, buildJsonObject {
            putJsonObject("amount") { put("type", "Decimal"); put("value", "0.30") }
        })

        // Retrieve and compare
        val floatPrecision = client.kvGet(floatPrecisionKey)
        val decimalPrecision = client.kvGet(decimalPrecisionKey)

        // kvGet returns the value as a JsonPrimitive string containing JSON;
        // parse it to extract the nested object
        val floatParsed = if (floatPrecision is JsonPrimitive) {
            Json.parseToJsonElement(floatPrecision.content).jsonObject
        } else if (floatPrecision is JsonObject) {
            floatPrecision
        } else null

        val decimalParsed = if (decimalPrecision is JsonPrimitive) {
            Json.parseToJsonElement(decimalPrecision.content).jsonObject
        } else if (decimalPrecision is JsonObject) {
            decimalPrecision
        } else null

        val floatAmount = floatParsed?.get("amount")?.let { getValue<Double>(it) }
        val decimalAmount = decimalParsed?.get("amount")?.let { getDecimalValue(it) }

        println("  Float 0.1 + 0.2 = $floatAmount (should be 0.3)")
        println("  Decimal \"0.30\" = $decimalAmount (exact!)")

    } finally {
        // =====================================================================
        // Cleanup
        // =====================================================================
        println("\n=== Cleanup ===")
        try {
            client.kvBatchDelete(allKeys)
            println("Cleaned up ${allKeys.size} test keys")
        } catch (e: Exception) {
            println("Cleanup error: ${e.message}")
        }

        client.close()

        println("\n=== Summary ===")
        println("Use fieldDecimal() for monetary values, percentages, and")
        println("any case where floating-point errors are unacceptable.")
        println("fieldDecimal() stores values as strings internally,")
        println("preserving exact precision across all operations.")
        println("\n=== Example Complete ===")
    }
}
