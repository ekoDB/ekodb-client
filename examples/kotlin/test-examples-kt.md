make test-examples-kotlin 🟣 [36mBuilding Kotlin client library...[0m To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE Task :jar UP-TO-DATE Task :assemble UP-TO-DATE Task
> :compileTestKotlin NO-SOURCE Task :compileTestJava NO-SOURCE Task
> :processTestResources NO-SOURCE Task :testClasses UP-TO-DATE Task :test
> NO-SOURCE Task :check UP-TO-DATE Task :build UP-TO-DATE

BUILD SUCCESSFUL in 3s 2 actionable tasks: 2 up-to-date ✅ [32mKotlin client
built![0m 🧪 [36mRunning Kotlin client library examples...[0m To honour the JVM
settings for this build a single-use Daemon process will be forked. For more on
this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Simple CRUD Example ===

=== Create === ✓ Inserted user:
Record(fields={id=StringValue(value=QP0_syT_o0ZeTKuy9dpqvm44l12fP4AffzH5P9ywb-YHsiBJsa1fvCUvptBO9JFtMe3ZYmdth9STp9YrqKhjeg)})
User ID:
QP0_syT_o0ZeTKuy9dpqvm44l12fP4AffzH5P9ywb-YHsiBJsa1fvCUvptBO9JFtMe3ZYmdth9STp9YrqKhjeg

=== Read === ✓ Found user by ID:
Record(fields={age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
id=StringValue(value=QP0_syT_o0ZeTKuy9dpqvm44l12fP4AffzH5P9ywb-YHsiBJsa1fvCUvptBO9JFtMe3ZYmdth9STp9YrqKhjeg),
name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)})})

=== Update === ✓ Updated user:
Record(fields={name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
city=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=San Francisco)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
id=StringValue(value=QP0_syT_o0ZeTKuy9dpqvm44l12fP4AffzH5P9ywb-YHsiBJsa1fvCUvptBO9JFtMe3ZYmdth9STp9YrqKhjeg),
age=ObjectValue(value={value=IntegerValue(value=29),
type=StringValue(value=Integer)})})

=== Query === ✓ Found 1 users matching query

- Record(fields={id=StringValue(value=QP0_syT_o0ZeTKuy9dpqvm44l12fP4AffzH5P9ywb-YHsiBJsa1fvCUvptBO9JFtMe3ZYmdth9STp9YrqKhjeg),
  active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  name=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Alice Johnson)}),
  email=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=alice@example.com)}),
  city=ObjectValue(value={value=StringValue(value=San Francisco),
  type=StringValue(value=String)}),
  age=ObjectValue(value={value=IntegerValue(value=29),
  type=StringValue(value=Integer)})})

=== Delete === ✓ Deleted user with ID:
QP0_syT_o0ZeTKuy9dpqvm44l12fP4AffzH5P9ywb-YHsiBJsa1fvCUvptBO9JFtMe3ZYmdth9STp9YrqKhjeg

✓ Confirmed user was deleted

=== Cleanup === ✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Batch Operations Example ===

=== Batch Insert === ✓ Inserted 5 records IDs:
G6asODFMfC8H2ehnaXFjz0CXZZ0SQ7hmgPmNj_W5PrIKaqMN7zRU4xWvwWdvyggWGR0et80rk70WIBqmBCaGXQ,
F-tr0FvvI1yocSAI6BsVkkVm_rfObFJC74IpboyhD28GiO7WAJVYRQEdzon9jYYuA7CrbJXHfT_Z5b5hDrkQMg,
G_K04uAkVvSzhfHzQnz1ybdV7leEZSUJVhdAZvuivU1lDENnF-oaojKp7L-DUeuv4suvZwn1HvWHiMM3I8i9Ag...

=== Batch Update === ✓ Updated 3 records

=== Batch Delete === ✓ Deleted 2 records

=== Cleanup === ✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Query Builder Example ===

=== Inserting Sample Data === ✓ Inserted 5 records

=== Query 1: Equality (city = 'NYC') === ✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) === ✓ Found 3 records with age 25-31

=== Query 3: Sort by score (descending) === ✓ Top 3 scores:

- Score: ObjectValue(value={value=IntegerValue(value=95),
  type=StringValue(value=Integer)})
- Score: ObjectValue(value={value=IntegerValue(value=92),
  type=StringValue(value=Integer)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) === ✓ Found 4 high-scoring
adults

=== Query 5: IN (city IN ['NYC', 'LA']) === ✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) === ✓ Page 2 (2 records):

- ObjectValue(value={value=StringValue(value=Charlie),
  type=StringValue(value=String)})
- ObjectValue(value={value=StringValue(value=Diana),
  type=StringValue(value=String)})

=== Query 7: Contains (name contains 'a') === ✓ Found 2 names containing 'a'

=== Cleanup === ✓ Deleted collection: kotlin_query_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - KV Operations Example ===

=== KV Set === ✓ Set key: user:123

=== KV Get === ✓ Retrieved value:
{"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"user_id":"123","created_at":1765305699887} (Will expire in 10 seconds)

=== Set Multiple Keys === ✓ Set 3 configuration keys

=== KV Delete === ✓ Deleted key: user:123

=== Verify Deletion === ✓ Confirmed key was deleted

=== Cleanup === ✓ Cleaned up test keys

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Collection Management Example ===

=== List Collections === ✓ Found 21 collections

- schema_users_client_py
- schema_employees_client_py
- test_collection
- chat_configurations\_\_ek0_bench
- chat_messages\_\_ek0_bench

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-09T18:41:44.165390Z","last_modified":"2025-12-09T18:41:44.165390Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup === ✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Document TTL Example ===

=== Insert with TTL === ✓ Inserted document with 10s TTL Document ID:
vWWXJTa8Ldsj5a_meYGXbskwO2tAubBQwyK6Cg77v5oDJAfyD-uS08F8JA0yj139iF05sKSHwxYt7qX_lZxg9g

=== Verify Document Exists === ✓ Document found: [id, user_id, session_id,
created_at, ttl_duration]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
4ZJLa15K1x4_afd147CNTPE8RXxAnwXXL0DhmxHYS-nxnwbdzpwuNAkQfmBFZlvCLc7pnozuRK_rGOyn1gd1Iw

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists:
[ttl_duration, cache_key, id, value]

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client === ✓ WebSocket client created

=== Connect to WebSocket === ✓ Connected to WebSocket

=== Insert Test Record === ✓ Inserted test record

=== Find All via WebSocket === ✓ WebSocket findAll result:
{"payload":{"data":[{"id":"Yy2_0jfmuoMgpUv1zsPEln1i-VLDTIGOorCMmaXZh1EbJTQP_oweRWnxHF8dstQj-P5jvN03kfkWUbfe8W7X7A","name":{"type":"String","value":"Test
User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

=== Close WebSocket === ✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema === ✓ Created collection
'kotlin_schema_example' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
StringValue(value=NgP3LAVPCUT7dpu80P8L2kxyTKXK-VT2Ay9bjAkbD04C3El6u75jjS1PfKvEsZw2MeilSqaGxI8HbfDpYhhVLg)
✓ Inserted user 2:
StringValue(value=quJbDJzsaUeIW4TLZeV3HDHQl1-Gt-mY58z-b3qeYDEMRBDDf0Wo_8tKq7Z_XB36y4yxgo4Eqhm8HQa5GUA37Q)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 22 Sample:
[schema_users_client_py, schema_employees_client_py, test_collection,
chat_configurations__ek0_bench, chat_messages__ek0_bench]

=== Cleanup === ✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Search results for 'programming':
{"results":[{"record":{"description":"Learn Rust programming language with
hands-on examples and best practices.","views":8,"title":"Rust
Programming","id":"0Zb_pBgRyJKECfTfYXKod729WSetU-FDn4AIGA4qLdckhLISjShXskZwZYWoWP-2fBxSsjz6xHfW5rUMiGAjrQ","tags":"programming,rust,tutorial"},"score":19.8,"matched_fields":["description","title","tags"]},{"record":{"views":425,"tags":"programming,python,data-science","title":"Python
for Data Science","description":"Master Python for data analysis, machine
learning, and
visualization.","id":"mxLo-wBUymJXeJ08G2USv_FsoI0CbsD944Sro_ikJ75OlwAuXrWE6WVyGlCaDSaN1snWdotamOrnpkmQ64Gdww"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,javascript,web","views":854,"description":"Build
modern web applications using JavaScript, React, and
Node.js.","title":"JavaScript Web
Development","id":"MoyzlJavmItaTZxzSlUQBfdusesHNAbvuTAxUry6q1AH5oRy5AnkT5Qa6PlBs9lw1MBToCV6itdnRXa4d-osRg"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"views":850,"id":"\_9ZOm1cjVM9d33t_UlSfJ1Tigssguo5a63UQfjVpc0vR_uf2nCN5XsTG7BgFETRCapGwMmltClpjN5SnB1qevQ","tags":"ai,machine-learning,python","title":"Machine
Learning Basics","description":"Introduction to machine learning algorithms and
neural
networks."},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"description":"Master
Python for data analysis, machine learning, and visualization.","title":"Python
for Data
Science","tags":"programming,python,data-science","id":"mxLo-wBUymJXeJ08G2USv_FsoI0CbsD944Sro_ikJ75OlwAuXrWE6WVyGlCaDSaN1snWdotamOrnpkmQ64Gdww","views":425},"score":1.0,"matched_fields":["description"]},{"record":{"views":8,"tags":"programming,rust,tutorial","description":"Learn
Rust programming language with hands-on examples and best
practices.","title":"Rust
Programming","id":"0Zb_pBgRyJKECfTfYXKod729WSetU-FDn4AIGA4qLdckhLISjShXskZwZYWoWP-2fBxSsjz6xHfW5rUMiGAjrQ"},"score":0.5,"matched_fields":["description"]},{"record":{"id":"oMEAz5Zi3HxKX1mxeM9ufIB_dhMBQbKFQ_yW-Y5zbbOnozfJrZdhSXib1gaBevQLa-V8tbdLAchbMG10ujdVaA","views":2,"title":"Database
Design","description":"Learn database design principles, normalization, and
query
optimization.","tags":"database,design,sql"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup === ✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
StringValue(value=Jd03U9uygmL9cH-ksajF0jDI4vow2_iNtI6X0OMjaM6yiDfHN2EvZX1CKN11K0KZBc_2leTZZV5ubEBHWcCMIw)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1765305725205},"id":"Jd03U9uygmL9cH-ksajF0jDI4vow2_iNtI6X0OMjaM6yiDfHN2EvZX1CKN11K0KZBc_2leTZZV5ubEBHWcCMIw","name":{"type":"String","value":"WebSocket
TTL
Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Join Operations Example ===

=== Setting up sample data === ✓ Sample data created

=== Example 1: Single collection join (users with departments) === ✓ Found 2
users with department data

- Bob Smith: Sales
- Alice Johnson: Engineering

=== Example 2: Join with filtering === ✓ Found 1 users in Engineering

- Alice Johnson: Building A

=== Example 3: Join with user profiles === ✓ Found 2 users with profile data

- Bob Smith: Sales Manager
- Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data === ✓ Found 2 completed orders

- Mouse ($25) by Alice Johnson
- Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully

BUILD SUCCESSFUL in 4s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
hEkr3R1pmJ9cykn2mYps7s2tFBXqFse1LHeDi2_vGzRog6AtFplh8lwVLsjwAJV-FEYg442W-DACwwZxNxWlmA

=== Sending Chat Message === ✓ Chat response: Message ID:
"azRlZP8zIldwt-LgyLMu0XDrhWvGcXWcAiBNHVftmyDGWRFZRXzmyVwBbw9SGNGnGbKH41b5KYBbYI_IpLZumA"
Responses: ["I'm sorry, but the provided context does not contain any
information about what ekoDB is or what features it has."]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 5s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
pOijMiS-nMN0LYt--ZiOsf2I84L3ISe_UdJsK2XP-chgEGWGEM5ByOYeVnoNS-lMX50goi6yFsRcXpfaTFIVjQ

=== Sending Messages === ✓ Message 1 sent Responses: ["I'm sorry, but the
context provided does not contain specific information about any available
products. Could you please provide more details or a specific product you're
interested in?"]

✓ Message 2 sent Responses: ["Apologies, but there's no information provided
about the product's price in the context. Could you please provide more details
or specify which product you're asking about?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"pOijMiS-nMN0LYt--ZiOsf2I84L3ISe_UdJsK2XP-chgEGWGEM5ByOYeVnoNS-lMX50goi6yFsRcXpfaTFIVjQ"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"qPzuw8kyMdIRzPRoNPaMihmn4erUSygYd5lrVKTtckGyarUtEl7lfSXjRBq3LDDO6Xso-kz4-qiK4MlaIgIiLQ","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-09T18:42:22.458209Z"},"id":"wHIOF4yTNdnqrcRxU7Kdqzk72T9uE7kJXGfZsEes8nI98ADyeUQ9gJ07_DCUJNcIFrFQgCW7bK0vm7kHks1aBQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":153,"total_tokens":186}},"updated_at":{"type":"DateTime","value":"2025-12-09T18:42:22.458209Z"}},{"chat_id":{"type":"String","value":"pOijMiS-nMN0LYt--ZiOsf2I84L3ISe_UdJsK2XP-chgEGWGEM5ByOYeVnoNS-lMX50goi6yFsRcXpfaTFIVjQ"},"content":{"type":"String","value":"I'm
sorry, but the context provided does not contain specific information about any
available products. Could you please provide more details or a specific product
you're interested
in?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"qPzuw8kyMdIRzPRoNPaMihmn4erUSygYd5lrVKTtckGyarUtEl7lfSXjRBq3LDDO6Xso-kz4-qiK4MlaIgIiLQ","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-09T18:42:22.463572Z"},"id":"mTQPPQ-Ig-ItzUGmH8d9UoKR5Fd9KqD9wBsiSIGc0DFaQHIAWejm7kbC-JueYqm5pHB4j7Slo5azE8aEugrhZA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":153,"total_tokens":186}},"updated_at":{"type":"DateTime","value":"2025-12-09T18:42:22.463572Z"}},{"chat_id":{"type":"String","value":"pOijMiS-nMN0LYt--ZiOsf2I84L3ISe_UdJsK2XP-chgEGWGEM5ByOYeVnoNS-lMX50goi6yFsRcXpfaTFIVjQ"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-12-09T18:42:24.605268Z"},"id":"ysTlOHJXsB-b0RYii8iNYrv6NNHw66Gs2PiD3hbDxqxiBQSt3ZN0W2x4f-bfjeQNIV-zAg0tCHF7KGhH14EY4w","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":70,"total_tokens":103}},"updated_at":{"type":"DateTime","value":"2025-12-09T18:42:24.605268Z"}},{"chat_id":{"type":"String","value":"pOijMiS-nMN0LYt--ZiOsf2I84L3ISe_UdJsK2XP-chgEGWGEM5ByOYeVnoNS-lMX50goi6yFsRcXpfaTFIVjQ"},"content":{"type":"String","value":"Apologies,
but there's no information provided about the product's price in the context.
Could you please provide more details or specify which product you're asking
about?"},"created_at":{"type":"DateTime","value":"2025-12-09T18:42:24.608980Z"},"id":"QCRgoHNg0V2DdkvFqspSKlt0j6wYDgQn53RF1dEJOV-3oKYY3mTgXSyQSRsEFYXiCxTTylfxKG_C7qxmc6hvVQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":70,"total_tokens":103}},"updated_at":{"type":"DateTime","value":"2025-12-09T18:42:24.608980Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
CEsDz7CLwMDwZhoS6d8WetD9frzGVKdXbh-thQUE4UqGSUkuscMWnq8G_zu6_MeX7Gh66hbINNukDXrQeKvP3A

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 8s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
QyrffLSJVIb09z5dBcwP_OFkhNSpG3sYClkRfd-1l8i4yLAG948U00nF1hb4quvzXqP0O9Sq5O0s_BjOF3Wm4Q

=== Sending Initial Message === ✓ Message sent Responses: ["I'm sorry, but the
context provided does not include any information about the available products.
Could you please provide more details?"]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses: ["I'm
sorry, but the context provided doesn't contain any specific product
information. Could you please provide more details or clarify what kind of
products you're interested in?"]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
KS_EdkIzOo9gg3gEof0UfmrOFm6qYnv6lt7RNYs6yumCLaWK5VjfrSM5Qi_tsvIJ4IOjVlHZKvaZK-Zc1lCILg

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 10s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run 🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to no-operation
(NOP) logger implementation SLF4J(W): See
https://www.slf4j.org/codes.html#noProviders for further details. ✅ Client
initialized

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
9eRim0u4tfo-fzQb2i88ACi4ytZUzGyEtFoSfhvGsoOtwBoS0Cesif-wUNOjTl1R8k13uUuIlTvcyEqAUt7nVw
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"count":5,"avg_score":60.0,"status":"active"}
{"avg_score":50.0,"status":"inactive","count":5} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Script listing skipped (some scripts contain parameter placeholders) 🔍
Retrieved script: Get Active Users ✏️ Script updated 🗑️ Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 10 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ⚠️ Script cleanup skipped (some scripts
contain parameter placeholders)

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • Built-in error handling

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date ✅
[32mKotlin client examples complete![0m Kotlin example (requires kotlinc
installed)
