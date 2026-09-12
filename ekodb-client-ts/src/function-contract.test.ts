import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import type { FunctionStageConfig, UserFunction } from "./functions";

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
  it("round-trips every generated variant without loss", () => {
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

  it("preserves unknown fields and variants verbatim", () => {
    const futureFunction = {
      label: "future_contract",
      name: "Future contract",
      parameters: {},
      functions: [
        { type: "FindAll", collection: "items", future_field: true },
        { type: "FutureStage", future_value: { nested: true } },
      ],
      transaction_config: {
        enabled: true,
        auto_rollback: true,
        isolation_level: "Serializable",
      },
    };

    // TypeScript performs no lossy runtime transformation: values returned by
    // fetch retain their complete JSON shape when sent back by updateFunction.
    const decoded = futureFunction as unknown as UserFunction;
    expect(JSON.parse(JSON.stringify(decoded))).toEqual(futureFunction);
  });
});
