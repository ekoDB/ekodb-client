// ekoDB TypeScript Client - Goal Template CRUD Example
//
// Demonstrates creating, listing, getting, updating, and deleting goal templates.

import { EkoDBClient } from "@ekodb/ekodb-client";

async function main() {
  console.log("=== ekoDB Goal Template CRUD Example (TypeScript) ===\n");

  const baseUrl = process.env.API_BASE_URL || "http://localhost:8080";
  const apiKey = process.env.API_BASE_KEY;
  if (!apiKey) throw new Error("API_BASE_KEY environment variable is required");

  const client = new EkoDBClient(baseUrl, apiKey);

  // 1. Create a goal template
  console.log("--- Creating goal template ---");
  const template = await client.goalTemplateCreate({
    title: "Data Migration",
    description: "Template for migrating data between schemas",
    steps: [
      { description: "Analyze source schema" },
      { description: "Create target schema" },
      { description: "Migrate records" },
      { description: "Validate results" },
    ],
  });
  const templateId = template.id;
  console.log(`Created template: ${template.title} (id: ${templateId})`);

  // 2. List all templates
  console.log("\n--- Listing templates ---");
  const list = await client.goalTemplateList();
  console.log("Templates:", list);

  // 3. Get template by ID
  console.log("\n--- Getting template ---");
  const fetched = await client.goalTemplateGet(templateId);
  console.log(`Fetched: ${fetched.title}`);

  // 4. Update template
  console.log("\n--- Updating template ---");
  const updated = await client.goalTemplateUpdate(templateId, {
    description: "Updated: comprehensive data migration workflow",
  });
  console.log(`Updated description: ${updated.description}`);

  // 5. Delete template
  console.log("\n--- Deleting template ---");
  await client.goalTemplateDelete(templateId);
  console.log("Template deleted successfully");

  console.log("\n✓ Goal template CRUD example completed");
}

main().catch(console.error);
