package io.ekodb.client.types

import kotlinx.datetime.Instant
import kotlinx.serialization.*
import kotlinx.serialization.descriptors.*
import kotlinx.serialization.encoding.*
import kotlinx.serialization.json.*
import java.math.BigDecimal
import java.util.UUID

/**
 * Field type representing all supported data types in ekoDB
 * Serializes as raw JSON values (untagged) to match server format
 */
@Serializable(with = FieldTypeSerializer::class)
sealed class FieldType {
    data class StringValue(val value: String) : FieldType()
    data class IntegerValue(val value: Long) : FieldType()
    data class FloatValue(val value: Double) : FieldType()
    data class BooleanValue(val value: Boolean) : FieldType()
    data class ObjectValue(val value: Map<String, FieldType>) : FieldType()
    data class ArrayValue(val value: List<FieldType>) : FieldType()
    data class SetValue(val value: List<FieldType>) : FieldType()
    data class VectorValue(val value: List<Double>) : FieldType()
    data class DateTimeValue(val value: String) : FieldType()
    data class UUIDValue(val value: String) : FieldType()
    data class DecimalValue(val value: String) : FieldType()
    data class BinaryValue(val value: ByteArray) : FieldType() {
        override fun equals(other: Any?): Boolean {
            if (this === other) return true
            if (other !is BinaryValue) return false
            return value.contentEquals(other.value)
        }
        
        override fun hashCode(): Int = value.contentHashCode()
    }
    object NullValue : FieldType()
    
    companion object {
        fun string(value: String) = StringValue(value)
        fun integer(value: Long) = IntegerValue(value)
        fun float(value: Double) = FloatValue(value)
        fun boolean(value: Boolean) = BooleanValue(value)
        fun obj(value: Map<String, FieldType>) = ObjectValue(value)
        fun array(value: List<FieldType>) = ArrayValue(value)
        fun set(value: List<FieldType>) = SetValue(value)
        fun vector(value: List<Double>) = VectorValue(value)
        fun dateTime(value: Instant) = DateTimeValue(value.toString())
        fun uuid(value: UUID) = UUIDValue(value.toString())
        fun decimal(value: BigDecimal) = DecimalValue(value.toString())
        fun binary(value: ByteArray) = BinaryValue(value)
        fun nullValue() = NullValue
    }
}

object FieldTypeSerializer : KSerializer<FieldType> {
    override val descriptor: SerialDescriptor = buildClassSerialDescriptor("FieldType")
    
    override fun serialize(encoder: Encoder, value: FieldType) {
        require(encoder is JsonEncoder)
        // Serialize as raw JSON values (untagged) to match server's serde(untagged) format
        val element = when (value) {
            is FieldType.StringValue -> JsonPrimitive(value.value)
            is FieldType.IntegerValue -> JsonPrimitive(value.value)
            is FieldType.FloatValue -> JsonPrimitive(value.value)
            is FieldType.BooleanValue -> JsonPrimitive(value.value)
            is FieldType.ObjectValue -> JsonObject(value.value.mapValues { encoder.json.encodeToJsonElement(it.value) })
            is FieldType.ArrayValue -> JsonArray(value.value.map { encoder.json.encodeToJsonElement(it) })
            is FieldType.SetValue -> JsonArray(value.value.map { encoder.json.encodeToJsonElement(it) })
            is FieldType.VectorValue -> JsonArray(value.value.map { JsonPrimitive(it) })
            is FieldType.DateTimeValue -> JsonPrimitive(value.value)
            is FieldType.UUIDValue -> JsonPrimitive(value.value)
            is FieldType.DecimalValue -> JsonPrimitive(value.value)
            is FieldType.BinaryValue -> JsonArray(value.value.map { JsonPrimitive(it.toInt()) })
            is FieldType.NullValue -> JsonNull
        }
        encoder.encodeJsonElement(element)
    }
    
    override fun deserialize(decoder: Decoder): FieldType {
        require(decoder is JsonDecoder)
        val element = decoder.decodeJsonElement()
        return deserializeElement(element)
    }
    
    private fun deserializeElement(element: JsonElement): FieldType {
        return when (element) {
            is JsonNull -> FieldType.NullValue
            is JsonPrimitive -> when {
                element.isString -> FieldType.StringValue(element.content)
                element.booleanOrNull != null -> FieldType.BooleanValue(element.boolean)
                element.longOrNull != null -> FieldType.IntegerValue(element.long)
                element.doubleOrNull != null -> FieldType.FloatValue(element.double)
                else -> error("Unknown primitive type: $element")
            }
            is JsonArray -> FieldType.ArrayValue(element.map { deserializeElement(it) })
            is JsonObject -> FieldType.ObjectValue(element.mapValues { deserializeElement(it.value) })
            else -> error("Unknown FieldType: $element")
        }
    }
}
