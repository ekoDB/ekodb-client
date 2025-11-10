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
Record(fields={id=StringValue(value=\_tEBrM_1i_6sscs3DmX1zftvuiGQLkRn7ACiU1R-e-\_aLoNA_Tr_xEHT8HNFPFHtMki7voUz-SZHG5dixvdFaA)})
User ID:
\_tEBrM_1i_6sscs3DmX1zftvuiGQLkRn7ACiU1R-e-\_aLoNA_Tr_xEHT8HNFPFHtMki7voUz-SZHG5dixvdFaA

=== Read === ✓ Found user by ID:
Record(fields={name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
id=StringValue(value=\_tEBrM_1i_6sscs3DmX1zftvuiGQLkRn7ACiU1R-e-\_aLoNA_Tr_xEHT8HNFPFHtMki7voUz-SZHG5dixvdFaA),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)})})

=== Update === ✓ Updated user:
Record(fields={city=ObjectValue(value={value=StringValue(value=San Francisco),
type=StringValue(value=String)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
id=StringValue(value=\_tEBrM_1i_6sscs3DmX1zftvuiGQLkRn7ACiU1R-e-\_aLoNA_Tr_xEHT8HNFPFHtMki7voUz-SZHG5dixvdFaA),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=29)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)})})

=== Query === ✓ Found 1 users matching query

- Record(fields={name=ObjectValue(value={value=StringValue(value=Alice Johnson),
  type=StringValue(value=String)}),
  active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  id=StringValue(value=\_tEBrM_1i_6sscs3DmX1zftvuiGQLkRn7ACiU1R-e-\_aLoNA_Tr_xEHT8HNFPFHtMki7voUz-SZHG5dixvdFaA),
  email=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=alice@example.com)}),
  city=ObjectValue(value={value=StringValue(value=San Francisco),
  type=StringValue(value=String)}),
  age=ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=29)})})

=== Delete === ✓ Deleted user with ID:
\_tEBrM_1i_6sscs3DmX1zftvuiGQLkRn7ACiU1R-e-\_aLoNA_Tr_xEHT8HNFPFHtMki7voUz-SZHG5dixvdFaA

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
HZhnXHhnxh_OAEia19ipf-fmQnxCG537U_s96EqHebf-2T6Qd7G1eo1J2N2wNUOuVVGN03VrLLZ9FYd1nxLt_w,
XSCAtJXyF5-cASbcKU1WgEoN5P76J9Ut5sX4UH3QTg6MqWVVrCNhfg4Cy5DkO3XRWrRXrShPzhCnRo5R7EG2nA,
WtNVmIrnRcpAJUM515JhB7IKb3SEWK4wkc24OVrHvEKVhuMvcORp6NOstSCixQhwi8JTbp4aZgeD6clRrci3gw...

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
{"name":"Alice","email":"alice@example.com","role":"admin"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"created_at":1762748756711,"user_id":"123"} (Will expire in 10 seconds)

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

- ws_ttl_test
- schema_documents_client_ts
- schema_products_client_ts
- schema_documents_client_py
- batch_users

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-11-10T04:26:01.051124Z","last_modified":"2025-11-10T04:26:01.051124Z","bypass_ripple":false,"primary_key_alias":"id"}

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
f6ch19BesiNFelUC7Ky_OKz2YzlK7atDp1LcdCnSCphIVWeI2GlKK2cV2WHkS5EmCjlr56t4pjf7xUMMxwR_Kw

=== Verify Document Exists === ✓ Document found: [session_id, user_id,
created_at, ttl_duration, id]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
C2E8E_tCmYbfxFpJMkndllemj8BYguDxSzjV4azYtg_rU1tVLqbMgy7Mb_VVCPUTMjtKh1JPunwxHhP9UptvZw

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [cache_key,
ttl_duration, id, value]

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
{"payload":{"data":[{"id":"kGfCF9pQ7ePUqUPkbKznH87-4H-QamAVXOqKVu4c8U548DEe0uh0cij6-HXze_XFE9jQPTTdwKzLFT4eQWjEcQ","name":{"type":"String","value":"Test
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
StringValue(value=jIvZ3Q6JRue2BWimxk2CRugNR406VeMU6i4hQPBY5c8ncWd8FiCTuumwKCigA6yltldiOpraEweqyl2lTj3yQg)
✓ Inserted user 2:
StringValue(value=D8FS8yH38jK1-kJRpiM0w4DgybsE8vzPllnGoZTLPNTrHVn0vwN8GoGXtp6iTnYCXPAX5pVoRtg8LbQ9-mE3Ww)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 21 Sample: [ws_ttl_test,
schema_documents_client_ts, schema_products_client_ts,
schema_documents_client_py, batch_users]

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
{"results":[{"record":{"id":"aFOkfAhhtlrZXm7WSuVIJj39lbFuFIk-ET2H1m0yxICNSUke4K6StkT1ra1m9cia-\_nDbMNwl_wUVf8m29YCbw","views":365,"tags":"programming,rust,tutorial","title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices."},"score":19.8,"matched_fields":["tags","description","title"]},{"record":{"id":"3wxQ7W9RmrIoNLN_EyVcTMFZhrL6XpAsilfDhFjrZNOKKihgfmFSCxNFWb45F6wP8BI5KBCjOLRcqjITuGC8dQ","tags":"programming,javascript,web","title":"JavaScript
Web Development","description":"Build modern web applications using JavaScript,
React, and
Node.js.","views":185},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,python,data-science","id":"ju7d0tPIB1H9bgE1JvWlV6Gl1QKxUHG0DEkX2WUO46B07iG_tbpwaZ6RND-CUYn4opyhUDaHh2eIP_gQuOkTLA","views":717,"title":"Python
for Data Science","description":"Master Python for data analysis, machine
learning, and
visualization."},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":2}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"tags":"ai,machine-learning,python","id":"X1YkXcI8fk3mFfUkSYZh9ZDCmswFh0VhTzzmmCA3CFoLGaI-6urx8GWQLWQgUCsC4tEEbBjJbTn-d2I6IYbs2w","description":"Introduction
to machine learning algorithms and neural networks.","views":88,"title":"Machine
Learning
Basics"},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"description":"Master
Python for data analysis, machine learning, and
visualization.","id":"ju7d0tPIB1H9bgE1JvWlV6Gl1QKxUHG0DEkX2WUO46B07iG_tbpwaZ6RND-CUYn4opyhUDaHh2eIP_gQuOkTLA","title":"Python
for Data
Science","views":717,"tags":"programming,python,data-science"},"score":1.0,"matched_fields":["description"]},{"record":{"id":"aFOkfAhhtlrZXm7WSuVIJj39lbFuFIk-ET2H1m0yxICNSUke4K6StkT1ra1m9cia-\_nDbMNwl_wUVf8m29YCbw","views":365,"description":"Learn
Rust programming language with hands-on examples and best
practices.","tags":"programming,rust,tutorial","title":"Rust
Programming"},"score":0.5,"matched_fields":["description"]},{"record":{"id":"ax4rSpkbv99SH7pwmjsbyC4cg770jnUmG8OOujO88r78E4rR1l2vb_JJynT4Ift2V4MYG5V5o6PLjtvbiB5e7g","views":337,"description":"Learn
database design principles, normalization, and query
optimization.","tags":"database,design,sql","title":"Database
Design"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
StringValue(value=3Gbn1Evjg7_FSiVxo0_YLU9GxVy_GE2wBayWLO9jh3hbqu4qjMsF8k_vjtrqpyreBo6P4FU8iWlE9ocETh81Vw)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1762748782352},"id":"3Gbn1Evjg7_FSiVxo0_YLU9GxVy_GE2wBayWLO9jh3hbqu4qjMsF8k_vjtrqpyreBo6P4FU8iWlE9ocETh81Vw","name":{"type":"String","value":"WebSocket
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
aoqsNvhg3f7WEys2YitiiZh3ebBnczrfuMzHAIs79arXBZcprkdwrCs34DZlXx04MITED4pLjOzyj-aVDPZWKQ

=== Sending Chat Message === ✓ Chat response: Message ID:
"\_txrJR2ElUYqZPK8NBXHzSXwgbeWE6teB4euUGmYlQfaXFRjW0Jox8pywu42S805DC3oeVfvt3uHcMRdJ1AV1w"
Responses: ["ekoDB is a high-performance database that comes with intelligent
caching, real-time capabilities, and AI integration. It offers several key
features to enhance your data management experience. One of these features is an
AI Chat Integration, which allows you to query your database using natural
language and receive AI-powered responses with relevant context. Additionally,
ekoDB also supports different types of search features like full-text search,
vector search, and hybrid search with automatic context retrieval."]

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
0cLXBjBnOjbnQ5t2Vf2UYgQqvA0ieGKY6UfJju5hwURin-weSG0gUHgwyq2D1zb4Qfw1xaSW8dhEliFPwLj-Sg

=== Sending Messages === ✓ Message 1 sent Responses: ["Based on the information
provided, the available product is ekoDB, a high-performance database product
with AI capabilities. It is priced at $99."]

✓ Message 2 sent Responses: ["As an assistant, I'm sorry but I can't provide the
price information because there's no context or product specified in your
question. Could you please specify the product you're interested in?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"0cLXBjBnOjbnQ5t2Vf2UYgQqvA0ieGKY6UfJju5hwURin-weSG0gUHgwyq2D1zb4Qfw1xaSW8dhEliFPwLj-Sg"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":{"type":"String","value":"RQgknh2edyapWD8bXbAVMwwEZrcrvbDZBCEhljy23QSsCryCrIMR0dwaf3nayt87XbL45pIFqzSQi0PX8yjtRw"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-10T04:26:40.672365Z"},"id":"TzjOMsb_BUYsEYEAplSgl49z9QtgweK1MbWSqGdZVVHLW1yZHwCOfN_nqygM8uiV2HAy33UzNbjlMsLixXq41g","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":30},"prompt_tokens":{"type":"Integer","value":164},"total_tokens":{"type":"Integer","value":194}}},"updated_at":{"type":"DateTime","value":"2025-11-10T04:26:40.672365Z"}},{"chat_id":{"type":"String","value":"0cLXBjBnOjbnQ5t2Vf2UYgQqvA0ieGKY6UfJju5hwURin-weSG0gUHgwyq2D1zb4Qfw1xaSW8dhEliFPwLj-Sg"},"content":{"type":"String","value":"Based
on the information provided, the available product is ekoDB, a high-performance
database product with AI capabilities. It is priced at
$99."},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":{"type":"String","value":"RQgknh2edyapWD8bXbAVMwwEZrcrvbDZBCEhljy23QSsCryCrIMR0dwaf3nayt87XbL45pIFqzSQi0PX8yjtRw"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-11-10T04:26:40.674766Z"},"id":"5cWe_z4taVbebVWOsHR_OmwMPiqOxAgiUaiTuidtuJD5vqILArjG4g4fma6n5Lg2N61uzyDdYzpSaV38QlCVPg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":30},"prompt_tokens":{"type":"Integer","value":164},"total_tokens":{"type":"Integer","value":194}}},"updated_at":{"type":"DateTime","value":"2025-11-10T04:26:40.674766Z"}},{"chat_id":{"type":"String","value":"0cLXBjBnOjbnQ5t2Vf2UYgQqvA0ieGKY6UfJju5hwURin-weSG0gUHgwyq2D1zb4Qfw1xaSW8dhEliFPwLj-Sg"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-11-10T04:26:42.422992Z"},"id":"UVoFcsQwWEKUQ52WIl31fi1FoQ2xVb6H9Wz9f7uJ3Ah4Ino3rp8XYGhXqXPa-HlHPuTEtwsxJJntHx_UEj-pSA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":38},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":101}}},"updated_at":{"type":"DateTime","value":"2025-11-10T04:26:42.422992Z"}},{"chat_id":{"type":"String","value":"0cLXBjBnOjbnQ5t2Vf2UYgQqvA0ieGKY6UfJju5hwURin-weSG0gUHgwyq2D1zb4Qfw1xaSW8dhEliFPwLj-Sg"},"content":{"type":"String","value":"As
an assistant, I'm sorry but I can't provide the price information because
there's no context or product specified in your question. Could you please
specify the product you're interested
in?"},"created_at":{"type":"DateTime","value":"2025-11-10T04:26:42.427073Z"},"id":"8Kwve4Y21UY23tWRRWtZPz-0YdxEJKCPofXXYzXTpB8qrk9u-6y6ms96L-kw-sncTnW5h4ggdd2B2TQ8tf2ZFQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":38},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":101}}},"updated_at":{"type":"DateTime","value":"2025-11-10T04:26:42.427073Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
runvjCWvRnzRkT1_Dh7xfjBMATfqe_dMXkCPLdPKJU6EgstALh3gUe_kng9j5wGaN9H7a3xR5nhBIZGkfqTXDA

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
WmDjPlt7iquWWyh_K1H_nujT3jSsE6O9yq-wqyoo1B3pH3kVhHHn4qLURWB4_WhkTv3pTmA1HWl4JKbjEDnx1Q

=== Sending Initial Message === ✓ Message sent Responses: ["Based on the context
provided, the available product is ekoDB. It is a high-performance database
product priced at $99."]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses:
["Based on the provided context, there is one product available which is ekoDB.
This is a high-performance database product priced at $99."]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
JpvN6hSpKUZ_0tUtNRFHDbX9a0VLs4-9DIIH0KsyoRzLKqux-XG05v93orWHyM9yAMnLGc5S9Z3I85t0bnsqQQ

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
bwTa8Hx3oleryse0fgxNE-4Lt1lsLYzhxB3IWjS2gTOD559WU6IXsu-MLDKf_1I-HT-RipN--pY6OooW57r9Qg
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"status":"active","count":5,"avg_score":60.0}
{"status":"inactive","count":5,"avg_score":50.0} ⏱️ Execution time: 0ms

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
