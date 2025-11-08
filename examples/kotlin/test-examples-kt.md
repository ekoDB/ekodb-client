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
Record(fields={id=StringValue(value=HjnhWEtt_Ul5aLcxoT9KPgfuxfqswoHcBjYKem2TAbP0uGlVylwlsIX1gj_1nQZxhOdYFFMnEsJOYBGzCU4N2g)})
User ID:
HjnhWEtt_Ul5aLcxoT9KPgfuxfqswoHcBjYKem2TAbP0uGlVylwlsIX1gj_1nQZxhOdYFFMnEsJOYBGzCU4N2g

=== Read === ✓ Found user by ID:
Record(fields={email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)}),
id=StringValue(value=HjnhWEtt_Ul5aLcxoT9KPgfuxfqswoHcBjYKem2TAbP0uGlVylwlsIX1gj_1nQZxhOdYFFMnEsJOYBGzCU4N2g),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)})})

=== Update === ✓ Updated user:
Record(fields={active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
city=ObjectValue(value={value=StringValue(value=San Francisco),
type=StringValue(value=String)}),
email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=29)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
id=StringValue(value=HjnhWEtt_Ul5aLcxoT9KPgfuxfqswoHcBjYKem2TAbP0uGlVylwlsIX1gj_1nQZxhOdYFFMnEsJOYBGzCU4N2g)})

=== Query === ✓ Found 1 users matching query

- Record(fields={active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  city=ObjectValue(value={value=StringValue(value=San Francisco),
  type=StringValue(value=String)}),
  email=ObjectValue(value={value=StringValue(value=alice@example.com),
  type=StringValue(value=String)}),
  age=ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=29)}),
  id=StringValue(value=HjnhWEtt_Ul5aLcxoT9KPgfuxfqswoHcBjYKem2TAbP0uGlVylwlsIX1gj_1nQZxhOdYFFMnEsJOYBGzCU4N2g),
  name=ObjectValue(value={value=StringValue(value=Alice Johnson),
  type=StringValue(value=String)})})

=== Delete === ✓ Deleted user with ID:
HjnhWEtt_Ul5aLcxoT9KPgfuxfqswoHcBjYKem2TAbP0uGlVylwlsIX1gj_1nQZxhOdYFFMnEsJOYBGzCU4N2g

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
hjxLreb8sWuqo-0WLB9jnn6a4c1m5mby5yEEXc0ea7y24rptt6omwvyDjptCzp5pJISt8PSV4ByHweR7ETSUqg,
bIC5LwGHqcfe5d8WqP0n0m3Qy0AsrDpPEQmxkc1gyZVb94KJ8dhwWR7Ojh4ORhDoybOevYkGCbfx1UZ-lDVbWg,
QBmeeR1MwrFzGLf8GVqFr_yclSBQ8Bq6WDvkCl7EhJfvR0HcKNq3v3izKTWz5K8oU15dP_8uk99Lm6Yg1ANeTQ...

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
{"user_id":"123","created_at":1762583077792} (Will expire in 10 seconds)

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

- functions\_\_ek0_bench
- schema_products_client_ts
- websocket_test
- schema_documents_client_go
- chat_configurations\_\_ek0_bench

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-11-08T06:24:42.114137Z","last_modified":"2025-11-08T06:24:42.114137Z","bypass_ripple":false,"primary_key_alias":"id"}

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
7-ZYqvm75nvtDfyjjP-QyKzz1xs7yPfy-3mhCcGvbBTNBpmtHwV-K1YnkCHWNRNvcmndL1xmNK5kJgzFmq4bAA

=== Verify Document Exists === ✓ Document found: [user_id, id, created_at,
session_id, ttl_duration]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
pVzy43Z9UHo5P5sdcBgP2Zuhe7pJX0ZoGcVxwypiRczwMkVads17MrrOzQcYXnsjjxysd9IPxMx3FNnm9BYETw

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [value,
ttl_duration, cache_key, id]

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
{"payload":{"data":[{"id":"vBzvBWD1CdVzVKIu6CFj5mIO_tLd0J53f5m5iv16btAEfFGLDWlwB3DRLTQBj56W4yAkXxTCpituk6qS9_vgXA","name":{"type":"String","value":"Test
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
StringValue(value=dBeaTThzvsT9EtqbfjEvAhVk0yNaXcA61aa4np2ZRynzd1mV9S9vmNGX-t4u6NkQH_wypsEVHoZmlVIAxShSRA)
✓ Inserted user 2:
StringValue(value=2YDeUTf4fiTa4-yKt5xrVP4NMMyifIhQY_JG9tlrK28_wHhUWeKZLQXghJy4IYxiPbHNSTlJm7kDdOtCLJBgXA)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 22 Sample:
[functions__ek0_bench, schema_products_client_ts, websocket_test,
schema_documents_client_go, chat_configurations__ek0_bench]

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
{"results":[{"record":{"tags":"programming,rust,tutorial","views":68,"title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices.","id":"MAUnVuRSmpb-iQUXt5AoUxuyIotB12UeIYbIczS7uLKBgyBaXtSneNgAGtqwKLPsjcw1g0MqiI*dgRQRSI9yEQ"},"score":19.8,"matched_fields":["title","tags","description"]},{"record":{"id":"KGSA_WQuWmpt11iEPo2cAUS54oYmPtITFcYNCJlsIDIM7IlacYGlfSQ6pdvys5aBSKosvt3DDJ1wwP*-znFSyA","description":"Master
Python for data analysis, machine learning, and
visualization.","views":62,"tags":"programming,python,data-science","title":"Python
for Data
Science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"views":803,"tags":"programming,javascript,web","title":"JavaScript
Web Development","description":"Build modern web applications using JavaScript,
React, and
Node.js.","id":"hRCPQRGqcyFIOHJEh8Pb0iczW2DPvqNIXIYe8Ruw_1vVYkvOIl-oZDlGxjwpebgLuAM-fffy-GfwGtlYe46dlA"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"views":603,"id":"GBf-mbrN24H9ofpT4fyD7Maq1juHNfE4U5IVjtcybdFtNZF0M2v0hGaAJx*wAYDOcue2vAkQK0L2WHyVY2W63Q","title":"Machine
Learning Basics","tags":"ai,machine-learning,python","description":"Introduction
to machine learning algorithms and neural
networks."},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"id":"KGSA_WQuWmpt11iEPo2cAUS54oYmPtITFcYNCJlsIDIM7IlacYGlfSQ6pdvys5aBSKosvt3DDJ1wwP*-znFSyA","title":"Python
for Data Science","description":"Master Python for data analysis, machine
learning, and
visualization.","tags":"programming,python,data-science","views":62},"score":1.0,"matched_fields":["description"]},{"record":{"tags":"database,design,sql","description":"Learn
database design principles, normalization, and query
optimization.","id":"fD71w4uc-YSmvExbfLD5k4GPWwVsHMiOVgO_ubV9Lpss2D1blkzWw8uaAY60T97pvwNg-Qh0c8zgA3v_7g-mew","views":616,"title":"Database
Design"},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn
Rust programming language with hands-on examples and best
practices.","id":"MAUnVuRSmpb-iQUXt5AoUxuyIotB12UeIYbIczS7uLKBgyBaXtSneNgAGtqwKLPsjcw1g0MqiI_dgRQRSI9yEQ","title":"Rust
Programming","tags":"programming,rust,tutorial","views":68},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
StringValue(value=lOQ7-FBhkD5sAWMigDNYbQKgV6PtjUVpeiMa9npCBNPcrk8E1xyn7I3dmJK4scBwwaGGZY8Pv-6PSLEHYZUW_g)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1762583103436},"id":"lOQ7-FBhkD5sAWMigDNYbQKgV6PtjUVpeiMa9npCBNPcrk8E1xyn7I3dmJK4scBwwaGGZY8Pv-6PSLEHYZUW_g","name":{"type":"String","value":"WebSocket
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
vs2rNwL4LwmWADfQU_ZS3SMn1JMIOxdhUysNVLum1KXaspW3Pilkdr8bYFBT23IrRzJibRjUK359vr5RI3QqVQ

=== Sending Chat Message === ✓ Chat response: Message ID:
"T0BUqUDd2CfeBmsHzgdviClWd37UOB-EWFagShv2PQ9ioKU1f2_MNCIF_yYu3oErInZTftyCWJ0PVS4EI6UlFw"
Responses: ["ekoDB is a high-performance database that offers intelligent
caching, real-time capabilities, and AI integration. Some of its key features
include a chat feature that allows you to query your database using natural
language and receive AI-powered responses with relevant context. Additionally,
ekoDB supports various search features such as full-text search, vector search,
and hybrid search with automatic context retrieval."]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
> Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
A4IucbrO-WA6AQYcUmUS4fLK0mE0INknVS4gVg-FRIDS5zHUmUO9rj0XVEZN9kC4PWl9aTpXjolu6TcqyO1ZEA

=== Sending Messages === ✓ Message 1 sent Responses: ["Based on the context
provided, the product available is ekoDB. It is a high-performance database
product with AI capabilities. The price for this product is $99."]

✓ Message 2 sent Responses: ["Apologies, but there's no context provided about a
specific product. Could you please specify the product you're interested in?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"A4IucbrO-WA6AQYcUmUS4fLK0mE0INknVS4gVg-FRIDS5zHUmUO9rj0XVEZN9kC4PWl9aTpXjolu6TcqyO1ZEA"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":{"type":"String","value":"dr6b8FKvpePvfdYgLRChVH0R4d8S5WyG-oFs1nRml5CWeQs_nPcMb0ceDw7Nqdq7m5ofVZcp_BGbUR4zbZCZJw"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-08T06:25:20.812233Z"},"id":"pJAFAyX-EvKdnpYZ8bEjIq6YNO0qPsKEBMaXkhkBd4IR4Nuy6Bj-fmLB-75DJRYbkfIvyGV4AcZIc-KovZ8_qg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":34},"prompt_tokens":{"type":"Integer","value":169},"total_tokens":{"type":"Integer","value":203}}},"updated_at":{"type":"DateTime","value":"2025-11-08T06:25:20.812233Z"}},{"chat_id":{"type":"String","value":"A4IucbrO-WA6AQYcUmUS4fLK0mE0INknVS4gVg-FRIDS5zHUmUO9rj0XVEZN9kC4PWl9aTpXjolu6TcqyO1ZEA"},"content":{"type":"String","value":"Based
on the context provided, the product available is ekoDB. It is a
high-performance database product with AI capabilities. The price for this
product is
$99."},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":{"type":"String","value":"dr6b8FKvpePvfdYgLRChVH0R4d8S5WyG-oFs1nRml5CWeQs_nPcMb0ceDw7Nqdq7m5ofVZcp_BGbUR4zbZCZJw"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-08T06:25:20.813658Z"},"id":"p0OE1qcP6fu7S4kM-cvd3rmjtmAQgl71A2WfNAQDgrSXna8nijq5W2qji1ZQY4c_LpGjMtHl8f-l2o5wGTMd-A","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":34},"prompt_tokens":{"type":"Integer","value":169},"total_tokens":{"type":"Integer","value":203}}},"updated_at":{"type":"DateTime","value":"2025-11-08T06:25:20.813658Z"}},{"chat_id":{"type":"String","value":"A4IucbrO-WA6AQYcUmUS4fLK0mE0INknVS4gVg-FRIDS5zHUmUO9rj0XVEZN9kC4PWl9aTpXjolu6TcqyO1ZEA"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-11-08T06:25:21.938387Z"},"id":"9L-xFa84WvyNEWeAuoGXDLUjsGHust1Xg6pe5665AlydlrxeYWU52E5zb2GZnEddS0JqutqrKmSpDNZ6TJf08A","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":25},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":88}}},"updated_at":{"type":"DateTime","value":"2025-11-08T06:25:21.938387Z"}},{"chat_id":{"type":"String","value":"A4IucbrO-WA6AQYcUmUS4fLK0mE0INknVS4gVg-FRIDS5zHUmUO9rj0XVEZN9kC4PWl9aTpXjolu6TcqyO1ZEA"},"content":{"type":"String","value":"Apologies,
but there's no context provided about a specific product. Could you please
specify the product you're interested
in?"},"created_at":{"type":"DateTime","value":"2025-11-08T06:25:21.941836Z"},"id":"IV-rP-jxfU7py0VMGHhxdL3Is6-LLbG1CW5n3C5hDsxUZeFYPwQ9iEaDQv2hU7uH8_4cfQjPHsQoJxa-2y6raQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":25},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":88}}},"updated_at":{"type":"DateTime","value":"2025-11-08T06:25:21.941836Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
neII3GnWU0P87cMIrMFNDZ4iDEyDR7Pv5mpPW5ns3rs_jl22KPLDC25eppS_RCZ7XPD8ZeC2gVpI3bv6bbhvdQ

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
wWgO7DZ_pO-7NgSDEssXgCU_xJ9OxVITzyzbJOJH7_29_R-yWAxkNmUMr7JtlzXXHU2QHOoFPe2ywU2zhu3stw

=== Sending Initial Message === ✓ Message sent Responses: ["Based on the
information provided, the product available is ekoDB. It is a high-performance
database product priced at $99."]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses:
["Based on the context provided, the available product is ekoDB, which is a
high-performance database product. It is priced at $99."]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
9V7AhAzoUS1qC9TcOJxTpQAJ-7qyb8PZF93Tw7K9elMtp8RCZwaVnxbQJpxq5acgzoPlqZExDPoRhfqKVzFOOg

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

> Task :run 🚀 ekoDB Saved Functions Example (Kotlin)

SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to no-operation
(NOP) logger implementation SLF4J(W): See
https://www.slf4j.org/codes.html#noProviders for further details. 📋 Setting up
test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved:
-BrAGpfdVJpLIBJuKyXrcXwyHQ3E4jvsSgoDDZn9T0XYbT9rGTDYNApRi4YfW5R7Juv9UHvvFaSo5VwjYtzDqw
📊 Found 60 active users

📝 Example 2: Parameterized Function

✅ Function saved:
N83_YnfI91uephgKCt_oyoVOkZpsLDFvGL346hEA_PJCT9JusbCyYYbZgBJIyZ94A5QXA0UC1cLfTg3Lx5oS7A
📊 Found 60 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved:
0aenFYkYf-J8jXkmyNO8RAdRNxf0wI3pWBBFEip5d0cvmLELFTJPy5M1BzC8mx_Mdt0fMvXgUxRdtqO3Cxtx7w
📊 Statistics: 2 groups {"status":"active","count":30,"avg_score":60.0}
{"avg_score":50.0,"count":30,"status":"inactive"}

📝 Example 4: Function Management

📋 Total functions: 13 🔍 Retrieved function: Get Active Users ✏️ Function
updated 🗑️ Function deleted

✅ All examples completed!

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date ✅
[32mKotlin client examples complete![0m
