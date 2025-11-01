package io.ekodb.client.types

import kotlinx.datetime.Instant
import kotlinx.serialization.*
import kotlinx.serialization.builtins.*
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
        // Format-agnostic serialization - works with both JSON and CBOR
        when (value) {
            is FieldType.StringValue -> encoder.encodeString(value.value)
            is FieldType.IntegerValue -> encoder.encodeLong(value.value)
            is FieldType.FloatValue -> encoder.encodeDouble(value.value)
            is FieldType.BooleanValue -> encoder.encodeBoolean(value.value)
            is FieldType.ObjectValue -> {
                val mapSerializer: KSerializer<Map<String, FieldType>> = kotlinx.serialization.builtins.MapSerializer(
                    String.serializer(),
                    FieldTypeSerializer
                )
                encoder.encodeSerializableValue(mapSerializer, value.value)
            }
            is FieldType.ArrayValue, is FieldType.SetValue -> {
                val list = when (value) {
                    is FieldType.ArrayValue -> value.value
                    is FieldType.SetValue -> value.value
                    else -> emptyList()
                }
                val listSerializer: KSerializer<List<FieldType>> = kotlinx.serialization.builtins.ListSerializer(FieldTypeSerializer)
                encoder.encodeSerializableValue(listSerializer, list)
            }
            is FieldType.VectorValue -> {
                val listSerializer: KSerializer<List<Double>> = kotlinx.serialization.builtins.ListSerializer(Double.serializer())
                encoder.encodeSerializableValue(listSerializer, value.value)
            }
            is FieldType.DateTimeValue -> encoder.encodeString(value.value)
            is FieldType.UUIDValue -> encoder.encodeString(value.value)
            is FieldType.DecimalValue -> encoder.encodeString(value.value)
            is FieldType.BinaryValue -> {
                // Encode as array of integers for compatibility
                val listSerializer: KSerializer<List<Int>> = kotlinx.serialization.builtins.ListSerializer(Int.serializer())
                encoder.encodeSerializableValue(listSerializer, value.value.map { it.toInt() })
            }
            is FieldType.NullValue -> encoder.encodeNull()
        }
    }
    
    override fun deserialize(decoder: Decoder): FieldType {
        // For deserialization, we need to peek at the structure
        // Since we can't easily do that in a format-agnostic way, we'll use JsonDecoder when available
        return if (decoder is JsonDecoder) {
            val element = decoder.decodeJsonElement()
            deserializeElement(element)
        } else {
            // For CBOR and other formats, try to decode as a dynamic structure
            // This is a simplified approach - in practice, CBOR responses should be JSON
            decoder.decodeString().let { FieldType.StringValue(it) }
        }
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
