package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.extractFieldTypeValue
import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking

private const val COLLECTION = "client_transactions_accounts_kt"

private fun balance(record: Record): Long =
    (record["balance"]?.let(::extractFieldTypeValue) as? Number)?.toLong()
        ?: error("Record did not contain a numeric balance")

/**
 * Transaction Example using ekoDB Kotlin Client
 *
 * Demonstrates:
 * - Begin transaction with the server default or an explicit isolation level
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
    val activeTransactions = mutableListOf<String>()
    var failure: Throwable? = null

    try {
    // Setup test data
    println("=== Setup: Creating Test Accounts ===")
    val alice = client.insert(COLLECTION, Record.new()
        .insert("account_id", "ACC001")
        .insert("name", "Alice")
        .insert("balance", 1000))
    val aliceId = (alice["id"] as? FieldType.StringValue)?.value
        ?: error("No ID returned for Alice")
    println("Created Alice: \$1000 - ID: $aliceId")

    val bob = client.insert(COLLECTION, Record.new()
        .insert("account_id", "ACC002")
        .insert("name", "Bob")
        .insert("balance", 500))
    val bobId = (bob["id"] as? FieldType.StringValue)?.value
        ?: error("No ID returned for Bob")
    println("Created Bob: \$500 - ID: $bobId\n")

    // Example 1: Begin Transaction
    println("=== Example 1: Begin Transaction ===")
    val txId = client.beginTransaction()
    activeTransactions += txId
    println("Transaction ID (server-default isolation): $txId\n")

    // Example 2: Operations within Transaction
    println("=== Example 2: Operations within Transaction ===")
    client.update(COLLECTION, aliceId, Record.new().insert("balance", 800), transactionId = txId)
    client.update(COLLECTION, bobId, Record.new().insert("balance", 700), transactionId = txId)
    check(balance(client.findById(COLLECTION, aliceId, transactionId = txId)) == 800L)
    check(balance(client.findById(COLLECTION, bobId, transactionId = txId)) == 700L)
    println("Updated Alice: \$1000 → \$800")
    println("Updated Bob: \$500 → \$700\n")

    // Example 3: Transaction Status
    println("=== Example 3: Transaction Status ===")
    val status = client.getTransactionStatus(txId)
    println("Status: ${status["state"]}")
    println("Operations: ${status["operations_count"]}\n")
    check(status["state"] == "Active" && status["operations_count"] == 2) {
        "Unexpected transaction status: $status"
    }

    // Example 4: Commit Transaction
    println("=== Example 4: Commit Transaction ===")
    client.commitTransaction(txId)
    activeTransactions.remove(txId)
    println("✓ Transaction committed\n")
    check(balance(client.findById(COLLECTION, aliceId)) == 800L)
    check(balance(client.findById(COLLECTION, bobId)) == 700L)
    println("✓ Verified committed balances: Alice=\$800, Bob=\$700\n")

    // Example 5: Rollback Demo
    println("=== Example 5: Rollback Demo ===")
    val txId2 = client.beginTransaction("ReadCommitted")
    activeTransactions += txId2
    println("New transaction: $txId2")

    client.update(COLLECTION, bobId, Record.new().insert("balance", 600), transactionId = txId2)
    check(balance(client.findById(COLLECTION, bobId, transactionId = txId2)) == 600L)
    println("Updated Bob: \$700 → \$600 (in transaction)")

    val status2 = client.getTransactionStatus(txId2)
    println("Status before rollback: ${status2["state"]}")
    check(status2["state"] == "Active" && status2["operations_count"] == 1) {
        "Unexpected rollback transaction status: $status2"
    }

    client.rollbackTransaction(txId2)
    activeTransactions.remove(txId2)
    println("✓ Transaction rolled back\n")
    check(balance(client.findById(COLLECTION, bobId)) == 700L)
    println("✓ Verified Bob remains \$700 after rollback\n")
    } catch (error: Throwable) {
        failure = error
    } finally {
        println("=== Cleanup ===")
        for (transactionId in activeTransactions.asReversed()) {
            try {
                client.rollbackTransaction(transactionId)
            } catch (cleanupError: Throwable) {
                failure = failure?.also { it.addSuppressed(cleanupError) } ?: cleanupError
            }
        }
        try {
            client.deleteCollection(COLLECTION)
            println("✓ Deleted test account collection\n")
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
    println("✓ All client transaction examples completed")
}
