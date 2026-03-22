// ekoDB TypeScript Client - Schedule Management Integration Example
//
// Exercises: createSchedule, listSchedules, getSchedule, updateSchedule,
// pauseSchedule, resumeSchedule, deleteSchedule.

import { EkoDBClient } from "@ekodb/ekodb-client";

async function main() {
  console.log("=== ekoDB Schedule Management Example (TypeScript) ===\n");

  const baseUrl = process.env.API_BASE_URL || "http://localhost:8080";
  const apiKey = process.env.API_BASE_KEY;
  if (!apiKey) throw new Error("API_BASE_KEY environment variable is required");

  const client = new EkoDBClient(baseUrl, apiKey);

  // 1. Create a schedule
  console.log("--- Creating schedule ---");
  const schedule = await client.createSchedule({
    name: "Nightly Database Backup",
    cron: "0 2 * * *",
    action: "backup",
    config: {
      target: "s3://backups/ekodb/",
      retention_days: 30,
    },
  });
  const scheduleId = schedule.id as string;
  console.log(
    `Created schedule: ${schedule.name} (id: ${scheduleId}, cron: ${schedule.cron})`,
  );

  // 2. List all schedules
  console.log("\n--- Listing schedules ---");
  const list = await client.listSchedules();
  console.log("Schedules:", JSON.stringify(list, null, 2));

  // 3. Get schedule by ID
  console.log("\n--- Getting schedule ---");
  const fetched = await client.getSchedule(scheduleId);
  console.log(`Fetched: ${fetched.name} (cron: ${fetched.cron})`);

  // 4. Update schedule
  console.log("\n--- Updating schedule ---");
  const updated = await client.updateSchedule(scheduleId, {
    name: "Nightly Full Backup",
    cron: "0 3 * * *",
    config: {
      target: "s3://backups/ekodb/",
      retention_days: 60,
      compression: "zstd",
    },
  });
  console.log(`Updated: ${updated.name} (new cron: ${updated.cron})`);

  // 5. Pause schedule
  console.log("\n--- Pausing schedule ---");
  const paused = await client.pauseSchedule(scheduleId);
  console.log(`Schedule status after pause: ${paused.status}`);

  // 6. Resume schedule
  console.log("\n--- Resuming schedule ---");
  const resumed = await client.resumeSchedule(scheduleId);
  console.log(`Schedule status after resume: ${resumed.status}`);

  // 7. Delete schedule
  console.log("\n--- Deleting schedule ---");
  await client.deleteSchedule(scheduleId);
  console.log("Schedule deleted successfully");

  console.log("\n=== All schedule operations completed ===");
}

main().catch(console.error);
