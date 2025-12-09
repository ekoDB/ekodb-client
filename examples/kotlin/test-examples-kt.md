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
Record(fields={id=StringValue(value=DBMi7robOq2yk1h_VhsiIoP3URs_EJ2SRSyijL089hOY_oKelbkZnu89KDLZ_5AWB1jzTW8jg0i-S_S469DJKw)})
User ID:
DBMi7robOq2yk1h_VhsiIoP3URs_EJ2SRSyijL089hOY_oKelbkZnu89KDLZ_5AWB1jzTW8jg0i-S_S469DJKw

=== Read === ✓ Found user by ID:
Record(fields={id=StringValue(value=DBMi7robOq2yk1h_VhsiIoP3URs_EJ2SRSyijL089hOY_oKelbkZnu89KDLZ_5AWB1jzTW8jg0i-S_S469DJKw),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
age=ObjectValue(value={value=IntegerValue(value=28),
type=StringValue(value=Integer)}),
email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)})})

=== Update === ✓ Updated user:
Record(fields={age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=29)}),
city=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=San Francisco)}),
id=StringValue(value=DBMi7robOq2yk1h_VhsiIoP3URs_EJ2SRSyijL089hOY_oKelbkZnu89KDLZ_5AWB1jzTW8jg0i-S_S469DJKw),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)})})

=== Query === ✓ Found 1 users matching query

- Record(fields={city=ObjectValue(value={value=StringValue(value=San Francisco),
  type=StringValue(value=String)}),
  email=ObjectValue(value={value=StringValue(value=alice@example.com),
  type=StringValue(value=String)}),
  name=ObjectValue(value={value=StringValue(value=Alice Johnson),
  type=StringValue(value=String)}),
  active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  id=StringValue(value=DBMi7robOq2yk1h_VhsiIoP3URs_EJ2SRSyijL089hOY_oKelbkZnu89KDLZ_5AWB1jzTW8jg0i-S_S469DJKw),
  age=ObjectValue(value={value=IntegerValue(value=29),
  type=StringValue(value=Integer)})})

=== Delete === ✓ Deleted user with ID:
DBMi7robOq2yk1h_VhsiIoP3URs_EJ2SRSyijL089hOY_oKelbkZnu89KDLZ_5AWB1jzTW8jg0i-S_S469DJKw

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
DDuw1GWURg73wlFIrvL4qiAmvuwK7BmflKOirM1aGRRlvx6EESGZulzAagtiOlZAEjN8LlujYjqfbKzfvC4gBA,
mUD1XRGdILa852QqKNh5DfaHnvpZE4dQgLNkyU6ZcYfVU8vRL6FrfEsdesYUsKmINGWnleVJwLb3ta9enwhTLA,
hqvbKdfE3KDzbt643g1brbU1j64hWb1-nIW605ZXQUrSbkf7WO50z8YpLz4Mn3Z4jN0V_flVMOwE6rrn49_w3g...

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

- ObjectValue(value={value=StringValue(value=Charlie),
  type=StringValue(value=String)})
- ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Diana)})

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
{"role":"admin","email":"alice@example.com","name":"Alice"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"created_at":1765258857427,"user_id":"123"} (Will expire in 10 seconds)

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

- schema_products_client_ts
- schema_documents_client_py
- schema_documents_client_go
- schema_users_client_ts
- test_collection

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-09T05:41:01.657846Z","last_modified":"2025-12-09T05:41:01.657846Z","bypass_ripple":false,"primary_key_alias":"id"}

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
yl-wJSu97elnSTfcgRolZPSTTq2r3YBoSAmvXPVlE00BnlOiqSgCSs729iO3T94fxKqZVvK1yD9rGPp7wHFt_A

=== Verify Document Exists === ✓ Document found: [created_at, ttl_duration,
session_id, user_id, id]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
6tiBON2jg2RokB1849zVZXyq6mjtcs-JJoqrbf9OGJDd5kJiEyb06LthXVaROAUwZvlz01TneF7K9W4G29K1Dg

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
{"payload":{"data":[{"id":"3oEdzIzOSD5TlZaBHxWflCL2fLt8YDqYwjxlWdzxHewT_uNsM-6e4qFJLzk8fi3wy50ZwMzJZrn-vX78_NKXZQ","name":{"type":"String","value":"Test
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
StringValue(value=8Eziy2_mDlGPgmOK07VVepHCLVH_Jv8ScUPfxUVnLcbTINubzjELPnEPdOlmMbdlMfYF_E7ZfbyhoACHm7Gkgw)
✓ Inserted user 2:
StringValue(value=Km2JQq6OLThBhl7XWFmu0zXlrMU3jo4EjtVEH-RpocHlb8QklryrCTeaEvhAvzLAdKtWbh-G1CQfZp40ZSjcVQ)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 22 Sample:
[schema_products_client_ts, schema_documents_client_py,
schema_documents_client_go, kotlin_schema_example, schema_users_client_ts]

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
{"results":[{"record":{"views":21,"tags":"programming,rust,tutorial","title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices.","id":"sjX-PgZlkkthLg2HQXGGVAYRv4AWlUbjSMTfoq_2-lPWW0MMbo4g19sAb8e2avgTXwWOnQnkWwvXm1hmx0ivVg"},"score":19.8,"matched_fields":["description","tags","title"]},{"record":{"views":800,"title":"JavaScript
Web Development","description":"Build modern web applications using JavaScript,
React, and
Node.js.","tags":"programming,javascript,web","id":"DZ2_4lDPuV1DimgURCAaQ9bl39aDP6rSAww7aQbZlnLZHHVu7WzMIfCZi3XH7ReWnGg9YjVaRinqgKrU2S3GTQ"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":"Master
Python for data analysis, machine learning, and
visualization.","tags":"programming,python,data-science","id":"cqXU50FZ90ovbQmVdMKUETeNm406nIDMiBjEUPbgdkoQE1RAfiwjPW6rVTcivHGMR5QZAhh3DX24ISp-fSO9qA","title":"Python
for Data
Science","views":435},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"tags":"ai,machine-learning,python","id":"OJbmP3OTYitZQgCxZGJ3v9FZ0n0GhwCGz8EBK5q07HyjAv1qfR8f-17nq5MTjXLqtR-qtN7DZfCTrx0xaacMFA","description":"Introduction
to machine learning algorithms and neural
networks.","views":419,"title":"Machine Learning
Basics"},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"title":"Python
for Data Science","views":435,"description":"Master Python for data analysis,
machine learning, and
visualization.","tags":"programming,python,data-science","id":"cqXU50FZ90ovbQmVdMKUETeNm406nIDMiBjEUPbgdkoQE1RAfiwjPW6rVTcivHGMR5QZAhh3DX24ISp-fSO9qA"},"score":1.0,"matched_fields":["description"]},{"record":{"id":"mZ7Lf0i3Pi6A8b-viX0fc7mVMeXpP1eeA8ENkL8K33mvVOPvWSuBVJNB_UBl5SQ-rzf4fSwhDLeNiNIMqJ4r8A","description":"Learn
database design principles, normalization, and query
optimization.","views":639,"tags":"database,design,sql","title":"Database
Design"},"score":0.5,"matched_fields":["description"]},{"record":{"tags":"programming,rust,tutorial","id":"sjX-PgZlkkthLg2HQXGGVAYRv4AWlUbjSMTfoq_2-lPWW0MMbo4g19sAb8e2avgTXwWOnQnkWwvXm1hmx0ivVg","title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices.","views":21},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
StringValue(value=3vjQ9_EkRpbppTtAFiI60BoinPcaYhhSNlgb_X4v1zWq0TekVQ53BJoOPLoiPFTiIQ1hsy8ZWT6FnOLfM5eQIg)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1765258882772},"id":"3vjQ9_EkRpbppTtAFiI60BoinPcaYhhSNlgb_X4v1zWq0TekVQ53BJoOPLoiPFTiIQ1hsy8ZWT6FnOLfM5eQIg","name":{"type":"String","value":"WebSocket
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
vkK-IPDpRaOkF4_S-ne1PCGvc2Lg-D3C-3NIe9e-vcnsFTDKllYynudFWiecXHHtja83hQ9FWUpG55nj5qFrVQ

=== Sending Chat Message === ✓ Chat response: Message ID:
"htI9Y82FdaLkd-zxlq-xGi_2i0KplSH9x8Xl0tJPqNZ5tTft-z13SG7fBCoE2Ao1ICYfczrm9mmfHeoSv6weBg"
Responses: ["I'm sorry, but the provided context doesn't contain any specific
information about ekoDB or its features. However, ekoDB is typically a database,
and databases usually have features like data storage, data management, data
retrieval, and security features. It would be best if you refer to the specific
documentation of ekoDB for a comprehensive list of its features."]

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
sgYZC8swkGOQl9EC-lDbQQy6Z-XChJXbXi1fNhrA-QAxJvpU85wZVrxQSIcrKwtkM7o-JE6QVmR52_ZKanDeFw

=== Sending Messages === ✓ Message 1 sent Responses: ["I'm sorry, but the
context provided doesn't contain any specific information about available
products. Could you please provide more details or a different context?"]

✓ Message 2 sent Responses: ["Apologies, but the context provided does not
include information about the price of a specific product. Could you please
mention the product you're interested in?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat*id":{"type":"String","value":"sgYZC8swkGOQl9EC-lDbQQy6Z-XChJXbXi1fNhrA-QAxJvpU85wZVrxQSIcrKwtkM7o-JE6QVmR52_ZKanDeFw"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"feYOzhXKE2broMytoD2Sm3vthuJ7TnOpkL6KNMHR-4WznCigYHiQwcBHyGb8Mu-EtqO_iSSJ6bBUnonzrx2_SA","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-09T05:41:40.082368Z"},"id":"K6Me49vQfDW7gBS4l6CydhI3YVZ57AxPYS8qH--cVE1u*-bIExfu13V1x1CVaagvHsZsh45D6jwlbI8w_becZg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":29,"prompt_tokens":150,"total_tokens":179}},"updated_at":{"type":"DateTime","value":"2025-12-09T05:41:40.082368Z"}},{"chat_id":{"type":"String","value":"sgYZC8swkGOQl9EC-lDbQQy6Z-XChJXbXi1fNhrA-QAxJvpU85wZVrxQSIcrKwtkM7o-JE6QVmR52_ZKanDeFw"},"content":{"type":"String","value":"I'm
sorry, but the context provided doesn't contain any specific information about
available products. Could you please provide more details or a different
context?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"feYOzhXKE2broMytoD2Sm3vthuJ7TnOpkL6KNMHR-4WznCigYHiQwcBHyGb8Mu-EtqO_iSSJ6bBUnonzrx2_SA","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-09T05:41:40.088651Z"},"id":"zYdfeCekwcLKiCkOhe6vctyCdIwqy7--prQcDAIOoS72jQyu0dp3I9kWpYsu1DzSksSxgeORFiLC6yfKJb83FA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":29,"prompt_tokens":150,"total_tokens":179}},"updated_at":{"type":"DateTime","value":"2025-12-09T05:41:40.088651Z"}},{"chat_id":{"type":"String","value":"sgYZC8swkGOQl9EC-lDbQQy6Z-XChJXbXi1fNhrA-QAxJvpU85wZVrxQSIcrKwtkM7o-JE6QVmR52_ZKanDeFw"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-12-09T05:41:41.718482Z"},"id":"CPRPn18qy18ERfZxBb2U6YrXs928-bZZyABzIGuBrYJXTFcT2Mp78OwbW8qwFdwQVgZhma_0IRF-StjRNrQEsg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":30,"prompt_tokens":70,"total_tokens":100}},"updated_at":{"type":"DateTime","value":"2025-12-09T05:41:41.718482Z"}},{"chat_id":{"type":"String","value":"sgYZC8swkGOQl9EC-lDbQQy6Z-XChJXbXi1fNhrA-QAxJvpU85wZVrxQSIcrKwtkM7o-JE6QVmR52_ZKanDeFw"},"content":{"type":"String","value":"Apologies,
but the context provided does not include information about the price of a
specific product. Could you please mention the product you're interested
in?"},"created_at":{"type":"DateTime","value":"2025-12-09T05:41:41.719368Z"},"id":"ZRHA5LB-cDQEk87w5ssSsISwnb0DFRuCCjmCgBiIj7dbQzaP7hETGRqDRGtVq0hOA239JHNGOwPvaKeUKpwtaA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":30,"prompt_tokens":70,"total_tokens":100}},"updated_at":{"type":"DateTime","value":"2025-12-09T05:41:41.719368Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
NZ0oeQ20I3YPCBzyFl5U3Tyrp7kP0ZJyb6XCw4UXuPiDXjQ3JWz2Aoqpuhd8aIKesKGW2DNJV_1MDDofITVgPQ

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
hjk0IjJSsS1-zr-3_dW0JcE60OA1XbdoDMy7q1LSjhERjKdFVjXkJlo4eznmbT60TesLKchwCl1zJtiCg_Eqgg

=== Sending Initial Message === ✓ Message sent Responses: ["I'm sorry, but the
provided context does not include specific information about available products.
Could you please provide more details or context?"]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses: ["I'm
sorry, but the context provided does not contain any specific product
information. Can you provide more details or context?"]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
F1ax2mm2phQmWFqOO7INRlkwNqg1aXGqFn3Io6-C_aopbQX-Nm_Q2CwIBBVnBPAdnr-PCcN95EQfRsiEywy1rw

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
i6bN2N6ZHn4_5QFllaWN5Ek_UleP2INxXcNw8FBF8O_YdHixDSSvmY81-gTF_XHVSTXcrgwBYwGlcbMOGNe6Dg
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"avg_score":60.0,"status":"active","count":5}
{"avg_score":50.0,"count":5,"status":"inactive"} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 21 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
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
