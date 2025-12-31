make test-examples-kotlin
🟣 [36mBuilding Kotlin client library...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE
> Task :jar UP-TO-DATE
> Task :assemble UP-TO-DATE
> Task :compileTestKotlin NO-SOURCE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test NO-SOURCE
> Task :check UP-TO-DATE
> Task :build UP-TO-DATE

BUILD SUCCESSFUL in 3s
2 actionable tasks: 2 up-to-date
✅ [32mKotlin client built![0m
🧪 [36mRunning Kotlin client library examples...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple CRUD Example ===

=== Create ===
✓ Inserted user: Record(fields={id=StringValue(value=p8jDrws2fqXPclfks_zw6fBMAHO7xyN4O5UvHW_7FJyaklZbuxrwvWylVetMY_RxS95oyScxOzd1a2dKEzI2kQ)})
  User ID: p8jDrws2fqXPclfks_zw6fBMAHO7xyN4O5UvHW_7FJyaklZbuxrwvWylVetMY_RxS95oyScxOzd1a2dKEzI2kQ

=== Read ===
✓ Found user by ID: Record(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), id=StringValue(value=p8jDrws2fqXPclfks_zw6fBMAHO7xyN4O5UvHW_7FJyaklZbuxrwvWylVetMY_RxS95oyScxOzd1a2dKEzI2kQ), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)})})

=== Update ===
✓ Updated user: Record(fields={id=StringValue(value=p8jDrws2fqXPclfks_zw6fBMAHO7xyN4O5UvHW_7FJyaklZbuxrwvWylVetMY_RxS95oyScxOzd1a2dKEzI2kQ), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=p8jDrws2fqXPclfks_zw6fBMAHO7xyN4O5UvHW_7FJyaklZbuxrwvWylVetMY_RxS95oyScxOzd1a2dKEzI2kQ)})

=== Delete ===
✓ Deleted user with ID: p8jDrws2fqXPclfks_zw6fBMAHO7xyN4O5UvHW_7FJyaklZbuxrwvWylVetMY_RxS95oyScxOzd1a2dKEzI2kQ

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: 5ax711-g0zZYaRNQYOFsQCk1oYxDF7falmW6R79l8XPf7n86PHsTtJT4p0Dfmg_A19FGRFRC6DG6aS2j03aWgw, qu94s4iuFJRxSfMTe2s6jkatiLfBILlEB_BM0wMiicFAdbTex333OIRcYSOdGIdbxNZtV_s4_7kg1PRnc30Dfw, 8G_BnKGPp30uT9n9TKPRm-jwVD6HngYh-GEGkyrY28BAIiiHAq5qXsyxZCqhqfbrfBjFJ6kHOGXs-auALffjZQ...

=== Batch Update ===
✓ Updated 3 records

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 records

=== Query 1: Equality (city = 'NYC') ===
✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) ===
✓ Found 3 records with age 25-31

=== Query 3: Sort by score (descending) ===
✓ Top 3 scores:
  - Score: ObjectValue(value={value=IntegerValue(value=95), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={value=IntegerValue(value=92), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={value=IntegerValue(value=88), type=StringValue(value=Integer)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Charlie)})
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Diana)})

=== Query 7: Contains (name contains 'a') ===
✓ Found 2 names containing 'a'

=== Cleanup ===
✓ Deleted collection: kotlin_query_example

=== Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1767198102441}
  (Will expire in 10 seconds)

=== Set Multiple Keys ===
✓ Set 3 configuration keys

=== KV Delete ===
✓ Deleted key: user:123

=== Verify Deletion ===
✓ Confirmed key was deleted

=== Cleanup ===
✓ Cleaned up test keys

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 21 collections
  - schema_documents_client_go
  - websocket_test
  - ttl_cache
  - schema_employees_client_py
  - schema_users_client_ts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-31T16:21:46.845307Z","last_modified":"2025-12-31T16:21:46.845308Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: 5vfrMrnPORxpBQ9glrtw_Hwnd6Cm1Mxmxo-2CFDJcyy1-t4LObalUDJC8DDvxCNfHvIC3utD0tSzdKCwvdyiRQ

=== Verify Document Exists ===
✓ Document found: [user_id, created_at, ttl_duration, id, session_id]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: GdckQ-D2l7BdSrP2xXHOv1Jb3AOAupmy-Z1Y-paGU-91dlGvP5qihs_eqL2GQx9-n95PmxnCgLMP1VFLXbRzrw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [cache_key, ttl_duration, id, value]

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"payload":{"data":[{"id":"8xcI2zGumqpcLdoIKwynveQUUoglXk8NaH-glXcKUCd6Ozd0I7eFyYIl_k4GbcruLpCrHfoGhxiQkp_u7wH9Kw","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=4osdzy62ytkXtf44VIMjB8gWi-pReJ0ytidXxNJrpC5j7FMUdwKRy3EBnuSZEioIDpMC-XLZhEj93ZUQ8rC48Q)
✓ Inserted user 2: StringValue(value=p99RB6fvpXsemtnJC6Y_do7xXuxDCcUTYrVUaf8U2XI6-IEOM1L4hy5KDX1Xm91swXc9NPTcYTZEK6PAZ_Qnkg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_documents_client_go, websocket_test, ttl_cache, schema_employees_client_py, schema_users_client_ts]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"tags":"programming,rust,tutorial","title":"Rust Programming","views":166,"description":"Learn Rust programming language with hands-on examples and best practices.","id":"kJ_-dooEl6BA2D5_VBfuho6g-VxDWoGIGAVO0qxDOIR9wfbkm3cpWEI7aslDaoQVf6iYvUICUUFK2zymCZ1BXg"},"score":19.8,"matched_fields":["title","tags","description"]},{"record":{"tags":"programming,python,data-science","id":"FQD2wA29uVAH-hTW249hR2WDOkhMaSyZWvqf_CFuMAyYJvIQrPNUahD60FdbXsfn7vNnLKej0WNUPCL3GAwNnQ","description":"Master Python for data analysis, machine learning, and visualization.","views":944,"title":"Python for Data Science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":"Build modern web applications using JavaScript, React, and Node.js.","views":109,"id":"2zuyHw0lrKegqkFq4wtKZrXb3dRcLiFN-C6TPeSjjqs6D8hQNM22kTo03UpdvlUObv3-QuL8RqbAPG0If0v4vA","title":"JavaScript Web Development","tags":"programming,javascript,web"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"tags":"ai,machine-learning,python","title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","id":"Gu78MI6tNv9fdfN23HAUERR0bCYKYK7--FlN1k8nIgM7bDRv7LLqxktzwIfCM2Rhww69kT8tYltG8v_HPZ7HYg","views":505},"score":2.7,"matched_fields":["title","tags","description"]},{"record":{"tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization.","views":944,"id":"FQD2wA29uVAH-hTW249hR2WDOkhMaSyZWvqf_CFuMAyYJvIQrPNUahD60FdbXsfn7vNnLKej0WNUPCL3GAwNnQ","title":"Python for Data Science"},"score":1.0,"matched_fields":["description"]},{"record":{"tags":"database,design,sql","views":958,"description":"Learn database design principles, normalization, and query optimization.","title":"Database Design","id":"UIJ5g90F9kaGySXrFFeZjr3l6PrEK0PDFsZGAFKx-Myec5HwQM_XEhOGUA005NZDece7w-GlnjyIMGRlF5PAAw"},"score":0.5,"matched_fields":["description"]},{"record":{"id":"kJ_-dooEl6BA2D5_VBfuho6g-VxDWoGIGAVO0qxDOIR9wfbkm3cpWEI7aslDaoQVf6iYvUICUUFK2zymCZ1BXg","views":166,"description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","tags":"programming,rust,tutorial"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup ===
✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=BHGPVMx3ziYui5tdwEEfwr4M7f7vk29yfJj1zEbczmqYtYJExi0mblEzPZH5hlBy7cQif_Cf41l9uES3AB5bWw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767198128450},"id":"BHGPVMx3ziYui5tdwEEfwr4M7f7vk29yfJj1zEbczmqYtYJExi0mblEzPZH5hlBy7cQif_Cf41l9uES3AB5bWw","name":{"type":"String","value":"WebSocket TTL Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data ===
✓ Found 2 completed orders
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: XhpBPYD5xKIGjrz74UBzkNMucxgd_0CV0XplH5Ia_Iee37DWov-zqptaGc1RucE5Ms7jFADdrn4XibIIvOOwZg

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "kBzSC9fniUyYmjvMG0Hwe2AsQ0IhLnwKwEZ_4mItuN_hgYcma4WyT5H9w3T6Zyuc1MA4n6M-1xlgq_PKEqIqqw"
  Responses: ["ekoDB is a high-performance database that is designed with intelligent caching, real-time capabilities, and AI integration. It offers several unique features such as:\n\n1. AI Chat Integration: This feature allows you to query your database using natural language and get AI-powered responses with relevant context.\n\n2. Search Features: ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval, making data searches efficient and contextually relevant."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 6s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: pZv6DpFsBN3FxQkWXc7ytkoZtz9ikbf7mMM---kjQjcFTXxEqJXYOqph2JmvGXEnanUGJOJUx2H_bbBkdWSevQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Answer: The product available is \"ekoDB\", a high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"pZv6DpFsBN3FxQkWXc7ytkoZtz9ikbf7mMM---kjQjcFTXxEqJXYOqph2JmvGXEnanUGJOJUx2H_bbBkdWSevQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"VqRaDRb71FbsLLfRWDfFrmBaYC_AULM38UEoNrm35dR-AfE5OqgEtR2SZ29LoHHpDzFNDD851CgfeygG7sBMgg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-31T16:22:24.935673Z"},"id":"lx2OkVGpCDDmOtbpUr0Z2P9muT1bg237oxPw3LaQ0Y31NP98RPl6c5ZpsSNxmfiiqarJiZEE45JF3Mc8KrGUSw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":235,"total_tokens":261}},"updated_at":{"type":"DateTime","value":"2025-12-31T16:22:24.935673Z"}},{"chat_id":{"type":"String","value":"pZv6DpFsBN3FxQkWXc7ytkoZtz9ikbf7mMM---kjQjcFTXxEqJXYOqph2JmvGXEnanUGJOJUx2H_bbBkdWSevQ"},"content":{"type":"String","value":"Answer: The product available is \"ekoDB\", a high-performance database product with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"VqRaDRb71FbsLLfRWDfFrmBaYC_AULM38UEoNrm35dR-AfE5OqgEtR2SZ29LoHHpDzFNDD851CgfeygG7sBMgg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-31T16:22:24.938569Z"},"id":"y8vIAgwhHZv8ZIi03GkKSRBIoxRard7D0caDODhbeGN_95venpL-asKwiY-BuMX2bObKXNMmLAaZmIEi9O-a6A","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":235,"total_tokens":261}},"updated_at":{"type":"DateTime","value":"2025-12-31T16:22:24.938569Z"}},{"chat_id":{"type":"String","value":"pZv6DpFsBN3FxQkWXc7ytkoZtz9ikbf7mMM---kjQjcFTXxEqJXYOqph2JmvGXEnanUGJOJUx2H_bbBkdWSevQ"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"VqRaDRb71FbsLLfRWDfFrmBaYC_AULM38UEoNrm35dR-AfE5OqgEtR2SZ29LoHHpDzFNDD851CgfeygG7sBMgg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2025-12-31T16:22:25.672782Z"},"id":"eZw6q_Dzb5GXSiDufUeq1oJQ_vNuohFsAYf6quoAVcyxQELCV4UL4Bwvf74iG3Y0EdQb4HzKTUd9TP5bG-1DVg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":235,"total_tokens":247}},"updated_at":{"type":"DateTime","value":"2025-12-31T16:22:25.672782Z"}},{"chat_id":{"type":"String","value":"pZv6DpFsBN3FxQkWXc7ytkoZtz9ikbf7mMM---kjQjcFTXxEqJXYOqph2JmvGXEnanUGJOJUx2H_bbBkdWSevQ"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"VqRaDRb71FbsLLfRWDfFrmBaYC_AULM38UEoNrm35dR-AfE5OqgEtR2SZ29LoHHpDzFNDD851CgfeygG7sBMgg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2025-12-31T16:22:25.676416Z"},"id":"x_o6BODQlPFM3gjHf1OMPC88T2PSRxuVL8mqUACBao8TVZj5SQXJyaDyjxpYXVnhk91sqw0SiqUIT-9O8oM6Xw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":235,"total_tokens":247}},"updated_at":{"type":"DateTime","value":"2025-12-31T16:22:25.676416Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: tM_iDzK9Wzrc7tXgZ75UJWg2qDmvlaGc2OaDrShVYpgLSlvQcls81Ik4La2D6w4TS6qGXGhM2Prww_GyhmajtQ

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 5s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: i6SLBNbQGQC3KHDnxsDJ_C_7m7CyqaXwjxHs8cw-zuCeyXWJPXJ359Vk7HH17BhUEqbOs1xhMJxNnZ_4_AjA5g

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the provided context, the available product is ekoDB, a high-performance database product. It is priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the available context, the product that is available is a high-performance database product named \"ekoDB\". It is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: -OB-lUza4K56Q1JHnvRlFGh_pOedrl-mUI2kGVi7iIYtqwu5-8NeaYu4HYYlUy1EOVAsO2zQ5lNwIxDexndubA

=== Merging Sessions ===
✓ Merged sessions
  Merged session ID: null

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 7s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: y4F3uBlBnsepWUCPRPawRp3lms3UUPRDMNd_W72CqxY3yD3mTv_few7g92r103dXIRgznD517rzlECb6ejXVIw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"status":"active","avg_score":60.0}
   {"avg_score":50.0,"count":5,"status":"inactive"}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Script listing skipped (some scripts contain parameter placeholders)
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
⚠️  Script cleanup skipped (some scripts contain parameter placeholders)

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
Kotlin example (requires kotlinc installed)
