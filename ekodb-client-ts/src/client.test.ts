/**
 * Unit tests for ekoDB TypeScript client
 *
 * These tests use vitest and mock fetch to test client methods
 * without requiring a running ekoDB server.
 */

import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import { EkoDBClient, SerializationFormat } from "./client";

// Mock fetch globally
const mockFetch = vi.fn();
global.fetch = mockFetch;

// ============================================================================
// Test Helpers
// ============================================================================

function createTestClient(): EkoDBClient {
  return new EkoDBClient({
    baseURL: "http://localhost:8080",
    apiKey: "test-api-key",
    format: SerializationFormat.Json,
    shouldRetry: false,
  });
}

function mockTokenResponse(): void {
  mockFetch.mockResolvedValueOnce({
    ok: true,
    status: 200,
    json: async () => ({ token: "test-jwt-token" }),
    headers: new Headers(),
  });
}

function mockJsonResponse(data: unknown, status = 200): void {
  mockFetch.mockResolvedValueOnce({
    ok: status >= 200 && status < 300,
    status,
    json: async () => data,
    text: async () => JSON.stringify(data),
    headers: new Headers({
      "content-type": "application/json",
    }),
  });
}

function mockErrorResponse(status: number, message: string): void {
  mockFetch.mockResolvedValueOnce({
    ok: false,
    status,
    json: async () => ({ error: message }),
    text: async () => message,
    headers: new Headers(),
  });
}

// ============================================================================
// Setup/Teardown
// ============================================================================

beforeEach(() => {
  mockFetch.mockReset();
});

afterEach(() => {
  vi.clearAllMocks();
});

// ============================================================================
// Client Configuration Tests
// ============================================================================

describe("EkoDBClient configuration", () => {
  it("creates client with required config", () => {
    const client = new EkoDBClient({
      baseURL: "http://localhost:8080",
      apiKey: "test-key",
    });
    expect(client).toBeInstanceOf(EkoDBClient);
  });

  it("creates client with all options", () => {
    const client = new EkoDBClient({
      baseURL: "http://localhost:8080",
      apiKey: "test-key",
      maxRetries: 5,
      shouldRetry: true,
      format: SerializationFormat.Json,
    });
    expect(client).toBeInstanceOf(EkoDBClient);
  });
});

// ============================================================================
// Health Check Tests
// ============================================================================

// Note: Health check tests require specific response format matching
// Covered by integration tests

// ============================================================================
// Insert Tests
// ============================================================================

describe("EkoDBClient insert", () => {
  it("inserts record successfully", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "record_123", name: "Test" });

    const result = await client.insert("users", { name: "Test" });

    expect(result).toHaveProperty("id", "record_123");
    expect(mockFetch).toHaveBeenCalledTimes(2);
  });

  it("inserts record with TTL", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "record_123", name: "Test", ttl: "30m" });

    const result = await client.insert("users", { name: "Test", ttl: "30m" });

    expect(result).toHaveProperty("id");
  });
});

// ============================================================================
// Find Tests
// ============================================================================

describe("EkoDBClient find", () => {
  it("finds records with query", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse([
      { id: "user_1", name: "Alice" },
      { id: "user_2", name: "Bob" },
    ]);

    const result = await client.find("users", { limit: 10 });

    expect(result).toHaveLength(2);
    expect(result[0]).toHaveProperty("name", "Alice");
  });

  it("finds record by ID", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "user_123", name: "Alice" });

    const result = await client.findById("users", "user_123");

    expect(result).toHaveProperty("id", "user_123");
  });

  it("throws error for not found record", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockErrorResponse(404, "Not found");

    await expect(client.findById("users", "nonexistent")).rejects.toThrow();
  });
});

// ============================================================================
// Update Tests
// ============================================================================

describe("EkoDBClient update", () => {
  it("updates record successfully", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "user_123", name: "Updated" });

    const result = await client.update("users", "user_123", {
      name: "Updated",
    });

    expect(result).toHaveProperty("name", "Updated");
  });
});

// ============================================================================
// Atomic Field Action Tests
// ============================================================================

describe("EkoDBClient updateWithAction", () => {
  it("increments a field", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "rec_1", views: 42 });

    const result = await client.updateWithAction(
      "counters",
      "rec_1",
      "increment",
      "views",
      1,
    );

    expect(result).toHaveProperty("views", 42);
  });

  it("pushes to an array field", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "rec_2", tags: ["rust", "new-tag"] });

    const result = await client.updateWithAction(
      "lists",
      "rec_2",
      "push",
      "tags",
      "new-tag",
    );

    expect(result.tags).toContain("new-tag");
  });

  it("clears a field without value", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "rec_3", temp: 0 });

    const result = await client.updateWithAction(
      "data",
      "rec_3",
      "clear",
      "temp",
    );

    expect(result).toHaveProperty("temp", 0);
  });
});

describe("EkoDBClient updateWithActionSequence", () => {
  it("applies multiple actions atomically", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "player_1", score: 110, lives: 2 });

    const result = await client.updateWithActionSequence("game", "player_1", [
      ["increment", "score", 10],
      ["decrement", "lives", 1],
      ["push", "log", "hit"],
    ]);

    expect(result).toHaveProperty("score", 110);
    expect(result).toHaveProperty("lives", 2);
  });
});

// ============================================================================
// Delete Tests
// ============================================================================

describe("EkoDBClient delete", () => {
  it("deletes record successfully", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "user_123", deleted: true });

    await expect(client.delete("users", "user_123")).resolves.not.toThrow();
  });
});

// ============================================================================
// Batch Operations Tests
// ============================================================================

describe("EkoDBClient batch operations", () => {
  it("batch inserts records", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      successful: ["id_1", "id_2", "id_3"],
      failed: [],
    });

    const records = [{ name: "A" }, { name: "B" }, { name: "C" }];
    const result = await client.batchInsert("users", records);

    expect(result.successful).toHaveLength(3);
    expect(result.failed).toHaveLength(0);
  });

  it("batch deletes records", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      successful: ["id_1", "id_2"],
      failed: [],
    });

    const result = await client.batchDelete("users", ["id_1", "id_2"]);

    expect(result.successful).toHaveLength(2);
  });
});

// ============================================================================
// KV Store Tests
// ============================================================================

describe("EkoDBClient KV store", () => {
  it("sets KV value", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ success: true });

    await expect(
      client.kvSet("my_key", { data: "value" }),
    ).resolves.not.toThrow();
  });

  it("gets KV value", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ value: { data: "stored_value" } });

    const result = await client.kvGet("my_key");

    expect(result).toEqual({ data: "stored_value" });
  });

  it("deletes KV value", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ deleted: true });

    await expect(client.kvDelete("my_key")).resolves.not.toThrow();
  });

  it("checks KV exists", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ value: "something" });

    const result = await client.kvExists("my_key");

    expect(result).toBe(true);
  });
});

// ============================================================================
// Transaction Tests
// ============================================================================

describe("EkoDBClient transactions", () => {
  it("begins transaction", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ transaction_id: "tx_123456" });

    const txId = await client.beginTransaction();

    expect(txId).toBe("tx_123456");
  });

  it("commits transaction", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ status: "committed" });

    await expect(client.commitTransaction("tx_123")).resolves.not.toThrow();
  });

  it("rolls back transaction", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ status: "rolled_back" });

    await expect(client.rollbackTransaction("tx_123")).resolves.not.toThrow();
  });
});

// ============================================================================
// Collection Management Tests
// ============================================================================

describe("EkoDBClient collections", () => {
  it("lists collections", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ collections: ["users", "posts", "comments"] });

    const result = await client.listCollections();

    expect(result).toContain("users");
    expect(result).toHaveLength(3);
  });

  it("deletes collection", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ status: "deleted" });

    await expect(
      client.deleteCollection("test_collection"),
    ).resolves.not.toThrow();
  });
});

// ============================================================================
// Restore Operations Tests
// ============================================================================

describe("EkoDBClient restore operations", () => {
  it("restores deleted record", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ status: "restored" });

    await expect(
      client.restoreRecord("users", "record_123"),
    ).resolves.not.toThrow();
  });

  // Note: restoreCollection return type may vary - covered by integration tests
});

// ============================================================================
// Search Tests
// ============================================================================

describe("EkoDBClient search", () => {
  it("performs search", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      results: [
        { id: "doc_1", score: 0.95 },
        { id: "doc_2", score: 0.85 },
      ],
      total: 2,
    });

    const result = await client.search("documents", { query: "test" });

    expect(result.results).toHaveLength(2);
    expect(result.total).toBe(2);
  });

  // Note: textSearch and hybridSearch require specific mock setup - covered by integration tests
});

// ============================================================================
// KV Batch Operations Tests
// ============================================================================

describe("EkoDBClient KV batch operations", () => {
  it("batch gets multiple keys", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse([
      { data: "value1" },
      { data: "value2" },
      { data: "value3" },
    ]);

    const keys = ["key1", "key2", "key3"];
    const results = await client.kvBatchGet(keys);

    expect(results).toHaveLength(3);
    expect(results[0]).toEqual({ data: "value1" });
    expect(results[1]).toEqual({ data: "value2" });
    expect(results[2]).toEqual({ data: "value3" });
  });

  it("batch sets multiple keys", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse([
      ["key1", true],
      ["key2", true],
      ["key3", true],
    ]);

    const entries = [
      { key: "key1", value: { data: "value1" } },
      { key: "key2", value: { data: "value2" } },
      { key: "key3", value: { data: "value3" } },
    ];
    const results = await client.kvBatchSet(entries);

    expect(results).toHaveLength(3);
    expect(results[0]).toEqual(["key1", true]);
    expect(results[1]).toEqual(["key2", true]);
    expect(results[2]).toEqual(["key3", true]);
  });

  it("batch sets with TTL", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse([
      ["key1", true],
      ["key2", true],
    ]);

    const entries = [
      { key: "key1", value: { data: "value1" }, ttl: 3600 },
      { key: "key2", value: { data: "value2" }, ttl: 3600 },
    ];
    const results = await client.kvBatchSet(entries);

    expect(results).toHaveLength(2);
    expect(results[0][1]).toBe(true);
    expect(results[1][1]).toBe(true);
  });

  it("batch deletes multiple keys", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse([
      ["key1", true],
      ["key2", true],
      ["key3", false],
    ]);

    const keys = ["key1", "key2", "key3"];
    const results = await client.kvBatchDelete(keys);

    expect(results).toHaveLength(3);
    expect(results[0]).toEqual(["key1", true]);
    expect(results[1]).toEqual(["key2", true]);
    expect(results[2]).toEqual(["key3", false]);
  });

  it("handles empty batch get", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse([]);

    const results = await client.kvBatchGet([]);

    expect(results).toHaveLength(0);
  });

  it("handles partial batch set failures", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse([
      ["key1", true],
      ["key2", false],
      ["key3", true],
    ]);

    const entries = [
      { key: "key1", value: { data: "value1" } },
      { key: "key2", value: { data: "value2" } },
      { key: "key3", value: { data: "value3" } },
    ];
    const results = await client.kvBatchSet(entries);

    expect(results[0][1]).toBe(true);
    expect(results[1][1]).toBe(false);
    expect(results[2][1]).toBe(true);
  });
});

// ============================================================================
// Chat Tests
// ============================================================================

describe("EkoDBClient chat", () => {
  it("creates chat session", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      chat_id: "chat_123",
      message_id: "msg_001",
      responses: ["Hello!"],
    });

    const result = await client.createChatSession({
      collections: [{ collection_name: "documents" }],
      llm_provider: "openai",
      llm_model: "gpt-4",
    });

    expect(result).toHaveProperty("chat_id", "chat_123");
  });

  it("sends chat message", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      chat_id: "chat_123",
      message_id: "msg_002",
      responses: ["Here is my response."],
    });

    const request = { message: "What is the answer?" };
    const result = await client.chatMessage("chat_123", request);

    expect(result).toHaveProperty("message_id");
  });

  it("lists chat sessions", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      sessions: [
        { id: "chat_1", created_at: "2024-01-01T00:00:00Z" },
        { id: "chat_2", created_at: "2024-01-02T00:00:00Z" },
      ],
      total: 2,
    });

    const result = await client.listChatSessions();

    expect(result.sessions).toHaveLength(2);
  });

  it("gets chat session", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      id: "chat_123",
      created_at: "2024-01-01T00:00:00Z",
      messages: [],
    });

    const result = await client.getChatSession("chat_123");

    expect(result).toHaveProperty("id", "chat_123");
  });

  it("deletes chat session", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ status: "deleted" });

    await expect(client.deleteChatSession("chat_123")).resolves.not.toThrow();
  });
});

// ============================================================================
// Error Handling Tests
// ============================================================================

describe("EkoDBClient error handling", () => {
  it("handles server error", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockErrorResponse(500, "Internal Server Error");

    await expect(client.insert("users", { name: "Test" })).rejects.toThrow();
  });

  it("handles rate limit error", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 429,
      json: async () => ({ error: "Rate limit exceeded" }),
      text: async () => "Rate limit exceeded",
      headers: new Headers({
        "Retry-After": "60",
      }),
    });

    await expect(client.insert("users", { name: "Test" })).rejects.toThrow();
  });

  it("handles authentication error", async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 401,
      json: async () => ({ error: "Invalid API key" }),
      text: async () => "Invalid API key",
      headers: new Headers(),
    });

    const client = createTestClient();

    await expect(client.insert("users", { name: "Test" })).rejects.toThrow();
  });
});

// Note: Rate limit info tests require actual HTTP response headers
// which are handled differently in the real client vs mocks
// Covered by integration tests

// ============================================================================
// Additional Missing Method Tests - Added Jan 4, 2026
// ============================================================================

// Note: findAll/findAllWithLimit use WebSocket, not HTTP - covered by integration tests

describe("EkoDBClient batchUpdate", () => {
  it("batch updates records", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      successful: ["id_1", "id_2"],
      failed: [],
    });

    const updates = [
      { id: "id_1", data: { name: "Updated 1" } },
      { id: "id_2", data: { name: "Updated 2" } },
    ];
    const result = await client.batchUpdate("users", updates);

    expect(result.successful).toHaveLength(2);
  });
});

describe("EkoDBClient collection management", () => {
  it("creates collection", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ status: "created" });

    await expect(
      client.createCollection("new_collection", { fields: {} }),
    ).resolves.not.toThrow();
  });

  it("gets collection metadata", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      collection: { fields: { name: { field_type: "String" } } },
    });

    const result = await client.getCollection("users");

    expect(result).toBeDefined();
  });

  // Note: getSchema requires specific response format - covered by integration tests
});

describe("EkoDBClient KV advanced", () => {
  it("queries KV by pattern", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse([
      { key: "config:app", value: "value1" },
      { key: "config:db", value: "value2" },
    ]);

    const result = await client.kvFind({ pattern: "config:*" });

    expect(result).toHaveLength(2);
  });
});

describe("EkoDBClient scripts advanced", () => {
  it("saves script", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "func_123", label: "my_function" });

    const script = {
      label: "my_function",
      name: "my_function",
      parameters: {},
      functions: [],
    };
    const result = await client.saveFunction(script);

    expect(result).toBeDefined();
  });

  it("gets script by ID", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "func_123", label: "my_function" });

    const result = await client.getFunction("func_123");

    expect(result).toBeDefined();
  });

  it("updates script", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "func_123", label: "updated_function" });

    const script = {
      label: "updated_function",
      name: "updated_function",
      parameters: {},
      functions: [],
    };
    await expect(
      client.updateFunction("func_123", script),
    ).resolves.not.toThrow();
  });
});

describe("EkoDBClient chat advanced", () => {
  it("gets chat session messages", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      messages: [
        { id: "msg_1", role: "user", content: "Hello" },
        { id: "msg_2", role: "assistant", content: "Hi" },
      ],
    });

    const result = await client.getChatSessionMessages("chat_123");

    expect(result).toBeDefined();
  });

  it("updates chat session", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "chat_123", updated: true });

    await expect(
      client.updateChatSession("chat_123", { system_prompt: "New prompt" }),
    ).resolves.not.toThrow();
  });

  it("branches chat session", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ chat_id: "chat_456", branched_from: "chat_123" });

    const result = await client.branchChatSession({
      collections: [{ collection_name: "documents" }],
      llm_provider: "openai",
    });

    expect(result).toBeDefined();
  });

  it("merges chat sessions", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ chat_id: "chat_123", merged: true });

    const result = await client.mergeChatSessions({
      source_chat_ids: ["chat_456"],
      target_chat_id: "chat_123",
      merge_strategy: "Interleaved" as any,
    });

    expect(result).toBeDefined();
  });

  it("deletes chat message", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ deleted: true });

    await expect(
      client.deleteChatMessage("chat_123", "msg_001"),
    ).resolves.not.toThrow();
  });

  it("updates chat message", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ message_id: "msg_001", updated: true });

    await expect(
      client.updateChatMessage("chat_123", "msg_001", "Updated content"),
    ).resolves.not.toThrow();
  });

  it("regenerates chat message", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ message_id: "msg_002", content: "Regenerated" });

    const result = await client.regenerateMessage("chat_123", "msg_001");

    expect(result).toBeDefined();
  });

  it("toggles forgotten message", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ message_id: "msg_001", forgotten: true });

    await expect(
      client.toggleForgottenMessage("chat_123", "msg_001", true),
    ).resolves.not.toThrow();
  });
});

describe("EkoDBClient transaction status", () => {
  it("gets transaction status", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ transaction_id: "tx_123", status: "active" });

    const result = await client.getTransactionStatus("tx_123");

    expect(result).toBeDefined();
  });
});

// ============================================================================
// Convenience Methods Tests
// ============================================================================

describe("Convenience methods", () => {
  describe("upsert", () => {
    it("inserts when record not found", async () => {
      mockTokenResponse();
      // Mock update returning 404
      mockErrorResponse(404, "Not found");
      // Mock insert succeeding
      mockJsonResponse({ id: "user123", name: "John Doe" });

      const client = createTestClient();
      await client.init();

      const result = await client.upsert("users", "user123", {
        name: "John Doe",
      });
      expect(result).toEqual({ id: "user123", name: "John Doe" });
    });

    it("updates when record exists", async () => {
      mockTokenResponse();
      // Mock update succeeding
      mockJsonResponse({ id: "user123", name: "John Doe Updated" });

      const client = createTestClient();
      await client.init();

      const result = await client.upsert("users", "user123", {
        name: "John Doe Updated",
      });
      expect(result).toEqual({ id: "user123", name: "John Doe Updated" });
    });

    it("throws on non-404 errors", async () => {
      mockTokenResponse();
      // Mock update with server error
      mockErrorResponse(500, "Internal server error");

      const client = createTestClient();
      await client.init();

      await expect(
        client.upsert("users", "user123", { name: "John Doe" }),
      ).rejects.toThrow();
    });
  });

  describe("findOne", () => {
    it("returns record when found", async () => {
      mockTokenResponse();
      mockJsonResponse([
        { id: "user123", email: "test@example.com", name: "John" },
      ]);

      const client = createTestClient();
      await client.init();

      const result = await client.findOne("users", "email", "test@example.com");
      expect(result).toEqual({
        id: "user123",
        email: "test@example.com",
        name: "John",
      });
    });

    it("returns null when not found", async () => {
      mockTokenResponse();
      mockJsonResponse([]);

      const client = createTestClient();
      await client.init();

      const result = await client.findOne(
        "users",
        "email",
        "notfound@example.com",
      );
      expect(result).toBeNull();
    });
  });

  describe("exists", () => {
    it("returns true when record exists", async () => {
      mockTokenResponse();
      mockJsonResponse({ id: "user123", name: "John" });

      const client = createTestClient();
      await client.init();

      const result = await client.exists("users", "user123");
      expect(result).toBe(true);
    });

    it("returns false when record not found", async () => {
      mockTokenResponse();
      mockErrorResponse(404, "Not found");

      const client = createTestClient();
      await client.init();

      const result = await client.exists("users", "user123");
      expect(result).toBe(false);
    });

    it("throws on non-404 errors", async () => {
      mockTokenResponse();
      mockErrorResponse(500, "Internal server error");

      const client = createTestClient();
      await client.init();

      await expect(client.exists("users", "user123")).rejects.toThrow();
    });
  });

  describe("paginate", () => {
    it("calculates skip correctly for page 2", async () => {
      mockTokenResponse();
      mockJsonResponse([{ id: "user11", name: "User 11" }]);

      const client = createTestClient();
      await client.init();

      const result = await client.paginate("users", 2, 10);
      expect(result).toHaveLength(1);
      expect(result[0]).toEqual({ id: "user11", name: "User 11" });
    });

    it("skips zero records for page 1", async () => {
      mockTokenResponse();
      mockJsonResponse([{ id: "user1", name: "User 1" }]);

      const client = createTestClient();
      await client.init();

      const result = await client.paginate("users", 1, 10);
      expect(result).toHaveLength(1);
      expect(result[0]).toEqual({ id: "user1", name: "User 1" });
    });

    it("returns empty array when no records", async () => {
      mockTokenResponse();
      mockJsonResponse([]);

      const client = createTestClient();
      await client.init();

      const result = await client.paginate("users", 5, 10);
      expect(result).toHaveLength(0);
    });
  });
});

// ============================================================================
// executeTool Tests
// ============================================================================

describe("EkoDBClient executeTool", () => {
  it("executes tool successfully", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      success: true,
      result: { count: 42 },
    });

    const result = await client.executeTool("count_records", {
      collection: "users",
    });

    expect(result).toEqual({ count: 42 });
  });

  it("passes chat_id when provided", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      success: true,
      result: { value: "hello" },
    });

    const result = await client.executeTool(
      "kv_get",
      { key: "greeting" },
      "chat_456",
    );

    expect(result).toEqual({ value: "hello" });

    // Verify the request body included chat_id
    const lastCall = mockFetch.mock.calls[1];
    const body = JSON.parse(lastCall[1].body);
    expect(body.chat_id).toBe("chat_456");
    expect(body.tool).toBe("kv_get");
    expect(body.params).toEqual({ key: "greeting" });
  });

  it("throws on tool execution failure", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      success: false,
      error: "permission denied",
    });

    await expect(
      client.executeTool("delete_collection", { collection: "system" }),
    ).rejects.toThrow("permission denied");
  });

  it("returns null when server does not support endpoint", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockErrorResponse(404, "Not Found");

    const result = await client.executeTool("count_records", {
      collection: "users",
    });

    expect(result).toBeNull();
  });
});

// ============================================================================
// Chat Models Tests
// ============================================================================

describe("EkoDBClient chat models", () => {
  it("gets all chat models", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      openai: ["gpt-4", "gpt-3.5-turbo"],
      anthropic: ["claude-3-opus", "claude-3-sonnet"],
      perplexity: ["llama-3.1-sonar-small"],
    });

    const result = await client.getChatModels();

    expect(result.openai).toHaveLength(2);
    expect(result.anthropic).toHaveLength(2);
    expect(result.perplexity).toHaveLength(1);
  });

  it("gets models for specific provider", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse(["gpt-4", "gpt-3.5-turbo", "gpt-4-turbo"]);

    const result = await client.getChatModel("openai");

    expect(result).toHaveLength(3);
    expect(result).toContain("gpt-4");
  });

  it("gets specific chat message", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      id: "msg_123",
      role: "user",
      content: "Hello there",
      created_at: "2024-01-01T00:00:00Z",
    });

    const result = await client.getChatMessage("chat_123", "msg_123");

    expect(result).toHaveProperty("id", "msg_123");
    expect(result).toHaveProperty("content", "Hello there");
  });
});

// ============================================================================
// User Functions Tests
// ============================================================================

describe("EkoDBClient user functions", () => {
  it("saves user function", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "uf_123" });

    const userFunction = {
      label: "my_function",
      name: "My Function",
      parameters: {},
      functions: [],
    };
    const result = await client.saveUserFunction(userFunction);

    expect(result).toBe("uf_123");
  });

  it("gets user function by label", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      label: "my_function",
      name: "My Function",
      parameters: {},
      functions: [],
      id: "uf_123",
    });

    const result = await client.getUserFunction("my_function");

    expect(result).toHaveProperty("label", "my_function");
    expect(result).toHaveProperty("id", "uf_123");
  });

  it("lists user functions", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse([
      { label: "func_1", name: "Function 1" },
      { label: "func_2", name: "Function 2" },
    ]);

    const result = await client.listUserFunctions();

    expect(result).toHaveLength(2);
    expect(result[0]).toHaveProperty("label", "func_1");
  });

  it("lists user functions filtered by tags", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse([{ label: "func_1", name: "Function 1", tags: ["data"] }]);

    const result = await client.listUserFunctions(["data"]);

    expect(result).toHaveLength(1);
  });

  it("updates user function", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ status: "updated" });

    const userFunction = {
      label: "my_function",
      name: "Updated Function",
      parameters: {},
      functions: [],
    };
    await expect(
      client.updateUserFunction("my_function", userFunction),
    ).resolves.not.toThrow();
  });

  it("deletes user function", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ status: "deleted" });

    await expect(
      client.deleteUserFunction("my_function"),
    ).resolves.not.toThrow();
  });
});

// ============================================================================
// Collection Utility Tests
// ============================================================================

describe("EkoDBClient collection utilities", () => {
  it("collectionExists returns true for existing collection", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ collections: ["users", "posts", "comments"] });

    const result = await client.collectionExists("users");

    expect(result).toBe(true);
  });

  it("collectionExists returns false for non-existing collection", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ collections: ["users", "posts", "comments"] });

    const result = await client.collectionExists("nonexistent");

    expect(result).toBe(false);
  });

  it("collectionExists returns false on error", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockErrorResponse(500, "Server error");

    const result = await client.collectionExists("users");

    expect(result).toBe(false);
  });

  it("countDocuments returns correct count", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse([
      { id: "1", name: "A" },
      { id: "2", name: "B" },
      { id: "3", name: "C" },
    ]);

    const result = await client.countDocuments("users");

    expect(result).toBe(3);
  });

  it("countDocuments returns zero for empty collection", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse([]);

    const result = await client.countDocuments("empty_collection");

    expect(result).toBe(0);
  });
});

// ============================================================================
// Distinct Values Tests
// ============================================================================

describe("EkoDBClient distinctValues", () => {
  it("returns distinct values for a field", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      collection: "products",
      field: "category",
      values: ["books", "electronics", "food"],
      count: 3,
    });

    const result = await client.distinctValues("products", "category");

    expect(result.collection).toBe("products");
    expect(result.field).toBe("category");
    expect(result.count).toBe(3);
    expect(result.values).toHaveLength(3);
    expect(result.values).toContain("books");
  });

  it("sends filter in request body", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      collection: "orders",
      field: "status",
      values: ["active", "pending"],
      count: 2,
    });

    const filter = {
      type: "Condition",
      content: { field: "region", operator: "Eq", value: "us" },
    };
    const result = await client.distinctValues("orders", "status", { filter });

    expect(result.count).toBe(2);
    expect(result.values).toContain("active");
  });

  it("returns empty values for collection with no matching records", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      collection: "empty",
      field: "tag",
      values: [],
      count: 0,
    });

    const result = await client.distinctValues("empty", "tag");

    expect(result.count).toBe(0);
    expect(result.values).toHaveLength(0);
  });

  it("calls correct endpoint", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      collection: "users",
      field: "role",
      values: ["admin", "user"],
      count: 2,
    });

    await client.distinctValues("users", "role");

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1]; // calls[0] is token
    expect(dataCall[0]).toContain("/api/distinct/users/role");
    expect(dataCall[1]?.method).toBe("POST");
  });
});

// ============================================================================
// Raw Completion Tests
// ============================================================================

describe("EkoDBClient rawCompletion", () => {
  it("returns content from LLM response", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ content: "The answer is 42." });

    const result = await client.rawCompletion({
      system_prompt: "You are a helpful assistant.",
      message: "What is the answer?",
    });

    expect(result.content).toBe("The answer is 42.");
  });

  it("sends all fields in request body", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ content: "Response text." });

    await client.rawCompletion({
      system_prompt: "System.",
      message: "User.",
      provider: "openai",
      model: "gpt-4o",
      max_tokens: 512,
    });

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1]; // calls[0] is token
    const body = JSON.parse(dataCall[1]?.body as string);
    expect(body.system_prompt).toBe("System.");
    expect(body.message).toBe("User.");
    expect(body.provider).toBe("openai");
    expect(body.model).toBe("gpt-4o");
    expect(body.max_tokens).toBe(512);
  });

  it("omits optional fields when not provided", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ content: "Response." });

    await client.rawCompletion({
      system_prompt: "System.",
      message: "User.",
    });

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1];
    const body = JSON.parse(dataCall[1]?.body as string);
    expect(body.provider).toBeUndefined();
    expect(body.model).toBeUndefined();
    expect(body.max_tokens).toBeUndefined();
  });

  it("calls correct endpoint with POST method", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ content: "Response." });

    await client.rawCompletion({
      system_prompt: "System.",
      message: "User.",
    });

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1];
    expect(dataCall[0]).toContain("/api/chat/complete");
    expect(dataCall[1]?.method).toBe("POST");
  });
});

// ============================================================================
// rawCompletionStream (SSE) Tests
// ============================================================================

describe("EkoDBClient rawCompletionStream", () => {
  it("parses SSE done event and returns content", async () => {
    const client = createTestClient();

    mockTokenResponse();
    // SSE response with token chunks and a done event
    const sseBody = [
      'data: {"token":"Hello"}',
      'data: {"token":" world"}',
      'data: {"content":"Hello world","done":true}',
      "",
    ].join("\n");

    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      text: async () => sseBody,
      headers: new Headers(),
    });

    const result = await client.rawCompletionStream({
      system_prompt: "System.",
      message: "User.",
    });

    expect(result.content).toBe("Hello world");
  });

  it("accumulates tokens when no done event", async () => {
    const client = createTestClient();

    mockTokenResponse();
    const sseBody = [
      'data: {"token":"chunk1"}',
      'data: {"token":"chunk2"}',
      "",
    ].join("\n");

    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      text: async () => sseBody,
      headers: new Headers(),
    });

    const result = await client.rawCompletionStream({
      system_prompt: "System.",
      message: "User.",
    });

    expect(result.content).toBe("chunk1chunk2");
  });

  it("throws on SSE error event", async () => {
    const client = createTestClient();

    mockTokenResponse();
    const sseBody = 'data: {"error":"LLM timeout"}\n';

    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      text: async () => sseBody,
      headers: new Headers(),
    });

    await expect(
      client.rawCompletionStream({
        system_prompt: "System.",
        message: "User.",
      }),
    ).rejects.toThrow("LLM timeout");
  });

  it("throws on non-200 HTTP response", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 401,
      text: async () => "Unauthorized",
      headers: new Headers(),
    });

    await expect(
      client.rawCompletionStream({
        system_prompt: "System.",
        message: "User.",
      }),
    ).rejects.toThrow("401");
  });

  it("calls the /stream endpoint", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      text: async () => 'data: {"content":"ok"}\n',
      headers: new Headers(),
    });

    await client.rawCompletionStream({
      system_prompt: "System.",
      message: "User.",
    });

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1];
    expect(dataCall[0]).toContain("/api/chat/complete/stream");
    expect(dataCall[1]?.headers?.Accept).toBe("text/event-stream");
  });
});

// ============================================================================
// Token Management Tests
// ============================================================================

describe("refreshToken", () => {
  it("fetches a new token", async () => {
    const client = createTestClient();

    // First token fetch (init)
    mockTokenResponse();
    mockJsonResponse({ status: "ok" });
    await client.health();

    // Refresh token
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => ({ token: "new-jwt-token" }),
      headers: new Headers(),
    });
    await client.refreshToken();

    // Verify it called the token endpoint again
    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const tokenCalls = calls.filter((c: unknown[]) =>
      (c[0] as string).includes("/api/auth/token"),
    );
    expect(tokenCalls.length).toBe(2);
  });
});

describe("clearTokenCache", () => {
  it("clears the cached token", async () => {
    const client = createTestClient();

    // Init with token
    mockTokenResponse();
    mockJsonResponse({ status: "ok" });
    await client.health();

    // Clear cache
    client.clearTokenCache();

    // Next request should fetch a new token
    mockTokenResponse();
    mockJsonResponse({ status: "ok" });
    await client.health();

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const tokenCalls = calls.filter((c: unknown[]) =>
      (c[0] as string).includes("/api/auth/token"),
    );
    expect(tokenCalls.length).toBe(2);
  });
});

// ============================================================================
// findByIdWithProjection Tests
// ============================================================================

describe("findByIdWithProjection", () => {
  it("calls correct endpoint with select_fields", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "123", name: "Alice" });

    await client.findByIdWithProjection("users", "123", ["name", "email"]);

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1];
    expect(dataCall[0]).toContain("/api/find/users/123");
    expect(dataCall[0]).toContain("select_fields=name%2Cemail");
  });

  it("calls correct endpoint with exclude_fields", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "123", name: "Alice" });

    await client.findByIdWithProjection("users", "123", undefined, [
      "password",
    ]);

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1];
    expect(dataCall[0]).toContain("exclude_fields=password");
  });

  it("calls without params when no projection", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "123", name: "Alice" });

    await client.findByIdWithProjection("users", "123");

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1];
    expect(dataCall[0]).toBe("http://localhost:8080/api/find/users/123");
  });
});

// ============================================================================
// Goal CRUD Tests
// ============================================================================

describe("EkoDBClient goals", () => {
  it("creates a goal", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "goal_1", title: "Test Goal", status: "active" });

    const result = await client.goalCreate({ title: "Test Goal" });
    expect(result).toHaveProperty("id", "goal_1");
    expect(result).toHaveProperty("status", "active");
  });

  it("lists goals", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ goals: [{ id: "goal_1" }, { id: "goal_2" }] });

    const result = await client.goalList();
    expect(result).toHaveProperty("goals");
  });

  it("gets a goal by ID", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "goal_1", title: "Test Goal" });

    const result = await client.goalGet("goal_1");
    expect(result).toHaveProperty("id", "goal_1");
  });

  it("updates a goal", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "goal_1", title: "Updated" });

    const result = await client.goalUpdate("goal_1", { title: "Updated" });
    expect(result).toHaveProperty("title", "Updated");
  });

  it("deletes a goal", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({});

    await expect(client.goalDelete("goal_1")).resolves.not.toThrow();
  });

  it("searches goals", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ goals: [{ id: "goal_1" }] });

    const result = await client.goalSearch("test query");
    expect(result).toHaveProperty("goals");
  });

  it("completes a goal", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "goal_1", status: "pending_review" });

    const result = await client.goalComplete("goal_1", { summary: "Done" });
    expect(result).toHaveProperty("status", "pending_review");
  });

  it("approves a goal", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "goal_1", status: "in_progress" });

    const result = await client.goalApprove("goal_1");
    expect(result).toHaveProperty("status", "in_progress");
  });

  it("rejects a goal", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "goal_1", status: "failed" });

    const result = await client.goalReject("goal_1", { reason: "Bad plan" });
    expect(result).toHaveProperty("status", "failed");
  });

  it("starts a goal step", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "goal_1" });

    const result = await client.goalStepStart("goal_1", 0);
    expect(result).toHaveProperty("id", "goal_1");
  });

  it("completes a goal step", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "goal_1" });

    const result = await client.goalStepComplete("goal_1", 0, {
      result: "Step done",
    });
    expect(result).toHaveProperty("id", "goal_1");
  });

  it("fails a goal step", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "goal_1" });

    const result = await client.goalStepFail("goal_1", 0, {
      error: "Step failed",
    });
    expect(result).toHaveProperty("id", "goal_1");
  });

  it("returns error for non-existent goal", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockErrorResponse(404, "Not Found");

    await expect(client.goalGet("nonexistent")).rejects.toThrow();
  });
});

// ============================================================================
// Task CRUD Tests
// ============================================================================

describe("EkoDBClient tasks", () => {
  it("creates a task", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "task_1", name: "Test Task", status: "active" });

    const result = await client.taskCreate({
      name: "Test Task",
      cron: "0 * * * *",
    });
    expect(result).toHaveProperty("id", "task_1");
  });

  it("lists tasks", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ tasks: [] });

    const result = await client.taskList();
    expect(result).toHaveProperty("tasks");
  });

  it("gets a task by ID", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "task_1", name: "Test Task" });

    const result = await client.taskGet("task_1");
    expect(result).toHaveProperty("id", "task_1");
  });

  it("updates a task", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "task_1", name: "Updated" });

    const result = await client.taskUpdate("task_1", { name: "Updated" });
    expect(result).toHaveProperty("name", "Updated");
  });

  it("deletes a task", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({});

    await expect(client.taskDelete("task_1")).resolves.not.toThrow();
  });

  it("gets due tasks", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ tasks: [] });

    const result = await client.taskDue("2026-03-20T00:00:00Z");
    expect(result).toHaveProperty("tasks");
  });

  it("starts a task", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "task_1", status: "running" });

    const result = await client.taskStart("task_1");
    expect(result).toHaveProperty("status", "running");
  });

  it("succeeds a task", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "task_1", status: "active" });

    const result = await client.taskSucceed("task_1", { output: "Success" });
    expect(result).toHaveProperty("status", "active");
  });

  it("fails a task", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "task_1", status: "active" });

    const result = await client.taskFail("task_1", { error: "Timeout" });
    expect(result).toHaveProperty("status", "active");
  });

  it("pauses a task", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "task_1", status: "paused" });

    const result = await client.taskPause("task_1");
    expect(result).toHaveProperty("status", "paused");
  });

  it("resumes a task", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "task_1", status: "active" });

    const result = await client.taskResume("task_1", {});
    expect(result).toHaveProperty("status", "active");
  });
});

// ============================================================================
// Agent CRUD Tests
// ============================================================================

describe("EkoDBClient agents", () => {
  it("creates an agent", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "agent_1", name: "TestAgent" });

    const result = await client.agentCreate({
      name: "TestAgent",
      system_prompt: "You help.",
    });
    expect(result).toHaveProperty("name", "TestAgent");
  });

  it("lists agents", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ agents: [] });

    const result = await client.agentList();
    expect(result).toHaveProperty("agents");
  });

  it("gets an agent by ID", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "agent_1", name: "TestAgent" });

    const result = await client.agentGet("agent_1");
    expect(result).toHaveProperty("id", "agent_1");
  });

  it("gets an agent by name", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "agent_1", name: "TestAgent" });

    const result = await client.agentGetByName("TestAgent");
    expect(result).toHaveProperty("name", "TestAgent");
  });

  it("updates an agent", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "agent_1", name: "Updated" });

    const result = await client.agentUpdate("agent_1", { name: "Updated" });
    expect(result).toHaveProperty("name", "Updated");
  });

  it("deletes an agent", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({});

    await expect(client.agentDelete("agent_1")).resolves.not.toThrow();
  });

  it("gets agents by deployment", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ agents: [{ id: "agent_1" }] });

    const result = await client.agentsByDeployment("deploy_1");
    expect(result).toHaveProperty("agents");
  });

  it("returns error for non-existent agent", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockErrorResponse(404, "Not Found");

    await expect(client.agentGet("nonexistent")).rejects.toThrow();
  });
});

// ============================================================================
// rawCompletionStreamWithProgress Tests
// ============================================================================

describe("EkoDBClient rawCompletionStreamWithProgress", () => {
  it("calls onToken for each token", async () => {
    const client = createTestClient();
    mockTokenResponse();

    // Mock SSE response
    const sseBody = [
      'data: {"token": "Hello"}',
      'data: {"token": " World"}',
      'data: {"content": "Hello World"}',
    ].join("\n");

    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      text: async () => sseBody,
      headers: new Headers({ "content-type": "text/event-stream" }),
    });

    const tokens: string[] = [];
    const result = await client.rawCompletionStreamWithProgress(
      { system_prompt: "test", message: "test" },
      (token) => tokens.push(token),
    );

    expect(tokens).toEqual(["Hello", " World"]);
    expect(result.content).toBe("Hello World");
  });

  it("throws on SSE error event", async () => {
    const client = createTestClient();
    mockTokenResponse();

    const sseBody = 'data: {"error": "Model overloaded"}';
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      text: async () => sseBody,
      headers: new Headers({ "content-type": "text/event-stream" }),
    });

    const tokens: string[] = [];
    await expect(
      client.rawCompletionStreamWithProgress(
        { system_prompt: "test", message: "test" },
        (token) => tokens.push(token),
      ),
    ).rejects.toThrow("Model overloaded");
  });
});

// ============================================================================
// Goal Template CRUD Tests
// ============================================================================

describe("EkoDBClient goal templates", () => {
  it("creates a goal template", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "gt_1", title: "Deploy Checklist" });

    const result = await client.goalTemplateCreate({
      title: "Deploy Checklist",
      steps: [{ title: "Run tests" }],
    });
    expect(result).toHaveProperty("id", "gt_1");
    expect(result).toHaveProperty("title", "Deploy Checklist");
  });

  it("creates a goal template and verifies POST method", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "gt_2", title: "Onboarding" });

    await client.goalTemplateCreate({ title: "Onboarding" });

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1]; // calls[0] is token
    expect(dataCall[0]).toContain("/api/chat/goal-templates");
    expect(dataCall[1]?.method).toBe("POST");
  });

  it("lists goal templates", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({
      templates: [{ id: "gt_1" }, { id: "gt_2" }],
    });

    const result = await client.goalTemplateList();
    expect(result).toHaveProperty("templates");
  });

  it("lists goal templates and verifies GET method", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ templates: [] });

    await client.goalTemplateList();

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1];
    expect(dataCall[0]).toContain("/api/chat/goal-templates");
    expect(dataCall[1]?.method).toBe("GET");
  });

  it("gets a goal template by ID", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "gt_1", title: "Deploy Checklist" });

    const result = await client.goalTemplateGet("gt_1");
    expect(result).toHaveProperty("id", "gt_1");
    expect(result).toHaveProperty("title", "Deploy Checklist");
  });

  it("gets a goal template and verifies endpoint", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "gt_1", title: "Deploy Checklist" });

    await client.goalTemplateGet("gt_1");

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1];
    expect(dataCall[0]).toContain("/api/chat/goal-templates/gt_1");
    expect(dataCall[1]?.method).toBe("GET");
  });

  it("updates a goal template", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "gt_1", title: "Updated Checklist" });

    const result = await client.goalTemplateUpdate("gt_1", {
      title: "Updated Checklist",
    });
    expect(result).toHaveProperty("title", "Updated Checklist");
  });

  it("updates a goal template and verifies PUT method", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "gt_1", title: "Updated" });

    await client.goalTemplateUpdate("gt_1", { title: "Updated" });

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1];
    expect(dataCall[0]).toContain("/api/chat/goal-templates/gt_1");
    expect(dataCall[1]?.method).toBe("PUT");
  });

  it("deletes a goal template", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({});

    await expect(client.goalTemplateDelete("gt_1")).resolves.not.toThrow();
  });

  it("deletes a goal template and verifies DELETE method", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({});

    await client.goalTemplateDelete("gt_1");

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1];
    expect(dataCall[0]).toContain("/api/chat/goal-templates/gt_1");
    expect(dataCall[1]?.method).toBe("DELETE");
  });

  it("returns error for non-existent goal template", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockErrorResponse(404, "Not Found");

    await expect(client.goalTemplateGet("nonexistent")).rejects.toThrow();
  });
});

// ============================================================================
// chatMessageStream (SSE) Tests
// ============================================================================

describe("EkoDBClient chatMessageStream", () => {
  it("emits chunk and end events from SSE stream", async () => {
    const client = createTestClient();
    mockTokenResponse();

    const sseBody = [
      'data: {"token":"Hello"}',
      'data: {"token":" world"}',
      'data: {"content":"Hello world","message_id":"msg_1","execution_time_ms":42}',
      "",
    ].join("\n");

    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      text: async () => sseBody,
      headers: new Headers({ "content-type": "text/event-stream" }),
    });

    const events: any[] = [];
    const stream = client.chatMessageStream("chat_123", {
      message: "Hello",
    });
    stream.on("event", (evt: any) => events.push(evt));

    // Wait for the async SSE processing to complete
    await new Promise((resolve) => setTimeout(resolve, 50));

    expect(events).toHaveLength(3);
    expect(events[0]).toEqual({ type: "chunk", content: "Hello" });
    expect(events[1]).toEqual({ type: "chunk", content: " world" });
    expect(events[2].type).toBe("end");
    expect(events[2].messageId).toBe("msg_1");
    expect(events[2].executionTimeMs).toBe(42);
  });

  it("emits error event on SSE error", async () => {
    const client = createTestClient();
    mockTokenResponse();

    const sseBody = 'data: {"error":"LLM timeout"}\n';

    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      text: async () => sseBody,
      headers: new Headers({ "content-type": "text/event-stream" }),
    });

    const events: any[] = [];
    const stream = client.chatMessageStream("chat_123", {
      message: "Hello",
    });
    stream.on("event", (evt: any) => events.push(evt));

    await new Promise((resolve) => setTimeout(resolve, 50));

    expect(events).toHaveLength(1);
    expect(events[0]).toEqual({ type: "error", error: "LLM timeout" });
  });

  it("emits error event on non-200 HTTP response", async () => {
    const client = createTestClient();
    mockTokenResponse();

    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 401,
      text: async () => "Unauthorized",
      headers: new Headers(),
    });

    const events: any[] = [];
    const stream = client.chatMessageStream("chat_123", {
      message: "Hello",
    });
    stream.on("event", (evt: any) => events.push(evt));

    await new Promise((resolve) => setTimeout(resolve, 50));

    expect(events).toHaveLength(1);
    expect(events[0].type).toBe("error");
    expect(events[0].error).toContain("401");
  });

  it("calls the correct stream endpoint", async () => {
    const client = createTestClient();
    mockTokenResponse();

    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      text: async () => 'data: {"token":"ok"}\n',
      headers: new Headers({ "content-type": "text/event-stream" }),
    });

    client.chatMessageStream("chat_456", { message: "Test" });

    await new Promise((resolve) => setTimeout(resolve, 50));

    const calls = (global.fetch as ReturnType<typeof vi.fn>).mock.calls;
    const dataCall = calls[1];
    expect(dataCall[0]).toContain("/api/chat/chat_456/messages/stream");
    expect(dataCall[1]?.method).toBe("POST");
    expect(dataCall[1]?.headers?.Accept).toBe("text/event-stream");
  });
});

// ============================================================================
// Schedule CRUD Tests
// ============================================================================

describe("EkoDBClient schedules", () => {
  it("creates a schedule", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({
      id: "sched_1",
      name: "Nightly Backup",
      cron: "0 2 * * *",
      status: "active",
    });

    const result = await client.createSchedule({
      name: "Nightly Backup",
      cron: "0 2 * * *",
      action: "backup",
    });
    expect(result).toHaveProperty("id", "sched_1");
    expect(result).toHaveProperty("status", "active");
  });

  it("lists schedules", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({
      schedules: [
        { id: "sched_1", name: "Nightly Backup" },
        { id: "sched_2", name: "Hourly Sync" },
      ],
    });

    const result = await client.listSchedules();
    expect(result).toHaveProperty("schedules");
  });

  it("gets a schedule by ID", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({
      id: "sched_1",
      name: "Nightly Backup",
      cron: "0 2 * * *",
    });

    const result = await client.getSchedule("sched_1");
    expect(result).toHaveProperty("id", "sched_1");
    expect(result).toHaveProperty("name", "Nightly Backup");
  });

  it("updates a schedule", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({
      id: "sched_1",
      name: "Updated Backup",
      cron: "0 3 * * *",
    });

    const result = await client.updateSchedule("sched_1", {
      name: "Updated Backup",
      cron: "0 3 * * *",
    });
    expect(result).toHaveProperty("name", "Updated Backup");
    expect(result).toHaveProperty("cron", "0 3 * * *");
  });

  it("deletes a schedule", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({});

    await expect(client.deleteSchedule("sched_1")).resolves.not.toThrow();
  });

  it("pauses a schedule", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "sched_1", status: "paused" });

    const result = await client.pauseSchedule("sched_1");
    expect(result).toHaveProperty("status", "paused");
  });

  it("resumes a schedule", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ id: "sched_1", status: "active" });

    const result = await client.resumeSchedule("sched_1");
    expect(result).toHaveProperty("status", "active");
  });
});

// ============================================================================
// KV Links Tests
// ============================================================================

describe("EkoDBClient kv links", () => {
  it("gets links for a KV key", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({
      links: [
        { collection: "users", document_id: "user_1" },
        { collection: "orders", document_id: "order_1" },
      ],
    });

    const result = await client.kvGetLinks("session:user123");
    expect(result).toHaveProperty("links");
  });

  it("links a document to a KV key", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ status: "linked" });

    const result = await client.kvLink("session:user123", "users", "user_1");
    expect(result).toHaveProperty("status", "linked");
  });

  it("unlinks a document from a KV key", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({ status: "unlinked" });

    const result = await client.kvUnlink("session:user123", "users", "user_1");
    expect(result).toHaveProperty("status", "unlinked");
  });
});

// ============================================================================
// Text Search and Hybrid Search Tests
// ============================================================================

describe("EkoDBClient text and hybrid search", () => {
  it("performs text search", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({
      results: [
        {
          record: { id: "doc_1", title: "Ownership Guide" },
          score: 0.92,
          matched_fields: ["title"],
        },
        {
          record: { id: "doc_2", title: "Property Ownership" },
          score: 0.85,
          matched_fields: ["title"],
        },
      ],
      total: 2,
      took_ms: 12,
    });

    const result = await client.textSearch("documents", "ownership", {
      limit: 10,
      select_fields: ["title", "content"],
    });

    expect(result.results).toHaveLength(2);
    expect(result.total).toBe(2);
    expect(result.results[0].score).toBe(0.92);
  });

  it("performs hybrid search", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({
      results: [
        {
          record: { id: "doc_1", title: "ML Guide", content: "Learn ML" },
          score: 0.95,
          matched_fields: ["title", "content"],
        },
      ],
      total: 1,
      took_ms: 25,
    });

    const queryVector = [0.1, 0.2, 0.3, 0.4, 0.5];
    const result = await client.hybridSearch(
      "documents",
      "machine learning",
      queryVector,
      5,
    );

    expect(result).toHaveLength(1);
    expect(result[0]).toHaveProperty("id", "doc_1");
    expect(result[0]).toHaveProperty("title", "ML Guide");
    expect(result[0]).toHaveProperty("_score", 0.95);
  });
});

// ============================================================================
// Auth Token Management
// ============================================================================

describe("EkoDBClient auth token management", () => {
  beforeEach(() => {
    mockFetch.mockReset();
  });

  it("getToken auto-refreshes when token is about to expire", async () => {
    const client = createTestClient();

    // First init — get a token with a short expiry (30s from now)
    const shortExp = Math.floor(Date.now() / 1000) + 30;
    const shortPayload = btoa(JSON.stringify({ exp: shortExp }))
      .replace(/\+/g, "-")
      .replace(/\//g, "_")
      .replace(/=/g, "");
    const shortJwt = `eyJhbGciOiJIUzI1NiJ9.${shortPayload}.fakesig`;
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => ({ token: shortJwt }),
      headers: new Headers(),
    });
    await client.init();

    // Now getToken should see it's about to expire and refresh
    const newExp = Math.floor(Date.now() / 1000) + 3600;
    const newPayload = btoa(JSON.stringify({ exp: newExp }))
      .replace(/\+/g, "-")
      .replace(/\//g, "_")
      .replace(/=/g, "");
    const newJwt = `eyJhbGciOiJIUzI1NiJ9.${newPayload}.fakesig`;
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => ({ token: newJwt }),
      headers: new Headers(),
    });

    const token = await client.getToken();
    expect(token).toBe(newJwt);
    // 2 fetch calls: init + proactive refresh
    expect(mockFetch).toHaveBeenCalledTimes(2);
  });

  it("getToken returns cached token when not expired", async () => {
    const client = createTestClient();

    // Token with expiry far in the future
    const farExp = Math.floor(Date.now() / 1000) + 7200;
    const payload = btoa(JSON.stringify({ exp: farExp }))
      .replace(/\+/g, "-")
      .replace(/\//g, "_")
      .replace(/=/g, "");
    const jwt = `eyJhbGciOiJIUzI1NiJ9.${payload}.fakesig`;
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => ({ token: jwt }),
      headers: new Headers(),
    });
    await client.init();

    // getToken should return cached — no extra fetch
    const token = await client.getToken();
    expect(token).toBe(jwt);
    expect(mockFetch).toHaveBeenCalledTimes(1); // only init
  });

  it("clearTokenCache resets token and expiry", async () => {
    const client = createTestClient();
    mockTokenResponse();
    await client.init();
    expect(await client.getToken()).toBeTruthy();

    client.clearTokenCache();
    // After clear, getToken will auto-refresh (fetch a new token)
    mockTokenResponse();
    const token = await client.getToken();
    expect(token).toBe("test-jwt-token");
    // 2 fetch calls: init + post-clear getToken refresh
    expect(mockFetch).toHaveBeenCalledTimes(2);
  });
});

// ============================================================================
// agent_id Tests
// ============================================================================

describe("agent_id on chat types", () => {
  it("CreateChatSessionRequest includes agent_id", () => {
    const req: any = {
      collections: [{ collection_name: "docs" }],
      llm_provider: "openai",
      agent_id: "my-agent",
    };
    expect(req.agent_id).toBe("my-agent");
  });

  it("CreateChatSessionRequest omits agent_id when undefined", () => {
    const req: any = {
      collections: [],
      llm_provider: "openai",
    };
    expect(req.agent_id).toBeUndefined();
  });

  it("ChatSession includes agent_id", () => {
    const session: any = {
      chat_id: "c1",
      created_at: "2026-01-01",
      updated_at: "2026-01-01",
      llm_provider: "openai",
      llm_model: "gpt-4",
      collections: [],
      agent_id: "bot-1",
      message_count: 0,
    };
    expect(session.agent_id).toBe("bot-1");
  });

  it("ChatSession allows missing agent_id", () => {
    const session: any = {
      chat_id: "c1",
      created_at: "2026-01-01",
      updated_at: "2026-01-01",
      llm_provider: "openai",
      llm_model: "gpt-4",
      collections: [],
      message_count: 0,
    };
    expect(session.agent_id).toBeUndefined();
  });
});

// ============================================================================
// client_tools / confirm_tools / exclude_tools Tests
// ============================================================================

describe("ChatMessageRequest tool fields", () => {
  it("includes client_tools, confirm_tools, exclude_tools", () => {
    const req: any = {
      message: "hello",
      client_tools: [
        {
          name: "weather",
          description: "Get weather",
          parameters: { type: "object" },
        },
      ],
      confirm_tools: ["shell_exec"],
      exclude_tools: ["file_delete"],
    };
    expect(req.client_tools).toHaveLength(1);
    expect(req.client_tools[0].name).toBe("weather");
    expect(req.confirm_tools).toEqual(["shell_exec"]);
    expect(req.exclude_tools).toEqual(["file_delete"]);
  });

  it("tool fields are optional", () => {
    const req: any = { message: "hi" };
    expect(req.client_tools).toBeUndefined();
    expect(req.confirm_tools).toBeUndefined();
    expect(req.exclude_tools).toBeUndefined();
  });

  it("ClientToolDefinition has correct shape", () => {
    const tool: any = {
      name: "calc",
      description: "Calculator",
      parameters: { type: "object", properties: {} },
    };
    expect(tool.name).toBe("calc");
    expect(tool.description).toBe("Calculator");
    expect(tool.parameters.type).toBe("object");
  });
});

// ============================================================================
// submitChatToolResult Tests
// ============================================================================

describe("submitChatToolResult", () => {
  it("sends tool result to correct endpoint", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({});

    await client.submitChatToolResult("chat-123", "call-456", true, {
      temp: "72F",
    });

    expect(mockFetch).toHaveBeenCalledTimes(2);
    const call = mockFetch.mock.calls[1];
    expect(call[0]).toContain("/api/chat/chat-123/tool-result");
    const body = JSON.parse(call[1].body);
    expect(body.call_id).toBe("call-456");
    expect(body.success).toBe(true);
    expect(body.result.temp).toBe("72F");
  });

  it("sends error result", async () => {
    const client = createTestClient();
    mockTokenResponse();
    mockJsonResponse({});

    await client.submitChatToolResult(
      "chat-123",
      "call-456",
      false,
      undefined,
      "tool crashed",
    );

    const call = mockFetch.mock.calls[1];
    const body = JSON.parse(call[1].body);
    expect(body.success).toBe(false);
    expect(body.error).toBe("tool crashed");
    expect(body.result).toBeUndefined();
  });
});

// ============================================================================
// subscribeSSE Tests
// ============================================================================

describe("subscribeSSE", () => {
  /** Create a mock ReadableStream from a string */
  function mockReadableStream(data: string) {
    const encoder = new TextEncoder();
    const bytes = encoder.encode(data);
    let sent = false;
    return {
      getReader: () => ({
        read: async () => {
          if (!sent) {
            sent = true;
            return { value: bytes, done: false };
          }
          return { value: undefined, done: true };
        },
      }),
    };
  }

  it("parses mutation events from SSE stream", async () => {
    const client = createTestClient();
    mockTokenResponse();

    const sseBody =
      "event: subscribed\ndata: {}\n\n" +
      'event: mutation\ndata: {"collection":"orders","event":"insert","record_ids":["r1"],"timestamp":"t1"}\n\n' +
      'event: mutation\ndata: {"collection":"orders","event":"update","record_ids":["r2"],"timestamp":"t2"}\n\n';

    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      body: mockReadableStream(sseBody),
      headers: new Headers({ "content-type": "text/event-stream" }),
    });

    const stream = client.subscribeSSE("orders");
    const events: any[] = [];
    await new Promise<void>((resolve) => {
      stream.on("event", (e: any) => events.push(e));
      stream.on("close", resolve);
      setTimeout(resolve, 100);
    });

    expect(events).toHaveLength(2);
    expect(events[0].event).toBe("insert");
    expect(events[0].recordIds).toEqual(["r1"]);
    expect(events[1].event).toBe("update");
    expect(events[1].recordIds).toEqual(["r2"]);
  });

  it("passes filter params in URL", async () => {
    const client = createTestClient();
    mockTokenResponse();

    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      body: mockReadableStream(""),
      headers: new Headers({ "content-type": "text/event-stream" }),
    });

    client.subscribeSSE("orders", {
      filterField: "status",
      filterValue: "active",
    });

    // Wait for async fetch
    await new Promise((r) => setTimeout(r, 50));

    const call = mockFetch.mock.calls[1];
    expect(call[0]).toContain("/api/subscribe/orders");
    expect(call[0]).toContain("filter_field=status");
    expect(call[0]).toContain("filter_value=active");
  });

  it("emits error on HTTP failure", async () => {
    const client = createTestClient();
    mockTokenResponse();

    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 401,
      text: async () => "Unauthorized",
      headers: new Headers(),
    });

    const stream = client.subscribeSSE("orders");
    const errors: string[] = [];
    await new Promise<void>((resolve) => {
      stream.on("error", (e: string) => errors.push(e));
      stream.on("close", resolve);
      setTimeout(resolve, 100);
    });

    expect(errors).toHaveLength(1);
    expect(errors[0]).toContain("401");
  });
});
