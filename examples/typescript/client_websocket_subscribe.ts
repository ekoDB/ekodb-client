/**
 * WebSocket Subscription Example - Real-time mutation notifications
 *
 * Demonstrates subscribing to collection changes via WebSocket and
 * receiving push notifications when records are inserted, updated, or deleted.
 *
 * Requires: ekoDB server running on localhost:8080
 */

import WebSocket from "ws";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const WS_URL = process.env.WS_BASE_URL || "ws://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function getAuthToken(): Promise<string> {
  const res = await fetch(`${BASE_URL}/api/auth/token`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ api_key: API_KEY }),
  });
  if (!res.ok)
    throw new Error(`Authentication failed with status ${res.status}`);
  const data = (await res.json()) as { token: string };
  return data.token;
}

async function insertRecord(
  token: string,
  collection: string,
  record: object,
): Promise<any> {
  const res = await fetch(`${BASE_URL}/api/insert/${collection}`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(record),
  });
  if (!res.ok) {
    throw new Error(
      `Insert failed with status ${res.status}: ${await res.text()}`,
    );
  }
  return res.json();
}

async function deleteCollection(
  token: string,
  collection: string,
): Promise<void> {
  const res = await fetch(`${BASE_URL}/api/collections/${collection}`, {
    method: "DELETE",
    headers: { Authorization: `Bearer ${token}` },
  });
  if (!res.ok && res.status !== 404) {
    throw new Error(
      `Delete collection failed with status ${res.status}: ${await res.text()}`,
    );
  }
}

function waitForMessage(
  ws: WebSocket,
  description: string,
  predicate: (message: any) => boolean,
): Promise<any> {
  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => {
      cleanup();
      reject(new Error(`${description} timed out after 5 seconds`));
    }, 5000);
    const onMessage = (data: WebSocket.RawData) => {
      try {
        const message = JSON.parse(data.toString());
        if (!predicate(message)) return;
        cleanup();
        resolve(message);
      } catch (error) {
        cleanup();
        reject(error);
      }
    };
    const onError = (error: Error) => {
      cleanup();
      reject(error);
    };
    const onClose = () => {
      cleanup();
      reject(new Error(`WebSocket closed before ${description}`));
    };
    const cleanup = () => {
      clearTimeout(timeout);
      ws.off("message", onMessage);
      ws.off("error", onError);
      ws.off("close", onClose);
    };
    ws.on("message", onMessage);
    ws.on("error", onError);
    ws.on("close", onClose);
  });
}

async function main() {
  console.log("=== WebSocket Subscription Example ===\n");

  const collection = "ws_subscribe_example_ts";

  // Step 1: Authenticate
  const token = await getAuthToken();
  console.log("✓ Authentication successful");

  await deleteCollection(token, collection);

  // Step 2: Connect to WebSocket
  const ws = new WebSocket(`${WS_URL}/api/ws`, {
    headers: { Authorization: `Bearer ${token}` },
  });
  let runError: unknown;

  try {
    await new Promise<void>((resolve, reject) => {
      const onOpen = () => {
        cleanup();
        resolve();
      };
      const onError = (error: Error) => {
        cleanup();
        reject(error);
      };
      const cleanup = () => {
        clearTimeout(timeout);
        ws.off("open", onOpen);
        ws.off("error", onError);
      };
      const timeout = setTimeout(() => {
        cleanup();
        reject(new Error("WebSocket open timed out after 5 seconds"));
      }, 5000);
      ws.on("open", onOpen);
      ws.on("error", onError);
    });
    console.log("✓ WebSocket connected");

    // Step 3: Subscribe to collection
    console.log(`\n=== Subscribing to '${collection}' ===`);
    const subscribePromise = waitForMessage(
      ws,
      "subscription confirmation",
      (message) =>
        message.type === "Success" &&
        typeof message.payload?.data?.subscription_id === "string",
    );
    ws.send(JSON.stringify({ type: "Subscribe", payload: { collection } }));
    const subResponse = await subscribePromise;
    console.log(
      `✓ Subscribed (subscription_id: ${subResponse.payload.data.subscription_id})`,
    );

    // Step 4: Perform mutations and require both notifications.
    console.log("\n=== Performing mutations to trigger notifications ===");
    const firstNotification = waitForMessage(
      ws,
      "first mutation notification",
      (message) => message.type === "MutationNotification",
    );
    console.log("Inserting a record...");
    const firstInsert = await insertRecord(token, collection, {
      name: "Alice",
      role: "engineer",
      active: true,
    });
    console.log(`✓ Inserted record: ${firstInsert.id}`);
    const first = await firstNotification;
    if (
      first.payload?.event !== "insert" ||
      first.payload?.collection !== collection ||
      !first.payload?.record_ids?.includes(firstInsert.id)
    ) {
      throw new Error(
        `Unexpected first notification: ${JSON.stringify(first)}`,
      );
    }
    console.log(`  📡 Notification received for ${firstInsert.id}`);

    const secondNotification = waitForMessage(
      ws,
      "second mutation notification",
      (message) => message.type === "MutationNotification",
    );
    console.log("\nInserting another record...");
    const secondInsert = await insertRecord(token, collection, {
      name: "Bob",
      role: "designer",
      active: true,
    });
    console.log(`✓ Inserted record: ${secondInsert.id}`);
    const second = await secondNotification;
    if (
      second.payload?.event !== "insert" ||
      second.payload?.collection !== collection ||
      !second.payload?.record_ids?.includes(secondInsert.id)
    ) {
      throw new Error(
        `Unexpected second notification: ${JSON.stringify(second)}`,
      );
    }
    console.log(`  📡 Notification received for ${secondInsert.id}`);

    // Step 5: Unsubscribe and require confirmation.
    console.log("\n=== Unsubscribing ===");
    const unsubPromise = waitForMessage(
      ws,
      "unsubscribe confirmation",
      (message) => message.type === "Success",
    );
    ws.send(JSON.stringify({ type: "Unsubscribe", payload: { collection } }));
    const unsubResponse = await unsubPromise;
    console.log(
      `✓ Unsubscribed: ${JSON.stringify(unsubResponse.payload.data)}`,
    );

    console.log("\n✓ WebSocket subscription example completed successfully");
  } catch (error) {
    runError = error;
    throw error;
  } finally {
    if (ws.readyState === WebSocket.OPEN) ws.close();
    else if (ws.readyState === WebSocket.CONNECTING) ws.terminate();
    try {
      await deleteCollection(token, collection);
      console.log(`✓ Deleted collection '${collection}'`);
    } catch (cleanupError) {
      if (runError) {
        throw new AggregateError(
          [runError, cleanupError],
          "WebSocket subscription example and cleanup both failed",
        );
      }
      throw cleanupError;
    }
  }
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
