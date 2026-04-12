package io.ekodb.client.functions

import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.put
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Unit tests for stored-function Stage helpers and the [parameterRef]
 * structural placeholder helper.
 *
 * Feature parity with TS (Stage.param), Go (Parameter), Rust
 * (parameter_ref), and Python (Stage.param / parameter_ref). Server-side
 * behavior for structural parameter placeholders is covered by the Rust
 * integration tests in
 * `ekodb/ekodb_server/tests/function_parameters_tests.rs`.
 *
 * Requires ekoDB >= 0.41.0 for the server to actually resolve these
 * placeholders across every mutation stage.
 */
class FunctionStagesTest {

    private val json = Json { encodeDefaults = true }

    // ------------------------------------------------------------------
    // parameterRef()
    // ------------------------------------------------------------------

    @Test
    fun `parameterRef produces structural placeholder shape`() {
        val p = parameterRef("record")
        assertEquals("Parameter", p["type"]?.toString()?.trim('"'))
        assertEquals("record", p["name"]?.toString()?.trim('"'))
    }

    @Test
    fun `parameterRef preserves arbitrary name`() {
        assertEquals("user_id", parameterRef("user_id")["name"]?.toString()?.trim('"'))
        assertEquals("updates", parameterRef("updates")["name"]?.toString()?.trim('"'))
    }

    // ------------------------------------------------------------------
    // Insert with structural placeholder
    // ------------------------------------------------------------------

    @Test
    fun `Insert accepts whole-record Parameter placeholder`() {
        val stage = FunctionStageConfig.Insert(
            collection = "users",
            record = parameterRef("record"),
        )

        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject
        assertEquals("Insert", decoded["type"]?.toString()?.trim('"'))
        assertEquals("users", decoded["collection"]?.toString()?.trim('"'))
        val record = decoded["record"]?.jsonObject
        assertNotNull(record)
        assertEquals("Parameter", record["type"]?.toString()?.trim('"'))
        assertEquals("record", record["name"]?.toString()?.trim('"'))
    }

    @Test
    fun `Insert accepts per-field Parameter placeholders mixed with literals`() {
        val recordBody = buildJsonObject {
            put("label", "{{label}}")
            put("parent_id", parameterRef("parent_id"))
            put("kind", "item")
            put("tags", parameterRef("tags"))
        }
        val stage = FunctionStageConfig.Insert(
            collection = "items",
            record = recordBody,
        )

        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject
        val record = decoded["record"]!!.jsonObject
        assertEquals("{{label}}", record["label"]?.toString()?.trim('"'))
        assertEquals("item", record["kind"]?.toString()?.trim('"'))

        val parent = record["parent_id"]!!.jsonObject
        assertEquals("Parameter", parent["type"]?.toString()?.trim('"'))
        assertEquals("parent_id", parent["name"]?.toString()?.trim('"'))

        val tags = record["tags"]!!.jsonObject
        assertEquals("Parameter", tags["type"]?.toString()?.trim('"'))
        assertEquals("tags", tags["name"]?.toString()?.trim('"'))
    }

    // ------------------------------------------------------------------
    // UpdateById with structural placeholder
    // ------------------------------------------------------------------

    @Test
    fun `UpdateById accepts whole-updates Parameter placeholder`() {
        val stage = FunctionStageConfig.UpdateById(
            collection = "items",
            record_id = "{{id}}",
            updates = parameterRef("updates"),
        )

        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject
        assertEquals("UpdateById", decoded["type"]?.toString()?.trim('"'))
        assertEquals("{{id}}", decoded["record_id"]?.toString()?.trim('"'))
        val updates = decoded["updates"]!!.jsonObject
        assertEquals("Parameter", updates["type"]?.toString()?.trim('"'))
        assertEquals("updates", updates["name"]?.toString()?.trim('"'))
    }

    // ------------------------------------------------------------------
    // Update (filter-based) with structural filter value + updates
    // ------------------------------------------------------------------

    @Test
    fun `Update accepts Parameter placeholder in both filter value and updates`() {
        val filter = buildJsonObject {
            put("type", "Condition")
            put("content", buildJsonObject {
                put("field", "id")
                put("operator", "Eq")
                put("value", parameterRef("id"))
            })
        }
        val stage = FunctionStageConfig.Update(
            collection = "items",
            filter = filter,
            updates = parameterRef("updates"),
        )

        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject
        val filterValue = decoded["filter"]!!.jsonObject["content"]!!.jsonObject["value"]!!.jsonObject
        assertEquals("Parameter", filterValue["type"]?.toString()?.trim('"'))
        assertEquals("id", filterValue["name"]?.toString()?.trim('"'))

        val updates = decoded["updates"]!!.jsonObject
        assertEquals("Parameter", updates["type"]?.toString()?.trim('"'))
        assertEquals("updates", updates["name"]?.toString()?.trim('"'))
    }

    // ------------------------------------------------------------------
    // BatchInsert with per-record Parameter placeholders
    // ------------------------------------------------------------------

    @Test
    fun `BatchInsert accepts Parameter placeholders inside each record`() {
        val records: List<JsonObject> = listOf(
            buildJsonObject {
                put("actor", parameterRef("user_id"))
                put("at", "{{now}}")
                put("message", "created")
            },
            buildJsonObject {
                put("actor", parameterRef("user_id"))
                put("at", "{{now}}")
                put("message", "initialized")
            },
        )
        val stage = FunctionStageConfig.BatchInsert(
            collection = "audit_log",
            records = records,
        )

        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject
        assertEquals("BatchInsert", decoded["type"]?.toString()?.trim('"'))
        val recordArr = decoded["records"]!!.jsonArray
        assertEquals(2, recordArr.size)
        val firstActor = recordArr[0].jsonObject["actor"]!!.jsonObject
        assertEquals("user_id", firstActor["name"]?.toString()?.trim('"'))
        assertEquals(
            "initialized",
            recordArr[1].jsonObject["message"]?.toString()?.trim('"'),
        )
    }

    // ------------------------------------------------------------------
    // Crypto primitives: BcryptHash, BcryptVerify, RandomToken
    // ------------------------------------------------------------------
    //
    // Shape tests for the three stored-function crypto stages added in
    // ekoDB 0.41.0. Runtime behavior is covered server-side in
    // `ekodb/ekodb_server/tests/function_parameters_tests.rs`.

    @Test
    fun `BcryptHash serializes with explicit cost`() {
        val stage = FunctionStageConfig.BcryptHash(
            plain = "{{password}}",
            cost = 12,
            output_field = "password_hash",
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        assertEquals("BcryptHash", decoded["type"]?.toString()?.trim('"'))
        assertEquals("{{password}}", decoded["plain"]?.toString()?.trim('"'))
        assertEquals("12", decoded["cost"]?.toString())
        assertEquals(
            "password_hash",
            decoded["output_field"]?.toString()?.trim('"'),
        )
    }

    @Test
    fun `BcryptHash omits cost when null`() {
        val stage = FunctionStageConfig.BcryptHash(
            plain = "{{password}}",
            cost = null,
            output_field = "pw_hash",
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        val cost = decoded["cost"]
        assertEquals(
            true,
            cost == null || cost.toString() == "null",
            "cost should be omitted or null, got $cost",
        )
    }

    @Test
    fun `BcryptVerify wires all three fields`() {
        val stage = FunctionStageConfig.BcryptVerify(
            plain = "{{password}}",
            hash_field = "password_hash",
            output_field = "valid",
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        assertEquals("BcryptVerify", decoded["type"]?.toString()?.trim('"'))
        assertEquals("{{password}}", decoded["plain"]?.toString()?.trim('"'))
        assertEquals(
            "password_hash",
            decoded["hash_field"]?.toString()?.trim('"'),
        )
        assertEquals("valid", decoded["output_field"]?.toString()?.trim('"'))
    }

    @Test
    fun `RandomToken serializes with explicit encoding`() {
        val stage = FunctionStageConfig.RandomToken(
            bytes = 32,
            encoding = "hex",
            output_field = "session_token",
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        assertEquals("RandomToken", decoded["type"]?.toString()?.trim('"'))
        assertEquals("32", decoded["bytes"]?.toString())
        assertEquals("hex", decoded["encoding"]?.toString()?.trim('"'))
        assertEquals(
            "session_token",
            decoded["output_field"]?.toString()?.trim('"'),
        )
    }

    @Test
    fun `RandomToken accepts base64url encoding`() {
        val stage = FunctionStageConfig.RandomToken(
            bytes = 16,
            encoding = "base64url",
            output_field = "token",
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        assertEquals("base64url", decoded["encoding"]?.toString()?.trim('"'))
    }

    @Test
    fun `Crypto stages round-trip through JSON`() {
        val stages: List<FunctionStageConfig> = listOf(
            FunctionStageConfig.BcryptHash(
                plain = "{{password}}",
                cost = 12,
                output_field = "password_hash",
            ),
            FunctionStageConfig.BcryptVerify(
                plain = "{{password}}",
                hash_field = "password_hash",
                output_field = "valid",
            ),
            FunctionStageConfig.RandomToken(
                bytes = 32,
                encoding = "base64url",
                output_field = "token",
            ),
        )
        for (stage in stages) {
            val wire = json.encodeToString<FunctionStageConfig>(stage)
            val back = json.decodeFromString<FunctionStageConfig>(wire)
            val wireAgain = json.encodeToString<FunctionStageConfig>(back)
            assertEquals(
                wire,
                wireAgain,
                "crypto stage round-trip lost data: $stage",
            )
        }
    }

    // =========================================================================
    // Error Handling & Control Flow: TryCatch, Parallel, Sleep
    // =========================================================================

    @Test
    fun `TryCatch serializes with try and catch function lists`() {
        val stage = FunctionStageConfig.TryCatch(
            try_functions = listOf(FunctionStageConfig.FindAll(collection = "users")),
            catch_functions = listOf(FunctionStageConfig.FindAll(collection = "errors")),
            output_error_field = "api_error",
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        assertEquals("TryCatch", decoded["type"].toString().trim('"'))
        assertEquals(1, decoded["try_functions"]?.jsonArray?.size)
        assertEquals(1, decoded["catch_functions"]?.jsonArray?.size)
        assertEquals("api_error", decoded["output_error_field"].toString().trim('"'))
    }

    @Test
    fun `TryCatch omits output_error_field when null`() {
        val stage = FunctionStageConfig.TryCatch(
            try_functions = listOf(FunctionStageConfig.FindAll(collection = "a")),
            catch_functions = listOf(FunctionStageConfig.FindAll(collection = "b")),
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        val field = decoded["output_error_field"]
        assertTrue(
            field == null || field.toString() == "null",
            "output_error_field should be omitted or null, got $field",
        )
    }

    @Test
    fun `Parallel serializes with functions and wait_for_all`() {
        val stage = FunctionStageConfig.Parallel(
            functions = listOf(
                FunctionStageConfig.FindAll(collection = "a"),
                FunctionStageConfig.FindAll(collection = "b"),
            ),
            wait_for_all = false,
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        assertEquals("Parallel", decoded["type"].toString().trim('"'))
        assertEquals(2, decoded["functions"]?.jsonArray?.size)
        assertEquals("false", decoded["wait_for_all"].toString())
    }

    @Test
    fun `Parallel defaults wait_for_all to true`() {
        val stage = FunctionStageConfig.Parallel(
            functions = listOf(FunctionStageConfig.FindAll(collection = "a")),
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        assertEquals("true", decoded["wait_for_all"].toString())
    }

    @Test
    fun `Sleep serializes with numeric duration_ms`() {
        val stage = FunctionStageConfig.Sleep(duration_ms = JsonPrimitive(1000))
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        assertEquals("Sleep", decoded["type"].toString().trim('"'))
        assertEquals("1000", decoded["duration_ms"].toString())
    }

    @Test
    fun `Sleep accepts placeholder string`() {
        val stage = FunctionStageConfig.Sleep(duration_ms = JsonPrimitive("{{delay}}"))
        val wire = json.encodeToString<FunctionStageConfig>(stage)

        assertTrue(wire.contains("{{delay}}"))
    }

    // =========================================================================
    // Response Formatting: Return
    // =========================================================================

    @Test
    fun `Return serializes with fields and status_code`() {
        val stage = FunctionStageConfig.Return(
            fields = mapOf(
                "message" to JsonPrimitive("ok"),
                "user_id" to JsonPrimitive("{{id}}"),
            ),
            status_code = 201,
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        assertEquals("Return", decoded["type"].toString().trim('"'))
        assertNotNull(decoded["fields"])
        assertEquals("201", decoded["status_code"].toString())
    }

    @Test
    fun `Return omits status_code when null`() {
        val stage = FunctionStageConfig.Return(
            fields = mapOf("success" to JsonPrimitive(true)),
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        val field = decoded["status_code"]
        assertTrue(
            field == null || field.toString() == "null",
            "status_code should be omitted or null, got $field",
        )
    }

    // =========================================================================
    // Data Validation: Validate
    // =========================================================================

    @Test
    fun `Validate serializes with schema, data_field, and on_error`() {
        val schema = buildJsonObject {
            put("type", "object")
        }
        val stage = FunctionStageConfig.Validate(
            schema = schema,
            data_field = "{{input}}",
            on_error = listOf(FunctionStageConfig.FindAll(collection = "errors")),
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        assertEquals("Validate", decoded["type"].toString().trim('"'))
        assertEquals("{{input}}", decoded["data_field"].toString().trim('"'))
        assertEquals(1, decoded["on_error"]?.jsonArray?.size)
    }

    @Test
    fun `Validate omits on_error when null`() {
        val stage = FunctionStageConfig.Validate(
            schema = buildJsonObject { put("type", "object") },
            data_field = "record",
        )
        val wire = json.encodeToString<FunctionStageConfig>(stage)
        val decoded = Json.parseToJsonElement(wire).jsonObject

        val field = decoded["on_error"]
        assertTrue(
            field == null || field.toString() == "null",
            "on_error should be omitted or null, got $field",
        )
    }

    // =========================================================================
    // New stages JSON round-trip
    // =========================================================================

    @Test
    fun `New stages round-trip through JSON`() {
        val stages: List<FunctionStageConfig> = listOf(
            FunctionStageConfig.TryCatch(
                try_functions = listOf(FunctionStageConfig.FindAll("a")),
                catch_functions = listOf(FunctionStageConfig.FindAll("b")),
                output_error_field = "err",
            ),
            FunctionStageConfig.Parallel(
                functions = listOf(FunctionStageConfig.FindAll("a")),
                wait_for_all = true,
            ),
            FunctionStageConfig.Sleep(duration_ms = JsonPrimitive(500)),
            FunctionStageConfig.Return(
                fields = mapOf("ok" to JsonPrimitive(true)),
                status_code = 200,
            ),
            FunctionStageConfig.Validate(
                schema = buildJsonObject { put("type", "object") },
                data_field = "data",
            ),
        )
        for (stage in stages) {
            val wire = json.encodeToString<FunctionStageConfig>(stage)
            val back = json.decodeFromString<FunctionStageConfig>(wire)
            val wireAgain = json.encodeToString<FunctionStageConfig>(back)
            assertEquals(
                wire,
                wireAgain,
                "new stage round-trip lost data: $stage",
            )
        }
    }
}
