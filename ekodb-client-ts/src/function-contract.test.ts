import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import type { FunctionStageConfig } from "./functions";

interface ContractCase {
  name: string;
  fields: string[];
  stage: Record<string, unknown>;
}

interface ContractFixture {
  coverage_floor: number;
  variant_count: number;
  variants: ContractCase[];
}

const fixture: ContractFixture = JSON.parse(
  readFileSync(
    resolve(__dirname, "../../test-fixtures/function-stage-contract.json"),
    "utf8",
  ),
);

describe("generated function-stage contract", () => {
  it("keeps every generated variant JSON-lossless", () => {
    expect(fixture.variant_count).toBe(fixture.variants.length);
    expect(fixture.variants.length).toBeGreaterThanOrEqual(
      fixture.coverage_floor,
    );

    for (const testCase of fixture.variants) {
      const typed = testCase.stage as unknown as FunctionStageConfig;
      expect(JSON.parse(JSON.stringify(typed)), testCase.name).toEqual(
        testCase.stage,
      );
    }
  });
});
