package io.ekodb.client

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlin.test.assertFalse

class RateLimitInfoTest {

    @Test
    fun `basic construction`() {
        val info = RateLimitInfo(limit = 1000, remaining = 500, reset = 1234567890L)
        assertEquals(1000, info.limit)
        assertEquals(500, info.remaining)
        assertEquals(1234567890L, info.reset)
    }

    @Test
    fun `isNearLimit when plenty remaining`() {
        val info = RateLimitInfo(limit = 1000, remaining = 500, reset = 0L)
        assertFalse(info.isNearLimit())
    }

    @Test
    fun `isNearLimit when near threshold`() {
        val info = RateLimitInfo(limit = 1000, remaining = 100, reset = 0L)
        assertTrue(info.isNearLimit())
    }

    @Test
    fun `isNearLimit at exactly 10 percent`() {
        val info = RateLimitInfo(limit = 1000, remaining = 100, reset = 0L)
        assertTrue(info.isNearLimit())
    }

    @Test
    fun `isNearLimit below 10 percent`() {
        val info = RateLimitInfo(limit = 1000, remaining = 50, reset = 0L)
        assertTrue(info.isNearLimit())
    }

    @Test
    fun `isExceeded when zero remaining`() {
        val info = RateLimitInfo(limit = 1000, remaining = 0, reset = 0L)
        assertTrue(info.isExceeded())
    }

    @Test
    fun `isExceeded when remaining`() {
        val info = RateLimitInfo(limit = 1000, remaining = 1, reset = 0L)
        assertFalse(info.isExceeded())
    }

    @Test
    fun `remainingPercentage full`() {
        val info = RateLimitInfo(limit = 1000, remaining = 1000, reset = 0L)
        assertEquals(100.0, info.remainingPercentage(), 0.01)
    }

    @Test
    fun `remainingPercentage half`() {
        val info = RateLimitInfo(limit = 1000, remaining = 500, reset = 0L)
        assertEquals(50.0, info.remainingPercentage(), 0.01)
    }

    @Test
    fun `remainingPercentage zero`() {
        val info = RateLimitInfo(limit = 1000, remaining = 0, reset = 0L)
        assertEquals(0.0, info.remainingPercentage(), 0.01)
    }

    @Test
    fun `remainingPercentage with zero limit`() {
        val info = RateLimitInfo(limit = 0, remaining = 0, reset = 0L)
        assertEquals(0.0, info.remainingPercentage(), 0.01)
    }

    @Test
    fun `data class equality`() {
        val info1 = RateLimitInfo(1000, 500, 123L)
        val info2 = RateLimitInfo(1000, 500, 123L)
        assertEquals(info1, info2)
    }
}
