package io.ekodb.client

/**
 * A terminal HTTP error from an authenticated request.
 *
 * [responseBody] preserves the server response without requiring message parsing.
 * The body and message may contain server-echoed sensitive data; callers must
 * redact credentials and tokens before persisting or displaying diagnostics.
 *
 * A failed transaction call (begin, status, commit, rollback or a savepoint call)
 * carries the server's verdict: [message] is the server's `error` text (the raw
 * body when the body is not that JSON) and [retryable] is the server's
 * `retryable` flag. `true` means begin a new transaction and run the work again;
 * the client never re-sends a transaction call itself. [retryable] is `false`
 * for every other call and whenever the server does not say.
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
