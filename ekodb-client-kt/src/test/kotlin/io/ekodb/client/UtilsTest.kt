package io.ekodb.client

import io.ekodb.client.types.FieldType
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Unit tests for ekoDB Kotlin client utility functions
 */
class UtilsTest {

    // ========================================================================
    // getValue Tests
    // ========================================================================

    @Test
    fun `getValue extracts string from wrapped field`() {
        val field = mapOf("type" to "String", "value" to "hello world")
        assertEquals("hello world", getValue<String>(field))
    }

    @Test
    fun `getValue extracts integer from wrapped field`() {
        val field = mapOf("type" to "Integer", "value" to 42)
        assertEquals(42, getValue<Int>(field))
    }

    @Test
    fun `getValue extracts double from wrapped field`() {
        val field = mapOf("type" to "Float", "value" to 3.14)
        assertEquals(3.14, getValue<Double>(field))
    }

    @Test
    fun `getValue extracts boolean from wrapped field`() {
        val field = mapOf("type" to "Boolean", "value" to true)
        assertEquals(true, getValue<Boolean>(field))
    }

    @Test
    fun `getValue extracts null from wrapped field`() {
        val field = mapOf("type" to "Null", "value" to null)
        assertNull(getValue<Any>(field))
    }

    @Test
    fun `getValue returns plain string as-is`() {
        assertEquals("plain string", getValue<String>("plain string"))
    }

    @Test
    fun `getValue returns plain number as-is`() {
        assertEquals(123, getValue<Int>(123))
    }

    @Test
    fun `getValue returns plain boolean as-is`() {
        assertEquals(true, getValue<Boolean>(true))
        assertEquals(false, getValue<Boolean>(false))
    }

    @Test
    fun `getValue returns null for null input`() {
        assertNull(getValue<String>(null))
    }

    @Test
    fun `getValue handles FieldType StringValue`() {
        val field = FieldType.StringValue("test value")
        assertEquals("test value", getValue<String>(field))
    }

    @Test
    fun `getValue handles FieldType IntegerValue`() {
        val field = FieldType.IntegerValue(42)
        assertEquals(42, getValue<Int>(field))
    }

    @Test
    fun `getValue handles FieldType FloatValue`() {
        val field = FieldType.FloatValue(3.14)
        assertEquals(3.14, getValue<Double>(field))
    }

    @Test
    fun `getValue handles FieldType BooleanValue`() {
        val field = FieldType.BooleanValue(true)
        assertEquals(true, getValue<Boolean>(field))
    }

    // ========================================================================
    // getStringValue Tests
    // ========================================================================

    @Test
    fun `getStringValue extracts from wrapped field`() {
        val field = mapOf("type" to "String", "value" to "hello")
        assertEquals("hello", getStringValue(field))
    }

    @Test
    fun `getStringValue returns null for null input`() {
        assertNull(getStringValue(null))
    }

    // ========================================================================
    // getIntValue Tests
    // ========================================================================

    @Test
    fun `getIntValue extracts from wrapped field`() {
        val field = mapOf("type" to "Integer", "value" to 42)
        assertEquals(42, getIntValue(field))
    }

    @Test
    fun `getIntValue converts Long to Int`() {
        val field = mapOf("type" to "Integer", "value" to 42L)
        assertEquals(42, getIntValue(field))
    }

    @Test
    fun `getIntValue returns null for null input`() {
        assertNull(getIntValue(null))
    }

    // ========================================================================
    // getDoubleValue Tests
    // ========================================================================

    @Test
    fun `getDoubleValue extracts from wrapped field`() {
        val field = mapOf("type" to "Float", "value" to 3.14)
        assertEquals(3.14, getDoubleValue(field))
    }

    @Test
    fun `getDoubleValue returns null for null input`() {
        assertNull(getDoubleValue(null))
    }

    // ========================================================================
    // getBooleanValue Tests
    // ========================================================================

    @Test
    fun `getBooleanValue extracts from wrapped field`() {
        val field = mapOf("type" to "Boolean", "value" to true)
        assertEquals(true, getBooleanValue(field))
    }

    @Test
    fun `getBooleanValue returns null for null input`() {
        assertNull(getBooleanValue(null))
    }

    // ========================================================================
    // getValues Tests
    // ========================================================================

    @Test
    fun `getValues extracts multiple field values`() {
        val record = mapOf(
            "name" to mapOf("type" to "String", "value" to "John"),
            "age" to mapOf("type" to "Integer", "value" to 30),
            "active" to mapOf("type" to "Boolean", "value" to true)
        )
        val result = getValues(record, listOf("name", "age", "active"))
        assertEquals("John", result["name"])
        assertEquals(30, result["age"])
        assertEquals(true, result["active"])
    }

    @Test
    fun `getValues extracts only requested fields`() {
        val record = mapOf(
            "name" to mapOf("type" to "String", "value" to "John"),
            "age" to mapOf("type" to "Integer", "value" to 30),
            "email" to mapOf("type" to "String", "value" to "john@example.com")
        )
        val result = getValues(record, listOf("name", "email"))
        assertEquals(2, result.size)
        assertEquals("John", result["name"])
        assertEquals("john@example.com", result["email"])
    }

    @Test
    fun `getValues handles missing fields`() {
        val record = mapOf(
            "name" to mapOf("type" to "String", "value" to "John")
        )
        val result = getValues(record, listOf("name", "missing"))
        assertEquals("John", result["name"])
        assertNull(result["missing"])
    }

    @Test
    fun `getValues handles empty record`() {
        val result = getValues(emptyMap(), listOf("name", "age"))
        assertTrue(result.isEmpty())
    }

    @Test
    fun `getValues handles empty fields list`() {
        val record = mapOf("name" to mapOf("type" to "String", "value" to "John"))
        val result = getValues(record, emptyList())
        assertTrue(result.isEmpty())
    }

    // ========================================================================
    // Specialized Value Extractors Tests
    // ========================================================================

    @Test
    fun `getDateTimeValue parses ISO datetime string`() {
        val field = mapOf("type" to "DateTime", "value" to "2024-01-15T10:30:00Z")
        val result = getDateTimeValue(field)
        assertNotNull(result)
    }

    @Test
    fun `getDateTimeValue returns Date object as-is`() {
        val dt = java.util.Date()
        val field = mapOf("type" to "DateTime", "value" to dt)
        assertEquals(dt, getDateTimeValue(field))
    }

    @Test
    fun `getDateTimeValue returns null for null input`() {
        assertNull(getDateTimeValue(null))
    }

    @Test
    fun `getUUIDValue extracts UUID from wrapped field`() {
        val field = mapOf("type" to "UUID", "value" to "550e8400-e29b-41d4-a716-446655440000")
        assertEquals("550e8400-e29b-41d4-a716-446655440000", getUUIDValue(field))
    }

    @Test
    fun `getDecimalValue extracts from integer`() {
        val field = mapOf("type" to "Decimal", "value" to 42)
        assertEquals(42.0, getDecimalValue(field))
    }

    @Test
    fun `getDecimalValue extracts from double`() {
        val field = mapOf("type" to "Decimal", "value" to 3.14)
        assertEquals(3.14, getDecimalValue(field))
    }

    @Test
    fun `getDecimalValue extracts from string`() {
        val field = mapOf("type" to "Decimal", "value" to "123.456")
        assertEquals(123.456, getDecimalValue(field))
    }

    @Test
    fun `getDurationValue extracts milliseconds`() {
        val field = mapOf("type" to "Duration", "value" to 3600000L)
        assertEquals(3600000L, getDurationValue(field))
    }

    @Test
    fun `getDurationValue extracts from secs-nanos map`() {
        val field = mapOf("type" to "Duration", "value" to mapOf("secs" to 10, "nanos" to 500000000))
        assertEquals(10500L, getDurationValue(field))
    }

    @Test
    fun `getBytesValue extracts ByteArray`() {
        val bytes = byteArrayOf(104, 101, 108, 108, 111)
        val field = mapOf("type" to "Bytes", "value" to bytes)
        val result = getBytesValue(field)
        assertNotNull(result)
        assertTrue(bytes.contentEquals(result))
    }

    @Test
    fun `getBytesValue extracts from list of integers`() {
        val field = mapOf("type" to "Bytes", "value" to listOf(104, 101, 108, 108, 111))
        val result = getBytesValue(field)
        assertNotNull(result)
        assertEquals(5, result.size)
    }

    @Test
    fun `getBytesValue extracts from base64 string`() {
        val field = mapOf("type" to "Bytes", "value" to java.util.Base64.getEncoder().encodeToString("hello".toByteArray()))
        val result = getBytesValue(field)
        assertNotNull(result)
        assertEquals("hello", String(result))
    }

    @Test
    fun `getArrayValue extracts list`() {
        val field = mapOf("type" to "Array", "value" to listOf(1, 2, 3, 4, 5))
        val result = getArrayValue(field)
        assertEquals(listOf(1, 2, 3, 4, 5), result)
    }

    @Test
    fun `getSetValue extracts list`() {
        val field = mapOf("type" to "Set", "value" to listOf("a", "b", "c"))
        val result = getSetValue(field)
        assertEquals(listOf("a", "b", "c"), result)
    }

    @Test
    fun `getVectorValue extracts list of doubles`() {
        val field = mapOf("type" to "Vector", "value" to listOf(0.1, 0.2, 0.3))
        val result = getVectorValue(field)
        assertEquals(listOf(0.1, 0.2, 0.3), result)
    }

    @Test
    fun `getVectorValue converts integers to doubles`() {
        val field = mapOf("type" to "Vector", "value" to listOf(1, 2, 3))
        val result = getVectorValue(field)
        assertEquals(listOf(1.0, 2.0, 3.0), result)
    }

    @Test
    fun `getObjectValue extracts map`() {
        val field = mapOf("type" to "Object", "value" to mapOf("key" to "value", "count" to 5))
        val result = getObjectValue(field)
        assertEquals(mapOf("key" to "value", "count" to 5), result)
    }

    // ========================================================================
    // Field Builder Tests
    // ========================================================================

    @Test
    fun `fieldString creates String field`() {
        val result = fieldString("hello")
        assertEquals("String", result["type"])
        assertEquals("hello", result["value"])
    }

    @Test
    fun `fieldInteger creates Integer field`() {
        val result = fieldInteger(42)
        assertEquals("Integer", result["type"])
        assertEquals(42L, result["value"])
    }

    @Test
    fun `fieldFloat creates Float field`() {
        val result = fieldFloat(3.14)
        assertEquals("Float", result["type"])
        assertEquals(3.14, result["value"])
    }

    @Test
    fun `fieldBoolean creates Boolean field`() {
        val result = fieldBoolean(true)
        assertEquals("Boolean", result["type"])
        assertEquals(true, result["value"])
    }

    @Test
    fun `fieldUUID creates UUID field`() {
        val uuid = "550e8400-e29b-41d4-a716-446655440000"
        val result = fieldUUID(uuid)
        assertEquals("UUID", result["type"])
        assertEquals(uuid, result["value"])
    }

    @Test
    fun `fieldDecimal creates Decimal field`() {
        val result = fieldDecimal("123.456")
        assertEquals("Decimal", result["type"])
        assertEquals("123.456", result["value"])
    }

    @Test
    fun `fieldDateTime creates DateTime field from Date`() {
        val dt = java.util.Date()
        val result = fieldDateTime(dt)
        assertEquals("DateTime", result["type"])
        assertNotNull(result["value"])
    }

    @Test
    fun `fieldDateTimeString creates DateTime field from string`() {
        val result = fieldDateTimeString("2024-01-15T10:30:00Z")
        assertEquals("DateTime", result["type"])
        assertEquals("2024-01-15T10:30:00Z", result["value"])
    }

    @Test
    fun `fieldDuration creates Duration field`() {
        val result = fieldDuration(3600000L)
        assertEquals("Duration", result["type"])
        assertEquals(3600000L, result["value"])
    }

    @Test
    fun `fieldNumber creates Number field`() {
        val result = fieldNumber(42)
        assertEquals("Number", result["type"])
        assertEquals(42, result["value"])
    }

    @Test
    fun `fieldArray creates Array field`() {
        val result = fieldArray(listOf(1, 2, 3))
        assertEquals("Array", result["type"])
        assertEquals(listOf(1, 2, 3), result["value"])
    }

    @Test
    fun `fieldSet creates Set field`() {
        val result = fieldSet(listOf("a", "b", "c"))
        assertEquals("Set", result["type"])
        assertEquals(listOf("a", "b", "c"), result["value"])
    }

    @Test
    fun `fieldVector creates Vector field`() {
        val result = fieldVector(listOf(0.1, 0.2, 0.3))
        assertEquals("Vector", result["type"])
        assertEquals(listOf(0.1, 0.2, 0.3), result["value"])
    }

    @Test
    fun `fieldObject creates Object field`() {
        val result = fieldObject(mapOf("key" to "value"))
        assertEquals("Object", result["type"])
        assertEquals(mapOf("key" to "value"), result["value"])
    }

    @Test
    fun `fieldBytes creates Bytes field from ByteArray`() {
        val bytes = "hello".toByteArray()
        val result = fieldBytes(bytes)
        assertEquals("Bytes", result["type"])
        assertNotNull(result["value"])
    }

    @Test
    fun `fieldBinary creates Binary field from ByteArray`() {
        val bytes = byteArrayOf(1, 2, 3, 4)
        val result = fieldBinary(bytes)
        assertEquals("Binary", result["type"])
        assertNotNull(result["value"])
    }

    // ========================================================================
    // extractRecord Tests
    // ========================================================================

    @Test
    fun `extractRecord extracts all fields`() {
        val record = mapOf(
            "id" to "user_123",
            "name" to mapOf("type" to "String", "value" to "John Doe"),
            "age" to mapOf("type" to "Integer", "value" to 30),
            "active" to mapOf("type" to "Boolean", "value" to true)
        )
        val result = extractRecord(record)
        assertEquals("user_123", result["id"])
        assertEquals("John Doe", result["name"])
        assertEquals(30, result["age"])
        assertEquals(true, result["active"])
    }

    @Test
    fun `extractRecord preserves id field as-is`() {
        val record = mapOf(
            "id" to "user_123",
            "name" to mapOf("type" to "String", "value" to "John")
        )
        val result = extractRecord(record)
        assertEquals("user_123", result["id"])
    }

    @Test
    fun `extractRecord handles empty record`() {
        val result = extractRecord(emptyMap())
        assertTrue(result.isEmpty())
    }

    @Test
    fun `extractRecord handles nested objects`() {
        val record = mapOf(
            "user" to mapOf("type" to "Object", "value" to mapOf("name" to "John", "role" to "admin")),
            "tags" to mapOf("type" to "Array", "value" to listOf("python", "rust"))
        )
        val result = extractRecord(record)
        assertEquals(mapOf("name" to "John", "role" to "admin"), result["user"])
        assertEquals(listOf("python", "rust"), result["tags"])
    }

    // ========================================================================
    // getRecordId Tests
    // ========================================================================

    @Test
    fun `getRecordId extracts string id`() {
        val record = mapOf("id" to "user_123", "name" to "John")
        assertEquals("user_123", getRecordId(record))
    }

    @Test
    fun `getRecordId extracts FieldType StringValue id`() {
        val record = mapOf<String, Any?>("id" to FieldType.StringValue("user_456"), "name" to "John")
        assertEquals("user_456", getRecordId(record))
    }

    @Test
    fun `getRecordId extracts wrapped id`() {
        val record = mapOf("id" to mapOf("type" to "String", "value" to "user_789"))
        assertEquals("user_789", getRecordId(record))
    }

    @Test
    fun `getRecordId returns null for missing id`() {
        val record = mapOf("name" to "John")
        assertNull(getRecordId(record))
    }
}
