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
Record(fields={id=StringValue(value=3yJijN7r8ntdO5SnWpb_s9h9HUwYcUjFbonfVjScNSEYPDE-wWi1aCLydVYM1vqYY4T0So_nDLYDvd88aBqwqA)})
User ID:
3yJijN7r8ntdO5SnWpb_s9h9HUwYcUjFbonfVjScNSEYPDE-wWi1aCLydVYM1vqYY4T0So_nDLYDvd88aBqwqA

=== Read === ✓ Found user by ID:
Record(fields={age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
id=StringValue(value=3yJijN7r8ntdO5SnWpb_s9h9HUwYcUjFbonfVjScNSEYPDE-wWi1aCLydVYM1vqYY4T0So_nDLYDvd88aBqwqA),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)})})

=== Update === ✓ Updated user:
Record(fields={name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
age=ObjectValue(value={value=IntegerValue(value=29),
type=StringValue(value=Integer)}),
active=ObjectValue(value={value=BooleanValue(value=true),
type=StringValue(value=Boolean)}),
city=ObjectValue(value={value=StringValue(value=San Francisco),
type=StringValue(value=String)}),
id=StringValue(value=3yJijN7r8ntdO5SnWpb_s9h9HUwYcUjFbonfVjScNSEYPDE-wWi1aCLydVYM1vqYY4T0So_nDLYDvd88aBqwqA),
email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)})})

=== Query === ✓ Found 1 users matching query

- Record(fields={email=ObjectValue(value={value=StringValue(value=alice@example.com),
  type=StringValue(value=String)}),
  active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  city=ObjectValue(value={value=StringValue(value=San Francisco),
  type=StringValue(value=String)}),
  age=ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=29)}),
  name=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Alice Johnson)}),
  id=StringValue(value=3yJijN7r8ntdO5SnWpb_s9h9HUwYcUjFbonfVjScNSEYPDE-wWi1aCLydVYM1vqYY4T0So_nDLYDvd88aBqwqA)})

=== Delete === ✓ Deleted user with ID:
3yJijN7r8ntdO5SnWpb_s9h9HUwYcUjFbonfVjScNSEYPDE-wWi1aCLydVYM1vqYY4T0So_nDLYDvd88aBqwqA

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
9Fw1jsIDlyuDsT7cEW1SBUgJX8OPjG4mszFLSAiRHsacwv4Aj1UPyaKe_kpVvp-O75PuSuGDgGT-k91pAdkmSA,
d5fSbMcDNBH0o_Fu2x_oJ-TeRfv8w-OPbP3mQhbePGWStdp6cl88A7YWjf-2C94J3KVH9RAVitZjZSxSUj5hag,
3SXNaVSi2FAlEVxFJ9Ko1U4--60MVZh8rhWXB8nmrc8W4WpezsM97Umvs118df3mug4hXa6uni9L42TFK2Q4Ig...

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
- Score: ObjectValue(value={value=IntegerValue(value=92),
  type=StringValue(value=Integer)})
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
{"user_id":"123","created_at":1766219924365} (Will expire in 10 seconds)

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

- schema_employees_client_ts
- scripts\_\_ek0_bench
- websocket_test
- schema_documents_client_py
- schema_users_client_go

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-20T08:38:48.539030Z","last_modified":"2025-12-20T08:38:48.539030Z","bypass_ripple":false,"primary_key_alias":"id"}

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
NO6QcRM-pDDQ-7fRhv7zDOfP9EEnZKMBMhfK0Nd78N7awTkcLAOB1lglaPozaJ1eB4vyi95LzN7LeWeE_FDTmw

=== Verify Document Exists === ✓ Document found: [ttl_duration, id, session_id,
user_id, created_at]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
tEZ1jJ7KyB7bKd78aHaFV-fw7xbxqdUKNo18SXWl3vNX7zBcqycoAhutvoZgVNfLUaQTRQeX0kXDaZNSywbxDQ

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [cache_key,
ttl_duration, value, id]

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
{"payload":{"data":[{"id":"WCNvRgyX-YWGEly-xRexQ6pP-jzNX8VvXaPt7apAUljr4X_9b_qaxEZ7_oHB37bJklpp5EYrboNlHVmVKXFhcw","name":{"type":"String","value":"Test
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
StringValue(value=mzlxXcLypdyjaRRRyXsZJM6KSImCR6wFmBoOUQLGr0fTsUQJGeXzb4JL3SGSXmUMuk76_GPp6rOTDi_BM_4BHw)
✓ Inserted user 2:
StringValue(value=CCdP2ca1SjvmWPj08AOxtOc4aXD5lSbUVoIUd2fcTEGSUgqdwWdK2U-sm0DPLCO7n414Cu1QhcEUQWUZmBgWbw)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 22 Sample:
[schema_employees_client_ts, scripts__ek0_bench, websocket_test,
schema_documents_client_py, schema_users_client_go]

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
hands-on examples and best practices.","title":"Rust
Programming","views":60,"tags":"programming,rust,tutorial","id":"oOTI_G-gP52QhCpfdTnwMPCJiJXeoIiBWPPccliEOxNoR9p7IinfS5w8UxtHOBo158Jb5WJ8_QOoIWvcXJ3IMQ"},"score":19.8,"matched_fields":["title","tags","description"]},{"record":{"id":"Q_0kqWQdtjm8kQp2gizcbqEGPDxNUwKZ7vP6rKfi-GJ9mbjvXIlFtoQIECpj_HJZG5g47tvEziRndD-AvjNvaA","tags":"programming,javascript,web","views":13,"description":"Build
modern web applications using JavaScript, React, and
Node.js.","title":"JavaScript Web
Development"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"id":"AOnvVcrq-xp4rH8H-P4-MmtCwPBTc4NV4qFWwEuJ0izYwuF4WDmjJiceuH1wfhcliMZUA9DLAcnoJ9-btrgx6A","title":"Python
for Data Science","tags":"programming,python,data-science","description":"Master
Python for data analysis, machine learning, and
visualization.","views":356},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"views":856,"tags":"ai,machine-learning,python","id":"X2IsAYmYqXpgvQeZxrTKHUpw5XQkNFdLiGD3u56c7Jtv10QgtcpXgqYkRRwenPAkgQ71H04AsICSzYtxMyWZ4Q","description":"Introduction
to machine learning algorithms and neural networks.","title":"Machine Learning
Basics"},"score":2.7,"matched_fields":["title","description","tags"]},{"record":{"description":"Master
Python for data analysis, machine learning, and
visualization.","id":"AOnvVcrq-xp4rH8H-P4-MmtCwPBTc4NV4qFWwEuJ0izYwuF4WDmjJiceuH1wfhcliMZUA9DLAcnoJ9-btrgx6A","views":356,"tags":"programming,python,data-science","title":"Python
for Data
Science"},"score":1.0,"matched_fields":["description"]},{"record":{"tags":"programming,rust,tutorial","id":"oOTI_G-gP52QhCpfdTnwMPCJiJXeoIiBWPPccliEOxNoR9p7IinfS5w8UxtHOBo158Jb5WJ8_QOoIWvcXJ3IMQ","views":60,"title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices."},"score":0.5,"matched_fields":["description"]},{"record":{"title":"Database
Design","id":"N8RCiDAEY3uKN5sxqynAEV2WHZID--qbSKFwn8v5GL7Zxu4DDrTgFJJ2rDeB1LFnRqZ52wKijs7n4oXbIh82sg","views":521,"tags":"database,design,sql","description":"Learn
database design principles, normalization, and query
optimization."},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
StringValue(value=Xgg4bI7IOQwQKyK1Ekg5pNwjnaSQq3HUfVY0Gaw1LkgZcOP60JP78nu5enJXJ1tUM7LoHJGs_lcMek3H9RRs2A)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1766219949292},"id":"Xgg4bI7IOQwQKyK1Ekg5pNwjnaSQq3HUfVY0Gaw1LkgZcOP60JP78nu5enJXJ1tUM7LoHJGs_lcMek3H9RRs2A","name":{"type":"String","value":"WebSocket
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
BlhhwhZ4OjJxdcasv7_QNbUMnlGMud-QHJyTv-PoQkqu1I-q3IpQ6AjJwOuDcDb83zWWZJ_jKHE2-lYswiAW5w

=== Sending Chat Message === ✓ Chat response: Message ID:
"ZhpUUsd6-7TsfFwuFsMMmV-S5Iwxpa_Jg1ZXqCSAMWs-7KcuIYjEDtawLmlNHfRhE0H14p2PTDzPXgHv-kU4jw"
Responses: ["I'm sorry, but the provided context does not contain any
information about ekoDB and its features."]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
> Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
uD53qN9L6PgL0rmu5vhxRF5CZeR5PbsuDZGDFkRoyg8OxjnZAGcjfd2lafWCcrcl7dNd4l6DgpRkj8SkKbJmdA

=== Sending Messages === ✓ Message 1 sent Responses: ["I'm sorry, but the
context provided does not contain specific information about the available
products. Could you please provide more details?"]

✓ Message 2 sent Responses: ["Apologies, but there's no information given about
a specific product's price in the provided context. Could you please specify the
product you're inquiring about?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"uD53qN9L6PgL0rmu5vhxRF5CZeR5PbsuDZGDFkRoyg8OxjnZAGcjfd2lafWCcrcl7dNd4l6DgpRkj8SkKbJmdA"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"xzt28UHDHES0WBTORhAHRT5ozSxXtSasV4bjjFw2ZCCy9Qq7EIMUXkLlVVHvwutQPe0AGVq9pLi_IEG2BPPKsQ","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-20T08:39:24.705502Z"},"id":"EHBqu3da_kYid-ulRXz7gvr8V9nhx2927v36D4lpCsWDXHBFBWitK0O3gw8pW6ilVHp8S1uDxSXQZDMC_TufqQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":25,"prompt_tokens":156,"total_tokens":181}},"updated_at":{"type":"DateTime","value":"2025-12-20T08:39:24.705502Z"}},{"chat_id":{"type":"String","value":"uD53qN9L6PgL0rmu5vhxRF5CZeR5PbsuDZGDFkRoyg8OxjnZAGcjfd2lafWCcrcl7dNd4l6DgpRkj8SkKbJmdA"},"content":{"type":"String","value":"I'm
sorry, but the context provided does not contain specific information about the
available products. Could you please provide more
details?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"xzt28UHDHES0WBTORhAHRT5ozSxXtSasV4bjjFw2ZCCy9Qq7EIMUXkLlVVHvwutQPe0AGVq9pLi_IEG2BPPKsQ","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-20T08:39:24.708081Z"},"id":"cUMcLVqrYK1pgyhRMnjzcld2r40RZoOhWWJNWLi4ritU6pMripJ6gxjlPWMFWJfoCyw3LSsRru66zskthT_Cpg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":25,"prompt_tokens":156,"total_tokens":181}},"updated_at":{"type":"DateTime","value":"2025-12-20T08:39:24.708081Z"}},{"chat_id":{"type":"String","value":"uD53qN9L6PgL0rmu5vhxRF5CZeR5PbsuDZGDFkRoyg8OxjnZAGcjfd2lafWCcrcl7dNd4l6DgpRkj8SkKbJmdA"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-12-20T08:39:26.240909Z"},"id":"75aoc0ecNczFTwmPtOq0B5UcVK5YTjaRQw3lZ-gZ706XipktieA3jobHepdMhdAu2BTarhHoa2j70AHky9YrSQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":32,"prompt_tokens":70,"total_tokens":102}},"updated_at":{"type":"DateTime","value":"2025-12-20T08:39:26.240909Z"}},{"chat_id":{"type":"String","value":"uD53qN9L6PgL0rmu5vhxRF5CZeR5PbsuDZGDFkRoyg8OxjnZAGcjfd2lafWCcrcl7dNd4l6DgpRkj8SkKbJmdA"},"content":{"type":"String","value":"Apologies,
but there's no information given about a specific product's price in the
provided context. Could you please specify the product you're inquiring
about?"},"created_at":{"type":"DateTime","value":"2025-12-20T08:39:26.243977Z"},"id":"SC0SOJZW_IZ-rtDC7HGVPJvPoqAO2MjekRaYfRwYmK51ax8-0Ijv3BJWHJ9-RPDQzQ6CORJrrtZ0uuniK8DIxw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":32,"prompt_tokens":70,"total_tokens":102}},"updated_at":{"type":"DateTime","value":"2025-12-20T08:39:26.243977Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
bj6dwb_0xMfDKbdcoxsFDdjYmAQTrbRtJZcef6K0F_8u1RKvXM6-dDSvg4xEaMajoqX6-2a0q7t1h1EK5cK9hA

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
qHAiwp1Trmj4VtgP7LN9RMTaeG32qgthzPFbDg0eVbgLPw1Vs4apZeNhJ5-J4Gt3cKZ2vlMYeODrQUPcuIHYfQ

=== Sending Initial Message === ✓ Message sent Responses: ["I'm sorry, but the
context provided does not contain information about any specific products. Could
you please provide more details?"]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses: ["I'm
sorry, but the context provided doesn't give information about any specific
products. Could you please provide more details or specify the type of products
you're interested in?"]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
Lx2iJs96Ye7M13exjGZ7GQ\_\_hMFvNiJUKXfVjIv043I94AX6kRH0f2iCq3btv0pJ6MzmgWgiJCWsQre1dJAOYw

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 6s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
ahUrOx8-9GcQ8RF9Xt67MWQswstC5M3VCywD5bCrmhljwS1DpgYdhYJshZqdr8I1Pozu84fvZXtDzpAROHaSHQ
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"count":5,"status":"active","avg_score":60.0}
{"count":5,"status":"inactive","avg_score":50.0} ⏱️ Execution time: 0ms

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
