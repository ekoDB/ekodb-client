package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*

private fun unwrapField(field: JsonElement?): JsonElement? =
    if (field is JsonObject && "type" in field && "value" in field) field["value"] else field

private fun fieldDouble(field: JsonElement?): Double? =
    (unwrapField(field) as? JsonPrimitive)?.doubleOrNull

private fun fieldString(field: JsonElement?): String? =
    (unwrapField(field) as? JsonPrimitive)?.contentOrNull

private fun kvObject(result: Any?): JsonObject {
    var value = result as? JsonElement ?: error("KV response was not JSON: $result")
    repeat(3) {
        value = when {
            value is JsonPrimitive && value.isString ->
                Json.parseToJsonElement(value.content)
            value is JsonObject && "type" in value && "value" in value ->
                value["value"] ?: error("KV response value was null")
            value is JsonObject && value.size == 1 && "value" in value ->
                value["value"] ?: error("KV response value was null")
            else -> return@repeat
        }
    }
    return value as? JsonObject ?: error("KV response did not contain an object: $result")
}

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
    var primaryError: Throwable? = null

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
        check(floatResults.size == floatProducts.size) {
            "Expected ${floatProducts.size} float products, got ${floatResults.size}"
        }
        println("\nRetrieved float prices:")
        val expectedFloatPrices = listOf(29.99, 39.99, 49.99)
        val productNames = listOf("Widget A", "Widget B", "Widget C")
        for (i in floatResults.indices) {
            val obj = kvObject(floatResults[i])
            val actualPrice = fieldDouble(obj["price"])
                ?: error("Missing float price for ${productNames[i]}")
            val expectedPrice = expectedFloatPrices[i]
            val match = if (expectedPrice == actualPrice) "MATCH" else "PRECISION LOST"
            println("  ${productNames[i]}: \$$actualPrice (expected \$$expectedPrice) $match")
            check(actualPrice == expectedPrice) {
                "Float price for ${productNames[i]} was $actualPrice, expected $expectedPrice"
            }
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
        check(decimalResults.size == decimalProducts.size) {
            "Expected ${decimalProducts.size} decimal products, got ${decimalResults.size}"
        }
        val expectedDecimalPrices = listOf("29.99", "39.99", "49.99")
        println("\nRetrieved decimal prices:")
        for (i in decimalResults.indices) {
            val obj = kvObject(decimalResults[i])
            val actualPrice = fieldString(obj["price"])
                ?: error("Missing decimal price for ${productNames[i]}")
            val expectedPrice = expectedDecimalPrices[i]
            check(actualPrice == expectedPrice) {
                "Decimal price for ${productNames[i]} was $actualPrice, expected $expectedPrice"
            }
            println("  ${productNames[i]}: \$$actualPrice (expected \$$expectedPrice)")
        }

        // =====================================================================
        // Test 3: Real-world calculation comparison
        // =====================================================================
        println("\n=== Test 3: Sum Calculation Comparison ===")

        // Sum up float prices
        var floatSum = 0.0
        for ((index, result) in floatResults.withIndex()) {
            val price = fieldDouble(kvObject(result)["price"])
                ?: error("Missing float price for ${productNames[index]}")
            floatSum += price
        }
        println("  Float sum: \$$floatSum (expected \$119.97)")
        check(kotlin.math.abs(floatSum - 119.97) < 1e-9) {
            "Float sum was $floatSum, expected 119.97"
        }

        // Sum up decimal prices (come back as strings, convert for display)
        var decimalSum = java.math.BigDecimal.ZERO
        for ((index, result) in decimalResults.withIndex()) {
            val price = fieldString(kvObject(result)["price"])
                ?: error("Missing decimal price for ${productNames[index]}")
            decimalSum = decimalSum.add(price.toBigDecimal())
        }
        check(decimalSum == "119.97".toBigDecimal()) {
            "Decimal sum was $decimalSum, expected 119.97"
        }
        println("  Decimal sum: \$${decimalSum.toPlainString()} (expected \$119.97)")

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

        val floatParsed = kvObject(floatPrecision)
        val decimalParsed = kvObject(decimalPrecision)

        val floatAmount = fieldDouble(floatParsed["amount"])
            ?: error("Missing float precision amount in $floatPrecision")
        val decimalAmount = fieldString(decimalParsed["amount"])
            ?: error("Missing decimal precision amount in $decimalPrecision")
        println("  Float 0.1 + 0.2 = $floatAmount (should be 0.3)")
        println("  Decimal \"0.30\" = $decimalAmount (exact!)")
        check(floatAmount == 0.1 + 0.2) {
            "Float precision amount was $floatAmount, expected ${0.1 + 0.2}"
        }
        check(decimalAmount == "0.30") {
            "Decimal precision amount was $decimalAmount, expected 0.30"
        }

    } catch (error: Throwable) {
        primaryError = error
        throw error
    } finally {
        // =====================================================================
        // Cleanup
        // =====================================================================
        println("\n=== Cleanup ===")
        var cleanupError: Throwable? = null
        try {
            client.kvBatchDelete(allKeys)
            println("Cleaned up ${allKeys.size} test keys")
        } catch (error: Throwable) {
            println("Cleanup error: ${error.message}")
            cleanupError = error
        }

        try {
            client.close()
        } catch (error: Throwable) {
            if (cleanupError == null) {
                cleanupError = error
            } else {
                cleanupError.addSuppressed(error)
            }
        }

        cleanupError?.let { error ->
            if (primaryError == null) throw error
            primaryError.addSuppressed(error)
        }
    }

    println("\n=== Summary ===")
    println("Use fieldDecimal() for monetary values, percentages, and")
    println("any case where floating-point errors are unacceptable.")
    println("fieldDecimal() stores values as strings internally,")
    println("preserving exact precision across all operations.")
    println("\n=== Example Complete ===")
}
