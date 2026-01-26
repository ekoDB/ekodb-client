package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.Record
import io.ekodb.client.types.FieldType
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking

/**
 * Transaction Example using ekoDB Kotlin Client
 *
 * Demonstrates:
 * - Begin transaction with isolation level
 * - Operations within a transaction
 * - Transaction status checking
 * - Commit transaction
 * - Rollback transaction
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    println("✓ Client created\n")

    // Setup test data
    println("=== Setup: Creating Test Accounts ===")
    val alice = client.insert("test_accounts", Record.new()
        .insert("account_id", "ACC001")
        .insert("name", "Alice")
        .insert("balance", 1000))
    val aliceId = (alice["id"] as? FieldType.StringValue)?.value
        ?: error("No ID returned for Alice")
    println("Created Alice: \$1000 - ID: $aliceId")

    val bob = client.insert("test_accounts", Record.new()
        .insert("account_id", "ACC002")
        .insert("name", "Bob")
        .insert("balance", 500))
    val bobId = (bob["id"] as? FieldType.StringValue)?.value
        ?: error("No ID returned for Bob")
    println("Created Bob: \$500 - ID: $bobId\n")

    // Example 1: Begin Transaction
    println("=== Example 1: Begin Transaction ===")
    val txId = client.beginTransaction("ReadCommitted")
    println("Transaction ID: $txId\n")

    // Example 2: Operations within Transaction
    println("=== Example 2: Operations within Transaction ===")
    println("Updated Alice: \$1000 → \$800 (simulated)")
    println("Updated Bob: \$500 → \$700 (simulated)\n")

    // Example 3: Transaction Status
    println("=== Example 3: Transaction Status ===")
    val status = client.getTransactionStatus(txId)
    println("Status: ${status["state"]}")
    println("Operations: ${status["operations_count"]}\n")

    // Example 4: Commit Transaction
    println("=== Example 4: Commit Transaction ===")
    client.commitTransaction(txId)
    println("✓ Transaction committed\n")

    // Example 5: Rollback Demo
    println("=== Example 5: Rollback Demo ===")
    val txId2 = client.beginTransaction("ReadCommitted")
    println("New transaction: $txId2")

    val status2 = client.getTransactionStatus(txId2)
    println("Status before rollback: ${status2["state"]}")

    client.rollbackTransaction(txId2)
    println("✓ Transaction rolled back\n")

    // Cleanup
    println("=== Cleanup ===")
    client.delete("test_accounts", aliceId)
    client.delete("test_accounts", bobId)
    println("✓ Deleted test accounts\n")

    println("✓ All client transaction examples completed")
}
