package io.ekodb.client.types

import kotlinx.serialization.*
import kotlinx.serialization.builtins.MapSerializer
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.*

/**
 * A record in ekoDB
 * Uses operator overloads so you can use record["key"] instead of record.fields["key"]
 */
@Serializable(with = RecordSerializer::class)
data class EkoRecord(
    private val fields: MutableMap<String, FieldType> = mutableMapOf()
) : MutableMap<String, FieldType> by fields {
    
    fun insert(key: String, value: String) = apply { fields[key] = FieldType.string(value) }
    fun insert(key: String, value: Long) = apply { fields[key] = FieldType.integer(value) }
    fun insert(key: String, value: Int) = apply { fields[key] = FieldType.integer(value.toLong()) }
    fun insert(key: String, value: Double) = apply { fields[key] = FieldType.float(value) }
    fun insert(key: String, value: Boolean) = apply { fields[key] = FieldType.boolean(value) }
    fun insert(key: String, value: FieldType) = apply { fields[key] = value }
    
    /**
     * Set TTL duration for this record
     * Supported formats: "30s", "5m", "1h", "1d", "2w", seconds ("3600"), or ISO8601
     */
    fun withTtl(duration: String) = apply {
        fields["ttl"] = FieldType.string(duration)
    }
    
    /**
     * Set TTL with update-on-access behavior
     */
    fun withTtlUpdateOnAccess(duration: String, updateOnAccess: Boolean) = apply {
        fields["ttl"] = FieldType.string(duration)
        fields["ttl_update_on_access"] = FieldType.boolean(updateOnAccess)
    }
    
    companion object {
        fun new() = EkoRecord()
    }
}

// Type alias for backward compatibility and cleaner API
typealias Record = EkoRecord

object RecordSerializer : KSerializer<EkoRecord> {
    private val mapSerializer = MapSerializer(String.serializer(), FieldType.serializer())
    
    override val descriptor: SerialDescriptor = mapSerializer.descriptor
    
    override fun serialize(encoder: Encoder, value: EkoRecord) {
        // Serialize the internal map directly (flattened)
        encoder.encodeSerializableValue(mapSerializer, value as MutableMap<String, FieldType>)
    }
    
    override fun deserialize(decoder: Decoder): EkoRecord {
        val map = decoder.decodeSerializableValue(mapSerializer)
        return EkoRecord(map.toMutableMap())
    }
}
