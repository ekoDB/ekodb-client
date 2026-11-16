package io.ekodb.client

/**
 * What restoring a whole collection from trash reported.
 *
 * The two counts differ: a tombstone can be cleared while the record's data is
 * no longer recoverable, so [recordsRestored] can be lower than [clearedCount].
 *
 * @property clearedCount Tombstones removed from the collection (the server's `cleared_count`).
 * @property recordsRestored Records whose data was actually recovered (the server's
 *   `records_restored`); null when the server does not send it, as servers before 0.72.7 do not.
 */
data class RestoreCollectionResult(
    val clearedCount: Long,
    val recordsRestored: Long?,
)
