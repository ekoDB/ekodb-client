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
      timeout: 60000,
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
// Functions/Scripts Tests
// ============================================================================

describe("EkoDBClient functions", () => {
  it("calls script", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({
      results: [{ id: "user_1", name: "Alice" }],
    });

    const result = await client.callScript("my_function", { limit: 10 });

    expect(result).toHaveProperty("results");
  });

  it("lists scripts", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse([
      { id: "func_1", label: "function_1" },
      { id: "func_2", label: "function_2" },
    ]);

    const result = await client.listScripts();

    expect(result).toHaveLength(2);
  });

  it("deletes script", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ status: "deleted" });

    await expect(client.deleteScript("func_123")).resolves.not.toThrow();
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
    const result = await client.saveScript(script);

    expect(result).toBeDefined();
  });

  it("gets script by ID", async () => {
    const client = createTestClient();

    mockTokenResponse();
    mockJsonResponse({ id: "func_123", label: "my_function" });

    const result = await client.getScript("func_123");

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
      client.updateScript("func_123", script),
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
