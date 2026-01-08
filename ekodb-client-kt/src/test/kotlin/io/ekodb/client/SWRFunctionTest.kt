package io.ekodb.client

import io.ekodb.client.functions.FunctionStageConfig
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*

class SWRFunctionTest {

    private val json = Json { 
        prettyPrint = true
        ignoreUnknownKeys = true
    }

    @Test
    fun `test basic SWR function creation`() {
        val swr = FunctionStageConfig.SWR(
            cache_key = "user:{{user_id}}",
            ttl = JsonPrimitive("15m"),
            url = "https://api.example.com/users/{{user_id}}",
            method = "GET",
            headers = mapOf("User-Agent" to "ekoDB-Client"),
            body = null,
            timeout_seconds = 30,
            output_field = "user_data",
            collection = null
        )

        assertEquals("user:{{user_id}}", swr.cache_key)
        assertEquals(JsonPrimitive("15m"), swr.ttl)
        assertEquals("https://api.example.com/users/{{user_id}}", swr.url)
        assertEquals("GET", swr.method)
        assertEquals(mapOf("User-Agent" to "ekoDB-Client"), swr.headers)
        assertEquals(30, swr.timeout_seconds)
        assertEquals("user_data", swr.output_field)
        assertNull(swr.collection)
    }

    @Test
    fun `test SWR function with audit collection`() {
        val swr = FunctionStageConfig.SWR(
            cache_key = "product:{{id}}",
            ttl = JsonPrimitive("1h"),
            url = "https://api.example.com/products/{{id}}",
            method = "GET",
            headers = null,
            body = null,
            timeout_seconds = null,
            output_field = "product",
            collection = "swr_audit_trail"
        )

        assertEquals("product", swr.output_field)
        assertEquals("swr_audit_trail", swr.collection)
    }

    @Test
    fun `test SWR function with POST body`() {
        val body = buildJsonObject {
            put("query", JsonPrimitive("{{search_term}}"))
            put("limit", JsonPrimitive(10))
        }

        val swr = FunctionStageConfig.SWR(
            cache_key = "search:{{search_term}}",
            ttl = JsonPrimitive(900), // Integer seconds
            url = "https://api.example.com/search",
            method = "POST",
            headers = null,
            body = body,
            timeout_seconds = null,
            output_field = null,
            collection = null
        )

        assertEquals("POST", swr.method)
        assertEquals(JsonPrimitive(900), swr.ttl)
        assertEquals(body, swr.body)
    }

    @Test
    fun `test SWR function TTL formats`() {
        // Duration string
        val swr1 = FunctionStageConfig.SWR(
            cache_key = "test",
            ttl = JsonPrimitive("30m"),
            url = "https://example.com",
            method = "GET",
            headers = null,
            body = null,
            timeout_seconds = null,
            output_field = null,
            collection = null
        )
        assertEquals(JsonPrimitive("30m"), swr1.ttl)

        // Integer seconds
        val swr2 = FunctionStageConfig.SWR(
            cache_key = "test",
            ttl = JsonPrimitive(1800),
            url = "https://example.com",
            method = "GET",
            headers = null,
            body = null,
            timeout_seconds = null,
            output_field = null,
            collection = null
        )
        assertEquals(JsonPrimitive(1800), swr2.ttl)

        // ISO timestamp
        val swr3 = FunctionStageConfig.SWR(
            cache_key = "test",
            ttl = JsonPrimitive("2024-12-31T23:59:59Z"),
            url = "https://example.com",
            method = "GET",
            headers = null,
            body = null,
            timeout_seconds = null,
            output_field = null,
            collection = null
        )
        assertEquals(JsonPrimitive("2024-12-31T23:59:59Z"), swr3.ttl)
    }

    @Test
    fun `test SWR function with custom timeout`() {
        val swr = FunctionStageConfig.SWR(
            cache_key = "slow:api",
            ttl = JsonPrimitive("5m"),
            url = "https://slow-api.example.com/data",
            method = "GET",
            headers = null,
            body = null,
            timeout_seconds = 120,
            output_field = null,
            collection = null
        )

        assertEquals(120, swr.timeout_seconds)
    }

    @Test
    fun `test SWR function parameter substitution patterns`() {
        val swr = FunctionStageConfig.SWR(
            cache_key = "user:{{user_id}}:profile:{{profile_id}}",
            ttl = JsonPrimitive("15m"),
            url = "https://api.example.com/{{resource}}/{{id}}",
            method = "GET",
            headers = null,
            body = null,
            timeout_seconds = null,
            output_field = null,
            collection = null
        )

        val serialized = json.encodeToString(swr)
        
        assertTrue(serialized.contains("user:{{user_id}}:profile:{{profile_id}}"))
        assertTrue(serialized.contains("{{resource}}/{{id}}"))
    }

    @Test
    fun `test SWR function serialization roundtrip`() {
        val original = FunctionStageConfig.SWR(
            cache_key = "user:{{user_id}}",
            ttl = JsonPrimitive("15m"),
            url = "https://api.example.com/users/{{user_id}}",
            method = "GET",
            headers = mapOf("User-Agent" to "ekoDB-Client"),
            body = null,
            timeout_seconds = 30,
            output_field = "user_data",
            collection = null
        )

        val serialized = json.encodeToString<FunctionStageConfig>(original)
        val deserialized = json.decodeFromString<FunctionStageConfig>(serialized)

        assertTrue(deserialized is FunctionStageConfig.SWR)
        val swrDeserialized = deserialized as FunctionStageConfig.SWR
        
        assertEquals("user:{{user_id}}", swrDeserialized.cache_key)
        assertEquals("GET", swrDeserialized.method)
        assertEquals(30, swrDeserialized.timeout_seconds)
        assertEquals("user_data", swrDeserialized.output_field)
    }

    @Test
    fun `test SWR function with all optional fields null`() {
        val swr = FunctionStageConfig.SWR(
            cache_key = "minimal",
            ttl = JsonPrimitive("15m"),
            url = "https://example.com",
            method = "GET",
            headers = null,
            body = null,
            timeout_seconds = null,
            output_field = null,
            collection = null
        )

        // Required fields should be present
        assertEquals("minimal", swr.cache_key)
        assertEquals(JsonPrimitive("15m"), swr.ttl)
        assertEquals("https://example.com", swr.url)
        assertEquals("GET", swr.method)
        
        // Optional fields should be null
        assertNull(swr.headers)
        assertNull(swr.body)
        assertNull(swr.timeout_seconds)
        assertNull(swr.output_field)
        assertNull(swr.collection)
    }
}
