/**
 * Schema Management Example
 *
 * Demonstrates schema creation and management:
 * - Creating collections with schemas
 * - Field types and constraints
 * - Indexes (text, vector, btree, hash)
 * - Validation rules
 */

require("dotenv").config();
const {
  EkoDBClient,
  SchemaBuilder,
  FieldTypeSchemaBuilder,
  VectorIndexAlgorithm,
  DistanceMetric,
} = require("@ekodb/ekodb-client");

async function main() {
  const client = new EkoDBClient(
    process.env.API_BASE_URL || "http://localhost:8080",
    process.env.API_BASE_KEY || "a-test-api-key-from-ekodb-0123456789",
  );
  await client.init();

  console.log("=== Schema Management Examples ===\n");

  // Use unique collection names
  const usersCollection = "schema_users_client_js";
  const productsCollection = "schema_products_client_js";
  const documentsCollection = "schema_documents_client_js";
  const employeesCollection = "schema_employees_client_js";

  // Cleanup: Delete collections if they exist from previous runs
  try {
    await client.deleteCollection(usersCollection);
    await client.deleteCollection(productsCollection);
    await client.deleteCollection(documentsCollection);
    await client.deleteCollection(employeesCollection);
  } catch (e) {
    // Collections don't exist, that's fine
  }

  try {
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

    try {
      await client.createCollection(usersCollection, userSchema);
      console.log("✅ User schema created\n");
    } catch (e) {
      if (e.message?.includes("already exists")) {
        console.log("⚠️  User schema already exists (skipping)\n");
      } else {
        throw e;
      }
    }

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

    try {
      await client.createCollection(productsCollection, productSchema);
      console.log("✅ Product schema with indexes created\n");
    } catch (e) {
      if (e.message?.includes("already exists")) {
        console.log("⚠️  Product schema already exists (skipping)\n");
      } else {
        throw e;
      }
    }

    // Example 3: Schema with vector index for semantic search
    console.log("3. Creating document schema with vector index:");
    const VecAlgo = VectorIndexAlgorithm || { HNSW: "hnsw" };
    const DistMet = DistanceMetric || { Cosine: "cosine" };

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
          VecAlgo.HNSW || "hnsw",
          DistMet.Cosine || "cosine",
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

    try {
      await client.createCollection(documentsCollection, documentSchema);
      console.log("✅ Document schema with vector index created\n");
    } catch (e) {
      if (e.message?.includes("already exists")) {
        console.log("⚠️  Document schema already exists (skipping)\n");
      } else {
        throw e;
      }
    }

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
      .addField(
        "bio",
        new FieldTypeSchemaBuilder("String").textIndex("english"),
      )
      .bypassRipple(false)
      .build();

    try {
      await client.createCollection(employeesCollection, employeeSchema);
      console.log("✅ Employee schema with all constraints created\n");
    } catch (e) {
      if (e.message?.includes("already exists")) {
        console.log("⚠️  Employee schema already exists (skipping)\n");
      } else {
        throw e;
      }
    }

    // Example 7: Update constraints on an existing field.
    // Current servers replace a named field's constraints rather than merging
    // them, so each update sends unique, required and enums together; max,
    // min and regex left out are cleared, and the default is kept.
    console.log("7. Updating constraints on an existing field:");
    await client.updateSchemaConstraints(usersCollection, {
      status: { unique: false, required: true, enums: ["active", "pending"] },
    });
    const narrowed = (await client.getSchema(usersCollection)).fields.status;
    console.log(
      "status: required =",
      narrowed.required,
      ", enums =",
      narrowed.enums,
    );
    if (narrowed.required !== true || narrowed.enums?.length !== 2) {
      throw new Error(
        `Constraint update not applied: ${JSON.stringify(narrowed)}`,
      );
    }

    // An empty enums list clears the field's enums.
    await client.updateSchemaConstraints(usersCollection, {
      status: { unique: false, required: true, enums: [] },
    });
    const cleared = (await client.getSchema(usersCollection)).fields.status;
    console.log("status after clearing: enums =", cleared.enums ?? []);
    if ((cleared.enums ?? []).length !== 0) {
      throw new Error(`Enums were not cleared: ${JSON.stringify(cleared)}`);
    }
    console.log("✅ Constraints updated and read back\n");

    console.log("✅ Schema management examples completed!");
  } catch (error) {
    // Cleanup on error
    try {
      await client.deleteCollection(usersCollection);
      await client.deleteCollection(productsCollection);
      await client.deleteCollection(documentsCollection);
      await client.deleteCollection(employeesCollection);
    } catch (e) {
      // Ignore cleanup errors
    }
    throw error;
  }
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
