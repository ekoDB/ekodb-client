/** Operators accepted by ekoDB condition expressions. */
export type QueryConditionOperator =
  | "Eq"
  | "Ne"
  | "Gt"
  | "Gte"
  | "Lt"
  | "Lte"
  | "In"
  | "NotIn"
  | "Contains"
  | "StartsWith"
  | "EndsWith";

export interface QueryConditionExpression {
  type: "Condition";
  content: {
    field: string;
    operator: QueryConditionOperator;
    value: unknown;
  };
}

export interface QueryLogicalExpression {
  type: "Logical";
  content: {
    operator: "And" | "Or" | "Not";
    expressions: QueryExpression[];
  };
}

/** The adjacently-tagged filter shape accepted by ekoDB. */
export type QueryExpression = QueryConditionExpression | QueryLogicalExpression;

/** Validate raw JSON while preserving correctly-shaped object callers. */
export function queryExpression(
  value: QueryExpression | Record<string, unknown>,
): QueryExpression {
  assertQueryExpression(value);
  return value;
}

export function assertQueryExpression(
  value: unknown,
): asserts value is QueryExpression {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new TypeError("query expression must be an object");
  }

  const expression = value as Record<string, unknown>;
  const content = expression.content;
  if (
    typeof content !== "object" ||
    content === null ||
    Array.isArray(content)
  ) {
    throw new TypeError("query expression must contain an object `content`");
  }
  const body = content as Record<string, unknown>;

  if (expression.type === "Condition") {
    if (typeof body.field !== "string" || typeof body.operator !== "string") {
      throw new TypeError(
        "Condition content requires string `field` and `operator`",
      );
    }
    if (!("value" in body)) {
      throw new TypeError("Condition content requires `value`");
    }
    return;
  }

  if (expression.type === "Logical") {
    if (typeof body.operator !== "string" || !Array.isArray(body.expressions)) {
      throw new TypeError(
        "Logical content requires string `operator` and array `expressions`",
      );
    }
    body.expressions.forEach(assertQueryExpression);
    return;
  }

  throw new TypeError(
    "query expression `type` must be `Condition` or `Logical`",
  );
}
