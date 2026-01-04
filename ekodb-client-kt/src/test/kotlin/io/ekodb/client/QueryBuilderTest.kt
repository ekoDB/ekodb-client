package io.ekodb.client

import kotlinx.serialization.json.*
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Unit tests for ekoDB Kotlin client QueryBuilder
 */
class QueryBuilderTest {

    // ========================================================================
    // Basic Tests
    // ========================================================================

    @Test
    fun `creates empty query builder`() {
        val qb = QueryBuilder()
        assertNotNull(qb)
    }

    @Test
    fun `builds empty query`() {
        val query = QueryBuilder().build()
        assertNull(query.filter)
        assertNull(query.sort)
        assertNull(query.limit)
        assertNull(query.skip)
    }

    @Test
    fun `companion new creates instance`() {
        val qb = QueryBuilder.new()
        assertNotNull(qb)
    }

    // ========================================================================
    // Equality Operators Tests
    // ========================================================================

    @Test
    fun `builds eq filter`() {
        val query = QueryBuilder()
            .eq("status", "active")
            .build()

        assertNotNull(query.filter)
        val filter = query.filter as JsonObject
        assertEquals("Condition", filter["type"]?.jsonPrimitive?.content)
        val content = filter["content"]?.jsonObject
        assertEquals("status", content?.get("field")?.jsonPrimitive?.content)
        assertEquals("Eq", content?.get("operator")?.jsonPrimitive?.content)
        assertEquals("active", content?.get("value")?.jsonPrimitive?.content)
    }

    @Test
    fun `builds ne filter`() {
        val query = QueryBuilder()
            .ne("status", "deleted")
            .build()

        assertNotNull(query.filter)
        val filter = query.filter as JsonObject
        val content = filter["content"]?.jsonObject
        assertEquals("Ne", content?.get("operator")?.jsonPrimitive?.content)
    }

    // ========================================================================
    // Comparison Operators Tests
    // ========================================================================

    @Test
    fun `builds gt filter`() {
        val query = QueryBuilder()
            .gt("age", 18)
            .build()

        assertNotNull(query.filter)
        val filter = query.filter as JsonObject
        val content = filter["content"]?.jsonObject
        assertEquals("Gt", content?.get("operator")?.jsonPrimitive?.content)
        assertEquals(18, content?.get("value")?.jsonPrimitive?.int)
    }

    @Test
    fun `builds gte filter`() {
        val query = QueryBuilder()
            .gte("score", 80)
            .build()

        val filter = query.filter as JsonObject
        val content = filter["content"]?.jsonObject
        assertEquals("Gte", content?.get("operator")?.jsonPrimitive?.content)
    }

    @Test
    fun `builds lt filter`() {
        val query = QueryBuilder()
            .lt("price", 100.5)
            .build()

        val filter = query.filter as JsonObject
        val content = filter["content"]?.jsonObject
        assertEquals("Lt", content?.get("operator")?.jsonPrimitive?.content)
    }

    @Test
    fun `builds lte filter`() {
        val query = QueryBuilder()
            .lte("count", 1000)
            .build()

        val filter = query.filter as JsonObject
        val content = filter["content"]?.jsonObject
        assertEquals("Lte", content?.get("operator")?.jsonPrimitive?.content)
    }

    // ========================================================================
    // Array Operators Tests
    // ========================================================================

    @Test
    fun `builds inArray filter`() {
        val query = QueryBuilder()
            .inArray("status", listOf("active", "pending"))
            .build()

        val filter = query.filter as JsonObject
        val content = filter["content"]?.jsonObject
        assertEquals("In", content?.get("operator")?.jsonPrimitive?.content)
        val values = content?.get("value")?.jsonArray
        assertEquals(2, values?.size)
    }

    @Test
    fun `builds nin filter`() {
        val query = QueryBuilder()
            .nin("role", listOf("blocked", "deleted"))
            .build()

        val filter = query.filter as JsonObject
        val content = filter["content"]?.jsonObject
        assertEquals("NotIn", content?.get("operator")?.jsonPrimitive?.content)
    }

    // ========================================================================
    // String Operators Tests
    // ========================================================================

    @Test
    fun `builds contains filter`() {
        val query = QueryBuilder()
            .contains("email", "@example.com")
            .build()

        val filter = query.filter as JsonObject
        val content = filter["content"]?.jsonObject
        assertEquals("Contains", content?.get("operator")?.jsonPrimitive?.content)
        assertEquals("@example.com", content?.get("value")?.jsonPrimitive?.content)
    }

    @Test
    fun `builds regex filter`() {
        val query = QueryBuilder()
            .regex("phone", "^\\+1")
            .build()

        val filter = query.filter as JsonObject
        val content = filter["content"]?.jsonObject
        assertEquals("Regex", content?.get("operator")?.jsonPrimitive?.content)
    }

    // ========================================================================
    // Multiple Filters (Auto AND) Tests
    // ========================================================================

    @Test
    fun `combines multiple filters with AND logic`() {
        val query = QueryBuilder()
            .eq("status", "active")
            .gt("age", 18)
            .contains("email", "@company.com")
            .build()

        val filter = query.filter as JsonObject
        assertEquals("Logical", filter["type"]?.jsonPrimitive?.content)
        val content = filter["content"]?.jsonObject
        assertEquals("And", content?.get("operator")?.jsonPrimitive?.content)
        val expressions = content?.get("expressions")?.jsonArray
        assertEquals(3, expressions?.size)
    }

    // ========================================================================
    // Sorting Tests
    // ========================================================================

    @Test
    fun `builds ascending sort`() {
        val query = QueryBuilder()
            .sortAsc("name")
            .build()

        assertNotNull(query.sort)
        val sort = query.sort as JsonArray
        assertEquals(1, sort.size)
        val sortField = sort[0].jsonObject
        assertEquals("name", sortField["field"]?.jsonPrimitive?.content)
        assertEquals(true, sortField["ascending"]?.jsonPrimitive?.boolean)
    }

    @Test
    fun `builds descending sort`() {
        val query = QueryBuilder()
            .sortDesc("created_at")
            .build()

        val sort = query.sort as JsonArray
        val sortField = sort[0].jsonObject
        assertEquals(false, sortField["ascending"]?.jsonPrimitive?.boolean)
    }

    @Test
    fun `builds multiple sorts`() {
        val query = QueryBuilder()
            .sortDesc("created_at")
            .sortAsc("name")
            .build()

        val sort = query.sort as JsonArray
        assertEquals(2, sort.size)
        assertEquals("created_at", sort[0].jsonObject["field"]?.jsonPrimitive?.content)
        assertEquals(false, sort[0].jsonObject["ascending"]?.jsonPrimitive?.boolean)
        assertEquals("name", sort[1].jsonObject["field"]?.jsonPrimitive?.content)
        assertEquals(true, sort[1].jsonObject["ascending"]?.jsonPrimitive?.boolean)
    }

    @Test
    fun `sortAscending is alias for sortAsc`() {
        val query = QueryBuilder()
            .sortAscending("name")
            .build()

        val sort = query.sort as JsonArray
        assertEquals(true, sort[0].jsonObject["ascending"]?.jsonPrimitive?.boolean)
    }

    @Test
    fun `sortDescending is alias for sortDesc`() {
        val query = QueryBuilder()
            .sortDescending("name")
            .build()

        val sort = query.sort as JsonArray
        assertEquals(false, sort[0].jsonObject["ascending"]?.jsonPrimitive?.boolean)
    }

    // ========================================================================
    // Pagination Tests
    // ========================================================================

    @Test
    fun `builds limit`() {
        val query = QueryBuilder()
            .limit(10)
            .build()

        assertEquals(10, query.limit)
    }

    @Test
    fun `builds skip`() {
        val query = QueryBuilder()
            .skip(20)
            .build()

        assertEquals(20, query.skip)
    }

    @Test
    fun `builds limit and skip together`() {
        val query = QueryBuilder()
            .limit(10)
            .skip(20)
            .build()

        assertEquals(10, query.limit)
        assertEquals(20, query.skip)
    }

    // ========================================================================
    // Join Tests
    // ========================================================================

    @Test
    fun `builds join configuration with map`() {
        val joinConfig = mapOf(
            "collection" to "users",
            "local_field" to "user_id",
            "foreign_field" to "id",
            "as" to "user"
        )

        val query = QueryBuilder()
            .join(joinConfig)
            .build()

        assertNotNull(query.join)
    }

    @Test
    fun `builds join with joinWith helper`() {
        val query = QueryBuilder()
            .joinWith(
                collection = "users",
                localField = "user_id",
                foreignField = "id",
                alias = "user_data"
            )
            .build()

        assertNotNull(query.join)
        val join = query.join as JsonObject
        assertEquals("users", join["collection"]?.jsonPrimitive?.content)
        assertEquals("user_id", join["local_field"]?.jsonPrimitive?.content)
        assertEquals("id", join["foreign_field"]?.jsonPrimitive?.content)
        assertEquals("user_data", join["as"]?.jsonPrimitive?.content)
    }

    @Test
    fun `joinWith uses collection as default alias`() {
        val query = QueryBuilder()
            .joinWith(
                collection = "users",
                localField = "user_id",
                foreignField = "id"
            )
            .build()

        val join = query.join as JsonObject
        assertEquals("users", join["as"]?.jsonPrimitive?.content)
    }

    // ========================================================================
    // Bypass Flags Tests
    // ========================================================================

    @Test
    fun `builds bypass_cache true`() {
        val query = QueryBuilder()
            .bypassCache(true)
            .build()

        assertEquals(true, query.bypassCache)
    }

    @Test
    fun `builds bypass_cache default true`() {
        val query = QueryBuilder()
            .bypassCache()
            .build()

        assertEquals(true, query.bypassCache)
    }

    @Test
    fun `builds bypass_cache false`() {
        val query = QueryBuilder()
            .bypassCache(false)
            .build()

        assertEquals(false, query.bypassCache)
    }

    @Test
    fun `builds bypass_ripple`() {
        val query = QueryBuilder()
            .bypassRipple(true)
            .build()

        assertEquals(true, query.bypassRipple)
    }

    // ========================================================================
    // Chaining Tests
    // ========================================================================

    @Test
    fun `supports full method chaining`() {
        val query = QueryBuilder()
            .eq("status", "active")
            .gt("age", 18)
            .sortDesc("created_at")
            .sortAsc("name")
            .limit(10)
            .skip(20)
            .bypassCache(true)
            .build()

        // Check filter exists
        assertNotNull(query.filter)

        // Check sort exists
        val sort = query.sort as JsonArray
        assertEquals(2, sort.size)

        // Check pagination
        assertEquals(10, query.limit)
        assertEquals(20, query.skip)

        // Check bypass flag
        assertEquals(true, query.bypassCache)
    }

    @Test
    fun `returns this for method chaining`() {
        val qb = QueryBuilder()

        assertTrue(qb.eq("a", 1) === qb)
        assertTrue(qb.ne("b", 2) === qb)
        assertTrue(qb.gt("c", 3) === qb)
        assertTrue(qb.gte("d", 4) === qb)
        assertTrue(qb.lt("e", 5) === qb)
        assertTrue(qb.lte("f", 6) === qb)
        assertTrue(qb.inArray("g", listOf(7)) === qb)
        assertTrue(qb.nin("h", listOf(8)) === qb)
        assertTrue(qb.contains("i", "j") === qb)
        assertTrue(qb.regex("k", "l") === qb)
        assertTrue(qb.sortAsc("m") === qb)
        assertTrue(qb.sortDesc("n") === qb)
        assertTrue(qb.limit(1) === qb)
        assertTrue(qb.skip(0) === qb)
        assertTrue(qb.bypassCache() === qb)
        assertTrue(qb.bypassRipple() === qb)
    }

    // ========================================================================
    // SortOrder Enum Tests
    // ========================================================================

    @Test
    fun `SortOrder has correct values`() {
        assertEquals(SortOrder.ASC, SortOrder.valueOf("ASC"))
        assertEquals(SortOrder.DESC, SortOrder.valueOf("DESC"))
    }
}
