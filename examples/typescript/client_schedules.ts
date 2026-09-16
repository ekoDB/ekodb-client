// ekoDB TypeScript Client - Schedule Management Integration Example
//
// Exercises: createSchedule, listSchedules, getSchedule, updateSchedule,
// triggerSchedule, pauseSchedule, resumeSchedule, deleteSchedule.
// A temporary no-op function makes the trigger step self-contained.

import { EkoDBClient, Stage } from "@ekodb/ekodb-client";

async function main() {
  console.log("=== ekoDB Schedule Management Example (TypeScript) ===\n");

  const baseUrl = process.env.API_BASE_URL || "http://localhost:8080";
  const apiKey = process.env.API_BASE_KEY;
  if (!apiKey) throw new Error("API_BASE_KEY environment variable is required");

  const client = new EkoDBClient(baseUrl, apiKey);

  const functionLabel = `schedule_noop_typescript_${process.pid}_${Date.now()}`;
  const functionId = await client.saveFunction({
    label: functionLabel,
    name: "Schedule example no-op",
    parameters: {},
    functions: [Stage.returnResponse({ triggered: true }, 200)],
  });

  let scheduleId: string | undefined;
  let primaryError: unknown;
  try {
    console.log("--- Creating schedule ---");
    const schedule = await client.createSchedule({
      name: "Nightly Database Backup",
      function_label: functionLabel,
      cron_expression: "0 0 2 * * *",
      enabled: true,
    });
    scheduleId = schedule.id as string;
    if (
      !scheduleId ||
      schedule.name !== "Nightly Database Backup" ||
      schedule.cron_expression !== "0 0 2 * * *" ||
      schedule.enabled !== true
    ) {
      throw new Error(
        `Unexpected created schedule: ${JSON.stringify(schedule)}`,
      );
    }
    console.log(
      `Created schedule: ${schedule.name} (id: ${scheduleId}, cron: ${schedule.cron_expression})`,
    );

    // 2. List all schedules
    console.log("\n--- Listing schedules ---");
    const list = await client.listSchedules();
    console.log("Schedules:", JSON.stringify(list, null, 2));
    if (
      !Array.isArray(list.schedules) ||
      !list.schedules.some((item: any) => item.id === scheduleId)
    ) {
      throw new Error(
        `Created schedule ${scheduleId} was absent from listSchedules`,
      );
    }

    // 3. Get schedule by ID
    console.log("\n--- Getting schedule ---");
    const fetched = await client.getSchedule(scheduleId);
    console.log(`Fetched: ${fetched.name} (cron: ${fetched.cron_expression})`);
    if (
      fetched.id !== scheduleId ||
      fetched.cron_expression !== "0 0 2 * * *"
    ) {
      throw new Error(
        `Unexpected fetched schedule: ${JSON.stringify(fetched)}`,
      );
    }

    // 4. Update schedule
    console.log("\n--- Updating schedule ---");
    const updated = await client.updateSchedule(scheduleId, {
      name: "Nightly Full Backup",
      cron_expression: "0 0 3 * * *",
    });
    console.log(
      `Updated: ${updated.name} (new cron: ${updated.cron_expression})`,
    );
    if (
      updated.name !== "Nightly Full Backup" ||
      updated.cron_expression !== "0 0 3 * * *"
    ) {
      throw new Error(
        `Unexpected updated schedule: ${JSON.stringify(updated)}`,
      );
    }

    // 5. Trigger immediately
    console.log("\n--- Triggering schedule ---");
    const triggered = await client.triggerSchedule(scheduleId);
    console.log("Trigger response:", JSON.stringify(triggered, null, 2));
    if (triggered.schedule_id !== scheduleId) {
      throw new Error(
        `Unexpected trigger response: ${JSON.stringify(triggered)}`,
      );
    }

    // 6. Pause schedule
    console.log("\n--- Pausing schedule ---");
    const paused = await client.pauseSchedule(scheduleId);
    console.log(`Schedule enabled after pause: ${paused.enabled}`);
    if (paused.enabled !== false) {
      throw new Error(`Pause left schedule enabled: ${JSON.stringify(paused)}`);
    }

    // 7. Resume schedule
    console.log("\n--- Resuming schedule ---");
    const resumed = await client.resumeSchedule(scheduleId);
    console.log(`Schedule enabled after resume: ${resumed.enabled}`);
    if (resumed.enabled !== true) {
      throw new Error(
        `Resume left schedule disabled: ${JSON.stringify(resumed)}`,
      );
    }

    // 8. Delete schedule
    console.log("\n--- Deleting schedule ---");
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    const cleanupErrors: unknown[] = [];
    if (scheduleId) {
      try {
        await client.deleteSchedule(scheduleId);
        console.log("Schedule deleted successfully");
      } catch (error) {
        cleanupErrors.push(error);
      }
    }
    try {
      await client.deleteFunction(functionId);
    } catch (error) {
      cleanupErrors.push(error);
    }
    if (cleanupErrors.length > 0) {
      if (primaryError) {
        console.error("Cleanup errors:", cleanupErrors);
      } else {
        throw new AggregateError(cleanupErrors, "Schedule cleanup failed");
      }
    }
  }

  console.log("\n=== All schedule operations completed ===");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
