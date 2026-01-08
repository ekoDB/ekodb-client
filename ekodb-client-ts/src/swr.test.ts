import { describe, it, expect } from "vitest";
import { Stage, FunctionStageConfig } from "./functions";

describe("SWR Function Tests", () => {
  describe("Basic SWR stage creation", () => {
    it("should create basic SWR stage with required fields", () => {
      const stage = Stage.swr(
        "user:{{user_id}}",
        "15m",
        "https://api.example.com/users/{{user_id}}",
        "GET",
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.type).toBe("SWR");
      expect(stage.cache_key).toBe("user:{{user_id}}");
      expect(stage.ttl).toBe("15m");
      expect(stage.url).toBe("https://api.example.com/users/{{user_id}}");
      expect(stage.method).toBe("GET");
    });

    it("should create SWR stage with custom headers", () => {
      const headers = {
        "User-Agent": "ekoDB-Client",
        Authorization: "Bearer {{token}}",
      };

      const stage = Stage.swr(
        "api:resource",
        "1h",
        "https://api.example.com/resource",
        "GET",
        headers,
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.headers).toEqual(headers);
    });

    it("should create SWR stage with POST body", () => {
      const body = { query: "{{search_term}}", limit: 10 };

      const stage = Stage.swr(
        "search:{{search_term}}",
        900,
        "https://api.example.com/search",
        "POST",
        undefined,
        body,
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.method).toBe("POST");
      expect(stage.body).toEqual(body);
      expect(stage.ttl).toBe(900);
    });

    it("should create SWR stage with audit collection", () => {
      const stage = Stage.swr(
        "product:{{id}}",
        "1h",
        "https://api.example.com/products/{{id}}",
        "GET",
        undefined,
        undefined,
        undefined,
        "product",
        "swr_audit_trail",
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.output_field).toBe("product");
      expect(stage.collection).toBe("swr_audit_trail");
    });

    it("should create SWR stage with custom timeout", () => {
      const stage = Stage.swr(
        "slow:api",
        "5m",
        "https://slow-api.example.com/data",
        "GET",
        undefined,
        undefined,
        120,
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.timeout_seconds).toBe(120);
    });
  });

  describe("TTL format support", () => {
    it("should support duration string TTL", () => {
      const stage = Stage.swr(
        "test",
        "30m",
        "https://example.com",
        "GET",
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.ttl).toBe("30m");
    });

    it("should support integer seconds TTL", () => {
      const stage = Stage.swr(
        "test",
        1800,
        "https://example.com",
        "GET",
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.ttl).toBe(1800);
    });

    it("should support ISO timestamp TTL", () => {
      const stage = Stage.swr(
        "test",
        "2024-12-31T23:59:59Z",
        "https://example.com",
        "GET",
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.ttl).toBe("2024-12-31T23:59:59Z");
    });
  });

  describe("Optional fields handling", () => {
    it("should omit optional fields when undefined", () => {
      const stage = Stage.swr(
        "minimal",
        "15m",
        "https://example.com",
        "GET",
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.headers).toBeUndefined();
      expect(stage.body).toBeUndefined();
      expect(stage.timeout_seconds).toBeUndefined();
      expect(stage.output_field).toBeUndefined();
      expect(stage.collection).toBeUndefined();
    });

    it("should include optional fields when provided", () => {
      const stage = Stage.swr(
        "test",
        "15m",
        "https://example.com",
        "GET",
        { "User-Agent": "Test" },
        { data: "value" },
        60,
        "response",
        "audit",
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.headers).toBeDefined();
      expect(stage.body).toBeDefined();
      expect(stage.timeout_seconds).toBe(60);
      expect(stage.output_field).toBe("response");
      expect(stage.collection).toBe("audit");
    });
  });

  describe("Parameter substitution patterns", () => {
    it("should preserve parameter placeholders in cache_key", () => {
      const stage = Stage.swr(
        "user:{{user_id}}:profile:{{profile_id}}",
        "15m",
        "https://example.com",
        "GET",
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.cache_key).toBe("user:{{user_id}}:profile:{{profile_id}}");
    });

    it("should preserve parameter placeholders in URL", () => {
      const stage = Stage.swr(
        "api:{{resource}}",
        "15m",
        "https://api.example.com/{{resource}}/{{id}}",
        "GET",
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.url).toBe("https://api.example.com/{{resource}}/{{id}}");
    });

    it("should preserve parameter placeholders in body", () => {
      const body = {
        query: "{{search_term}}",
        filter: "{{filter}}",
      };

      const stage = Stage.swr(
        "search",
        "15m",
        "https://example.com/search",
        "POST",
        undefined,
        body,
      ) as Extract<FunctionStageConfig, { type: "SWR" }>;

      expect(stage.body).toEqual(body);
    });
  });

  describe("Serialization", () => {
    it("should serialize to JSON correctly", () => {
      const stage = Stage.swr(
        "user:{{user_id}}",
        "15m",
        "https://api.example.com/users/{{user_id}}",
        "GET",
        { "User-Agent": "ekoDB-Client" },
        undefined,
        30,
        "user_data",
      );

      const json = JSON.stringify(stage);
      const parsed = JSON.parse(json);

      expect(parsed.type).toBe("SWR");
      expect(parsed.cache_key).toBe("user:{{user_id}}");
      expect(parsed.ttl).toBe("15m");
      expect(parsed.url).toBe("https://api.example.com/users/{{user_id}}");
      expect(parsed.method).toBe("GET");
      expect(parsed.headers).toEqual({ "User-Agent": "ekoDB-Client" });
      expect(parsed.timeout_seconds).toBe(30);
      expect(parsed.output_field).toBe("user_data");
    });
  });
});
