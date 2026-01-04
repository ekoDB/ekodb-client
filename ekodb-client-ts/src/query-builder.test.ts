/**
 * Unit tests for ekoDB TypeScript client QueryBuilder
 */

import { describe, it, expect } from "vitest";
import { QueryBuilder, SortOrder } from "./query-builder";

// ============================================================================
// Basic Tests
// ============================================================================

describe("QueryBuilder basics", () => {
  it("creates empty query builder", () => {
    const qb = new QueryBuilder();
    expect(qb).toBeInstanceOf(QueryBuilder);
  });

  it("builds empty query", () => {
    const query = new QueryBuilder().build();
    expect(query).toEqual({});
  });
});

// ============================================================================
// Equality Operators Tests
// ============================================================================

describe("QueryBuilder equality operators", () => {
  it("builds eq filter", () => {
    const query = new QueryBuilder().eq("status", "active").build();

    expect(query.filter).toEqual({
      type: "Condition",
      content: {
        field: "status",
        operator: "Eq",
        value: "active",
      },
    });
  });

  it("builds ne filter", () => {
    const query = new QueryBuilder().ne("status", "deleted").build();

    expect(query.filter.content.operator).toBe("Ne");
  });
});

// ============================================================================
// Comparison Operators Tests
// ============================================================================

describe("QueryBuilder comparison operators", () => {
  it("builds gt filter", () => {
    const query = new QueryBuilder().gt("age", 18).build();

    expect(query.filter.content.operator).toBe("Gt");
    expect(query.filter.content.value).toBe(18);
  });

  it("builds gte filter", () => {
    const query = new QueryBuilder().gte("score", 80).build();

    expect(query.filter.content.operator).toBe("Gte");
  });

  it("builds lt filter", () => {
    const query = new QueryBuilder().lt("price", 100.5).build();

    expect(query.filter.content.operator).toBe("Lt");
  });

  it("builds lte filter", () => {
    const query = new QueryBuilder().lte("count", 1000).build();

    expect(query.filter.content.operator).toBe("Lte");
  });
});

// ============================================================================
// Array Operators Tests
// ============================================================================

describe("QueryBuilder array operators", () => {
  it("builds in filter", () => {
    const query = new QueryBuilder()
      .in("status", ["active", "pending"])
      .build();

    expect(query.filter.content.operator).toBe("In");
    expect(query.filter.content.value).toEqual(["active", "pending"]);
  });

  it("builds nin filter", () => {
    const query = new QueryBuilder()
      .nin("role", ["blocked", "deleted"])
      .build();

    expect(query.filter.content.operator).toBe("NotIn");
  });
});

// ============================================================================
// String Operators Tests
// ============================================================================

describe("QueryBuilder string operators", () => {
  it("builds contains filter", () => {
    const query = new QueryBuilder().contains("email", "@example.com").build();

    expect(query.filter.content.operator).toBe("Contains");
    expect(query.filter.content.value).toBe("@example.com");
  });

  it("builds startsWith filter", () => {
    const query = new QueryBuilder().startsWith("name", "John").build();

    expect(query.filter.content.operator).toBe("StartsWith");
  });

  it("builds endsWith filter", () => {
    const query = new QueryBuilder().endsWith("filename", ".pdf").build();

    expect(query.filter.content.operator).toBe("EndsWith");
  });

  it("builds regex filter", () => {
    const query = new QueryBuilder().regex("phone", "^\\+1").build();

    expect(query.filter.content.operator).toBe("Regex");
  });
});

// ============================================================================
// Logical Operators Tests
// ============================================================================

describe("QueryBuilder logical operators", () => {
  it("builds and filter", () => {
    const conditions = [
      {
        type: "Condition",
        content: { field: "status", operator: "Eq", value: "active" },
      },
      {
        type: "Condition",
        content: { field: "age", operator: "Gt", value: 18 },
      },
    ];

    const query = new QueryBuilder().and(conditions).build();

    expect(query.filter.type).toBe("Logical");
    expect(query.filter.content.operator).toBe("And");
    expect(query.filter.content.expressions).toHaveLength(2);
  });

  it("builds or filter", () => {
    const conditions = [
      {
        type: "Condition",
        content: { field: "role", operator: "Eq", value: "admin" },
      },
      {
        type: "Condition",
        content: { field: "role", operator: "Eq", value: "super_admin" },
      },
    ];

    const query = new QueryBuilder().or(conditions).build();

    expect(query.filter.content.operator).toBe("Or");
  });

  it("builds not filter", () => {
    const condition = {
      type: "Condition",
      content: { field: "deleted", operator: "Eq", value: true },
    };

    const query = new QueryBuilder().not(condition).build();

    expect(query.filter.content.operator).toBe("Not");
  });
});

// ============================================================================
// Multiple Filters (Auto AND) Tests
// ============================================================================

describe("QueryBuilder multiple filters", () => {
  it("combines multiple filters with AND logic", () => {
    const query = new QueryBuilder()
      .eq("status", "active")
      .gt("age", 18)
      .contains("email", "@company.com")
      .build();

    expect(query.filter.type).toBe("Logical");
    expect(query.filter.content.operator).toBe("And");
    expect(query.filter.content.expressions).toHaveLength(3);
  });
});

// ============================================================================
// Sorting Tests
// ============================================================================

describe("QueryBuilder sorting", () => {
  it("builds ascending sort", () => {
    const query = new QueryBuilder().sortAsc("name").build();

    expect(query.sort).toHaveLength(1);
    expect(query.sort![0].field).toBe("name");
    expect(query.sort![0].ascending).toBe(true);
  });

  it("builds descending sort", () => {
    const query = new QueryBuilder().sortDesc("created_at").build();

    expect(query.sort![0].ascending).toBe(false);
  });

  it("builds multiple sorts", () => {
    const query = new QueryBuilder()
      .sortDesc("created_at")
      .sortAsc("name")
      .build();

    expect(query.sort).toHaveLength(2);
    expect(query.sort![0].field).toBe("created_at");
    expect(query.sort![0].ascending).toBe(false);
    expect(query.sort![1].field).toBe("name");
    expect(query.sort![1].ascending).toBe(true);
  });
});

// ============================================================================
// Pagination Tests
// ============================================================================

describe("QueryBuilder pagination", () => {
  it("builds limit", () => {
    const query = new QueryBuilder().limit(10).build();

    expect(query.limit).toBe(10);
  });

  it("builds skip", () => {
    const query = new QueryBuilder().skip(20).build();

    expect(query.skip).toBe(20);
  });

  it("builds page (convenience method)", () => {
    // Page 2 with 20 items per page = skip 40
    const query = new QueryBuilder().page(2, 20).build();

    expect(query.limit).toBe(20);
    expect(query.skip).toBe(40);
  });

  it("builds page 0", () => {
    const query = new QueryBuilder().page(0, 10).build();

    expect(query.skip).toBe(0);
    expect(query.limit).toBe(10);
  });
});

// ============================================================================
// Join Tests
// ============================================================================

describe("QueryBuilder join", () => {
  it("builds join configuration", () => {
    const joinConfig = {
      collections: ["users"],
      local_field: "user_id",
      foreign_field: "id",
      as_field: "user",
    };

    const query = new QueryBuilder().join(joinConfig).build();

    expect(query.join).toEqual(joinConfig);
  });
});

// ============================================================================
// Bypass Flags Tests
// ============================================================================

describe("QueryBuilder bypass flags", () => {
  it("builds bypass_cache true", () => {
    const query = new QueryBuilder().bypassCache(true).build();

    expect(query.bypass_cache).toBe(true);
  });

  it("builds bypass_cache false (not included)", () => {
    const query = new QueryBuilder().bypassCache(false).build();

    expect(query.bypass_cache).toBeUndefined();
  });

  it("builds bypass_cache default (true)", () => {
    const query = new QueryBuilder().bypassCache().build();

    expect(query.bypass_cache).toBe(true);
  });

  it("builds bypass_ripple", () => {
    const query = new QueryBuilder().bypassRipple(true).build();

    expect(query.bypass_ripple).toBe(true);
  });
});

// ============================================================================
// Chaining Tests
// ============================================================================

describe("QueryBuilder chaining", () => {
  it("supports full method chaining", () => {
    const query = new QueryBuilder()
      .eq("status", "active")
      .gt("age", 18)
      .sortDesc("created_at")
      .sortAsc("name")
      .limit(10)
      .skip(20)
      .bypassCache(true)
      .build();

    // Check filter exists
    expect(query.filter).toBeDefined();

    // Check sort exists
    expect(query.sort).toHaveLength(2);

    // Check pagination
    expect(query.limit).toBe(10);
    expect(query.skip).toBe(20);

    // Check bypass flag
    expect(query.bypass_cache).toBe(true);
  });

  it("returns this for method chaining", () => {
    const qb = new QueryBuilder();

    expect(qb.eq("a", 1)).toBe(qb);
    expect(qb.ne("b", 2)).toBe(qb);
    expect(qb.gt("c", 3)).toBe(qb);
    expect(qb.gte("d", 4)).toBe(qb);
    expect(qb.lt("e", 5)).toBe(qb);
    expect(qb.lte("f", 6)).toBe(qb);
    expect(qb.in("g", [7])).toBe(qb);
    expect(qb.nin("h", [8])).toBe(qb);
    expect(qb.contains("i", "j")).toBe(qb);
    expect(qb.startsWith("k", "l")).toBe(qb);
    expect(qb.endsWith("m", "n")).toBe(qb);
    expect(qb.regex("o", "p")).toBe(qb);
    expect(qb.sortAsc("q")).toBe(qb);
    expect(qb.sortDesc("r")).toBe(qb);
    expect(qb.limit(1)).toBe(qb);
    expect(qb.skip(0)).toBe(qb);
    expect(qb.bypassCache()).toBe(qb);
    expect(qb.bypassRipple()).toBe(qb);
  });
});

// ============================================================================
// Raw Filter Tests
// ============================================================================

describe("QueryBuilder rawFilter", () => {
  it("adds raw filter expression", () => {
    const rawFilter = {
      type: "Condition",
      content: {
        field: "custom",
        operator: "CustomOp",
        value: "custom_value",
      },
    };

    const query = new QueryBuilder().rawFilter(rawFilter).build();

    expect(query.filter).toEqual(rawFilter);
  });
});

// ============================================================================
// SortOrder Enum Tests
// ============================================================================

describe("SortOrder enum", () => {
  it("has correct values", () => {
    expect(SortOrder.Asc).toBe("asc");
    expect(SortOrder.Desc).toBe("desc");
  });
});
