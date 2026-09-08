import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { SearchQueryBuilder } from "./search";
import { QueryBuilder } from "./query-builder";

const golden = JSON.parse(
  readFileSync(
    resolve(__dirname, "../../test-fixtures/search-requests.json"),
    "utf8",
  ),
);
const vector = [0.1, 0.2, 0.3];
const base = (query = "") =>
  new SearchQueryBuilder(query)
    .bypassRipple(false)
    .bypassCache(false)
    .limit(10);

describe("cross-client search wire cases", () => {
  const cases = {
    filtered_vector: base()
      .vector(vector)
      .vectorMetric("cosine")
      .vectorK(10)
      .vectorThreshold(0.25)
      .filters(new QueryBuilder().eq("category", "ml").build().filter)
      .build(),
    hybrid_custom_weights: base("machine learning")
      .vector(vector)
      .textWeight(0.7)
      .vectorWeight(0.3)
      .build(),
    named_vector_field: base()
      .vector(vector)
      .vectorField("title_embedding")
      .vectorMetric("dotproduct")
      .vectorK(10)
      .build(),
    full_text: base("enginer")
      .fields(["title", "body"])
      .weights({ title: 2.5, body: 1.5 })
      .fuzzy(true)
      .maxEditDistance(2)
      .build(),
  };
  for (const [name, query] of Object.entries(cases)) {
    it(name, () =>
      expect(JSON.parse(JSON.stringify(query))).toEqual(golden[name]),
    );
  }
  it("omits unset options", () => {
    expect(
      JSON.parse(JSON.stringify(new SearchQueryBuilder("").build())),
    ).toEqual({ query: "" });
  });
});
