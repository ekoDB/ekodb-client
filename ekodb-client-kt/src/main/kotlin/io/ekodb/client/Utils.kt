package io.ekodb.client

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
 * @param field The field value from ekoDB (may be Map with type/value or a plain value)
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
    
    // Try to extract from map structure
    if (field is Map<*, *>) {
        val value = field["value"]
        return value as? T
    }
    
    return field as? T
}

/**
 * Extract a string value from an ekoDB field
 */
fun getStringValue(field: Any?): String? = getValue<String>(field)

/**
 * Extract an integer value from an ekoDB field
 */
fun getIntValue(field: Any?): Int? = getValue<Int>(field)

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
        return try {
            java.text.SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss").parse(val_)
        } catch (e: Exception) {
            null
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
