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
Record(fields={id=StringValue(value=wSvKi-yRIPcuALetPLM3C64b5CNKs56f8K9XiNCo96dROuw6zqQV5A0Vh-qneP00VID6ehptKEQfwUFdmdgR_A)})
User ID:
wSvKi-yRIPcuALetPLM3C64b5CNKs56f8K9XiNCo96dROuw6zqQV5A0Vh-qneP00VID6ehptKEQfwUFdmdgR_A

=== Read === ✓ Found user by ID:
Record(fields={name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
id=StringValue(value=wSvKi-yRIPcuALetPLM3C64b5CNKs56f8K9XiNCo96dROuw6zqQV5A0Vh-qneP00VID6ehptKEQfwUFdmdgR_A),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)})})

=== Update === ✓ Updated user:
Record(fields={email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
city=ObjectValue(value={value=StringValue(value=San Francisco),
type=StringValue(value=String)}),
name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=29)}),
id=StringValue(value=wSvKi-yRIPcuALetPLM3C64b5CNKs56f8K9XiNCo96dROuw6zqQV5A0Vh-qneP00VID6ehptKEQfwUFdmdgR_A)})

=== Query === ✓ Found 1 users matching query

- Record(fields={active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  city=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=San Francisco)}),
  name=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Alice Johnson)}),
  age=ObjectValue(value={value=IntegerValue(value=29),
  type=StringValue(value=Integer)}),
  email=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=alice@example.com)}),
  id=StringValue(value=wSvKi-yRIPcuALetPLM3C64b5CNKs56f8K9XiNCo96dROuw6zqQV5A0Vh-qneP00VID6ehptKEQfwUFdmdgR_A)})

=== Delete === ✓ Deleted user with ID:
wSvKi-yRIPcuALetPLM3C64b5CNKs56f8K9XiNCo96dROuw6zqQV5A0Vh-qneP00VID6ehptKEQfwUFdmdgR_A

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
oY4ckp466jvlgn3SdAhPOpHGNEv0a2zp2iUgo4voDC0aQQlt-RrYCwv7KPprLpy9datwSkIyzu4zHuFKTgFyww,
BWQsxHLvFLAfXijVh9d3-TCVXBjy_99MrWvIpQpwmMvraVuWpYYaZW4hpibtIdwvRhaVQaER4oW43QEteTBIFQ,
cXZvviK-gD3R1c8yji22cZHkVGo2O7zUa99BEaEbXLmni6jbTlomc_y5PTqQQbkzJoG5n34u8h2ilvjRh4c1sQ...

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

- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=95)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=92)})
- Score: ObjectValue(value={value=IntegerValue(value=88),
  type=StringValue(value=Integer)})

=== Query 4: Complex (score > 80 AND age >= 25) === ✓ Found 4 high-scoring
adults

=== Query 5: IN (city IN ['NYC', 'LA']) === ✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) === ✓ Page 2 (2 records):

- ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Charlie)})
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
{"role":"admin","name":"Alice","email":"alice@example.com"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"created_at":1765262530624,"user_id":"123"} (Will expire in 10 seconds)

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

- schema_employees_client_py
- schema_products_client_go
- schema_documents_client_go
- schema_users_client_py
- websocket_test

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-09T06:42:14.850313Z","last_modified":"2025-12-09T06:42:14.850314Z","bypass_ripple":false,"primary_key_alias":"id"}

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
tNMU0os4OS8sZ1jvqwmROtFxGdTv63int8L-2qUCKE0uDlXLvmbahzYR8b-XZpqYPD4N_m_hpREIVZYQslfisQ

=== Verify Document Exists === ✓ Document found: [id, created_at, session_id,
user_id, ttl_duration]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
\_XSWYI21welVEqFrBkfLTw80tfQ2V3syJKYmYfe4_mAgpkn-aSOHXSD7c0FJrYqPqzVzkPyrCkgVHYSt4WRB-A

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [id,
cache_key, ttl_duration, value]

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
{"payload":{"data":[{"id":"5nL1q-gMDRhlwSRsjzOBVKyu_lsCHdB7BnTcfBwCNYoPmwU-Wbk5wV-TrkuP0_AOnu3f8R3458GppwdVTGiQxQ","name":{"type":"String","value":"Test
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
StringValue(value=MS7WdKI6-WWZU9qWVxxF8lm82VLJUt2kp-IRJJUo1uv6kkCuGfTm3npeilXIQgbHTFjXEu8WrE_1x-iPOqVs0g)
✓ Inserted user 2:
StringValue(value=TpldEhD5GQeqzuBoIByQzs6fx-OKOniFzuC2nkibe28EOghpFQQlDxwaAGNPobV2zm0uNbPWgTPDU1J-6DAS2A)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 22 Sample:
[schema_employees_client_py, kotlin_schema_example, schema_products_client_go,
schema_documents_client_go, schema_users_client_py]

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
{"results":[{"record":{"id":"IaGAXRfQJIm9Kpq0GJVTY37QqGsqIX1FHoEkV5KwX25mxCroFKtPXrzNOMWJvfjSMdQn7sdJQyWqF43dIsna2g","tags":"programming,rust,tutorial","views":252,"title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices."},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"description":"Build
modern web applications using JavaScript, React, and
Node.js.","tags":"programming,javascript,web","id":"nxlEb2faJO50NiYP8vDCSMz5W-jFhGhQUnISOy-5X2WGsugKzWDADWoxGaw2AjHq1LAlFCJalwPkQE9N0yrxKg","views":432,"title":"JavaScript
Web
Development"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"id":"BCMkr7V3HSMqtRyz0ADT5vrNWvHA3m_IwdtaiHKRBvoRaWHmI-3BVuAKTpykDL_emQ0btKRAR4yCztwPGviT1w","title":"Python
for Data Science","description":"Master Python for data analysis, machine
learning, and
visualization.","tags":"programming,python,data-science","views":552},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"description":"Introduction to machine learning
algorithms and neural networks.","views":923,"title":"Machine Learning
Basics","tags":"ai,machine-learning,python","id":"4e7kiO4u2mPAzIu4G330ZacrfLa-jlzcTjZk3a82TwWGYPz9yPfLQHPL3rtdqsJ86X_4XvuF7VyxAHp7apAkHQ"},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"description":"Master
Python for data analysis, machine learning, and visualization.","title":"Python
for Data
Science","views":552,"id":"BCMkr7V3HSMqtRyz0ADT5vrNWvHA3m_IwdtaiHKRBvoRaWHmI-3BVuAKTpykDL_emQ0btKRAR4yCztwPGviT1w","tags":"programming,python,data-science"},"score":1.0,"matched_fields":["description"]},{"record":{"description":"Learn
database design principles, normalization, and query
optimization.","id":"3YnzEStGg6wb27G6LS5f6-bFHqgsGDPC2QNTobk9zgTXCOat3Mo7G1pIAyw5igh8_iC-yXgkiDk9UVd49p4yAw","title":"Database
Design","tags":"database,design,sql","views":510},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn
Rust programming language with hands-on examples and best
practices.","id":"IaGAXRfQJIm9Kpq0GJVTY37QqGsqIX1FHoEkV5KwX25mxCroFKtPXrzNOMWJvfjSMdQn7sdJQyWqF43dIsna2g","views":252,"title":"Rust
Programming","tags":"programming,rust,tutorial"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
StringValue(value=2hwxOQ6emv8STEp5L7PnRxmlTC5i_KyLeOreUDqc7pU2zD5LyVH4lDnMlEXPS8xfiOZ6N1crHNxkiuTChHxUzg)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1765262555826},"id":"2hwxOQ6emv8STEp5L7PnRxmlTC5i_KyLeOreUDqc7pU2zD5LyVH4lDnMlEXPS8xfiOZ6N1crHNxkiuTChHxUzg","name":{"type":"String","value":"WebSocket
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

- Alice Johnson: Engineering
- Bob Smith: Sales

=== Example 2: Join with filtering === ✓ Found 1 users in Engineering

- Alice Johnson: Building A

=== Example 3: Join with user profiles === ✓ Found 2 users with profile data

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

=== Example 4: Join orders with user data === ✓ Found 2 completed orders

- Laptop ($1200) by Alice Johnson
- Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully

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
> Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
w6xFbh5G1TSMio4O_z1G-tTj0fHQMwFHsXj0ySMSnj1Bfys22ZTA0ufV0oBb62rmPN7CBBn9VYPIm76GsRj-Yg

=== Sending Chat Message === ✓ Chat response: Message ID:
"GHKk2Xk-r_OVpzxiALra-tQz3h_Afw5JGAfYy_jZQpchOY2Cnlv928JEXoun4gLO_8KZh5rfyz4Axha7QfjZcQ"
Responses: ["I'm sorry for the inconvenience, but the context provided does not
contain any specific information about ekoDB or its features. Could you please
provide more details or a different context? I'd be glad to assist you with the
right information."]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 7s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
jQ-mql9ApPLJmQlnK462WYrSw4Xz8Vgz71NWvZCkplAXcQ_KOutyq3tvX-VBYerhVeFNubLcFA76gCH8wH3JIg

=== Sending Messages === ✓ Message 1 sent Responses: ["I'm sorry, but the
context provided does not include specific information about available products.
Could you please provide more details or context?"]

✓ Message 2 sent Responses: ["I'm sorry, but there's no information provided
about the product or its price in the context. Could you please provide more
details?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"jQ-mql9ApPLJmQlnK462WYrSw4Xz8Vgz71NWvZCkplAXcQ_KOutyq3tvX-VBYerhVeFNubLcFA76gCH8wH3JIg"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"1YDlwG54PeucCUd0L-vUlSmOOCg3E6OazJ2nzMmlfFU6eR1Qfq1F8MugKQlcj74xUzwqJb_yW3FgBLSnu3Cj5w","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-09T06:42:53.891381Z"},"id":"gtQS2zX7g0ZgV16E4mAp1M145F3kLCXuJ-oJ0_OYopijSmsq_S3pW8z1Me8G96CTMZSfnz_KjZnXYoH5tWSzKQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":158,"total_tokens":184}},"updated_at":{"type":"DateTime","value":"2025-12-09T06:42:53.891381Z"}},{"chat_id":{"type":"String","value":"jQ-mql9ApPLJmQlnK462WYrSw4Xz8Vgz71NWvZCkplAXcQ_KOutyq3tvX-VBYerhVeFNubLcFA76gCH8wH3JIg"},"content":{"type":"String","value":"I'm
sorry, but the context provided does not include specific information about
available products. Could you please provide more details or
context?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"1YDlwG54PeucCUd0L-vUlSmOOCg3E6OazJ2nzMmlfFU6eR1Qfq1F8MugKQlcj74xUzwqJb_yW3FgBLSnu3Cj5w","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-09T06:42:53.894886Z"},"id":"UzHq2L5Yk6Dr5ewZw1USXTog_uQw7t89FDewff2ojeh6gZO3UL55eHAEMK6n4HQCkLHG4BtQe869Qfmq0N6gow","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":158,"total_tokens":184}},"updated_at":{"type":"DateTime","value":"2025-12-09T06:42:53.894886Z"}},{"chat_id":{"type":"String","value":"jQ-mql9ApPLJmQlnK462WYrSw4Xz8Vgz71NWvZCkplAXcQ_KOutyq3tvX-VBYerhVeFNubLcFA76gCH8wH3JIg"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-12-09T06:42:55.321462Z"},"id":"kbozPGUxKkaWngfoTTICWRjZexblR7-Bdg1g4YgtBkhwPyKpm1Pa301V_9Ovg6H3LC6lKrEQ19f0C7sXjW-YEw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":27,"prompt_tokens":70,"total_tokens":97}},"updated_at":{"type":"DateTime","value":"2025-12-09T06:42:55.321462Z"}},{"chat_id":{"type":"String","value":"jQ-mql9ApPLJmQlnK462WYrSw4Xz8Vgz71NWvZCkplAXcQ_KOutyq3tvX-VBYerhVeFNubLcFA76gCH8wH3JIg"},"content":{"type":"String","value":"I'm
sorry, but there's no information provided about the product or its price in the
context. Could you please provide more
details?"},"created_at":{"type":"DateTime","value":"2025-12-09T06:42:55.324718Z"},"id":"3Q3aFWhjcETN6J3nVZT1tMgn3e5v7GtVr_HkWvcIAh0OK7aIiYVkSfDldYPlki0MC9LVa3vPV2hRUSu0ojyv3Q","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":27,"prompt_tokens":70,"total_tokens":97}},"updated_at":{"type":"DateTime","value":"2025-12-09T06:42:55.324718Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
p43fU7mEirBQB9li20PnMZeERlttLSmoKYKA_3cw2rchXTKGEP95esGkTYwi0FUHUNnt2m2J4eEQjkeXE463tg

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 6s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
vfVFT2wW7LSP7omm3gIzyJty926EIGVyM-2CWm4D3xFb8l-eV5RoJTQ6Vs6K0XeHbh755zllWdQPykNR2JwnGw

=== Sending Initial Message === ✓ Message sent Responses: ["I'm sorry, but the
context provided does not contain information about any specific products. Could
you please provide more details or a different context?"]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses: ["I'm
sorry, but the context provided does not include any information about available
products. Can you provide more details or try again?"]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
aX4zzkC5xiItRHpGT_ASx3o-fwJ-FOxjLGuJwqbxSarrppSehvPYiz7zXeKJEVv6FbDYn0Qb25pJTBgloHTh9g

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 7s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
6rMqCJMox4UUUAu3dhXPg5Ismaf_VpXwDTpHDv_nAQca-wT_9ogvJUmZE71YzBgmcFY6zF86VVHFoCJf3jqCFQ
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"avg_score":50.0,"status":"inactive","count":5}
{"status":"active","count":5,"avg_score":60.0} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 27 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 10 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • Built-in error handling

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date ✅
[32mKotlin client examples complete![0m Kotlin example (requires kotlinc
installed)
