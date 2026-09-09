import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { expect, it } from "vitest";
import { encode, decode } from "@msgpack/msgpack";
import { Field } from "./utils";

it("preserves distinct Array and Vector values in the complete insertion payload", () => {
  const golden = JSON.parse(
    readFileSync(resolve(__dirname, "../../test-fixtures/vector-record.json"), "utf8"),
  );
  const record = {
    label: "vector-golden",
    count: 7,
    eligible: true,
    ordinary: [1.0, 0.0, 0.0],
    embedding: Field.vector([1.0, 0.0, 0.0]),
  };
  expect(JSON.parse(JSON.stringify(record))).toEqual(golden);
  expect(decode(encode(record))).toEqual(golden);
  expect(Array.isArray(record.ordinary)).toBe(true);
  expect(Array.isArray(record.embedding)).toBe(false);
});
