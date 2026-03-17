/**
 * Unit tests for ekoDB WebSocket client
 *
 * Uses a mock WebSocket server to test the dispatcher pattern.
 */

import { describe, it, expect, beforeEach, afterEach } from "vitest";
import { WebSocketServer, WebSocket as WS } from "ws";
import { WebSocketClient, EventStream } from "./client";

// ============================================================================
// Test Helpers
// ============================================================================

let wss: WebSocketServer;
let port: number;
let serverConnections: WS[] = [];

function startServer(): Promise<number> {
  return new Promise((resolve) => {
    wss = new WebSocketServer({ port: 0 });
    wss.on("connection", (ws) => {
      serverConnections.push(ws);
    });
    wss.on("listening", () => {
      const addr = wss.address();
      resolve(typeof addr === "string" ? 0 : addr!.port);
    });
  });
}

function getLastConnection(): WS {
  return serverConnections[serverConnections.length - 1];
}

function waitForMessage(ws: WS): Promise<any> {
  return new Promise((resolve) => {
    ws.once("message", (data) => {
      resolve(JSON.parse(data.toString()));
    });
  });
}

// ============================================================================
// Tests
// ============================================================================

describe("WebSocketClient", () => {
  beforeEach(async () => {
    serverConnections = [];
    port = await startServer();
  });

  afterEach(() => {
    for (const ws of serverConnections) {
      ws.close();
    }
    wss.close();
  });

  // --------------------------------------------------------------------------
  // findAll
  // --------------------------------------------------------------------------

  describe("findAll", () => {
    it("sends FindAll request and returns data", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      const resultPromise = client.findAll("users");

      // Wait for connection and message
      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const msg = await waitForMessage(ws);

      expect(msg.type).toBe("FindAll");
      expect(msg.payload.collection).toBe("users");

      // Send response
      ws.send(
        JSON.stringify({
          type: "Success",
          payload: {
            message_id: msg.messageId,
            data: [{ id: "1", name: "Alice" }],
          },
        }),
      );

      const result = await resultPromise;
      expect(result).toEqual([{ id: "1", name: "Alice" }]);

      client.close();
    });

    it("rejects on error response", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      const resultPromise = client.findAll("users");

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const msg = await waitForMessage(ws);

      ws.send(
        JSON.stringify({
          type: "Error",
          payload: { message_id: msg.messageId },
          message: "Collection not found",
        }),
      );

      await expect(resultPromise).rejects.toThrow("Collection not found");

      client.close();
    });
  });

  // --------------------------------------------------------------------------
  // subscribe
  // --------------------------------------------------------------------------

  describe("subscribe", () => {
    it("sends Subscribe request and receives mutations", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      const streamPromise = client.subscribe("orders");

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const msg = await waitForMessage(ws);

      expect(msg.type).toBe("Subscribe");
      expect(msg.payload.collection).toBe("orders");

      // Ack subscription
      ws.send(
        JSON.stringify({
          type: "Success",
          payload: { message_id: msg.messageId, status: "subscribed" },
        }),
      );

      const stream = await streamPromise;
      expect(stream).toBeInstanceOf(EventStream);

      // Receive mutation
      const mutationPromise = new Promise((resolve) => {
        stream.on("mutation", resolve);
      });

      ws.send(
        JSON.stringify({
          type: "MutationNotification",
          payload: {
            collection: "orders",
            event: "insert",
            record_ids: ["order-1"],
            timestamp: "2026-03-13T00:00:00Z",
          },
        }),
      );

      const mutation: any = await mutationPromise;
      expect(mutation.event).toBe("insert");
      expect(mutation.recordIds).toEqual(["order-1"]);

      client.close();
    });

    it("supports filter options", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      const streamPromise = client.subscribe("orders", {
        filterField: "status",
        filterValue: "pending",
      });

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const msg = await waitForMessage(ws);

      expect(msg.payload.filter_field).toBe("status");
      expect(msg.payload.filter_value).toBe("pending");

      // Ack
      ws.send(
        JSON.stringify({
          type: "Success",
          payload: { message_id: msg.messageId },
        }),
      );

      await streamPromise;
      client.close();
    });
  });

  // --------------------------------------------------------------------------
  // chatSend
  // --------------------------------------------------------------------------

  describe("chatSend", () => {
    it("sends ChatSend and receives streaming chunks", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      const streamPromise = client.chatSend("chat-1", "Hello");

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const msg = await waitForMessage(ws);

      expect(msg.type).toBe("ChatSend");
      expect(msg.payload.chat_id).toBe("chat-1");
      expect(msg.payload.message).toBe("Hello");

      const stream = await streamPromise;
      const events: any[] = [];
      stream.on("event", (e) => events.push(e));

      // Send chunks
      ws.send(
        JSON.stringify({
          type: "ChatStreamChunk",
          payload: { chat_id: "chat-1", content: "Hi " },
        }),
      );
      ws.send(
        JSON.stringify({
          type: "ChatStreamChunk",
          payload: { chat_id: "chat-1", content: "there!" },
        }),
      );
      ws.send(
        JSON.stringify({
          type: "ChatStreamEnd",
          payload: {
            chat_id: "chat-1",
            message_id: "msg-1",
            execution_time_ms: 150,
          },
        }),
      );

      // Wait for close
      await new Promise((r) => stream.on("close", r));

      expect(events).toHaveLength(3);
      expect(events[0]).toEqual({ type: "chunk", content: "Hi " });
      expect(events[1]).toEqual({ type: "chunk", content: "there!" });
      expect(events[2].type).toBe("end");
      expect(events[2].messageId).toBe("msg-1");
      expect(events[2].executionTimeMs).toBe(150);

      client.close();
    });

    it("handles chat stream error", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      const streamPromise = client.chatSend("chat-2", "test");

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      await waitForMessage(ws);

      const stream = await streamPromise;
      const events: any[] = [];
      stream.on("event", (e) => events.push(e));

      ws.send(
        JSON.stringify({
          type: "ChatStreamError",
          payload: { chat_id: "chat-2", error: "Model unavailable" },
        }),
      );

      await new Promise((r) => stream.on("close", r));
      expect(events[0]).toEqual({ type: "error", error: "Model unavailable" });
      expect(stream.closed).toBe(true);

      client.close();
    });

    it("sends options with ChatSend", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      const streamPromise = client.chatSend("chat-3", "Hello", {
        bypassRipple: true,
        maxIterations: 5,
        excludeTools: ["shell_exec"],
      });

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const msg = await waitForMessage(ws);

      expect(msg.payload.bypass_ripple).toBe(true);
      expect(msg.payload.max_iterations).toBe(5);
      expect(msg.payload.exclude_tools).toEqual(["shell_exec"]);

      const stream = await streamPromise;
      ws.send(
        JSON.stringify({
          type: "ChatStreamEnd",
          payload: {
            chat_id: "chat-3",
            message_id: "msg-x",
            execution_time_ms: 10,
          },
        }),
      );
      await new Promise((r) => stream.on("close", r));

      client.close();
    });
  });

  // --------------------------------------------------------------------------
  // registerClientTools
  // --------------------------------------------------------------------------

  describe("registerClientTools", () => {
    it("sends RegisterClientTools and waits for ack", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      const tools = [
        {
          name: "get_weather",
          description: "Get weather for a location",
          parameters: {
            type: "object",
            properties: { location: { type: "string" } },
          },
        },
      ];

      const regPromise = client.registerClientTools("chat-1", tools);

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const msg = await waitForMessage(ws);

      expect(msg.type).toBe("RegisterClientTools");
      expect(msg.payload.chat_id).toBe("chat-1");
      expect(msg.payload.tools).toHaveLength(1);
      expect(msg.payload.tools[0].name).toBe("get_weather");

      // Ack
      ws.send(
        JSON.stringify({
          type: "Success",
          payload: { status: "tools_registered" },
        }),
      );

      await regPromise;
      client.close();
    });
  });

  // --------------------------------------------------------------------------
  // sendToolResult
  // --------------------------------------------------------------------------

  describe("sendToolResult", () => {
    it("sends ClientToolResult frame", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      // Need to trigger a connection first
      const streamPromise = client.chatSend("chat-1", "test");
      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      await waitForMessage(ws); // ChatSend

      const stream = await streamPromise;

      // Server sends tool call
      ws.send(
        JSON.stringify({
          type: "ClientToolCall",
          payload: {
            chat_id: "chat-1",
            call_id: "call-123",
            tool_name: "get_weather",
            arguments: { location: "NYC" },
          },
        }),
      );

      // Wait for tool call event
      const toolCallEvent: any = await new Promise((resolve) => {
        stream.on("event", (e: any) => {
          if (e.type === "toolCall") resolve(e);
        });
      });

      expect(toolCallEvent.toolName).toBe("get_weather");
      expect(toolCallEvent.callId).toBe("call-123");

      // Send tool result
      const resultMsg = waitForMessage(ws);
      await client.sendToolResult("chat-1", "call-123", true, {
        temp: "72F",
      });

      const sent = await resultMsg;
      expect(sent.type).toBe("ClientToolResult");
      expect(sent.payload.chat_id).toBe("chat-1");
      expect(sent.payload.call_id).toBe("call-123");
      expect(sent.payload.success).toBe(true);
      expect(sent.payload.result).toEqual({ temp: "72F" });

      // End the stream
      ws.send(
        JSON.stringify({
          type: "ChatStreamEnd",
          payload: {
            chat_id: "chat-1",
            message_id: "msg-1",
            execution_time_ms: 500,
          },
        }),
      );
      await new Promise((r) => stream.on("close", r));

      client.close();
    });

    it("sends error tool result", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      // Connect
      const findPromise = client.findAll("test");
      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const findMsg = await waitForMessage(ws);
      ws.send(
        JSON.stringify({
          type: "Success",
          payload: { message_id: findMsg.messageId, data: [] },
        }),
      );
      await findPromise;

      const resultMsg = waitForMessage(ws);
      await client.sendToolResult(
        "chat-1",
        "call-456",
        false,
        undefined,
        "API rate limited",
      );

      const sent = await resultMsg;
      expect(sent.payload.success).toBe(false);
      expect(sent.payload.error).toBe("API rate limited");
      expect(sent.payload.result).toBeUndefined();

      client.close();
    });
  });

  // --------------------------------------------------------------------------
  // Connection lifecycle
  // --------------------------------------------------------------------------

  describe("connection lifecycle", () => {
    it("reuses existing connection", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      // First call
      const p1 = client.findAll("col1");
      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const m1 = await waitForMessage(ws);
      ws.send(
        JSON.stringify({
          type: "Success",
          payload: { message_id: m1.messageId, data: [{ id: "1" }] },
        }),
      );
      await p1;

      // Second call should reuse connection
      const p2 = client.findAll("col2");
      const m2 = await waitForMessage(ws);
      ws.send(
        JSON.stringify({
          type: "Success",
          payload: { message_id: m2.messageId, data: [{ id: "2" }] },
        }),
      );
      const r2 = await p2;
      expect(r2).toEqual([{ id: "2" }]);

      expect(serverConnections.length).toBe(1); // Only one connection
      client.close();
    });

    it("close cleans up", () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );
      // Close without connecting should be safe
      client.close();
    });
  });

  // --------------------------------------------------------------------------
  // EventStream
  // --------------------------------------------------------------------------

  describe("EventStream", () => {
    it("emits events to listeners", () => {
      const stream = new EventStream();
      const received: any[] = [];
      stream.on("data", (d) => received.push(d));
      stream.emit("data", "hello");
      stream.emit("data", "world");
      expect(received).toEqual(["hello", "world"]);
    });

    it("tracks closed state", () => {
      const stream = new EventStream();
      expect(stream.closed).toBe(false);
      stream.close();
      expect(stream.closed).toBe(true);
    });

    it("emits close event", () => {
      const stream = new EventStream();
      let closed = false;
      stream.on("close", () => {
        closed = true;
      });
      stream.close();
      expect(closed).toBe(true);
    });
  });
});
