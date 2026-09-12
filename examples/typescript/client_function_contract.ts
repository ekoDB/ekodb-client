/** Offline contract smoke test for stored-function stage serialization. */

import assert from "node:assert/strict";
import {
  Stage,
  queryExpression,
  type FunctionStageConfig,
  type UserFunction,
} from "@ekodb/ekodb-client";

const filter = queryExpression({
  type: "Condition",
  content: { field: "status", operator: "Eq", value: "active" },
});

const stages: FunctionStageConfig[] = [
  Stage.query("items", filter, undefined, 5),
  Stage.update("items", filter, { status: "ready" }),
  Stage.delete("items", filter),
  Stage.group(
    ["category"],
    [
      { output_field: "labels", operation: "AddToSet", input_field: "label" },
      {
        output_field: "score_stddev",
        operation: "StandardDeviation",
        input_field: "score",
      },
      {
        output_field: "unique_users",
        operation: "ApproxDistinct",
        input_field: "user_id",
      },
    ],
  ),
  Stage.if(
    {
      type: "FieldGreaterThanOrEqual",
      value: { field: "stock", value: 1 },
    },
    [Stage.setField("available", true)],
  ),
  Stage.upsert("items", "sku", "sku-1", { name: "Widget" }),
  Stage.increment("items", "item-1", "views", 2),
  Stage.push("items", "item-1", "tags", "new"),
  Stage.setField("source", "contract"),
  Stage.addFields([
    {
      field_name: "total",
      expression: { type: "Field", name: "price" },
    },
  ]),
  Stage.currentDatetime("processed_at"),
  Stage.vectorSearch("items", [0.1, 0.2], 3, 0.8),
  Stage.hybridSearch("items", "widget", [0.1, 0.2], 3),
  Stage.findOneAndUpdate("items", "item-1", { status: "claimed" }, true, 60),
  Stage.updateWithAction("items", "item-1", "increment", "views", 1, true),
  Stage.batchDelete("items", ["item-1", "item-2"], true),
  Stage.embed("body", "embedding", "text-embedding-3-small"),
  Stage.httpRequest(
    "https://example.com/items",
    "GET",
    undefined,
    undefined,
    10,
    "response",
  ),
  Stage.jwtSign(
    { sub: "user-1" },
    "public-key-material",
    "token",
    300,
    "EdDSA",
  ),
];

const contract: UserFunction = {
  label: "client_function_contract",
  name: "Client function contract",
  parameters: {},
  functions: stages,
  transaction_config: {
    enabled: true,
    auto_rollback: true,
    isolation_level: "Serializable",
  },
};

const encoded = JSON.parse(JSON.stringify(contract)) as UserFunction;
const byType = new Map<string, FunctionStageConfig>(
  encoded.functions.map((stage) => [stage.type, stage]),
);
assert.equal(encoded.transaction_config?.isolation_level, "Serializable");
for (const stageType of ["Query", "Update", "Delete"] as const) {
  assert.equal(
    (byType.get(stageType) as { filter: { type: string } }).filter.type,
    "Condition",
  );
}
assert.equal(
  (byType.get("Group") as Extract<FunctionStageConfig, { type: "Group" }>)
    .functions[2].operation,
  "ApproxDistinct",
);
assert.equal(
  (byType.get("If") as Extract<FunctionStageConfig, { type: "If" }>).condition
    .type,
  "FieldGreaterThanOrEqual",
);
assert.deepEqual(
  (
    byType.get("BatchDelete") as Extract<
      FunctionStageConfig,
      { type: "BatchDelete" }
    >
  ).record_ids,
  ["item-1", "item-2"],
);
assert.equal(
  (byType.get("Embed") as Extract<FunctionStageConfig, { type: "Embed" }>)
    .input_field,
  "body",
);
const request = byType.get("HttpRequest") as Extract<
  FunctionStageConfig,
  { type: "HttpRequest" }
>;
assert.equal(request.timeout_seconds, 10);
assert.equal(request.output_field, "response");
assert.equal(
  (byType.get("JwtSign") as { algorithm: string }).algorithm,
  "EdDSA",
);
for (const stageType of [
  "Upsert",
  "Increment",
  "Push",
  "SetField",
  "AddFields",
  "CurrentDatetime",
  "VectorSearch",
  "HybridSearch",
  "FindOneAndUpdate",
  "UpdateWithAction",
]) {
  assert.ok(byType.has(stageType), `missing ${stageType} stage`);
}

console.log("client_function_contract: ok");
