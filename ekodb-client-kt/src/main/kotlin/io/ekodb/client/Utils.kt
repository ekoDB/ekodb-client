package io.ekodb.client

import io.ekodb.client.types.FieldType

/**
 * Utility functions for working with ekoDB records
 */

/**
 * Represents an ekoDB field with type and value
 */
data class FieldValue(
    val type: String,
    val value: Any?
)

/**
 * Extract the value from an ekoDB field object.
 * ekoDB returns fields as {"type": "String", "value": "..."} objects.
 * This helper safely extracts the value or returns the input if it's not a field object.
 *
 * @param field The field value from ekoDB (may be FieldType, Map with type/value, or a plain value)
 * @return The extracted value
 *
 * Example:
 * ```kotlin
 * val user = client.findByID("users", userId)
 * val email = getValue<String>(user["email"])
 * val age = getValue<Int>(user["age"])
 * ```
 */
inline fun <reified T> getValue(field: Any?): T? {
    if (field == null) return null
    
    // Handle FieldType sealed class instances
    if (field is FieldType) {
        val extracted = extractFieldTypeValue(field)
        return convertValue<T>(extracted)
    }
    
    // Try to extract from map structure
    if (field is Map<*, *>) {
        val value = field["value"]
        return convertValue<T>(value)
    }
    
    return convertValue<T>(field)
}

/**
 * Convert a value to the target type, handling numeric conversions
 */
inline fun <reified T> convertValue(value: Any?): T? {
    if (value == null) return null
    if (value is T) return value
    
    // Handle numeric conversions
    if (value is Number) {
        return when (T::class) {
            Int::class -> value.toInt() as T
            Long::class -> value.toLong() as T
            Double::class -> value.toDouble() as T
            Float::class -> value.toFloat() as T
            else -> value as? T
        }
    }
    
    return value as? T
}

/**
 * Extract the raw value from a FieldType.
 * Handles nested ObjectValue containing {type, value} structure from server responses.
 */
fun extractFieldTypeValue(field: FieldType): Any? {
    return when (field) {
        is FieldType.StringValue -> field.value
        is FieldType.IntegerValue -> field.value
        is FieldType.FloatValue -> field.value
        is FieldType.BooleanValue -> field.value
        is FieldType.ObjectValue -> {
            // Check if this is a wrapped type from server: {type: "String", value: "..."}
            val typeField = field.value["type"]
            val valueField = field.value["value"]
            if (typeField != null && valueField != null) {
                // This is a wrapped type, extract the inner value
                extractFieldTypeValue(valueField)
            } else {
                // Regular object, convert to Map<String, Any?>
                field.value.mapValues { (_, v) -> extractFieldTypeValue(v) }
            }
        }
        is FieldType.ArrayValue -> field.value.map { extractFieldTypeValue(it) }
        is FieldType.SetValue -> field.value.map { extractFieldTypeValue(it) }
        is FieldType.VectorValue -> field.value
        is FieldType.DateTimeValue -> field.value
        is FieldType.UUIDValue -> field.value
        is FieldType.DecimalValue -> field.value.toDoubleOrNull() ?: field.value
        is FieldType.BinaryValue -> field.value
        is FieldType.NullValue -> null
    }
}

/**
 * Extract a string value from an ekoDB field
 */
fun getStringValue(field: Any?): String? = getValue<String>(field)

/**
 * Extract an integer value from an ekoDB field
 */
fun getIntValue(field: Any?): Int? {
    val val_ = getValue<Any>(field) ?: return null
    return when (val_) {
        is Int -> val_
        is Long -> val_.toInt()
        is Number -> val_.toInt()
        else -> null
    }
}

/**
 * Extract a long value from an ekoDB field
 */
fun getLongValue(field: Any?): Long? = getValue<Long>(field)

/**
 * Extract a double value from an ekoDB field
 */
fun getDoubleValue(field: Any?): Double? = getValue<Double>(field)

/**
 * Extract a boolean value from an ekoDB field
 */
fun getBooleanValue(field: Any?): Boolean? = getValue<Boolean>(field)

/**
 * Extract values from multiple fields in a record.
 * Useful for processing entire records returned from ekoDB.
 *
 * @param record The record object from ekoDB
 * @param fields List of field names to extract values from
 * @return Map with extracted values
 *
 * Example:
 * ```kotlin
 * val user = client.findByID("users", userId)
 * val values = getValues(user, listOf("email", "first_name", "status"))
 * val email = values["email"] as? String
 * ```
 */
fun getValues(record: Map<String, Any?>, fields: List<String>): Map<String, Any?> {
    val result = mutableMapOf<String, Any?>()
    for (field in fields) {
        record[field]?.let { value ->
            result[field] = getValue<Any>(value)
        }
    }
    return result
}

/**
 * Extract a Date value from an ekoDB DateTime field
 */
fun getDateTimeValue(field: Any?): java.util.Date? {
    val val_= getValue<Any>(field) ?: return null
    if (val_ is java.util.Date) return val_
    if (val_ is String) {
        // Try multiple date formats
        val formats = listOf(
            "yyyy-MM-dd'T'HH:mm:ss.SSSSSS'Z'",
            "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'",
            "yyyy-MM-dd'T'HH:mm:ss'Z'",
            "yyyy-MM-dd'T'HH:mm:ss",
            "EEE MMM dd HH:mm:ss zzz yyyy"  // Java Date.toString() format
        )
        for (format in formats) {
            try {
                val sdf = java.text.SimpleDateFormat(format, java.util.Locale.US)
                sdf.timeZone = java.util.TimeZone.getTimeZone("UTC")
                return sdf.parse(val_)
            } catch (e: Exception) {
                // Try next format
            }
        }
    }
    return null
}

/**
 * Extract a UUID string from an ekoDB UUID field
 */
fun getUUIDValue(field: Any?): String? = getValue<String>(field)

/**
 * Extract a Double from an ekoDB Decimal field
 */
fun getDecimalValue(field: Any?): Double? {
    val val_ = getValue<Any>(field) ?: return null
    return when (val_) {
        is Double -> val_
        is Int -> val_.toDouble()
        is Long -> val_.toDouble()
        is Float -> val_.toDouble()
        is String -> val_.toDoubleOrNull()
        else -> null
    }
}

/**
 * Extract a Long (milliseconds) from an ekoDB Duration field
 */
fun getDurationValue(field: Any?): Long? {
    val val_ = getValue<Any>(field) ?: return null
    return when (val_) {
        is Long -> val_
        is Int -> val_.toLong()
        is Double -> val_.toLong()
        is Map<*, *> -> {
            val secs = (val_["secs"] as? Number)?.toLong() ?: 0L
            val nanos = (val_["nanos"] as? Number)?.toLong() ?: 0L
            secs * 1000 + nanos / 1_000_000
        }
        else -> null
    }
}

/**
 * Extract a ByteArray from an ekoDB Bytes field
 */
fun getBytesValue(field: Any?): ByteArray? {
    val val_ = getValue<Any>(field) ?: return null
    return when (val_) {
        is ByteArray -> val_
        is List<*> -> val_.mapNotNull { (it as? Number)?.toByte() }.toByteArray()
        is String -> {
            try {
                java.util.Base64.getDecoder().decode(val_)
            } catch (e: Exception) {
                null
            }
        }
        else -> null
    }
}

/**
 * Extract a ByteArray from an ekoDB Binary field
 */
fun getBinaryValue(field: Any?): ByteArray? = getBytesValue(field)

/**
 * Extract a List from an ekoDB Array field
 */
fun getArrayValue(field: Any?): List<Any?>? {
    val val_ = getValue<Any>(field) ?: return null
    return if (val_ is List<*>) val_ else null
}

/**
 * Extract a List from an ekoDB Set field
 */
fun getSetValue(field: Any?): List<Any?>? {
    val val_ = getValue<Any>(field) ?: return null
    return if (val_ is List<*>) val_ else null
}

/**
 * Extract a List of Doubles from an ekoDB Vector field
 */
fun getVectorValue(field: Any?): List<Double>? {
    val val_ = getValue<Any>(field) ?: return null
    return if (val_ is List<*>) {
        val_.mapNotNull { 
            when (it) {
                is Double -> it
                is Number -> it.toDouble()
                else -> null
            }
        }
    } else null
}

/**
 * Extract a Map from an ekoDB Object field
 */
fun getObjectValue(field: Any?): Map<String, Any?>? {
    val val_ = getValue<Any>(field) ?: return null
    @Suppress("UNCHECKED_CAST")
    return if (val_ is Map<*, *>) val_ as? Map<String, Any?> else null
}

/**
 * Extract record ID from a record.
 * Handles both FieldType.StringValue and plain string IDs.
 *
 * @param record The record object from ekoDB
 * @return The extracted ID string, or null if not found
 *
 * Example:
 * ```kotlin
 * val records = client.batchInsert(collection, data)
 * val ids = records.successful.mapNotNull { getRecordId(it) }
 * ```
 */
fun getRecordId(record: Map<String, Any?>): String? {
    val idField = record["id"] ?: return null
    
    // Handle FieldType.StringValue from Kotlin client
    if (idField is io.ekodb.client.types.FieldType.StringValue) {
        return idField.value
    }
    
    // Handle direct string or map structure
    return getStringValue(idField)
}

/**
 * Transform an entire record by extracting all field values.
 * Preserves the 'id' field and extracts values from all other fields.
 *
 * @param record The record object from ekoDB
 * @return Map with all field values extracted
 *
 * Example:
 * ```kotlin
 * val user = client.findByID("users", userId)
 * val plainUser = extractRecord(user)
 * // plainUser is now Map<String, Any?> with plain values
 * ```
 */
fun extractRecord(record: Map<String, Any?>): Map<String, Any?> {
    val result = mutableMapOf<String, Any?>()
    for ((key, value) in record) {
        result[key] = if (key == "id") {
            value
        } else {
            getValue<Any>(value)
        }
    }
    return result
}

// =============================================================================
// Wrapped Type Builders
// =============================================================================
// These functions create wrapped type objects for sending to ekoDB.
// Use these when inserting/updating records with special field types.
//
// Example:
//   client.insert("orders", mapOf(
//       "id" to fieldUUID("550e8400-e29b-41d4-a716-446655440000"),
//       "total" to fieldDecimal("99.99"),
//       "created_at" to fieldDateTime(Date()),
//       "tags" to fieldSet(listOf("sale", "featured")),
//   ))

/**
 * Create a UUID field value
 */
fun fieldUUID(value: String): Map<String, Any> = mapOf("type" to "UUID", "value" to value)

/**
 * Create a Decimal field value for precise numeric values
 */
fun fieldDecimal(value: String): Map<String, Any> = mapOf("type" to "Decimal", "value" to value)

/**
 * Create a DateTime field value from a Date
 */
fun fieldDateTime(value: java.util.Date): Map<String, Any> {
    val formatter = java.text.SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss'Z'")
    formatter.timeZone = java.util.TimeZone.getTimeZone("UTC")
    return mapOf("type" to "DateTime", "value" to formatter.format(value))
}

/**
 * Create a DateTime field value from an RFC3339 string
 */
fun fieldDateTimeString(value: String): Map<String, Any> = mapOf("type" to "DateTime", "value" to value)

/**
 * Create a Duration field value (in milliseconds)
 */
fun fieldDuration(milliseconds: Long): Map<String, Any> = mapOf("type" to "Duration", "value" to milliseconds)

/**
 * Create a Number field value (flexible numeric type)
 */
fun fieldNumber(value: Number): Map<String, Any> = mapOf("type" to "Number", "value" to value)

/**
 * Create a Set field value (unique elements)
 */
fun fieldSet(values: List<Any?>): Map<String, Any> = mapOf("type" to "Set", "value" to values)

/**
 * Create a Vector field value (for embeddings/similarity search)
 */
fun fieldVector(values: List<Double>): Map<String, Any> = mapOf("type" to "Vector", "value" to values)

/**
 * Create a Binary field value from bytes
 */
fun fieldBinary(value: ByteArray): Map<String, Any> = 
    mapOf("type" to "Binary", "value" to java.util.Base64.getEncoder().encodeToString(value))

/**
 * Create a Binary field value from a base64 string
 */
fun fieldBinaryBase64(value: String): Map<String, Any> = mapOf("type" to "Binary", "value" to value)

/**
 * Create a Bytes field value from bytes
 */
fun fieldBytes(value: ByteArray): Map<String, Any> = 
    mapOf("type" to "Bytes", "value" to java.util.Base64.getEncoder().encodeToString(value))

/**
 * Create a Bytes field value from a base64 string
 */
fun fieldBytesBase64(value: String): Map<String, Any> = mapOf("type" to "Bytes", "value" to value)

/**
 * Create an Array field value
 */
fun fieldArray(values: List<Any?>): Map<String, Any> = mapOf("type" to "Array", "value" to values)

/**
 * Create an Object field value
 */
fun fieldObject(value: Map<String, Any?>): Map<String, Any?> = mapOf("type" to "Object", "value" to value)

/**
 * Create a String field value (explicit wrapping)
 */
fun fieldString(value: String): Map<String, Any> = mapOf("type" to "String", "value" to value)

/**
 * Create an Integer field value (explicit wrapping)
 */
fun fieldInteger(value: Long): Map<String, Any> = mapOf("type" to "Integer", "value" to value)

/**
 * Create a Float field value (explicit wrapping)
 */
fun fieldFloat(value: Double): Map<String, Any> = mapOf("type" to "Float", "value" to value)

/**
 * Create a Boolean field value (explicit wrapping)
 */
fun fieldBoolean(value: Boolean): Map<String, Any> = mapOf("type" to "Boolean", "value" to value)
