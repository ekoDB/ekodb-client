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

    it("includes context_window in end event", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      const streamPromise = client.chatSend("chat-cw", "test");

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      await waitForMessage(ws);

      const stream = await streamPromise;
      const events: any[] = [];
      stream.on("event", (e) => events.push(e));

      ws.send(
        JSON.stringify({
          type: "ChatStreamEnd",
          payload: {
            chat_id: "chat-cw",
            message_id: "msg-cw",
            execution_time_ms: 100,
            context_window: 128000,
          },
        }),
      );

      await new Promise((r) => stream.on("close", r));

      expect(events).toHaveLength(1);
      expect(events[0].type).toBe("end");
      expect(events[0].contextWindow).toBe(128000);

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

  // --------------------------------------------------------------------------
  // rawCompletion
  // --------------------------------------------------------------------------

  describe("rawCompletion", () => {
    it("sends RawComplete request and returns content", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      const resultPromise = client.rawCompletion({
        system_prompt: "You are helpful.",
        message: "Say hello.",
      });

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const msg = await waitForMessage(ws);

      expect(msg.type).toBe("RawComplete");
      expect(msg.payload.system_prompt).toBe("You are helpful.");
      expect(msg.payload.message).toBe("Say hello.");

      ws.send(
        JSON.stringify({
          type: "Success",
          payload: {
            data: { content: "Hello! How can I help?" },
          },
        }),
      );

      const result = await resultPromise;
      expect(result.content).toBe("Hello! How can I help?");

      client.close();
    });

    it("includes optional fields when provided", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      const resultPromise = client.rawCompletion({
        system_prompt: "System.",
        message: "User.",
        provider: "openai",
        model: "gpt-4o-mini",
        max_tokens: 512,
      });

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const msg = await waitForMessage(ws);

      expect(msg.payload.provider).toBe("openai");
      expect(msg.payload.model).toBe("gpt-4o-mini");
      expect(msg.payload.max_tokens).toBe(512);

      ws.send(
        JSON.stringify({
          type: "Success",
          payload: { data: { content: "Done." } },
        }),
      );

      await resultPromise;
      client.close();
    });

    it("rejects on error response", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
      );

      const resultPromise = client.rawCompletion({
        system_prompt: "System.",
        message: "User.",
      });

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      await waitForMessage(ws);

      ws.send(
        JSON.stringify({
          type: "Error",
          message: "Model not found",
        }),
      );

      await expect(resultPromise).rejects.toThrow("Model not found");

      client.close();
    });
  });

  // --------------------------------------------------------------------------
  // Per-request timeout
  // --------------------------------------------------------------------------

  describe("per-request timeout", () => {
    it("rejects a pending request when no response arrives", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
        { requestTimeoutMs: 50, autoReconnect: false },
      );

      const resultPromise = client.findAll("users");

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      await waitForMessage(ws); // receive FindAll but never respond

      await expect(resultPromise).rejects.toThrow(/timed out after 50ms/);

      client.close();
    });

    it("does not time out when a response arrives in time", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
        { requestTimeoutMs: 500, autoReconnect: false },
      );

      const resultPromise = client.findAll("users");

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const msg = await waitForMessage(ws);
      ws.send(
        JSON.stringify({
          type: "Success",
          payload: { message_id: msg.messageId, data: [{ id: "1" }] },
        }),
      );

      await expect(resultPromise).resolves.toEqual([{ id: "1" }]);
      client.close();
    });
  });

  // --------------------------------------------------------------------------
  // Reject pending requests on disconnect
  // --------------------------------------------------------------------------

  describe("disconnect handling", () => {
    it("rejects in-flight requests when the socket drops", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
        { autoReconnect: false },
      );

      const resultPromise = client.findAll("users");

      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      await waitForMessage(ws); // FindAll received, never answered

      // Server drops the connection.
      ws.close();

      await expect(resultPromise).rejects.toThrow(/connection closed/i);

      client.close();
    });
  });

  // --------------------------------------------------------------------------
  // Backoff helper
  // --------------------------------------------------------------------------

  describe("computeBackoff", () => {
    it("grows exponentially, stays within jittered bounds, and caps", () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
        { reconnectInitialDelayMs: 200, reconnectMaxDelayMs: 5000 },
      );

      // attempt 0 -> base 200, +/-25% => [150, 250]
      for (let i = 0; i < 20; i++) {
        const d0 = client.computeBackoff(0);
        expect(d0).toBeGreaterThanOrEqual(150);
        expect(d0).toBeLessThanOrEqual(250);
      }

      // attempt 3 -> base 1600, +/-25% => [1200, 2000]
      const d3 = client.computeBackoff(3);
      expect(d3).toBeGreaterThanOrEqual(1200);
      expect(d3).toBeLessThanOrEqual(2000);

      // attempt 20 -> capped at 5000, +/-25% => [3750, 6250]
      const dBig = client.computeBackoff(20);
      expect(dBig).toBeGreaterThanOrEqual(3750);
      expect(dBig).toBeLessThanOrEqual(6250);

      client.close();
    });
  });

  // --------------------------------------------------------------------------
  // Auto-reconnect + re-subscribe + fresh token
  // --------------------------------------------------------------------------

  describe("auto-reconnect", () => {
    it("re-subscribes after a socket drop and re-evaluates the token", async () => {
      // Token provider returns a new token each call so we can assert the
      // reconnect used a freshly-obtained token (not a stale snapshot).
      let tokenCalls = 0;
      const tokenProvider = () => `token-${++tokenCalls}`;

      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        tokenProvider,
        {
          autoReconnect: true,
          reconnectInitialDelayMs: 10,
          reconnectMaxDelayMs: 30,
        },
      );

      // Establish the subscription on the first connection.
      const streamPromise = client.subscribe("orders");
      await new Promise((r) => wss.once("connection", r));
      const ws1 = getLastConnection();
      const sub1 = await waitForMessage(ws1);
      expect(sub1.type).toBe("Subscribe");
      expect(sub1.payload.collection).toBe("orders");
      ws1.send(
        JSON.stringify({
          type: "Success",
          payload: { message_id: sub1.messageId, status: "subscribed" },
        }),
      );
      const stream = await streamPromise;
      expect(tokenCalls).toBe(1);

      // Arm the next-connection handler BEFORE dropping so we don't miss it.
      const nextConn = new Promise<WS>((resolve) => {
        wss.once("connection", (ws: WS) => resolve(ws));
      });

      // Simulate a transient drop: server closes the socket.
      ws1.close();

      // The client should auto-reconnect (new connection) and re-send Subscribe.
      const ws2 = await nextConn;
      const sub2 = await waitForMessage(ws2);
      expect(sub2.type).toBe("Subscribe");
      expect(sub2.payload.collection).toBe("orders");

      // Reconnect re-evaluated the token provider.
      expect(tokenCalls).toBeGreaterThanOrEqual(2);

      // Ack the re-subscribe.
      ws2.send(
        JSON.stringify({
          type: "Success",
          payload: { message_id: sub2.messageId, status: "subscribed" },
        }),
      );

      // The SAME stream still delivers mutations over the new socket.
      const mutationPromise = new Promise<any>((resolve) => {
        stream.on("mutation", resolve);
      });
      ws2.send(
        JSON.stringify({
          type: "MutationNotification",
          payload: {
            collection: "orders",
            event: "insert",
            record_ids: ["order-9"],
            timestamp: "2026-06-02T00:00:00Z",
          },
        }),
      );
      const mutation = await mutationPromise;
      expect(mutation.recordIds).toEqual(["order-9"]);
      expect(stream.closed).toBe(false);

      client.close();
    });

    it("does not reconnect after an intentional close()", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        "test-token",
        { autoReconnect: true, reconnectInitialDelayMs: 10 },
      );

      const streamPromise = client.subscribe("widgets");
      await new Promise((r) => wss.once("connection", r));
      const ws = getLastConnection();
      const sub = await waitForMessage(ws);
      ws.send(
        JSON.stringify({
          type: "Success",
          payload: { message_id: sub.messageId, status: "subscribed" },
        }),
      );
      await streamPromise;

      const connectionsBefore = serverConnections.length;

      // Intentional close: must NOT reconnect.
      client.close();

      // Give any erroneous reconnect time to land.
      await new Promise((r) => setTimeout(r, 80));
      expect(serverConnections.length).toBe(connectionsBefore);
    });
  });

  describe("auth token validation", () => {
    it("rejects connect when the token provider returns null (no Bearer null)", async () => {
      const client = new WebSocketClient(
        `ws://localhost:${port}/api/ws`,
        () => null,
      );
      await expect(client.findAll("users")).rejects.toThrow(
        /token is unavailable/i,
      );
      client.close();
    });
  });
});
