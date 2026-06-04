package io.ekodb.client

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Unit tests for the retry/backoff clamping (issue #142). The clamps keep a
 * hostile or misconfigured server response from sleeping the client for an
 * unbounded time, matching the Rust client's bounded retry policy.
 */
class RetryClampTest {

    @Test
    fun `clampRetryAfterSeconds bounds hostile and missing values`() {
        // Hostile/huge value is capped at the maximum.
        assertEquals(EkoDBClient.MAX_RETRY_AFTER_SECONDS, EkoDBClient.clampRetryAfterSeconds(99_999L))
        // Boundary value is preserved.
        assertEquals(60L, EkoDBClient.clampRetryAfterSeconds(60L))
        // A reasonable value passes through unchanged.
        assertEquals(5L, EkoDBClient.clampRetryAfterSeconds(5L))
        // Missing/unparseable header defaults to 1s.
        assertEquals(1L, EkoDBClient.clampRetryAfterSeconds(null))
        // Negative values clamp to 0 (never negative delay).
        assertEquals(0L, EkoDBClient.clampRetryAfterSeconds(-3L))
    }

    @Test
    fun `clampBackoffMs grows exponentially then caps`() {
        assertEquals(1_000L, EkoDBClient.clampBackoffMs(0))
        assertEquals(2_000L, EkoDBClient.clampBackoffMs(1))
        assertEquals(4_000L, EkoDBClient.clampBackoffMs(2))
        // A large attempt is capped at MAX_BACKOFF_MS without overflowing.
        assertEquals(EkoDBClient.MAX_BACKOFF_MS, EkoDBClient.clampBackoffMs(20))
        assertTrue(EkoDBClient.clampBackoffMs(100) <= EkoDBClient.MAX_BACKOFF_MS)
    }
}
