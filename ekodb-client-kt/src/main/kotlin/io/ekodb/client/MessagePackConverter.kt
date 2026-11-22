package io.ekodb.client

import io.ktor.http.ContentType
import io.ktor.http.content.ByteArrayContent
import io.ktor.http.content.OutgoingContent
import io.ktor.serialization.ContentConverter
import io.ktor.util.reflect.TypeInfo
import io.ktor.utils.io.ByteReadChannel
import io.ktor.utils.io.toByteArray
import kotlinx.serialization.KSerializer
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.serializer
import java.nio.charset.Charset

/** The content type the server reads and writes MessagePack bodies under. */
internal val MessagePackContentType = ContentType("application", "msgpack")

/**
 * Content negotiation for `application/msgpack`, registered when a client uses
 * [SerializationFormat.MESSAGEPACK] (#418).
 *
 * A body is encoded with its kotlinx serializer to the same [kotlinx.serialization.json.JsonElement]
 * the JSON path sends, then packed with msgpack-core ([jsonElementToMsgpack]). A reply is unpacked
 * ([msgpackToJsonElement]) and decoded with the settings the JSON path's content negotiation
 * uses, so a reply carrying a field the client does not know is read the same way in both
 * formats.
 */
internal class MessagePackConverter(
    private val json: Json = Json {
        isLenient = true
        ignoreUnknownKeys = true
    },
) : ContentConverter {
    override suspend fun serialize(
        contentType: ContentType,
        charset: Charset,
        typeInfo: TypeInfo,
        value: Any?,
    ): OutgoingContent {
        // A body that already is a JsonElement is packed as it is: encoding it through the serializer
        // rewrites an integer literal beyond the unsigned 64-bit range as a double.
        @Suppress("UNCHECKED_CAST")
        val element = if (value is JsonElement) {
            value
        } else {
            json.encodeToJsonElement(serializerFor(typeInfo) as KSerializer<Any?>, value)
        }
        return ByteArrayContent(jsonElementToMsgpack(element), contentType)
    }

    override suspend fun deserialize(charset: Charset, typeInfo: TypeInfo, content: ByteReadChannel): Any? =
        json.decodeFromJsonElement(serializerFor(typeInfo), msgpackToJsonElement(content.toByteArray()))

    private fun serializerFor(typeInfo: TypeInfo): KSerializer<*> =
        typeInfo.kotlinType?.let { json.serializersModule.serializer(it) }
            ?: json.serializersModule.serializer(typeInfo.type.java)
}
