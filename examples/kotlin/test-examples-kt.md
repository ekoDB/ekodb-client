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
✓ Inserted user: Record(fields={id=StringValue(value=WDmcWxvOvgBa7AkmWgmBZTZbH_g4J1DdV5v9P0fqWH-FDccF1knTGYDFg2YV-eewkHLaUVjVsuk6ZTH8J79OtQ)})
  User ID: WDmcWxvOvgBa7AkmWgmBZTZbH_g4J1DdV5v9P0fqWH-FDccF1knTGYDFg2YV-eewkHLaUVjVsuk6ZTH8J79OtQ

=== Read ===
✓ Found user by ID: Record(fields={age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=WDmcWxvOvgBa7AkmWgmBZTZbH_g4J1DdV5v9P0fqWH-FDccF1knTGYDFg2YV-eewkHLaUVjVsuk6ZTH8J79OtQ), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Update ===
✓ Updated user: Record(fields={id=StringValue(value=WDmcWxvOvgBa7AkmWgmBZTZbH_g4J1DdV5v9P0fqWH-FDccF1knTGYDFg2YV-eewkHLaUVjVsuk6ZTH8J79OtQ), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), id=StringValue(value=WDmcWxvOvgBa7AkmWgmBZTZbH_g4J1DdV5v9P0fqWH-FDccF1knTGYDFg2YV-eewkHLaUVjVsuk6ZTH8J79OtQ), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})

=== Delete ===
✓ Deleted user with ID: WDmcWxvOvgBa7AkmWgmBZTZbH_g4J1DdV5v9P0fqWH-FDccF1knTGYDFg2YV-eewkHLaUVjVsuk6ZTH8J79OtQ

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

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
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: jTegHbTaUmQJ5O_dPQ29NE6eRWwqgLcTOQPH4At-u0rz22TTE78YBSJt08vsRVCxXeH_fxiELmWkNYIG26Ub3w, nR4LaRuVnKtuTm2tglJO3luOZsawJFZzkgnZdiNQgpIsEeTx2v_LKlbo6jRiXgDplQqFXSzzMpgu3Ilqzj-srw, ToHS9AVdv-UElaz22ZbSt_zdZpIr7IhtFPHA1SrpaByeW6f3YGlZ2a_DKz4OBBh1NjjfJZwhJq8y5d7FySaAnw...

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
  - ObjectValue(value={value=StringValue(value=Charlie), type=StringValue(value=String)})
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Diana)})

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
✓ Retrieved value: {"name":"Alice","email":"alice@example.com","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"created_at":1766963961041,"user_id":"123"}
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
  - schema_employees_client_py
  - batch_users
  - schema_products_client_py
  - test_collection
  - schema_documents_client_ts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-28T23:19:25.256180Z","last_modified":"2025-12-28T23:19:25.256180Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: VLd5FiRemabrELtmbE1F59EK0tobUOvWj1QzjBIMVc_BGxMJ3z0aTUpOmAG0pA7zM1k65VTQIcHsx_r2nA1jDQ

=== Verify Document Exists ===
✓ Document found: [user_id, session_id, created_at, id, ttl_duration]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: D2LyG9hjvb6NV56JWqXausUfIAWTn0QLHRHDvvQiufwsR1eNHdxa0CQr1hywkIgGC0UYd7xz_TjZGlaUDkCdfw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [id, ttl_duration, cache_key, value]

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"payload":{"data":[{"id":"rjjPwwTsnoing3tdEsMKQRL6XiLzccfzFmaFgTnBwC0qAsHCXJAplvGvBw2n0zu_DAbRv_kvuLS_9hZfGrOTLw","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=ykOeUb2yG4bH7C1JSfeUROvQ-l9YR33GbxIq3RoyirkPasw85Evf-_uXuipBoO9LQVVQnQ2uOa2GcYWlwMxVGQ)
✓ Inserted user 2: StringValue(value=wkQ6ZncNOAW9V1KRN9Ute2-uiv7xt4sNRrfx8UhqxC1msFGAzFtLsw_V78vbtAyI47Ev5byY8HWWIib-gdbzAg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_employees_client_py, batch_users, schema_products_client_py, test_collection, schema_documents_client_ts]

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
  {"results":[{"record":{"title":"Rust Programming","description":"Learn Rust programming language with hands-on examples and best practices.","id":"igciM42xxBu6kuvOW83TRpRmnWWIFNF3m1QcDOE-zbQRyMulRLWmU0fnjnl4DCFMwZlX09-cfpaZ7WAyrAc2KA","views":424,"tags":"programming,rust,tutorial"},"score":19.8,"matched_fields":["title","description","tags"]},{"record":{"title":"Python for Data Science","id":"saj7TVQ185BxxZFZucm0zU4zhojmVmKajgNcGwDNmltCRHFXc5GYzAIhV3FR9-ywESPONrz58Zy8oYTjOLf7SA","views":443,"tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization."},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":"Build modern web applications using JavaScript, React, and Node.js.","tags":"programming,javascript,web","title":"JavaScript Web Development","id":"CyBH1uhu2i4FNHHAaamfS9B2Eo6CNiuneNdQlLuyJMflu1SLspmBHal_AVHNWd9cZZzK2XUAG6yJohypwUx7jQ","views":255},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"id":"6rAtUdOZXVrZPrKXYr9lDJyXJfd7ki_ckvZWf7JhynWF9AI2fa-w9zosyLOJNFAm3Fw5925RHLhJwSNODwXubA","tags":"ai,machine-learning,python","title":"Machine Learning Basics","views":611,"description":"Introduction to machine learning algorithms and neural networks."},"score":2.7,"matched_fields":["tags","description","title"]},{"record":{"title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization.","tags":"programming,python,data-science","views":443,"id":"saj7TVQ185BxxZFZucm0zU4zhojmVmKajgNcGwDNmltCRHFXc5GYzAIhV3FR9-ywESPONrz58Zy8oYTjOLf7SA"},"score":1.0,"matched_fields":["description"]},{"record":{"tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","views":424,"id":"igciM42xxBu6kuvOW83TRpRmnWWIFNF3m1QcDOE-zbQRyMulRLWmU0fnjnl4DCFMwZlX09-cfpaZ7WAyrAc2KA"},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn database design principles, normalization, and query optimization.","title":"Database Design","tags":"database,design,sql","views":942,"id":"g8Vl-Jp64G1Rke3Uxh6bxIVPv5p_mJtWw1v2s1c0cuamQCcwitv_E33szT4BIAhyMaL0JU9MuMMGHZSrhXh7Qw"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=BrRmN_twtPw9teHUiQckWrMI83hrPetll_sQy1NNA2Ifnk-7ARJ3VahAGTo7jOowkYwuJCVcjxSqyyRIyAM1eg)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1766963987164},"id":"BrRmN_twtPw9teHUiQckWrMI83hrPetll_sQy1NNA2Ifnk-7ARJ3VahAGTo7jOowkYwuJCVcjxSqyyRIyAM1eg","name":{"type":"String","value":"WebSocket TTL Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

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
  - Alice Johnson: Engineering
  - Bob Smith: Sales

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

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
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: -fJGYNHaUc4qGM-6FVy-ZITOaXRVHbs_AiO1GE6zz_yBr74YiUS4-WPVgAa7g84Od848WjrPUmbJeXYEfDzjFA

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "NnIoGbin41mL9NBlO7IR84nQoDVn3vJMUrXXAEBf_uyV3cK-J0oTYG3tD1mHDXjPy97JRNr1tABVYBTC4dUOhg"
  Responses: ["Answer: ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. It offers several notable features. One of these is the AI Chat Integration, which allows you to query your database using natural language and receive AI-powered responses with relevant context. ekoDB also supports different types of search features, including full-text search, vector search, and hybrid search with automatic context retrieval."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: olkmFM_uc66QC_cz1eotsyc8efUQ5ob_OTp8DnjkwV2T4GAoi3e1RNsH9K72_cnZOzK44bkur9vbhoEc_rSClw

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is \"ekoDB\", a high-performance database product with AI capabilities. The price for this product is $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"olkmFM_uc66QC_cz1eotsyc8efUQ5ob_OTp8DnjkwV2T4GAoi3e1RNsH9K72_cnZOzK44bkur9vbhoEc_rSClw"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"x9cEHn6Xkh0aBng_xYDKiWVf08yWnyD6oZJs1Y1gpsfdLgMiWpFGMYEbXXfsQh-RyHqZUM6HyDWfQcGZO-MThg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-28T23:20:06.327938Z"},"id":"9L9v0fXygQchb44D29hoc6FR_7ONs_0umf0mitR0aG5qg8itWAnaIC9BL6hP5Cw8T3kGynCCowz5PV_aQJ0n-g","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":240,"total_tokens":266}},"updated_at":{"type":"DateTime","value":"2025-12-28T23:20:06.327938Z"}},{"chat_id":{"type":"String","value":"olkmFM_uc66QC_cz1eotsyc8efUQ5ob_OTp8DnjkwV2T4GAoi3e1RNsH9K72_cnZOzK44bkur9vbhoEc_rSClw"},"content":{"type":"String","value":"The available product is \"ekoDB\", a high-performance database product with AI capabilities. The price for this product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"x9cEHn6Xkh0aBng_xYDKiWVf08yWnyD6oZJs1Y1gpsfdLgMiWpFGMYEbXXfsQh-RyHqZUM6HyDWfQcGZO-MThg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-28T23:20:06.333539Z"},"id":"NzQbXv2IXRHiiwJ_xGjW8rwnsAGtKGeFsgq18JZHIen61qbNaBfgI_vb0ebdM7OCoRPqnpyD2rUSzDV9wA-XoQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":26,"prompt_tokens":240,"total_tokens":266}},"updated_at":{"type":"DateTime","value":"2025-12-28T23:20:06.333539Z"}},{"chat_id":{"type":"String","value":"olkmFM_uc66QC_cz1eotsyc8efUQ5ob_OTp8DnjkwV2T4GAoi3e1RNsH9K72_cnZOzK44bkur9vbhoEc_rSClw"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"x9cEHn6Xkh0aBng_xYDKiWVf08yWnyD6oZJs1Y1gpsfdLgMiWpFGMYEbXXfsQh-RyHqZUM6HyDWfQcGZO-MThg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2025-12-28T23:20:07.097744Z"},"id":"z-SYW1ZrQWNu6HbCJzGP2v0tpaJEkL7CsAZMvygfFOjJslmDvK4TX3iA44FH_JXrV_eZcWL8__8t9T4GLtWtvQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":240,"total_tokens":252}},"updated_at":{"type":"DateTime","value":"2025-12-28T23:20:07.097744Z"}},{"chat_id":{"type":"String","value":"olkmFM_uc66QC_cz1eotsyc8efUQ5ob_OTp8DnjkwV2T4GAoi3e1RNsH9K72_cnZOzK44bkur9vbhoEc_rSClw"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"x9cEHn6Xkh0aBng_xYDKiWVf08yWnyD6oZJs1Y1gpsfdLgMiWpFGMYEbXXfsQh-RyHqZUM6HyDWfQcGZO-MThg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2025-12-28T23:20:07.103944Z"},"id":"l3nOK1ZM9S0tY-Tmy8W3l1WslDqIp5aLlt0TP4ull-jGd5RiXAVS3bE4OjBjNpHEqcUZWyq-Vesn-SQiTBYPjg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":240,"total_tokens":252}},"updated_at":{"type":"DateTime","value":"2025-12-28T23:20:07.103944Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: h8trlpy3sCIEw-e-2V0xhgj64yRknHkHtFzBw1TxWhhsj_WJeq_VAAaPXmsdIL56VWxL7s5lOJR5fk8CC--0gg

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
✓ Created session: tzh5ke_VnHbtP3caQvNKiEFf-kz1e9pcu0tUkA_I-S7Arl7WARjT8YtS2hj7y5huaONXzql6Us8T6elA11Pw4A

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the context provided, one of the available products is ekoDB. This is a high-performance database product."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the context provided, the available product is ekoDB. It's a high-performance database product priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: FQTloqDAr6ok5cs9xZWxuxtBeNt53grehmxcwBY4QIpS7vSVhuR_FA8_3zBFEsEAsOK2OAY3Hlt9p5GlhFeMdw

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

✅ Script saved: NAvCqRxeiN2CZ9d5MNz8ytZZTBaNyFa1JYYcnfyNJbtbfrf6IxXS9LogsguL_UP9hLzSlavUol0lDMZOgjBSKg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","count":5,"avg_score":60.0}
   {"status":"inactive","count":5,"avg_score":50.0}
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
