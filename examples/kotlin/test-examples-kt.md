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

BUILD SUCCESSFUL in 4s 2 actionable tasks: 2 up-to-date ✅ [32mKotlin client
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
Record(fields={id=StringValue(value=DYLDzoSxXtf2Se0dxlK22sEMk9rNl9pUEObwn6C6foszoOtDQ-CgwseRWHV2UH1tIt5FrSahUbVPBukz6zCMmA)})
User ID:
DYLDzoSxXtf2Se0dxlK22sEMk9rNl9pUEObwn6C6foszoOtDQ-CgwseRWHV2UH1tIt5FrSahUbVPBukz6zCMmA

=== Read === ✓ Found user by ID:
Record(fields={age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
id=StringValue(value=DYLDzoSxXtf2Se0dxlK22sEMk9rNl9pUEObwn6C6foszoOtDQ-CgwseRWHV2UH1tIt5FrSahUbVPBukz6zCMmA)})

=== Update === ✓ Updated user:
Record(fields={city=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=San Francisco)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=29)}),
id=StringValue(value=DYLDzoSxXtf2Se0dxlK22sEMk9rNl9pUEObwn6C6foszoOtDQ-CgwseRWHV2UH1tIt5FrSahUbVPBukz6zCMmA)})

=== Query === ✓ Found 1 users matching query

- Record(fields={email=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=alice@example.com)}),
  city=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=San Francisco)}),
  active=ObjectValue(value={value=BooleanValue(value=true),
  type=StringValue(value=Boolean)}),
  name=ObjectValue(value={value=StringValue(value=Alice Johnson),
  type=StringValue(value=String)}),
  age=ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=29)}),
  id=StringValue(value=DYLDzoSxXtf2Se0dxlK22sEMk9rNl9pUEObwn6C6foszoOtDQ-CgwseRWHV2UH1tIt5FrSahUbVPBukz6zCMmA)})

=== Delete === ✓ Deleted user with ID:
DYLDzoSxXtf2Se0dxlK22sEMk9rNl9pUEObwn6C6foszoOtDQ-CgwseRWHV2UH1tIt5FrSahUbVPBukz6zCMmA

✓ Confirmed user was deleted

=== Cleanup === ✓ Deleted collection: kotlin_users_example

=== Example Complete ===

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
> Kotlin Client - Batch Operations Example ===

=== Batch Insert === ✓ Inserted 5 records IDs:
Iv9qPC9jhprV2XojPYJBh1Oxa3VoNvH8OTPt4woi3hXI7FUjbpxVCbrcDQGJxvDjq4Syyd7qnTH1kS26m4hPSQ,
jX_MjHC4VQEnFqK92G5xVH7Xv3D79zkVnvMdgX6cuCMUshxGqRkGHiNjVXtrU1PU0UPqH-dZkSlhU6GrzSt9oA,
xwIBFxpaNKGWv4v_9zhW3caFcF8ydX3sweaXX6Qmy_oFb2dLQH0Ro9pppQ0hxbF1c0apTG3eRMoAO8ths5mp9A...

=== Batch Update === ✓ Updated 3 records

=== Batch Delete === ✓ Deleted 2 records

=== Cleanup === ✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

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
> Kotlin Client - Query Builder Example ===

=== Inserting Sample Data === ✓ Inserted 5 records

=== Query 1: Equality (city = 'NYC') === ✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) === ✓ Found 3 records with age 25-31

=== Query 3: Sort by score (descending) === ✓ Top 3 scores:

- Score: ObjectValue(value={value=IntegerValue(value=95),
  type=StringValue(value=Integer)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=92)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) === ✓ Found 4 high-scoring
adults

=== Query 5: IN (city IN ['NYC', 'LA']) === ✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) === ✓ Page 2 (2 records):

- ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Charlie)})
- ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Diana)})

=== Query 7: Contains (name contains 'a') === ✓ Found 2 names containing 'a'

=== Cleanup === ✓ Deleted collection: kotlin_query_example

=== Example Complete ===

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
> Kotlin Client - KV Operations Example ===

=== KV Set === ✓ Set key: user:123

=== KV Get === ✓ Retrieved value:
{"role":"admin","name":"Alice","email":"alice@example.com"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"created_at":1766278221922,"user_id":"123"} (Will expire in 10 seconds)

=== Set Multiple Keys === ✓ Set 3 configuration keys

=== KV Delete === ✓ Deleted key: user:123

=== Verify Deletion === ✓ Confirmed key was deleted

=== Cleanup === ✓ Cleaned up test keys

=== Example Complete ===

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
> Kotlin Client - Collection Management Example ===

=== List Collections === ✓ Found 21 collections

- schema_documents_client_py
- websocket_test
- schema_employees_client_py
- schema_documents_client_go
- schema_products_client_go

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-21T00:50:26.882476Z","last_modified":"2025-12-21T00:50:26.882476Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup === ✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

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
> Kotlin Client - Document TTL Example ===

=== Insert with TTL === ✓ Inserted document with 10s TTL Document ID:
Rhkz3_qZO13s0RY_nShvFPtcJ7f8eWIvR757qY425U0rnjSDnOaln2S08z5VXuF3tqA-aikPVK1T9IYhnE3hFQ

=== Verify Document Exists === ✓ Document found: [created_at, id, session_id,
user_id, ttl_duration]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
K_yScS0KTgMieE5F-4TKdVPg-yfKC3RkWNoSBOrzM-Zilic4oKzPxhSmHd57NCtX8VGFiwESPJK6DVHRgs6YUA

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [cache_key,
ttl_duration, id, value]

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

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
> Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client === ✓ WebSocket client created

=== Connect to WebSocket === ✓ Connected to WebSocket

=== Insert Test Record === ✓ Inserted test record

=== Find All via WebSocket === ✓ WebSocket findAll result:
{"payload":{"data":[{"id":"j16MA2iaMDaF1UaZ-CygKE9mKfmdl2TZ00YetpHQBKa7C7tF8qF3FOVr3TXI6aL6bq-lvnJAMr4YvIk2pO2d6A","name":{"type":"String","value":"Test
User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

=== Close WebSocket === ✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

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
> Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema === ✓ Created collection
'kotlin_schema_example' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
StringValue(value=Va-RKxvWmuYlgRHSSmNgQhngwwqxEB2bgamLVvfSqXVi4jiFTZ_SDmhXhFZV5Kd90-\_V_ce_cMXFJakwkdnMaQ)
✓ Inserted user 2:
StringValue(value=q5NOZVDFPSq5LTPId7klddKfELyH2g4B4gGVt7MxCO0NavyG-d5RANLVlIkMQSOUiHNNWF3GuZqBZ01ekVk0Hw)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 22 Sample:
[schema_documents_client_py, websocket_test, schema_employees_client_py,
schema_products_client_go, schema_documents_client_go]

=== Cleanup === ✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

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
> Kotlin Client - Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Search results for 'programming':
{"results":[{"record":{"description":"Learn Rust programming language with
hands-on examples and best
practices.","views":365,"tags":"programming,rust,tutorial","id":"t-pnMBiLjq99KXBFsoRO9brBMu-CEYNY0aDd_5m198OtpLHEErUPPA-OWm25xtlksfLJ8PHIVEAbLt6DqxAVYg","title":"Rust
Programming"},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"id":"Orh2u_I_J4DzNsn4xdRNk0QU7-dzBnQvpD2q0g43WxiCJsr-0b4B_sX-i8YEtS6XHjJYpYItHam1VvnnKVr1IQ","title":"JavaScript
Web Development","description":"Build modern web applications using JavaScript,
React, and
Node.js.","tags":"programming,javascript,web","views":759},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"views":162,"title":"Python
for Data Science","description":"Master Python for data analysis, machine
learning, and
visualization.","id":"QBJiHtQ521LuWUqSENoOZehB8H1tAtdioQFblez-FQ3rZn10nQcEFDUWBpo4ViGjEP0cu8ot9OnQpzx30s2qsw","tags":"programming,python,data-science"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"description":"Introduction to machine learning
algorithms and neural
networks.","id":"YZah-ArpamKEqiM2zQsBO9kd_tkS_x8UvVTr2n7F1EfPb3Qeo2Xn0xC6Ymz4PkYM7Rj7y7UvkeYba_HuFR0sAw","tags":"ai,machine-learning,python","views":790,"title":"Machine
Learning
Basics"},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"tags":"programming,python,data-science","id":"QBJiHtQ521LuWUqSENoOZehB8H1tAtdioQFblez-FQ3rZn10nQcEFDUWBpo4ViGjEP0cu8ot9OnQpzx30s2qsw","views":162,"description":"Master
Python for data analysis, machine learning, and visualization.","title":"Python
for Data
Science"},"score":1.0,"matched_fields":["description"]},{"record":{"description":"Learn
Rust programming language with hands-on examples and best
practices.","views":365,"tags":"programming,rust,tutorial","title":"Rust
Programming","id":"t-pnMBiLjq99KXBFsoRO9brBMu-CEYNY0aDd_5m198OtpLHEErUPPA-OWm25xtlksfLJ8PHIVEAbLt6DqxAVYg"},"score":0.5,"matched_fields":["description"]},{"record":{"views":488,"id":"cbZ0uiEWRzUO6tr6ZAITYxheiAPF26pGlW2-8Jbk3QqVxYfOx8ibzY1Lr2mzKBVEVeelddYax5Kxh_A7BXI31A","title":"Database
Design","description":"Learn database design principles, normalization, and
query
optimization.","tags":"database,design,sql"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup === ✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

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
> Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
StringValue(value=JfHgmInJrPi0tTZx-KOAoWq8CivhOC-RRCAFfddbuHwRvkhbYTEhjCrmMg7vTms2fLK3wy18ilZvlLbjjVUdyQ)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1766278251460},"id":"JfHgmInJrPi0tTZx-KOAoWq8CivhOC-RRCAFfddbuHwRvkhbYTEhjCrmMg7vTms2fLK3wy18ilZvlLbjjVUdyQ","name":{"type":"String","value":"WebSocket
TTL
Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration

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
c9d3Y8ewnf9MimH-TdhlUSTuu0jBCGvNFPABwxWqMwXHCkAq7fPz1tG0Pvj0zhyKlkLhdIJ7KkN7CVLqN4g1hw

=== Sending Chat Message === ✓ Chat response: Message ID:
"pTaBV29wFeCPpS-LT_MD4IOd4cXt8Y6ABZZxrqJRx0MBWJQw8uH1428HBOYJ-tPa_rxGhWjlmjGt63t6H_JcsA"
Responses: ["I'm sorry, but the provided context doesn't contain any specific
information about ekoDB or its features. Could you please provide more
information or context?"]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 28s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
21tc0ZL5xevd9DvKaU5he5n9dYIeW8e5sYN8hk5iuO-bh2C0FtwR9wBut5byshTcws2lmzrIpkjM2BtoiOGFsg

=== Sending Messages === ✓ Message 1 sent Responses: ["I'm sorry, but the
context provided does not contain specific information about any available
products. Could you please provide more details or context?"]

✓ Message 2 sent Responses: ["I'm sorry, but I can't provide the information
you're looking for because the context doesn't specify which product's price
you're asking about. Could you please provide more details or specify the
product?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat*id":{"type":"String","value":"21tc0ZL5xevd9DvKaU5he5n9dYIeW8e5sYN8hk5iuO-bh2C0FtwR9wBut5byshTcws2lmzrIpkjM2BtoiOGFsg"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"z5nvL6Bja9ayvUYpSap-KHmRokENKiYCsuiZfh5IZJPlq-w1wvpAzyTuVzW49inXYdIqGqX9kmLvn0yZBB8N1g","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-21T00:51:32.545516Z"},"id":"10Y48OXtQ_uwHEfuCQODnqEYWoUOGM-hpZ6AJBOENrz9BQB-GVii8sxBSyM3W-nMTXguSol6--XdBF3ITEjf4Q","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":27,"prompt_tokens":155,"total_tokens":182}},"updated_at":{"type":"DateTime","value":"2025-12-21T00:51:32.545516Z"}},{"chat_id":{"type":"String","value":"21tc0ZL5xevd9DvKaU5he5n9dYIeW8e5sYN8hk5iuO-bh2C0FtwR9wBut5byshTcws2lmzrIpkjM2BtoiOGFsg"},"content":{"type":"String","value":"I'm
sorry, but the context provided does not contain specific information about any
available products. Could you please provide more details or
context?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"z5nvL6Bja9ayvUYpSap-KHmRokENKiYCsuiZfh5IZJPlq-w1wvpAzyTuVzW49inXYdIqGqX9kmLvn0yZBB8N1g","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-21T00:51:32.549352Z"},"id":"fj_az8roYBQK9sl4UH7Oaa-OqcqGpVpK8bdnomCQVWC6dCLQBb0ogeAl-Si7EjUUTUve9aPt2Hd*-eo2h18cig","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":27,"prompt_tokens":155,"total_tokens":182}},"updated_at":{"type":"DateTime","value":"2025-12-21T00:51:32.549352Z"}},{"chat_id":{"type":"String","value":"21tc0ZL5xevd9DvKaU5he5n9dYIeW8e5sYN8hk5iuO-bh2C0FtwR9wBut5byshTcws2lmzrIpkjM2BtoiOGFsg"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-12-21T00:51:34.186469Z"},"id":"Wrv4I9CNgyIF-yp6yf6UT1ZRDIMj5T2tlFJ-qopVyRjKeTxEe4ud8N4mRJCYj6CHo8AP-xnN3fLC0BK7wFsMXQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":41,"prompt_tokens":70,"total_tokens":111}},"updated_at":{"type":"DateTime","value":"2025-12-21T00:51:34.186469Z"}},{"chat_id":{"type":"String","value":"21tc0ZL5xevd9DvKaU5he5n9dYIeW8e5sYN8hk5iuO-bh2C0FtwR9wBut5byshTcws2lmzrIpkjM2BtoiOGFsg"},"content":{"type":"String","value":"I'm
sorry, but I can't provide the information you're looking for because the
context doesn't specify which product's price you're asking about. Could you
please provide more details or specify the
product?"},"created_at":{"type":"DateTime","value":"2025-12-21T00:51:34.189195Z"},"id":"eciWUq5HpTWWpCUQ-d9V98DrhmrsDFJaMc9cYz9yV_dOVMZwolzR3XYwDs5CRfMwh6Cy6ctiXzmOSNmz97wPdA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":41,"prompt_tokens":70,"total_tokens":111}},"updated_at":{"type":"DateTime","value":"2025-12-21T00:51:34.189195Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
3EW1hxDZdhdIpFl3rOLPuRZxazXVtyjaa0poSZtKREEMZ7vzIlkNgwBj5Hg2Go9Ff8aEOGVRm15gyN6JGh8WaQ

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_sessions_example

✓ Chat session management example completed successfully

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
> Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
kX1v2DYNJHfXjaLO-q9j9FZh81RLW3F9ULSqpOnmvnXklN8xAOt3ZtflMeXT1ft3mf8MGkkKp6IHGW33m671mA

=== Sending Initial Message === ✓ Message sent Responses: ["I'm sorry, but the
context provided does not contain any specific information about available
products. Could you please provide more details or try again?"]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses: ["I'm
sorry, but the context provided does not include information about any specific
products. Could you please provide more details or clarify what type of products
you're interested in?"]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
loMignSlsWBaEn-oJlJY7DiwlfXYHWQVoJWvnP1qdF_qrae_f0VGbMMueyf9KXUYqLmyDSF_1PNgUtDb4-T0rQ

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 8s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
5Ppjw4l7d\_\_5fhNv-zUcQn-FM2_Vs8L0NPg98bHVB8xMlYtOZWRrk1cShfn6WTkg5AyxBP7VWQx9Xs6SnDcoLw
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"status":"active","avg_score":60.0,"count":5}
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

BUILD SUCCESSFUL in 4s 2 actionable tasks: 1 executed, 1 up-to-date ✅
[32mKotlin client examples complete![0m Kotlin example (requires kotlinc
installed)
