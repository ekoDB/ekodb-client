package io.ekodb.client

import kotlinx.serialization.json.*
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class JoinConfigTest {

    @Test
    fun `single collection join`() {
        val join = JoinConfig.single("users", "user_id", "id", "user")
        assertEquals(listOf("users"), join.collections)
        assertEquals("user_id", join.localField)
        assertEquals("id", join.foreignField)
        assertEquals("user", join.asField)
    }

    @Test
    fun `multi collection join`() {
        val join = JoinConfig.multi(
            listOf("users", "profiles"),
            "user_id", "id", "user_info"
        )
        assertEquals(2, join.collections.size)
        assertTrue(join.collections.contains("users"))
        assertTrue(join.collections.contains("profiles"))
    }

    @Test
    fun `toJsonObject serialization`() {
        val join = JoinConfig.single("users", "user_id", "id", "user")
        val json = join.toJsonObject()

        val collections = json["collections"]?.jsonArray
        assertEquals(1, collections?.size)
        assertEquals("users", collections?.get(0)?.jsonPrimitive?.content)
        assertEquals("user_id", json["local_field"]?.jsonPrimitive?.content)
        assertEquals("id", json["foreign_field"]?.jsonPrimitive?.content)
        assertEquals("user", json["as_field"]?.jsonPrimitive?.content)
    }

    @Test
    fun `toMap conversion`() {
        val join = JoinConfig.single("users", "user_id", "id", "user")
        val map = join.toMap()

        @Suppress("UNCHECKED_CAST")
        val collections = map["collections"] as List<String>
        assertEquals(listOf("users"), collections)
        assertEquals("user_id", map["local_field"])
        assertEquals("id", map["foreign_field"])
        assertEquals("user", map["as_field"])
    }

    @Test
    fun `multi collection toJsonObject`() {
        val join = JoinConfig.multi(
            listOf("users", "profiles", "settings"),
            "user_id", "id", "user_info"
        )
        val json = join.toJsonObject()
        val collections = json["collections"]?.jsonArray
        assertEquals(3, collections?.size)
    }

    @Test
    fun `data class equality`() {
        val join1 = JoinConfig.single("users", "user_id", "id", "user")
        val join2 = JoinConfig.single("users", "user_id", "id", "user")
        assertEquals(join1, join2)
    }

    @Test
    fun `data class copy`() {
        val join = JoinConfig.single("users", "user_id", "id", "user")
        val modified = join.copy(asField = "user_data")
        assertEquals("user_data", modified.asField)
        assertEquals("user", join.asField) // original unchanged
    }
}
