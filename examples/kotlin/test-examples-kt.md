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

BUILD SUCCESSFUL in 5s 2 actionable tasks: 2 up-to-date ✅ [32mKotlin client
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
Record(fields={id=StringValue(value=ycwDOGDs4yvyHRmRmcuQpoopppbz70oUWw-JVQqAj4nTvRkGZRn0c5lC4heGlQpc24xws6tkIYgEz54WieR0Jg)})
User ID:
ycwDOGDs4yvyHRmRmcuQpoopppbz70oUWw-JVQqAj4nTvRkGZRn0c5lC4heGlQpc24xws6tkIYgEz54WieR0Jg

=== Read === ✓ Found user by ID:
Record(fields={id=StringValue(value=ycwDOGDs4yvyHRmRmcuQpoopppbz70oUWw-JVQqAj4nTvRkGZRn0c5lC4heGlQpc24xws6tkIYgEz54WieR0Jg),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
active=ObjectValue(value={value=BooleanValue(value=true),
type=StringValue(value=Boolean)}),
name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)})})

=== Update === ✓ Updated user:
Record(fields={age=ObjectValue(value={value=IntegerValue(value=29),
type=StringValue(value=Integer)}),
city=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=San Francisco)}),
id=StringValue(value=ycwDOGDs4yvyHRmRmcuQpoopppbz70oUWw-JVQqAj4nTvRkGZRn0c5lC4heGlQpc24xws6tkIYgEz54WieR0Jg),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
active=ObjectValue(value={value=BooleanValue(value=true),
type=StringValue(value=Boolean)}),
name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)})})

=== Query === ✓ Found 1 users matching query

- Record(fields={name=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Alice Johnson)}),
  id=StringValue(value=ycwDOGDs4yvyHRmRmcuQpoopppbz70oUWw-JVQqAj4nTvRkGZRn0c5lC4heGlQpc24xws6tkIYgEz54WieR0Jg),
  active=ObjectValue(value={value=BooleanValue(value=true),
  type=StringValue(value=Boolean)}),
  age=ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=29)}),
  city=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=San Francisco)}),
  email=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=alice@example.com)})})

=== Delete === ✓ Deleted user with ID:
ycwDOGDs4yvyHRmRmcuQpoopppbz70oUWw-JVQqAj4nTvRkGZRn0c5lC4heGlQpc24xws6tkIYgEz54WieR0Jg

✓ Confirmed user was deleted

=== Cleanup === ✓ Deleted collection: kotlin_users_example

=== Example Complete ===

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
> Kotlin Client - Batch Operations Example ===

=== Batch Insert === ✓ Inserted 5 records IDs:
TPsLMr3ZYbv0K3WviH0anRieCUAULPpsat9sFnh9RUX6qJoi_JeNgnr20aZOXVaSlRn6G8VZXMZL7tqvRMTrfQ,
h_y5bZy61YiEUQjcw8Jy3Whgv_7sC7n_42fbsR2shnYQVVWJzkvE4lnufGjty-rZFDtWaBRCyKHLtB_9opFHdw,
3C_SwnqmYeFCv_hazL4NlG1dRtWNaIbYIuaozXXDPxKBC3IQU03nrPjpzovsbL0zLQ26x7-egA7VyeNbmTOCqw...

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
{"created_at":1765205151442,"user_id":"123"} (Will expire in 10 seconds)

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

=== List Collections === ✓ Found 20 collections

- schema_employees_client_go
- schema_products_client_go
- websocket_test
- schema_documents_client_ts
- schema_employees_client_ts

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-08T14:45:55.685161Z","last_modified":"2025-12-08T14:45:55.685161Z","bypass_ripple":false,"primary_key_alias":"id"}

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
t0F0AwqIxASJJSvasumhM3c2R1gXrbd58h-ZBhW_ALylwH61lsj55J9Y5E1MWGn_peK1sG3NhK6QfdmN3zjxUA

=== Verify Document Exists === ✓ Document found: [session_id, created_at,
ttl_duration, user_id, id]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
zIoC5SVPkbGdGqSjRoMK29fhLEQGo6w05C3b4IwTLROU8yiZL08q_tHCr6RMWGtpyQ3s6fUFRuh-DACQPzQU5g

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [id,
ttl_duration, value, cache_key]

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
{"payload":{"data":[{"id":"a5nrYJVRJe4tXqN63nBOl97Q9WI8IiB-V-Sa6hBxycETXh5EtnibLlIFUt09BPPm5ZKrBdWZTTItzKPDxbOFlg","name":{"type":"String","value":"Test
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
StringValue(value=4kpGYHC0QCu0iOi0Pc3ouy8Q0LqdeZi3MaVieFV99G_yn\_\_ppcuNRwlmiEadg6EgwGtLTBCFBS5N9f1KZzjwhg)
✓ Inserted user 2:
StringValue(value=8DhQ6gvhKXs-jEhM29W-Wagwghv0IAmzxeFsEJM9wYXTDuvvSI14x4NqPVWjrAOeN9zFooSFBmeOVQEo9Jsxpg)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 21 Sample:
[schema_employees_client_go, schema_products_client_go, websocket_test,
schema_documents_client_ts, schema_employees_client_ts]

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
{"results":[{"record":{"title":"Rust
Programming","tags":"programming,rust,tutorial","description":"Learn Rust
programming language with hands-on examples and best
practices.","id":"DuATXOIyK0oxoyj5cKRXfbnNIj05cNmATRWpggSVYqY1I6zrY1uqXyT5CmBAFxxdrqiFH3FkUv7_LCfZemmJSA","views":834},"score":19.8,"matched_fields":["title","tags","description"]},{"record":{"description":"Build
modern web applications using JavaScript, React, and
Node.js.","tags":"programming,javascript,web","title":"JavaScript Web
Development","id":"AXfkiYY6By2xFWVT479vT93hff8GmceUMdJ9Eueodv2qOSkAtFYOukD4cVQfMeSn9Q8NClFK5UR81pfQ6e0eIQ","views":611},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,python,data-science","description":"Master
Python for data analysis, machine learning, and visualization.","title":"Python
for Data
Science","views":220,"id":"TvoA-\_uMOPMFI4cJKlt7pt_VfRSd3QHBJ-XHFrO1tGjup0J_uW6Qu5jGFHKba9VnANpjZA1yNEzP-sps8eswNg"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"description":"Introduction to machine learning
algorithms and neural
networks.","id":"ohUO20FnYKr6_MDDdGWB9NkUSZJYVzOwISzu05-TEfHlRcwG359q67he9RpiRNeqcXvTnT6BkQrOkHsMEb8JBg","tags":"ai,machine-learning,python","title":"Machine
Learning
Basics","views":903},"score":2.7,"matched_fields":["title","description","tags"]},{"record":{"id":"TvoA-\_uMOPMFI4cJKlt7pt_VfRSd3QHBJ-XHFrO1tGjup0J_uW6Qu5jGFHKba9VnANpjZA1yNEzP-sps8eswNg","tags":"programming,python,data-science","title":"Python
for Data Science","views":220,"description":"Master Python for data analysis,
machine learning, and
visualization."},"score":1.0,"matched_fields":["description"]},{"record":{"views":535,"description":"Learn
database design principles, normalization, and query
optimization.","title":"Database
Design","tags":"database,design,sql","id":"JMxTXwW-adM3O1n-z62bnmMvo6l0MQWpSDL0Eg1XRTuKVDeHaOWAPCDAMv7xxlNkckxrupy7_cWH5j-XQr731A"},"score":0.5,"matched_fields":["description"]},{"record":{"id":"DuATXOIyK0oxoyj5cKRXfbnNIj05cNmATRWpggSVYqY1I6zrY1uqXyT5CmBAFxxdrqiFH3FkUv7_LCfZemmJSA","tags":"programming,rust,tutorial","title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices.","views":834},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
StringValue(value=ckUiQ5UlYoyYepl9B1rZJgQNFb0Y1UBYWDmyf4o2tSiMbGH7f8ZsrtO1DBhid-KYoqypjUguYriIlD1qqR-rYQ)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1765205177035},"id":"ckUiQ5UlYoyYepl9B1rZJgQNFb0Y1UBYWDmyf4o2tSiMbGH7f8ZsrtO1DBhid-KYoqypjUguYriIlD1qqR-rYQ","name":{"type":"String","value":"WebSocket
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
cr3r6_e0Eq2S0dEYv9SZ2LmOmF9lOHFaE65z8s41mW8nhpWhV73Y91tqlG1PK-wI3AiCJ2Np5AGKnTykzlkJUg

=== Sending Chat Message === ✓ Chat response: Message ID:
"w7LC-s6mANDQ0JcMqUpGfiYcdtpNNMCMU7mvcjEpBL-02biIONKd2KLCRsYYQzD2P1p4Xoe9nqXRUAXR9UTXyQ"
Responses: ["ekoDB is a high-performance database with intelligent caching,
real-time capabilities, and AI integration. It offers several features including
full-text search, vector search, and hybrid search with automatic context
retrieval. Additionally, ekoDB integrates an AI Chat feature that allows you to
query your database using natural language and receive AI-powered responses with
relevant context."]

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
aBYuR8g3V629eCA9eBrFInhB03UN8AUFlajLZh_1Dw0aFfU_cL5qvPlHUR7mQOnWEqV-9YCuuCOIK9a_S9KUYA

=== Sending Messages === ✓ Message 1 sent Responses: ["Answer: The available
product is a high-performance database product named \"ekoDB\". It has AI
capabilities and is priced at $99."]

✓ Message 2 sent Responses: ["Apologies, but there seems to be no context
provided regarding any product to provide its price. Could you please specify
the product you're asking about?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"aBYuR8g3V629eCA9eBrFInhB03UN8AUFlajLZh_1Dw0aFfU_cL5qvPlHUR7mQOnWEqV-9YCuuCOIK9a_S9KUYA"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"x7qLfE_RozXT5AUCzv8GKC_cMhOgiEGviLLAP0Lix78ZELl0eRkIgtyTFDLBy6Q_88yBGatmT8XuxMEIIm8hlA","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-08T14:46:35.850886Z"},"id":"pp4djELwPAkNTIx-n7kdxhTpIrJRe2k1eYfMBSsmpsIpw6Vy57LYWspyCtniv3Vojt_8UOZ0UqSJLTBXXzm-Dg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":27,"prompt_tokens":192,"total_tokens":219}},"updated_at":{"type":"DateTime","value":"2025-12-08T14:46:35.850886Z"}},{"chat_id":{"type":"String","value":"aBYuR8g3V629eCA9eBrFInhB03UN8AUFlajLZh_1Dw0aFfU_cL5qvPlHUR7mQOnWEqV-9YCuuCOIK9a_S9KUYA"},"content":{"type":"String","value":"Answer:
The available product is a high-performance database product named \"ekoDB\". It
has AI capabilities and is priced at
$99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"x7qLfE_RozXT5AUCzv8GKC_cMhOgiEGviLLAP0Lix78ZELl0eRkIgtyTFDLBy6Q_88yBGatmT8XuxMEIIm8hlA","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-08T14:46:35.856596Z"},"id":"d_P2sNBrnppYfyk97aBI1SXj4ElJ8bmaFVihnRhJ5yI7rgBUmzJeOZzHZo3wd9JsnwZwa4VvlflJMlRII9Eosw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":27,"prompt_tokens":192,"total_tokens":219}},"updated_at":{"type":"DateTime","value":"2025-12-08T14:46:35.856596Z"}},{"chat_id":{"type":"String","value":"aBYuR8g3V629eCA9eBrFInhB03UN8AUFlajLZh_1Dw0aFfU_cL5qvPlHUR7mQOnWEqV-9YCuuCOIK9a_S9KUYA"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-12-08T14:46:37.567655Z"},"id":"6uMsqaw0mkeC-QTJCUNCNLl5AuotlPd6FfeZOQErk_dc244M3JWpemf7-E1w-YaYt_fgsVIMrVcUWBEtAPjOnQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":30,"prompt_tokens":70,"total_tokens":100}},"updated_at":{"type":"DateTime","value":"2025-12-08T14:46:37.567655Z"}},{"chat_id":{"type":"String","value":"aBYuR8g3V629eCA9eBrFInhB03UN8AUFlajLZh_1Dw0aFfU_cL5qvPlHUR7mQOnWEqV-9YCuuCOIK9a_S9KUYA"},"content":{"type":"String","value":"Apologies,
but there seems to be no context provided regarding any product to provide its
price. Could you please specify the product you're asking
about?"},"created_at":{"type":"DateTime","value":"2025-12-08T14:46:37.572281Z"},"id":"ZnRXE82jXMrt6cQzafoiZN9RhaIp0snRInEvAXI0X0TBW7SV3EwGJi85dxSPOEhddbi0-qYrMz1uKSoxBQRNlw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":30,"prompt_tokens":70,"total_tokens":100}},"updated_at":{"type":"DateTime","value":"2025-12-08T14:46:37.572281Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
aRa0cP1Q4h7Hvs8w65Q37E0gq5_I0vN-41p971hK6o5NqNkZFlyZ0Wj0DTwYsDgInfeK-QqnMrlDETft0oKOUg

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
GKGpGxOHujGesIm35zDMrJCVkPUaoGXT2H046T\_-YTcsqsUk854viG7T3fNCKwzf2_QeJOZcuWkoVa4UkLBlZQ

=== Sending Initial Message === ✓ Message sent Responses: ["Answer: The
available product is a high-performance database product named ekoDB. It is
priced at $99."]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses:
["Answer: The available product is a high-performance database product named
ekoDB. It is priced at $99."]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
v691\_\_hnyk1qa5Bk1D2NcgM5Bt7RHfn9o4cOay_epjmGpdSIgNefrxDH8SHq07I4xRh29jWqcEvXQgW5vK_rwA

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 9s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
kECVBlWXLjNoQS9DK5REJYmjiIaNBk91dE5YLUBmRWJdGV5kxTc4Gs8rk1IV0kyOhLJHm8noGSQv0rD9WMMKKQ
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"status":"inactive","avg_score":50.0,"count":5}
{"avg_score":60.0,"status":"active","count":5} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 19 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
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
[32mKotlin client examples complete![0m
