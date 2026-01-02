/**
 * Schema Management Example
 *
 * Demonstrates schema creation and management:
 * - Creating collections with schemas
 * - Field types and constraints
 * - Indexes (text, vector, btree, hash)
 * - Validation rules
 */

import {
  EkoDBClient,
  SchemaBuilder,
  FieldTypeSchemaBuilder,
  VectorIndexAlgorithm,
  DistanceMetric,
} from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

async function main() {
  const client = new EkoDBClient(
    process.env.API_BASE_URL || "http://localhost:8080",
    process.env.API_BASE_KEY || "a-test-api-key-from-ekodb",
  );
  await client.init();

  console.log("=== Schema Management Examples ===\n");

  // Use unique collection names
  const usersCollection = "schema_users_client_ts";
  const productsCollection = "schema_products_client_ts";
  const documentsCollection = "schema_documents_client_ts";
  const employeesCollection = "schema_employees_client_ts";

  // Cleanup: Delete collections if they exist from previous runs
  try {
    await client.deleteCollection(usersCollection);
    await client.deleteCollection(productsCollection);
    await client.deleteCollection(documentsCollection);
    await client.deleteCollection(employeesCollection);
  } catch (e) {
    // Collections don't exist, that's fine
  }

  // Example 1: Create a simple user schema
  console.log("1. Creating user schema with basic fields:");
  const userSchema = new SchemaBuilder()
    .addField(
      "name",
      new FieldTypeSchemaBuilder("String").required().pattern("^[a-zA-Z ]+$"),
    )
    .addField(
      "email",
      new FieldTypeSchemaBuilder("String")
        .required()
        .unique()
        .pattern("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"),
    )
    .addField("age", new FieldTypeSchemaBuilder("Integer").range(0, 150))
    .addField(
      "status",
      new FieldTypeSchemaBuilder("String")
        .defaultValue("active")
        .enums(["active", "inactive", "suspended"]),
    )
    .build();

  await client.createCollection(usersCollection, userSchema);
  console.log("✅ User schema created\n");

  // Example 2: Schema with text index for full-text search
  console.log("2. Creating product schema with text index:");
  const productSchema = new SchemaBuilder()
    .addField(
      "name",
      new FieldTypeSchemaBuilder("String").required().textIndex("english"),
    )
    .addField(
      "description",
      new FieldTypeSchemaBuilder("String").textIndex("english"),
    )
    .addField(
      "price",
      new FieldTypeSchemaBuilder("Float").required().range(0, undefined),
    )
    .addField(
      "category",
      new FieldTypeSchemaBuilder("String").required().btreeIndex(),
    )
    .addField(
      "sku",
      new FieldTypeSchemaBuilder("String").required().unique().hashIndex(),
    )
    .build();

  await client.createCollection(productsCollection, productSchema);
  console.log("✅ Product schema with indexes created\n");

  // Example 3: Schema with vector index for semantic search
  console.log("3. Creating document schema with vector index:");
  const documentSchema = new SchemaBuilder()
    .addField(
      "title",
      new FieldTypeSchemaBuilder("String").required().textIndex("english"),
    )
    .addField(
      "content",
      new FieldTypeSchemaBuilder("String").required().textIndex("english"),
    )
    .addField(
      "embedding",
      new FieldTypeSchemaBuilder("Array").vectorIndex(
        VectorIndexAlgorithm.HNSW,
        DistanceMetric.Cosine,
        16,
        200,
      ),
    )
    .addField("tags", new FieldTypeSchemaBuilder("Array"))
    .addField(
      "created_at",
      new FieldTypeSchemaBuilder("DateTime").defaultValue(
        new Date().toISOString(),
      ),
    )
    .build();

  await client.createCollection(documentsCollection, documentSchema);
  console.log("✅ Document schema with vector index created\n");

  // Example 4: Get collection schema
  console.log("4. Retrieving collection schema:");
  const retrievedSchema = await client.getSchema(usersCollection);
  console.log("Schema fields:", Object.keys(retrievedSchema.fields));
  console.log("Schema version:", retrievedSchema.version);
  console.log();

  // Example 5: Get collection metadata
  console.log("5. Retrieving collection metadata:");
  const metadata = await client.getCollection(usersCollection);
  console.log(
    "Collection has",
    Object.keys(metadata.collection.fields).length,
    "fields",
  );
  console.log();

  // Example 6: Complex schema with all constraint types
  console.log("6. Creating employee schema with all constraint types:");
  const employeeSchema = new SchemaBuilder()
    .addField(
      "employee_id",
      new FieldTypeSchemaBuilder("String")
        .required()
        .unique()
        .pattern("^EMP-[0-9]{6}$")
        .hashIndex(),
    )
    .addField("first_name", new FieldTypeSchemaBuilder("String").required())
    .addField("last_name", new FieldTypeSchemaBuilder("String").required())
    .addField(
      "department",
      new FieldTypeSchemaBuilder("String")
        .required()
        .enums(["Engineering", "Sales", "Marketing", "HR", "Finance"])
        .btreeIndex(),
    )
    .addField(
      "salary",
      new FieldTypeSchemaBuilder("Float").required().range(30000, 500000),
    )
    .addField("hire_date", new FieldTypeSchemaBuilder("DateTime").required())
    .addField(
      "is_active",
      new FieldTypeSchemaBuilder("Boolean").defaultValue(true),
    )
    .addField("bio", new FieldTypeSchemaBuilder("String").textIndex("english"))
    .bypassRipple(false)
    .build();

  await client.createCollection(employeesCollection, employeeSchema);
  console.log("✅ Employee schema with all constraints created\n");

  console.log("✅ Schema management examples completed!");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
