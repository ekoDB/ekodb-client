package io.ekodb.client

import io.ekodb.client.types.CollectionConfig
import io.ekodb.client.types.FieldSearchOptions
import io.ekodb.client.types.TextSearchOptions
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.encodeToJsonElement
import kotlin.test.Test
import kotlin.test.assertEquals

class FieldSearchOptionsTest {
    @Test
    fun `chat retrieval fields use field_name like Rust including nested search options`() {
        val config = CollectionConfig(
            collectionName = "articles",
            fields = listOf(FieldSearchOptions("title", TextSearchOptions(fuzzyMatch = true))),
        )
        val expected = Json.parseToJsonElement("""{"collection_name":"articles","fields":[{"field_name":"title","search_options":{"fuzzy_match":true}}]}""")
        assertEquals(expected, Json.encodeToJsonElement(config))
        assertEquals(config, Json.decodeFromString<CollectionConfig>(expected.toString()))
    }
}
