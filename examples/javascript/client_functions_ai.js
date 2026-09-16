/**
 * AI Functions Example - Chat and Embed Operations
 *
 * Demonstrates AI operations in scripts:
 * - Chat completions with context
 * - Embedding generation
 * - Simple AI workflows
 */

const { EkoDBClient, Stage, ChatMessage } = require("@ekodb/ekodb-client");
const dotenv = require("dotenv");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

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

  const articles = [
    {
      title: "Getting Started with ekoDB",
      content: "ekoDB is a high-performance database with AI capabilities...",
      status: "published",
    },
    {
      title: "Advanced Query Patterns",
      content: "Learn how to build complex queries in ekoDB...",
      status: "published",
    },
    {
      title: "Draft Article",
      content: "This is a draft that needs review...",
      status: "draft",
    },
  ];

  for (const article of articles) {
    await client.insert("ai_articles", article);
  }

  console.log(`✅ Created ${articles.length} articles\n`);
}

async function chatCompletionScript(client) {
  console.log("📝 Example 1: Simple Chat Completion\n");

  const script = {
    label: "ai_assistant",
    name: "AI Chat Assistant",
    description: "Simple AI chat completion",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.chat(
        [
          ChatMessage.system(
            "You are a helpful database assistant. Be concise and technical.",
          ),
          ChatMessage.user("What are the benefits of using vector databases?"),
        ],
        "gpt-4",
        0.7,
      ),
    ],
    tags: ["ai", "chat"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Chat function saved");

  const result = await client.callFunction(scriptId);
  const response =
    result.records[0]?.response?.value ?? result.records[0]?.response;
  if (typeof response !== "string" || response.trim().length === 0) {
    throw new Error("Chat function returned no response text");
  }
  console.log("🤖 AI Response:");
  console.log(`   ${response}`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function embedGenerationScript(client) {
  console.log("📝 Example 2: Generate Embeddings\n");

  const script = {
    label: "generate_embeddings",
    name: "Generate Article Embeddings",
    description: "Generate embeddings for all articles",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll("ai_articles"),
      Stage.embed("content", "embedding", "text-embedding-ada-002"),
    ],
    tags: ["ai", "embeddings"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Embedding function saved");

  const result = await client.callFunction(scriptId);
  if (result.records.length !== 3) {
    throw new Error(
      `Embedding function returned ${result.records.length} records; expected 3`,
    );
  }
  console.log(`📊 Generated embeddings for ${result.records.length} articles`);
  result.records.forEach((article, i) => {
    const embedding = article.embedding?.value ?? article.embedding;
    if (!Array.isArray(embedding) || embedding.length === 0) {
      throw new Error(`Article ${i + 1} did not contain an embedding vector`);
    }
    const embeddingSize = embedding.length;
    console.log(
      `   ${i + 1}. "${article.title?.value || article.title}" - ${embeddingSize}D vector`,
    );
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function listArticlesScript(client) {
  console.log("📝 Example 3: List All Articles\n");

  const script = {
    label: "list_articles",
    name: "List All Articles",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll("ai_articles")],
    tags: ["articles"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  console.log(`📊 Found ${result.records.length} articles`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function cleanup(client, scriptIds) {
  console.log("🧹 Cleaning up...");

  const errors = [];
  for (const id of [...scriptIds].reverse()) {
    try {
      await client.deleteFunction(id);
    } catch (error) {
      errors.push(error);
    }
  }
  try {
    await client.deleteCollection("ai_articles");
  } catch (error) {
    errors.push(error);
  }
  if (errors.length > 0) {
    throw new AggregateError(errors, "AI functions cleanup failed");
  }
  console.log("✅ Cleanup complete\n");
}

async function main() {
  console.log("🚀 ekoDB AI Functions Example\n");
  console.log(
    "⚠️  Note: These examples require AI API credentials (OpenAI, etc.)\n",
  );

  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();

  const scriptIds = [];
  let completed = false;
  try {
    await setupTestData(client);
    scriptIds.push(await chatCompletionScript(client));
    scriptIds.push(await embedGenerationScript(client));
    scriptIds.push(await listArticlesScript(client));

    completed = true;
  } catch (error) {
    console.error("❌ Error:", error.message);
    if (error.message.includes("AI") || error.message.includes("model")) {
      console.log(
        "\n💡 Tip: Make sure AI features are configured in your ekoDB server",
      );
    }
    process.exitCode = 1;
  } finally {
    await cleanup(client, scriptIds);
  }
  if (completed) console.log("✅ All AI examples completed!");
}

main();
