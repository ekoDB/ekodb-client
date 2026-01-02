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
✓ Inserted user: Record(fields={id=StringValue(value=kwl9hUlpPjaBdZ20ESnmD_fXbiGAuc5W-VklG5cHX6tMfHGYMqMyS-wKHG99GkMRHlpAdyQ0o5iUd5LEcH18Eg)})
  User ID: kwl9hUlpPjaBdZ20ESnmD_fXbiGAuc5W-VklG5cHX6tMfHGYMqMyS-wKHG99GkMRHlpAdyQ0o5iUd5LEcH18Eg

=== Read ===
✓ Found user by ID: Record(fields={active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), id=StringValue(value=kwl9hUlpPjaBdZ20ESnmD_fXbiGAuc5W-VklG5cHX6tMfHGYMqMyS-wKHG99GkMRHlpAdyQ0o5iUd5LEcH18Eg), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Update ===
✓ Updated user: Record(fields={id=StringValue(value=kwl9hUlpPjaBdZ20ESnmD_fXbiGAuc5W-VklG5cHX6tMfHGYMqMyS-wKHG99GkMRHlpAdyQ0o5iUd5LEcH18Eg), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=kwl9hUlpPjaBdZ20ESnmD_fXbiGAuc5W-VklG5cHX6tMfHGYMqMyS-wKHG99GkMRHlpAdyQ0o5iUd5LEcH18Eg), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)})})

=== Delete ===
✓ Deleted user with ID: kwl9hUlpPjaBdZ20ESnmD_fXbiGAuc5W-VklG5cHX6tMfHGYMqMyS-wKHG99GkMRHlpAdyQ0o5iUd5LEcH18Eg

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

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
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: cupu4vtD-2xmELxAvvwrC3Yc4H9DSCcZXHjSl1sJNYLx_eN0Dt0Vpd9kivUqKmrT7Riu0S-rS7GODuv40YXgtw, nNQ4mFck3rM-4s6il24jOwme7l4awWs3ow3W4kjZVFKZ2SStKh1Jlu57VSXgmbx16FLKMJywv3ShEoPYVDPFGw, FKEUJpKyFjzMEEdSek8T3iB1VzjOFD02HUh0-mByFvYSOYnNS9A6AKGsQm5M_GcH1eaPDJBbINAac5D2A_uZdg...

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
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=95)})
  - Score: ObjectValue(value={value=IntegerValue(value=92), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={value=StringValue(value=Charlie), type=StringValue(value=String)})
  - ObjectValue(value={value=StringValue(value=Diana), type=StringValue(value=String)})

=== Query 7: Contains (name contains 'a') ===
✓ Found 2 names containing 'a'

=== Cleanup ===
✓ Deleted collection: kotlin_query_example

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
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"role":"admin","email":"alice@example.com","name":"Alice"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"created_at":1767200776880,"user_id":"123"}
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
  - schema_products_client_ts
  - schema_users_client_ts
  - chat_messages__ek0_testing
  - schema_employees_client_py
  - test_accounts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-31T17:06:21.069970Z","last_modified":"2025-12-31T17:06:21.069971Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: 2tWNtv_2xUPefewX49yArZOo5Rs9EJEEFSPvaFmARF_cn6bT35Do50Ytq3EzW3_dtYiblEfCR6cbFKcFl9rICw

=== Verify Document Exists ===
✓ Document found: [user_id, id, created_at, session_id, ttl_duration]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: UHRzkjK9MeHHqyqm43NN2LSHfzdPlAUg4YMZo9QsgJTyBOh3ygy3fXgzM2-vbEAFCeUwW0bAwuRP4SOZDeLAMQ

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [ttl_duration, id, cache_key, value]

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
  {"payload":{"data":[{"id":"BPEPgV9qkeH3xBrajUr7N-pwp5ydfHJCf4IGkGp1f8-yS1FQUlo0bhEcio6k7ql_qfetwSFsXA1dJlQ0Ac2obQ","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=4XER7hC-3_jIXlbwWWNipSdpXSmtETz1H7oV-Uj9el82MKFP9MyWaMPojau5BZAAmAf0w8q9K69LA8I_yKXRCg)
✓ Inserted user 2: StringValue(value=xP0OGV-acYqLSArrAIpFuYJ50fxmjK_ypxl85pFE_igWdCnxwBaJcNVauCD6fYg9B3uhLKjBIl_o4O9rmqVUTA)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_products_client_ts, schema_users_client_ts, chat_messages__ek0_testing, schema_employees_client_py, test_accounts]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

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
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"views":713,"id":"h9KDriEIQTXgZCrBnUllUeogKebrycvHbF8VPI7ymC5rDV3hCrgJntxbOMzJA5xjIjW5Tbco5Zy9gWY-uHn-Kw","title":"Rust Programming","description":"Learn Rust programming language with hands-on examples and best practices.","tags":"programming,rust,tutorial"},"score":19.8,"matched_fields":["description","title","tags"]},{"record":{"tags":"programming,python,data-science","views":58,"id":"O41pftYO8NUp7FZG_aqMTmvpqgX2v0EHhKk4Ed15SiwhbU_mkdshXofC3o5bHA4muoSb6ghpbT9cZMUei9vW1g","description":"Master Python for data analysis, machine learning, and visualization.","title":"Python for Data Science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,javascript,web","title":"JavaScript Web Development","id":"cRWJmnlB-pzqCsfsdvpO1XY81EO_3s35N3Nrj7grBCLOISa6FIO9DBHNIJlTnE7-hzYWt-kQfvqTXcPZtI2aiQ","views":918,"description":"Build modern web applications using JavaScript, React, and Node.js."},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"title":"Machine Learning Basics","tags":"ai,machine-learning,python","id":"p8u-HXVxkCTSInOecbMrbW79ahg5A8CFZhccKmagLw4wM40yo9d4XAYJ99y4YCXBMl20mcaJZqtHi1621Tc07Q","description":"Introduction to machine learning algorithms and neural networks.","views":52},"score":2.7,"matched_fields":["title","tags","description"]},{"record":{"title":"Python for Data Science","views":58,"tags":"programming,python,data-science","id":"O41pftYO8NUp7FZG_aqMTmvpqgX2v0EHhKk4Ed15SiwhbU_mkdshXofC3o5bHA4muoSb6ghpbT9cZMUei9vW1g","description":"Master Python for data analysis, machine learning, and visualization."},"score":1.0,"matched_fields":["description"]},{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","views":713,"id":"h9KDriEIQTXgZCrBnUllUeogKebrycvHbF8VPI7ymC5rDV3hCrgJntxbOMzJA5xjIjW5Tbco5Zy9gWY-uHn-Kw","tags":"programming,rust,tutorial","title":"Rust Programming"},"score":0.5,"matched_fields":["description"]},{"record":{"title":"Database Design","tags":"database,design,sql","description":"Learn database design principles, normalization, and query optimization.","views":34,"id":"r_wVMmKya1YiqL-36uMz8pxmnXZakN0rLnBoYudDpEIvA751vmhOGc3MnObGkV_ouzrsDzNMK9AVHW7ryhtKwg"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=hYFHuUiujx599cP3k-vIfUonzZjL2aBWTAjyDkdLfRVFWZ4poRy_9K39l_vdIzeP0Bpvoask5tKcwACEsCcqcg)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767200801944},"id":"hYFHuUiujx599cP3k-vIfUonzZjL2aBWTAjyDkdLfRVFWZ4poRy_9K39l_vdIzeP0Bpvoask5tKcwACEsCcqcg","name":{"type":"String","value":"WebSocket TTL Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

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
✓ Created session: ynh-0QVkZq4i3_Uhv8vRkl9ku6xUO9VYZ6GI6ByWs_SKMn4c1MGzxopLM7M-2vIc4vmIS4RAE3VGQEV3fimc_Q

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "HIVzst7BNUnagNcahCuL4CwLNn9BmLJLN4l7etEEltNAvT7UWPBbjBeASjm3tscOO0RAxIxJp-Kgz94csOVT_Q"
  Responses: ["ekoDB is a high-performance database with several distinct features. It is designed with intelligent caching and real-time capabilities, and it also integrates artificial intelligence. One of the unique features of ekoDB is its AI Chat Integration, which allows you to query the database using natural language and receive AI-powered responses with relevant context.\n\nIn addition to this, ekoDB also supports various search capabilities, including full-text search, vector search, and hybrid search. These search features come with automatic context retrieval, enhancing the overall user experience."]

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
✓ Created session: FyWzFtkkH4W9DcSKqcBvYZa9jnziX0brggYh95w8YQYGuoYlwQfj6-7kKywx0nA7Lnn4A3MCZcI4T8NAhSefQw

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The product available is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the product \"ekoDB\" is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"FyWzFtkkH4W9DcSKqcBvYZa9jnziX0brggYh95w8YQYGuoYlwQfj6-7kKywx0nA7Lnn4A3MCZcI4T8NAhSefQw"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"mNlcouNej776398GS69lg3WMKzn1mCdP9ka9QF3YabZ8V8sAjePj-kWSPTjegebfBaRExHqqLQ8PrlSjsJhYxQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-31T17:06:58.803162Z"},"id":"ekJRDK-vAsjUTWQQ-4GTPxgYTMMPHdwvavFVVhSxwGOjeO1QeqjtPbckAIehxqP0DjFBhpD39uqvTFEVSlN9uQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":24,"prompt_tokens":237,"total_tokens":261}},"updated_at":{"type":"DateTime","value":"2025-12-31T17:06:58.803162Z"}},{"chat_id":{"type":"String","value":"FyWzFtkkH4W9DcSKqcBvYZa9jnziX0brggYh95w8YQYGuoYlwQfj6-7kKywx0nA7Lnn4A3MCZcI4T8NAhSefQw"},"content":{"type":"String","value":"The product available is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"mNlcouNej776398GS69lg3WMKzn1mCdP9ka9QF3YabZ8V8sAjePj-kWSPTjegebfBaRExHqqLQ8PrlSjsJhYxQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-31T17:06:58.806465Z"},"id":"jjSf7sQYGgHkF26oVp3P6ajURU4ggkkrmF322triWpnFMCLoO-a5cAR487FFWR7-_1NEU2WrqBes_ssXrEtibg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":24,"prompt_tokens":237,"total_tokens":261}},"updated_at":{"type":"DateTime","value":"2025-12-31T17:06:58.806465Z"}},{"chat_id":{"type":"String","value":"FyWzFtkkH4W9DcSKqcBvYZa9jnziX0brggYh95w8YQYGuoYlwQfj6-7kKywx0nA7Lnn4A3MCZcI4T8NAhSefQw"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"mNlcouNej776398GS69lg3WMKzn1mCdP9ka9QF3YabZ8V8sAjePj-kWSPTjegebfBaRExHqqLQ8PrlSjsJhYxQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2025-12-31T17:07:00.032237Z"},"id":"t_OjWanhsCPN_Uca2BhK5lrrcIXsNTfvjxmdmPjLCBxndKrNyDCmN18vid0B0pM0-LbnfuSQ_qXKcr5Jg5yYEQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":13,"prompt_tokens":237,"total_tokens":250}},"updated_at":{"type":"DateTime","value":"2025-12-31T17:07:00.032237Z"}},{"chat_id":{"type":"String","value":"FyWzFtkkH4W9DcSKqcBvYZa9jnziX0brggYh95w8YQYGuoYlwQfj6-7kKywx0nA7Lnn4A3MCZcI4T8NAhSefQw"},"content":{"type":"String","value":"The price of the product \"ekoDB\" is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"mNlcouNej776398GS69lg3WMKzn1mCdP9ka9QF3YabZ8V8sAjePj-kWSPTjegebfBaRExHqqLQ8PrlSjsJhYxQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2025-12-31T17:07:00.035693Z"},"id":"K_OgGQ8Had5gmKPIAyzegiZiEFOA-9OjqLnjVjlbQBTkLOYEJOU3nx_arJQO2SI6DS2F2hzRSt5Y2_TUCAGpCQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":13,"prompt_tokens":237,"total_tokens":250}},"updated_at":{"type":"DateTime","value":"2025-12-31T17:07:00.035693Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: KGxW_koQiXSBy981CQbB-jDCg9hgvhQqIwJO9vB3HRc5PALCAFNaZZ7OC3Gumx4wAM6-6CeybkQhC0oKDU9gtg

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

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
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: pEM2YAUNr64e5zAqOHfFgTGfkxubSnkeABJH7fjfCxKbQthxM5CElw-h7VlFgTlyNIIJpLl7QXSYb8Zs-mqqhw

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information given, one product that is available is ekoDB. It is a high-performance database product and is priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the information I have, the product available is \"ekoDB\". It is a high-performance database product. The price of this product is $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: JPReDaVNOc_jIRcV1vrDM7H6OKj1jSzc6DuZA6ZPPFEDk9sJ1HuxT25yRwqw8Ee1idgUP7dYawDjQH7yLICxgQ

=== Merging Sessions ===
✓ Merged sessions
  Merged session ID: null

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 8s
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

✅ Script saved: hJ4i0ikl6nRBeqQesSlR0AfhU7fLr7dMhYzc5oCmkESDeJNfxM_TcqaOG9rNBxwpo626h7bhuGR1KOl6SRmy4A
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":60.0,"count":5,"status":"active"}
   {"avg_score":50.0,"status":"inactive","count":5}
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
