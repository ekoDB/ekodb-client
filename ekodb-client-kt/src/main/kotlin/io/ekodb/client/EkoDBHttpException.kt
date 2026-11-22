package io.ekodb.client

/**
 * A terminal HTTP error from an authenticated request.
 *
 * [responseBody] preserves the server response without requiring message parsing.
 * The body and message may contain server-echoed sensitive data; callers must
 * redact credentials and tokens before persisting or displaying diagnostics.
 *
 * A failed call on an existing transaction (commit, rollback, or a create,
 * rollback-to or release savepoint call) carries the server's verdict:
 * [message] is the server's `error` text (the raw body when the body is not
 * that JSON) and [retryable] is the server's `retryable` flag. `true` means
 * begin a new transaction and run the work again; the client never re-sends
 * those calls itself. [retryable] is `false` for every other call (begin and
 * status included) and whenever the flag is absent or not a boolean.
 *
 * The constructor is `@JvmOverloads`, so Java callers keep the
 * `(int, String)` constructor.
 */
class EkoDBHttpException
    @JvmOverloads
    constructor(
        val statusCode: Int,
        val responseBody: String,
        override val message: String = "Request failed with status $statusCode: $responseBody",
        val retryable: Boolean = false,
    ) : Exception(message)
