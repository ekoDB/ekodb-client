/**
 * Search Functions Example - Basic Search Operations
 *
 * Demonstrates simple search and query operations using functions
 */

const { EkoDBClient, Stage } = require("@ekodb/ekodb-client");
const dotenv = require("dotenv");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
const RUN_SUFFIX = `${process.pid}_${Date.now()}`;
const COLLECTION = `search_docs_js_${RUN_SUFFIX}`;
const LABELS = {
  list: `list_all_docs_js_${RUN_SUFFIX}`,
  category: `docs_by_category_js_${RUN_SUFFIX}`,
  titles: `doc_titles_js_${RUN_SUFFIX}`,
  project: `project_docs_js_${RUN_SUFFIX}`,
  all: `all_docs_js_${RUN_SUFFIX}`,
};

function scalar(value) {
  return value && typeof value === "object" && "value" in value
    ? value.value
    : value;
}

function requireRecordCount(result, expected, label) {
  if (!Array.isArray(result.records))
    throw new Error(`${label} response omitted records`);
  if (result.records.length !== expected) {
    throw new Error(
      `${label} returned ${result.records.length} records; expected ${expected}`,
    );
  }
  return result.records;
}

/**
 * Save a function idempotently.
 *
 * The server returns HTTP 409 ("A function with label 'X' already exists.")
 * when a function with the same fixed label already exists. On that error we
 * UPDATE the existing function via PUT /api/functions/{label} (the server's
 * GET/PUT/DELETE routes accept either the encrypted ID or the label), then
 * resolve and return its encrypted ID so the rest of the example keeps working.
 * Any other error is propagated.
 */
async function saveOrUpdate(client, script) {
  try {
    return await client.saveFunction(script);
  } catch (error) {
    if (error.message && error.message.includes("already exists")) {
      await client.updateFunction(script.label, script);
      console.log(
        `ℹ️  Function '${script.label}' already existed — updated instead`,
      );
      const existing = await client.getFunction(script.label);
      return existing.id;
    }
    throw error;
  }
}

async function setupTestData(client) {
  console.log("📋 Setting up test data...");

  const documents = [
    {
      title: "Introduction to Machine Learning",
      content:
        "Machine learning is a subset of artificial intelligence that focuses on building systems that learn from data.",
      category: "AI",
      tags: ["ml", "ai", "data science"],
    },
    {
      title: "Database Design Principles",
      content:
        "Good database design involves normalization, indexing, and understanding query patterns.",
      category: "Database",
      tags: ["database", "design", "sql"],
    },
    {
      title: "Vector Databases Explained",
      content:
        "Vector databases store and retrieve data based on semantic similarity using embeddings.",
      category: "Database",
      tags: ["vector", "database", "embeddings"],
    },
    {
      title: "Natural Language Processing",
      content:
        "NLP enables computers to understand, interpret, and generate human language.",
      category: "AI",
      tags: ["nlp", "ai", "text"],
    },
    {
      title: "Getting Started with ekoDB",
      content:
        "ekoDB is a high-performance database with AI capabilities and built-in search.",
      category: "Database",
      tags: ["ekodb", "database", "tutorial"],
    },
  ];

  for (const doc of documents) {
    await client.insert(COLLECTION, doc);
  }

  console.log(`✅ Inserted ${documents.length} documents\n`);
}

async function listAllDocsScript(client) {
  console.log("📝 Example 1: List All Documents\n");

  const script = {
    label: LABELS.list,
    name: "List All Documents",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll(COLLECTION)],
    tags: ["search", "list"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 5, "List documents");
  console.log(`📊 Found ${records.length} documents`);
  records.forEach((doc, i) => {
    console.log(
      `   ${i + 1}. ${doc.title?.value || doc.title} (${doc.category?.value || doc.category})`,
    );
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function groupByCategoryScript(client) {
  console.log("📝 Example 2: Count Documents by Category\n");

  const script = {
    label: LABELS.category,
    name: "Documents by Category",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
      Stage.group(
        ["category"],
        [{ output_field: "count", operation: "Count" }],
      ),
    ],
    tags: ["search", "analytics"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 2, "Group documents");
  const counts = Object.fromEntries(
    records.map((group) => [
      String(scalar(group.category)),
      Number(scalar(group.count)),
    ]),
  );
  if (counts.AI !== 2 || counts.Database !== 3) {
    throw new Error(
      `Category counts were ${JSON.stringify(counts)}; expected AI=2 and Database=3`,
    );
  }
  console.log(`📊 Documents by category:`);
  records.forEach((cat) => {
    console.log(
      `   ${cat.category?.value || cat.category}: ${cat.count?.value || cat.count} documents`,
    );
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function selectFieldsScript(client) {
  console.log("📝 Example 3: Select Specific Fields\n");

  const script = {
    label: LABELS.titles,
    name: "Document Titles",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
      Stage.project(["title", "category"], false),
    ],
    tags: ["search", "projection"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 5, "Document titles");
  console.log(`📊 Document titles (${records.length} docs):`);
  records.forEach((doc, i) => {
    console.log(`   ${i + 1}. ${doc.title?.value || doc.title}`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function projectDocsScript(client) {
  console.log("📝 Example 4: Project Document Fields\n");

  const script = {
    label: LABELS.project,
    name: "Project Document Fields",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
      Stage.project(["title", "category"], false),
    ],
    tags: ["search", "projection"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 5, "Projected documents");
  console.log(`📊 Projected documents (showing first 3):`);
  records.slice(0, 3).forEach((doc, i) => {
    console.log(`   ${i + 1}. ${doc.title?.value || doc.title}`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function allFieldsScript(client) {
  console.log("📝 Example 5: All Document Fields\n");

  const script = {
    label: LABELS.all,
    name: "All Document Fields",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll(COLLECTION)],
    tags: ["search", "all"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 5, "All documents");
  console.log(`📊 All documents (${records.length} total, showing first 2):`);
  records.slice(0, 2).forEach((doc, i) => {
    console.log(
      `   ${i + 1}. ${doc.title?.value || doc.title} (${doc.category?.value || doc.category})`,
    );
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function cleanup(client, scriptIds, primaryError) {
  console.log("🧹 Cleaning up...");

  const cleanupErrors = [];
  for (const id of scriptIds) {
    try {
      await client.deleteFunction(id);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      if (!message.includes("status 404") && !/not found/i.test(message)) {
        cleanupErrors.push(error);
      }
    }
  }
  try {
    await client.deleteCollection(COLLECTION);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    if (!message.includes("status 404") && !/not found/i.test(message))
      cleanupErrors.push(error);
  }
  if (cleanupErrors.length > 0) {
    if (primaryError) console.error("Cleanup errors:", cleanupErrors);
    else
      throw new AggregateError(
        cleanupErrors,
        "Search functions cleanup failed",
      );
  } else {
    console.log("✅ Cleanup complete\n");
  }
}

async function main() {
  console.log("🚀 ekoDB Search Functions Example\n");

  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  const scriptIds = [];
  let primaryError;
  try {
    await setupTestData(client);
    scriptIds.push(await listAllDocsScript(client));
    scriptIds.push(await groupByCategoryScript(client));
    scriptIds.push(await selectFieldsScript(client));
    scriptIds.push(await projectDocsScript(client));
    scriptIds.push(await allFieldsScript(client));
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    await cleanup(client, scriptIds, primaryError);
  }

  console.log("✅ All search function examples completed!");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exitCode = 1;
});
