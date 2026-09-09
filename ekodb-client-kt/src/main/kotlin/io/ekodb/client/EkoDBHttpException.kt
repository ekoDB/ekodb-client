package io.ekodb.client

/**
 * A terminal HTTP error from an authenticated request.
 *
 * [responseBody] preserves the server response without requiring message parsing.
 * The body and message may contain server-echoed sensitive data; callers must
 * redact credentials and tokens before persisting or displaying diagnostics.
 */
class EkoDBHttpException(val statusCode: Int, val responseBody: String) :
    Exception("Request failed with status $statusCode: $responseBody")
