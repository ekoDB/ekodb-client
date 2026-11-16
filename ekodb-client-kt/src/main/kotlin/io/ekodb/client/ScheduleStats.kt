package io.ekodb.client

/**
 * The server's schedule summary, from [EkoDBClient.getScheduleStats].
 *
 * @property activeSchedules The number of schedules the server is running.
 */
data class ScheduleStats(
    val activeSchedules: Long,
)
