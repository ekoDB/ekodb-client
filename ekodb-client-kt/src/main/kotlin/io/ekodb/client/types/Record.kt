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
 * Serializes with flattened fields (no "fields" wrapper) to match server's serde(flatten)
 */
@Serializable(with = RecordSerializer::class)
data class Record(
    val fields: MutableMap<String, FieldType> = mutableMapOf()
) {
    operator fun get(key: String): FieldType? = fields[key]
    
    operator fun set(key: String, value: FieldType) {
        fields[key] = value
    }
    
    fun insert(key: String, value: String) = apply { fields[key] = FieldType.string(value) }
    fun insert(key: String, value: Long) = apply { fields[key] = FieldType.integer(value) }
    fun insert(key: String, value: Int) = apply { fields[key] = FieldType.integer(value.toLong()) }
    fun insert(key: String, value: Double) = apply { fields[key] = FieldType.float(value) }
    fun insert(key: String, value: Boolean) = apply { fields[key] = FieldType.boolean(value) }
    fun insert(key: String, value: FieldType) = apply { fields[key] = value }
    
    fun remove(key: String): FieldType? = fields.remove(key)
    
    fun containsKey(key: String): Boolean = fields.containsKey(key)
    
    fun size(): Int = fields.size
    
    fun isEmpty(): Boolean = fields.isEmpty()
    
    /**
     * Set TTL duration for this record
     * Duration format: "30s", "5m", "1h", "1d"
     */
    fun withTtl(duration: String) = apply {
        fields["ttl_duration"] = FieldType.string(duration)
    }
    
    /**
     * Set TTL with update-on-access behavior
     */
    fun withTtlUpdateOnAccess(duration: String, updateOnAccess: Boolean) = apply {
        fields["ttl_duration"] = FieldType.string(duration)
        fields["ttl_update_on_access"] = FieldType.boolean(updateOnAccess)
    }
    
    companion object {
        fun new() = Record()
    }
}

object RecordSerializer : KSerializer<Record> {
    private val mapSerializer = MapSerializer(String.serializer(), FieldType.serializer())
    
    override val descriptor: SerialDescriptor = mapSerializer.descriptor
    
    override fun serialize(encoder: Encoder, value: Record) {
        // Serialize fields directly (flattened) without "fields" wrapper
        encoder.encodeSerializableValue(mapSerializer, value.fields)
    }
    
    override fun deserialize(decoder: Decoder): Record {
        val map = decoder.decodeSerializableValue(mapSerializer)
        return Record(map.toMutableMap())
    }
}
