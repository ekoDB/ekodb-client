/**
 * Unit tests for ekoDB TypeScript client utility functions
 */

import { describe, it, expect } from "vitest";
import {
  getValue,
  getValues,
  extractRecord,
  getDateTimeValue,
  getUUIDValue,
  getDecimalValue,
  getDurationValue,
  getBytesValue,
  getBinaryValue,
  getArrayValue,
  getSetValue,
  getVectorValue,
  getObjectValue,
  Field,
} from "./utils";

// ============================================================================
// getValue Tests
// ============================================================================

describe("getValue", () => {
  it("extracts value from wrapped string field", () => {
    const field = { type: "String", value: "hello world" };
    expect(getValue(field)).toBe("hello world");
  });

  it("extracts value from wrapped integer field", () => {
    const field = { type: "Integer", value: 42 };
    expect(getValue(field)).toBe(42);
  });

  it("extracts value from wrapped float field", () => {
    const field = { type: "Float", value: 3.14 };
    expect(getValue(field)).toBe(3.14);
  });

  it("extracts value from wrapped boolean field", () => {
    const field = { type: "Boolean", value: true };
    expect(getValue(field)).toBe(true);
  });

  it("extracts value from wrapped null field", () => {
    const field = { type: "Null", value: null };
    expect(getValue(field)).toBeNull();
  });

  it("returns plain string as-is", () => {
    expect(getValue("plain string")).toBe("plain string");
  });

  it("returns plain number as-is", () => {
    expect(getValue(123)).toBe(123);
  });

  it("returns plain boolean as-is", () => {
    expect(getValue(true)).toBe(true);
    expect(getValue(false)).toBe(false);
  });

  it("returns null as-is", () => {
    expect(getValue(null)).toBeNull();
  });

  it("returns undefined as-is", () => {
    expect(getValue(undefined)).toBeUndefined();
  });

  it("returns plain array as-is", () => {
    const arr = [1, 2, 3];
    expect(getValue(arr)).toEqual([1, 2, 3]);
  });

  it("returns object without value key as-is", () => {
    const obj = { name: "test", count: 5 };
    expect(getValue(obj)).toEqual({ name: "test", count: 5 });
  });

  it("supports generic type parameter", () => {
    const field = { type: "String", value: "typed" };
    const result: string = getValue<string>(field);
    expect(result).toBe("typed");
  });
});

// ============================================================================
// getValues Tests
// ============================================================================

describe("getValues", () => {
  it("extracts multiple field values from record", () => {
    const record = {
      name: { type: "String", value: "John" },
      age: { type: "Integer", value: 30 },
      active: { type: "Boolean", value: true },
    };
    const result = getValues(record, ["name", "age", "active"]);
    expect(result).toEqual({ name: "John", age: 30, active: true });
  });

  it("extracts only requested fields", () => {
    const record = {
      name: { type: "String", value: "John" },
      age: { type: "Integer", value: 30 },
      email: { type: "String", value: "john@example.com" },
    };
    const result = getValues(record, ["name", "email"]);
    expect(result).toEqual({ name: "John", email: "john@example.com" });
    expect(result).not.toHaveProperty("age");
  });

  it("handles missing fields gracefully", () => {
    const record = {
      name: { type: "String", value: "John" },
    };
    const result = getValues(record, ["name", "missing"]);
    expect(result.name).toBe("John");
    expect(result.missing).toBeUndefined();
  });

  it("handles empty record", () => {
    const result = getValues({}, ["name", "age"]);
    expect(result.name).toBeUndefined();
    expect(result.age).toBeUndefined();
  });

  it("handles empty fields list", () => {
    const record = { name: { type: "String", value: "John" } };
    const result = getValues(record, []);
    expect(result).toEqual({});
  });

  it("handles mixed wrapped and plain values", () => {
    const record = {
      wrapped: { type: "String", value: "wrapped_value" },
      plain: "plain_value",
    };
    const result = getValues(record, ["wrapped", "plain"]);
    expect(result).toEqual({ wrapped: "wrapped_value", plain: "plain_value" });
  });
});

// ============================================================================
// Specialized Value Extractors Tests
// ============================================================================

describe("getDateTimeValue", () => {
  it("parses ISO format datetime string", () => {
    const field = { type: "DateTime", value: "2024-01-15T10:30:00Z" };
    const result = getDateTimeValue(field);
    expect(result).toBeInstanceOf(Date);
    expect(result?.getFullYear()).toBe(2024);
    expect(result?.getMonth()).toBe(0); // January is 0
    expect(result?.getDate()).toBe(15);
  });

  it("parses ISO format with timezone offset", () => {
    const field = { type: "DateTime", value: "2024-01-15T10:30:00+00:00" };
    const result = getDateTimeValue(field);
    expect(result).toBeInstanceOf(Date);
    expect(result?.getFullYear()).toBe(2024);
  });

  it("returns Date object as-is", () => {
    const dt = new Date(2024, 0, 15, 10, 30, 0);
    const field = { type: "DateTime", value: dt };
    const result = getDateTimeValue(field);
    expect(result).toBe(dt);
  });

  it("returns null for invalid datetime string", () => {
    const field = { type: "DateTime", value: "not-a-datetime" };
    const result = getDateTimeValue(field);
    expect(result).toBeNull();
  });

  it("returns null for null value", () => {
    expect(getDateTimeValue(null)).toBeNull();
  });
});

describe("getUUIDValue", () => {
  it("extracts UUID from wrapped field", () => {
    const field = {
      type: "UUID",
      value: "550e8400-e29b-41d4-a716-446655440000",
    };
    expect(getUUIDValue(field)).toBe("550e8400-e29b-41d4-a716-446655440000");
  });

  it("extracts UUID from plain string", () => {
    expect(getUUIDValue("550e8400-e29b-41d4-a716-446655440000")).toBe(
      "550e8400-e29b-41d4-a716-446655440000",
    );
  });

  it("returns null for non-string", () => {
    expect(getUUIDValue(12345)).toBeNull();
  });
});

describe("getDecimalValue", () => {
  it("extracts decimal from integer", () => {
    const field = { type: "Decimal", value: 42 };
    expect(getDecimalValue(field)).toBe(42);
  });

  it("extracts decimal from float", () => {
    const field = { type: "Decimal", value: 3.14159 };
    expect(getDecimalValue(field)).toBeCloseTo(3.14159);
  });

  it("extracts decimal from string representation", () => {
    const field = { type: "Decimal", value: "123.456" };
    expect(getDecimalValue(field)).toBeCloseTo(123.456);
  });

  it("returns null for invalid decimal string", () => {
    const field = { type: "Decimal", value: "not-a-number" };
    expect(getDecimalValue(field)).toBeNull();
  });
});

describe("getDurationValue", () => {
  it("extracts duration from number (milliseconds)", () => {
    const field = { type: "Duration", value: 3600000 };
    expect(getDurationValue(field)).toBe(3600000);
  });

  it("extracts duration from secs/nanos object", () => {
    const field = { type: "Duration", value: { secs: 10, nanos: 500000000 } };
    expect(getDurationValue(field)).toBeCloseTo(10500); // 10.5 seconds in ms
  });

  it("extracts duration from plain number", () => {
    expect(getDurationValue(10500)).toBe(10500);
  });
});

describe("getBytesValue", () => {
  it("extracts bytes from Uint8Array", () => {
    const bytes = new Uint8Array([104, 101, 108, 108, 111]);
    const field = { type: "Bytes", value: bytes };
    const result = getBytesValue(field);
    expect(result).toEqual(bytes);
  });

  it("extracts bytes from array of integers", () => {
    const field = { type: "Bytes", value: [104, 101, 108, 108, 111] };
    const result = getBytesValue(field);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(Array.from(result!)).toEqual([104, 101, 108, 108, 111]);
  });

  it("extracts bytes from base64 string", () => {
    const field = { type: "Bytes", value: btoa("hello") };
    const result = getBytesValue(field);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(new TextDecoder().decode(result!)).toBe("hello");
  });

  it("returns null for invalid bytes", () => {
    expect(getBytesValue(12345)).toBeNull();
  });
});

describe("getBinaryValue", () => {
  it("extracts binary from Uint8Array (alias for getBytesValue)", () => {
    const bytes = new Uint8Array([1, 2, 3, 4]);
    const field = { type: "Binary", value: bytes };
    expect(getBinaryValue(field)).toEqual(bytes);
  });
});

describe("getArrayValue", () => {
  it("extracts array from wrapped field", () => {
    const field = { type: "Array", value: [1, 2, 3, 4, 5] };
    expect(getArrayValue(field)).toEqual([1, 2, 3, 4, 5]);
  });

  it("extracts array from plain array", () => {
    expect(getArrayValue([1, 2, 3])).toEqual([1, 2, 3]);
  });

  it("returns null for non-array", () => {
    expect(getArrayValue("not an array")).toBeNull();
  });
});

describe("getSetValue", () => {
  it("extracts set from wrapped field", () => {
    const field = { type: "Set", value: ["a", "b", "c"] };
    expect(getSetValue(field)).toEqual(["a", "b", "c"]);
  });
});

describe("getVectorValue", () => {
  it("extracts vector from array of floats", () => {
    const field = { type: "Vector", value: [0.1, 0.2, 0.3, 0.4] };
    expect(getVectorValue(field)).toEqual([0.1, 0.2, 0.3, 0.4]);
  });

  it("converts integers to floats", () => {
    const field = { type: "Vector", value: [1, 2, 3] };
    expect(getVectorValue(field)).toEqual([1, 2, 3]);
  });

  it("filters out invalid values", () => {
    const field = { type: "Vector", value: [1, "invalid", 3] };
    const result = getVectorValue(field);
    expect(result).toEqual([1, 3]);
  });
});

describe("getObjectValue", () => {
  it("extracts object from wrapped field", () => {
    const field = { type: "Object", value: { key: "value", count: 5 } };
    expect(getObjectValue(field)).toEqual({ key: "value", count: 5 });
  });

  it("returns null for non-object", () => {
    expect(getObjectValue([1, 2, 3])).toBeNull();
  });

  it("returns null for array (not a plain object)", () => {
    const field = { type: "Object", value: [1, 2, 3] };
    expect(getObjectValue(field)).toBeNull();
  });
});

// ============================================================================
// Field Builder Tests
// ============================================================================

describe("Field builders", () => {
  it("builds string field", () => {
    expect(Field.string("hello")).toEqual({ type: "String", value: "hello" });
  });

  it("builds integer field", () => {
    expect(Field.integer(42)).toEqual({ type: "Integer", value: 42 });
  });

  it("builds integer field (floors float)", () => {
    expect(Field.integer(42.9)).toEqual({ type: "Integer", value: 42 });
  });

  it("builds float field", () => {
    expect(Field.float(3.14)).toEqual({ type: "Float", value: 3.14 });
  });

  it("builds boolean field", () => {
    expect(Field.boolean(true)).toEqual({ type: "Boolean", value: true });
    expect(Field.boolean(false)).toEqual({ type: "Boolean", value: false });
  });

  it("builds UUID field", () => {
    const uuid = "550e8400-e29b-41d4-a716-446655440000";
    expect(Field.uuid(uuid)).toEqual({ type: "UUID", value: uuid });
  });

  it("builds decimal field", () => {
    expect(Field.decimal("123.456")).toEqual({
      type: "Decimal",
      value: "123.456",
    });
  });

  it("builds dateTime field from Date", () => {
    const dt = new Date("2024-01-15T10:30:00Z");
    const result = Field.dateTime(dt);
    expect(result.type).toBe("DateTime");
    expect(result.value).toBe(dt.toISOString());
  });

  it("builds dateTime field from string", () => {
    const result = Field.dateTime("2024-01-15T10:30:00Z");
    expect(result).toEqual({ type: "DateTime", value: "2024-01-15T10:30:00Z" });
  });

  it("builds duration field", () => {
    expect(Field.duration(3600000)).toEqual({
      type: "Duration",
      value: 3600000,
    });
  });

  it("builds number field", () => {
    expect(Field.number(42)).toEqual({ type: "Number", value: 42 });
    expect(Field.number(3.14)).toEqual({ type: "Number", value: 3.14 });
  });

  it("builds array field", () => {
    expect(Field.array([1, 2, 3])).toEqual({ type: "Array", value: [1, 2, 3] });
  });

  it("builds set field", () => {
    expect(Field.set(["a", "b", "c"])).toEqual({
      type: "Set",
      value: ["a", "b", "c"],
    });
  });

  it("builds vector field", () => {
    expect(Field.vector([0.1, 0.2, 0.3])).toEqual({
      type: "Vector",
      value: [0.1, 0.2, 0.3],
    });
  });

  it("builds object field", () => {
    expect(Field.object({ key: "value" })).toEqual({
      type: "Object",
      value: { key: "value" },
    });
  });

  it("builds bytes field from Uint8Array", () => {
    const bytes = new Uint8Array([104, 101, 108, 108, 111]);
    const result = Field.bytes(bytes);
    expect(result.type).toBe("Bytes");
    expect(typeof result.value).toBe("string"); // Base64 encoded
  });

  it("builds bytes field from base64 string", () => {
    const result = Field.bytes("aGVsbG8=");
    expect(result).toEqual({ type: "Bytes", value: "aGVsbG8=" });
  });

  it("builds binary field from Uint8Array", () => {
    const bytes = new Uint8Array([1, 2, 3, 4]);
    const result = Field.binary(bytes);
    expect(result.type).toBe("Binary");
    expect(typeof result.value).toBe("string"); // Base64 encoded
  });
});

// ============================================================================
// extractRecord Tests
// ============================================================================

describe("extractRecord", () => {
  it("extracts all fields from a record", () => {
    const record = {
      id: "user_123",
      name: { type: "String", value: "John Doe" },
      age: { type: "Integer", value: 30 },
      active: { type: "Boolean", value: true },
    };
    const result = extractRecord(record);
    expect(result).toEqual({
      id: "user_123",
      name: "John Doe",
      age: 30,
      active: true,
    });
  });

  it("preserves id field as-is", () => {
    const record = {
      id: "user_123",
      name: { type: "String", value: "John" },
    };
    const result = extractRecord(record);
    expect(result.id).toBe("user_123");
  });

  it("handles nested objects", () => {
    const record = {
      user: { type: "Object", value: { name: "John", role: "admin" } },
      tags: { type: "Array", value: ["python", "rust"] },
    };
    const result = extractRecord(record);
    expect(result.user).toEqual({ name: "John", role: "admin" });
    expect(result.tags).toEqual(["python", "rust"]);
  });

  it("handles empty record", () => {
    expect(extractRecord({})).toEqual({});
  });

  it("handles null/undefined input", () => {
    expect(extractRecord(null)).toBeNull();
    expect(extractRecord(undefined)).toBeUndefined();
  });

  it("handles non-object input", () => {
    expect(extractRecord("string")).toBe("string");
    expect(extractRecord(123)).toBe(123);
  });

  it("handles null field values", () => {
    const record = {
      name: { type: "String", value: "John" },
      optional: { type: "Null", value: null },
    };
    const result = extractRecord(record);
    expect(result.name).toBe("John");
    expect(result.optional).toBeNull();
  });
});
