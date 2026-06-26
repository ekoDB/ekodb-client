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
> Task :compileTestKotlin UP-TO-DATE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test UP-TO-DATE
> Task :check UP-TO-DATE
> Task :build UP-TO-DATE

BUILD SUCCESSFUL in 4s
4 actionable tasks: 4 up-to-date
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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=6Ac2zl60V4owjc5AP8rChDNbkZkTc1KLSAkCResoVDFfQU93OipSciAD-B6BJh7NGt_G5qbX-8ENr65uGeCGpw)})
  User ID: 6Ac2zl60V4owjc5AP8rChDNbkZkTc1KLSAkCResoVDFfQU93OipSciAD-B6BJh7NGt_G5qbX-8ENr65uGeCGpw

=== Read ===
✓ Found user by ID: EkoRecord(fields={id=StringValue(value=6Ac2zl60V4owjc5AP8rChDNbkZkTc1KLSAkCResoVDFfQU93OipSciAD-B6BJh7NGt_G5qbX-8ENr65uGeCGpw), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={value=StringValue(value=Fri Jun 26 12:58:59 EDT 2026), type=StringValue(value=String)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Fri Jun 26 12:58:59 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: id, user_id, categories, created_at, data, tags, metadata, age, active, email, price, name, embedding

=== Update ===
✓ Updated user: EkoRecord(fields={metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), created_at=ObjectValue(value={value=StringValue(value=Fri Jun 26 12:58:59 EDT 2026), type=StringValue(value=String)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), id=StringValue(value=6Ac2zl60V4owjc5AP8rChDNbkZkTc1KLSAkCResoVDFfQU93OipSciAD-B6BJh7NGt_G5qbX-8ENr65uGeCGpw), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), id=StringValue(value=6Ac2zl60V4owjc5AP8rChDNbkZkTc1KLSAkCResoVDFfQU93OipSciAD-B6BJh7NGt_G5qbX-8ENr65uGeCGpw), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Fri Jun 26 12:58:59 EDT 2026)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})

=== Delete ===
✓ Deleted user with ID: 6Ac2zl60V4owjc5AP8rChDNbkZkTc1KLSAkCResoVDFfQU93OipSciAD-B6BJh7NGt_G5qbX-8ENr65uGeCGpw

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

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
  {"data":[{"status":{"value":"active","type":"String"},"id":"RGdPR50WpX51aHVuhUWM9Jsz0aKW4ebgzdwRrzoG_kPqbqrQrMUGeYOmwyioEYHubnL0XFf8ck4XRA_orzBJLg","name":{"type":"String","value":"Test User"}}]}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

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
  IDs: g3cG_8ginyVZ-cJDX-4c1asW4418FJpmDkn4E9pMf9kzO0Orx3A8TTDeb-tZ0lGLU5czE-uonDE0EyqZWvF42Q, a-2tvgYMv1aEqsWNynnd1o7mU0pmvuXWchjDRe--aSAsJzbRdIdBzg2gOigNmr7d_zKsH30Y56i7kzDAGxQRAw, unCVaQhr0F7o3D42iEwVw-0qIGqdmg2XTuC55QFvdzo6Mp9sL-QF0ifxmy3OzveV4XmEc16qMS0X6f0DImmrgg...

=== Batch Update ===
✓ Updated 3 records

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

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
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 29 collections
  - test_collection
  - schema_employees_client_js
  - functions__ek0_testing
  - ttl_cache
  - chat_messages__ek0_testing

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-06-26T16:59:18.120131Z","last_modified":"2026-06-26T16:59:18.120132Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

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
✓ Session value: {"user_id":"123","created_at":1782493162905}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  config:db: {"value":{"port":5432,"host":"localhost"}}
  config:cache: {"value":{"enabled":true,"ttl":3600}}
  config:api: {"value":{"retries":3,"timeout":30}}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 18

=== KV Delete ===
✓ Deleted key: user:123

=== Verify Deletion ===
✓ Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  config:db: deleted
  config:cache: deleted
  config:api: deleted

=== Example Complete ===

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
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hxOX87hT_LBxYF7ORxAT8QQ9-hqdaOLNwSTvubFSoPa9eP5N9ns0KdyYS3vH_wDsdJVUWuOlDYOSY_Hvaq8Zrg
Created Bob: $500 - ID: cNMhqWnNryFpwxx7Qcq3BMBP-mCUjdbBKR5G-tr5Z9m-XjSo7F_j0xwh_QveKsw6yZCeJ43iYHkJlrGSRGCsXQ

=== Example 1: Begin Transaction ===
Transaction ID: ab73f407-2a2b-497f-9919-4ae6c9ec5111

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9a2c2b52-127a-4243-ba62-92bb0c00a1de
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed

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
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=92)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Charlie)})
  - ObjectValue(value={value=StringValue(value=Diana), type=StringValue(value=String)})

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
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"title":{"value":"Rust Programming","type":"String"},"tags":{"type":"String","value":"programming,rust,tutorial"},"category":{"value":"programming","type":"String"},"views":{"type":"Integer","value":259},"description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"},"id":"D3XmqmHTNNqj1n9DSkGXmGaMco76FKLdvOjFdFyvHMZpUFsxZnj8Yp0LW67J9NbjtIL34090rHsRhGP5bYGvgA"},"score":26.400000000000002,"matched_fields":["tags","description","category","title"]},{"record":{"description":{"value":"Build modern web applications using JavaScript, React, and Node.js.","type":"String"},"views":{"type":"Integer","value":636},"category":{"type":"String","value":"programming"},"id":"T7W0zuj6NH0Js2kG2MMdElnO4qJYWy6PgO8fFZSY0WbqAInOSEaOtPkCEWtTkCBtZXCpuz94OuIpL9wq5ASIzw","title":{"value":"JavaScript Web Development","type":"String"},"tags":{"value":"programming,javascript,web","type":"String"}},"score":13.200000000000001,"matched_fields":["category","tags"]},{"record":{"title":{"value":"Python for Data Science","type":"String"},"views":{"type":"Integer","value":918},"tags":{"type":"String","value":"programming,python,data-science"},"category":{"type":"String","value":"programming"},"id":"tfXAXOvJBtY0dmarNkyP2ltSmKkFI6x_YpuZatsSUZi7SDOn0qBhwYYEYttxN7FUrhtk4ius5TaZnsiUU74s8w","description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."}},"score":13.200000000000001,"matched_fields":["tags","category"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"views":{"type":"Integer","value":610},"description":{"value":"Introduction to machine learning algorithms and neural networks.","type":"String"},"tags":{"type":"String","value":"ai,machine-learning,python"},"title":{"type":"String","value":"Machine Learning Basics"},"category":{"type":"String","value":"ai"},"id":"XaIqDYE_i9e7YF_8fWvykjFDydXV4PD3hp_4_VmOaVMQvDcn4fThofALMVa_taKxRwwmMgsoUhI9a1jjk5KIwQ"},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"id":"tfXAXOvJBtY0dmarNkyP2ltSmKkFI6x_YpuZatsSUZi7SDOn0qBhwYYEYttxN7FUrhtk4ius5TaZnsiUU74s8w","title":{"type":"String","value":"Python for Data Science"},"category":{"type":"String","value":"programming"},"views":{"value":918,"type":"Integer"},"tags":{"type":"String","value":"programming,python,data-science"}},"score":1.0,"matched_fields":["description"]},{"record":{"id":"D3XmqmHTNNqj1n9DSkGXmGaMco76FKLdvOjFdFyvHMZpUFsxZnj8Yp0LW67J9NbjtIL34090rHsRhGP5bYGvgA","description":{"type":"String","value":"Learn Rust programming language with hands-on examples and best practices."},"category":{"type":"String","value":"programming"},"title":{"value":"Rust Programming","type":"String"},"tags":{"type":"String","value":"programming,rust,tutorial"},"views":{"value":259,"type":"Integer"}},"score":0.5,"matched_fields":["description"]},{"record":{"id":"Fj8WGrfO47Og4Jqgx0BilxEtOjO0nd4QUja_l9nA9Nmzp6df40G1gnWE5FLpYPQbX537RdaUvw5_3O9Mh5JKtw","title":{"type":"String","value":"Database Design"},"category":{"type":"String","value":"database"},"views":{"value":303,"type":"Integer"},"tags":{"value":"database,design,sql","type":"String"},"description":{"type":"String","value":"Learn database design principles, normalization, and query optimization."}},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Search with a metadata pre-filter (category = programming) ===
✓ Found results in category 'programming' (database/ai excluded)
  {"results":[{"record":{"views":{"value":259,"type":"Integer"},"tags":{"type":"String","value":"programming,rust,tutorial"},"title":{"value":"Rust Programming","type":"String"},"category":{"value":"programming","type":"String"},"description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"},"id":"D3XmqmHTNNqj1n9DSkGXmGaMco76FKLdvOjFdFyvHMZpUFsxZnj8Yp0LW67J9NbjtIL34090rHsRhGP5bYGvgA"},"score":6.6000000000000005,"matched_fields":["description"]},{"record":{"id":"tfXAXOvJBtY0dmarNkyP2ltSmKkFI6x_YpuZatsSUZi7SDOn0qBhwYYEYttxN7FUrhtk4ius5TaZnsiUU74s8w","description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"tags":{"type":"String","value":"programming,python,data-science"},"views":{"type":"Integer","value":918},"category":{"type":"String","value":"programming"},"title":{"type":"String","value":"Python for Data Science"}},"score":6.6000000000000005,"matched_fields":["description"]}],"total":2,"execution_time_ms":0}

=== Cleanup ===
✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

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
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=k4Go3QRI_Ytyu6gHmlmKld83nBsyyWTl_xx9wXj9ow3cPlz84lFZEsgITtokInE715kj0mQMP4-3P8Xhxc59BA)
✓ Inserted user 2: StringValue(value=fr2jwunSKKbtoeHmVvSBkP05rzWdGl0gg1BBO7XveCJs_sAti07M8NWHU-QfjBhF6cwvBIyCXqcF2uEpu9TL4w)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 30
  Sample: [test_collection, schema_employees_client_js, functions__ek0_testing, ttl_cache, chat_messages__ek0_testing]

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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: a5EUNZMifPZmAcFKF2CqayUzxHTfBZZ8JB6QwizylSynvd8HboL7qXGPGBCoc0Jaj8y9lPV66h5xWiyS5Oyd7A

=== Verify Document Exists ===
✓ Document found: user_id, created_at, session_id, id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: skgfn404dGX5eepH11kjLSN6Tq5jVYqt_mxYK_GF7iyTWyT64gUGStfuVAONWX067FqYGZfWNAZzRTnCB9vFnw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: id, cache_key, value, ttl

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
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=SkvLPHzTnwFA5CbQC2QoTPWcfg94oCz1eDDEq1YIJQ_ToKyUloBDj4C_ZA7c9Sky8fpvrSD_DaY98aR9kMBjVg)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"id":"SkvLPHzTnwFA5CbQC2QoTPWcfg94oCz1eDDEq1YIJQ_ToKyUloBDj4C_ZA7c9Sky8fpvrSD_DaY98aR9kMBjVg","value":{"type":"Integer","value":42},"name":{"value":"WebSocket TTL Test","type":"String"},"created_at":{"type":"Integer","value":1782493200020},"ttl":"2026-06-26T18:00:00.147957Z"}]}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

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
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup function...
✓ Edge cache function created: uIKHXfKUvZzhvMKJLw0Ax-lnnvklTuIf1BH3uyL2yridMlntbX4rLt889qpeEt0pkmTicRBc-NJP5h3kLo1h5g

Call 1: Cache lookup
Found 1 cached entries
Response time: 16ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 8ms

🧹 Cleaning up...
✓ Cleanup complete

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

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
🚀 ekoDB Functions Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: S6pH7HIAHKREfPI_pjyHhueeW3gB2EpWC1KcRozZa7dpjJnzfpi5s94TqOkywbhfsCZxbc5DakJRNnCAwtjOvg
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 3 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"value":60.0,"type":"Float"},"status":{"value":"null","type":"String"}}
   {"avg_score":{"type":"Float","value":50.0},"status":{"type":"String","value":"inactive"},"count":{"value":5,"type":"Integer"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":60.0,"type":"Float"},"status":{"type":"String","value":"active"}}
⏱️  Execution time: 0ms

📝 Example 4: function Management

📋 Total functions: 16
🔍 Retrieved function: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

ℹ️  Function 'count_users' already existed — updated instead
✅ Count function saved
📊 Total user count: {"type":"Integer","value":15}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

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
🚀 ekoDB Kotlin Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 479ZWPfyBf-ie73TEVF_qBRnliDkfF1hf7Naw5VSESX1ukJb_mIWMBCv3BEC1obXsTAPvJusi8MPPvhL9ARjpw
📊 Found 2 category groups

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed with 1 results

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)

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
🚀 ekoDB Kotlin KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within functions
   • Combined wrapped types + KV workflows

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: StringValue(value=Kby30zDLN_1k4hevxPDFO5s1cMKB2YntlVFlnOnpmeLIokqOY-VKX5Ax-ZnqREHP4yUp9IGJ3Qhn--jyJxTv4w)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: KF22Tpy8Rkfzf-vf4K3TkttCbUOWigfMUqYP7jfe6j1b-iOBRQcRJlO8KEXQWK5nJPE0GV-A2rY2M2FEjewNvw
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: mBzJISR9AZ7WcoN1m4koiEIELvBd4lgDzyT-jApNNoZR1LVzWcbd4wyabjQ263izRNfvls77ChhS8KwWxrtWWQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: tfZhtxrOK9Uwx0cqxDs0tnT0VaxEc3cXQ0JpSszgVvaUrIvyXuqVX2fwHgK_BN0lj7rsUoXLjA3D4tSlOfJi4Q
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field* helpers for type-safe wrapped values
   ✅ fieldDecimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ FunctionStageConfig.Kv* classes work within functions

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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Setting up cache collection...
✓ Cache entry created

Step 2: Create SWR cache lookup function
✓ Created SWR function: swr_cache_lookup_kt (DeQvnVq9h6qFbIf9Xs0QHp3oVbirbLAb2Go-1ZBLGpJk4jxRH8n8iuBfUG23kGaZroRl9iG00WTNzUD5IyjYfg)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 8ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation

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
🚀 ekoDB Kotlin Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR function: github_user_native (WRjhuDs220Uy5qQUN8FBdDlaONq1dVKNpqd-ptMgCzIrQP5MBm_LOWZZrG6ke1TBGsYvaoqGJxlSleoUN6XbJg)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 3.2x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit (mQkPu9QANjfTcvs6mzfIItTYAthxUewh43EHNFMzQORmPbQidRw2CSTjzEsP6GKtsdKssARnkrNLDvTJNvKCIw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (6BGGUXOiMWJNVqsjxA5ummyRyEJbR_D8CoW-Ji7eLoyb0MqMDTthohKCVUImWJDYMZyLGahpNWf4uEJRicZWfA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache (4m-folNqmMUKJVy6STYCp0DAoD4t5RRJ0ldX4qxBm6TFmAwxv5s7nLVWMhQt1-m3JDYui4VtcDefWMm0LfsM-g)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test functions

✅ All examples completed!

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
🚀 ekoDB Kotlin Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"avg_price":{"value":367.0,"type":"Float"},"category":{"value":"Electronics","type":"String"},"count":{"value":5,"type":"Integer"}}
   {"avg_price":{"type":"Float","value":365.6666666666667},"count":{"type":"Integer","value":3},"category":{"value":"Furniture","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

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
🚀 ekoDB Kotlin AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   {"tokens_used":{"type":"Integer","value":265},"response":{"value":"Vector databases offer several benefits, including:\n\n1. **Efficient Similarity Search**: They enable fast retrieval of similar items using vector embeddings, which is ideal for applications like recommendation systems and image search.\n\n2. **Scalability**: Designed to handle large volumes of high-dimensional data, vector databases can efficiently scale with growing datasets.\n\n3. **High Performance**: Optimized for similarity searches and nearest neighbor queries, providing quick responses even for complex queries.\n\n4. **Semantic Understanding**: Capable of processing and understanding nuanced relationships in data, enhancing tasks like natural language processing and image recognition.\n\n5. **Support for Machine Learning**: They facilitate the integration of ML models, allowing for easy storage and querying of model outputs.\n\n6. **Multimodal Data Handling**: Can efficiently manage different types of data, such as text, audio, and images, by representing them in vector form.\n\n7. **Flexible Data Ingestion**: Support various methods for data input, making it easier to update and manage the database.\n\n8. **Rich Querying Capabilities**: Allow for advanced querying options based on vector similarity, improving user experience and data insight.","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed function saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI function examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text

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
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   {"status":{"type":"String","value":"active"},"count":{"type":"Integer","value":7}}
   {"status":{"type":"String","value":"inactive"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

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
🚀 ekoDB Kotlin Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. {"value":"Natural Language Processing","type":"String"} ({"type":"String","value":"AI"})
   2. {"value":"Database Design Principles","type":"String"} ({"type":"String","value":"Database"})
   3. {"value":"Getting Started with ekoDB","type":"String"} ({"value":"Database","type":"String"})
   4. {"value":"Introduction to Machine Learning","type":"String"} ({"type":"String","value":"AI"})
   5. {"type":"String","value":"Vector Databases Explained"} ({"value":"Database","type":"String"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"AI"},"count":{"type":"Integer","value":2}}
   {"category":{"type":"String","value":"Database"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

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
✓ Created session: LwPPlE9e3hVrs1nmTWL6mfEV2IBr9QQSMFS4z0IF7OeHxy15o2MANZvZVt353_PS6JoNt--5vV91UO9WbcoAZg

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "R-9YFKHrg5NzSTECoBKJRLG4ypGBfp7eBDjoYmK7Us80ApIL7shMjW1tQf9N39XZMb3zBaTuucoBBo0luxql0w"
  Responses: ["ekoDB is a high-performance database that integrates intelligent caching, real-time capabilities, and AI functionality. It is designed to support various features that enhance the user experience and application performance.\n\n### Key Features of ekoDB:\n1. **AI Chat Integration**: The chat feature allows users to query the database using natural language, providing AI-powered responses with relevant context.\n   \n2. **Search Capabilities**:\n   - **Full-Text Search**: Utilizes keyword matching to retrieve relevant content.\n   - **Vector Search**: Leverages semantic search through embeddings to find semantically relevant records, even without shared keywords.\n   - **Hybrid Search**: Combines both full-text and vector search capabilities, automatically retrieving context to improve search results.\n\n3. **Performance Optimizations**: ekoDB includes mechanisms for intelligent caching and is optimized for high-performance operations to ensure quick data retrieval and processing.\n\nThese features make ekoDB a versatile choice for applications requiring efficient data handling and integration of AI technologies."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 36s
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
✓ Created session: TlQi_VN6sI1-Y3yna1MFn3ZrU9Le0WbHOWJGgvh9PJMbHN8JdmYnoZQ-XMcGpCivmRSYsPX5FFMHT5llfQQyYA

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The available product is:\n\n- **Name:** ekoDB\n- **Description:** High-performance database product\n- **Price:** $99\n\nIf you have any further questions or need more details, feel free to ask!"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The price of ekoDB is $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: 9kV5S1XMdel6NsBju3nN2zarEmDQMLtQjN7EkWTyJ_w2AJdCKGx-ZoSH2yZ-xPDIwq1ZA7uTgDFQbJyjDtAusw

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 19s
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
✓ Created session: OxK4JsSEPgeXrsvn4BYl05hvXVAuBMMspULZxMmRgM8voJ6PaRSnYlqZunbturULV5qTSyIdeYIw_dhl-ssQow

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you have any other questions or need more information, feel free to ask!"]

✓ Message 2 sent
  Responses: ["The price of the product ekoDB is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"OxK4JsSEPgeXrsvn4BYl05hvXVAuBMMspULZxMmRgM8voJ6PaRSnYlqZunbturULV5qTSyIdeYIw_dhl-ssQow"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"MrbCo8eTQjX1Q_tbm_aHz9WjAOYm8i9aM3hFfusRMCsRlydFbeotyHecmDS34xyVwdF1GRZFS-P9tQ6p5x0CCA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-06-26T17:02:12.211150+00:00"},"id":"z9ySp70h9BfEZU4ZHR90s9pC2lvSe1x4qlD2nzkCuI7QvtlnVSnP26YAYxGn9p-cCIHmsB_PbsqYmbjiOm-Xcw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":65,"prompt_tokens":6740,"total_tokens":6805}},"updated_at":{"type":"DateTime","value":"2026-06-26T17:02:12.211150+00:00"}},{"chat_id":{"type":"String","value":"OxK4JsSEPgeXrsvn4BYl05hvXVAuBMMspULZxMmRgM8voJ6PaRSnYlqZunbturULV5qTSyIdeYIw_dhl-ssQow"},"content":{"type":"String","value":"The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you have any other questions or need more information, feel free to ask!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"MrbCo8eTQjX1Q_tbm_aHz9WjAOYm8i9aM3hFfusRMCsRlydFbeotyHecmDS34xyVwdF1GRZFS-P9tQ6p5x0CCA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-06-26T17:02:12.229263+00:00"},"id":"cb5mSqxY5pqTvx8LBis3Wb6AMOWVEcOuKOcsZBkNbSkwSFeZ8PNner8Qob7QAaVuAu4P0py-K22pohyNcXACSg","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":65,"prompt_tokens":6740,"total_tokens":6805}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example"},"id":"call_weqo62hudAP1XaZJFzXzlCNQ","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"MrbCo8eTQjX1Q_tbm_aHz9WjAOYm8i9aM3hFfusRMCsRlydFbeotyHecmDS34xyVwdF1GRZFS-P9tQ6p5x0CCA","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_weqo62hudAP1XaZJFzXzlCNQ","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-06-26T17:02:12.229263+00:00"}},{"chat_id":{"type":"String","value":"OxK4JsSEPgeXrsvn4BYl05hvXVAuBMMspULZxMmRgM8voJ6PaRSnYlqZunbturULV5qTSyIdeYIw_dhl-ssQow"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"MrbCo8eTQjX1Q_tbm_aHz9WjAOYm8i9aM3hFfusRMCsRlydFbeotyHecmDS34xyVwdF1GRZFS-P9tQ6p5x0CCA","price":99,"product":"ekoDB"},"score":2.0999999999999996}]},"created_at":{"type":"DateTime","value":"2026-06-26T17:02:19.225806+00:00"},"id":"HVwQMFU3q-JCf2rs8ea5e-F2Yl4kJVDy4QB7l1tgzNAt_EtDwjD0-nvVZ8rg0P50RjqW_9Opgj8vIIEp1Ffg7g","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":9,"prompt_tokens":3386,"total_tokens":3395}},"updated_at":{"type":"DateTime","value":"2026-06-26T17:02:19.225806+00:00"}},{"chat_id":{"type":"String","value":"OxK4JsSEPgeXrsvn4BYl05hvXVAuBMMspULZxMmRgM8voJ6PaRSnYlqZunbturULV5qTSyIdeYIw_dhl-ssQow"},"content":{"type":"String","value":"The price of the product ekoDB is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"MrbCo8eTQjX1Q_tbm_aHz9WjAOYm8i9aM3hFfusRMCsRlydFbeotyHecmDS34xyVwdF1GRZFS-P9tQ6p5x0CCA","price":99,"product":"ekoDB"},"score":2.0999999999999996}]},"created_at":{"type":"DateTime","value":"2026-06-26T17:02:19.260459+00:00"},"id":"0-kO1m4YEXMPb876qNu7xTHzT_gBt2y16WlcLIEzMHay2tKXuqMHrBJN3qLr-rSBi5U1xADOwqDqBNCX2OcuHw","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":9,"prompt_tokens":3386,"total_tokens":3395}},"updated_at":{"type":"DateTime","value":"2026-06-26T17:02:19.260459+00:00"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: -as2Cj_AyAuVdvZ83KNS6ZzQAIcVT7Vv23kVOV5TMNId3yzMNpm00mxVd0NfE9yUkICJ_rY3F-hJfWfRyYTuRA

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 24s
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
=== ekoDB Kotlin Client - Chat Models Example ===

=== List Chat Models ===
Available chat models by provider:
  openai:
    - text-embedding-ada-002
    - whisper-1
    - gpt-3.5-turbo
    - tts-1
    - gpt-3.5-turbo-16k
    - gpt-4-0613
    - gpt-4
    - davinci-002
    - babbage-002
    - gpt-3.5-turbo-instruct
    - gpt-3.5-turbo-instruct-0914
    - gpt-3.5-turbo-1106
    - tts-1-hd
    - tts-1-1106
    - tts-1-hd-1106
    - text-embedding-3-small
    - text-embedding-3-large
    - gpt-3.5-turbo-0125
    - gpt-4-turbo
    - gpt-4-turbo-2024-04-09
    - gpt-4o
    - gpt-4o-2024-05-13
    - gpt-4o-mini-2024-07-18
    - gpt-4o-mini
    - gpt-4o-2024-08-06
    - omni-moderation-latest
    - omni-moderation-2024-09-26
    - o1-2024-12-17
    - o1
    - o3-mini
    - o3-mini-2025-01-31
    - gpt-4o-2024-11-20
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-search-preview
    - gpt-4o-transcribe
    - gpt-4o-mini-transcribe
    - o1-pro-2025-03-19
    - o1-pro
    - gpt-4o-mini-tts
    - o3-2025-04-16
    - o4-mini-2025-04-16
    - o3
    - o4-mini
    - gpt-4.1-2025-04-14
    - gpt-4.1
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-mini
    - gpt-4.1-nano-2025-04-14
    - gpt-4.1-nano
    - gpt-image-1
    - o4-mini-deep-research
    - gpt-4o-transcribe-diarize
    - o4-mini-deep-research-2025-06-26
    - gpt-5-chat-latest
    - gpt-5-2025-08-07
    - gpt-5
    - gpt-5-mini-2025-08-07
    - gpt-5-mini
    - gpt-5-nano-2025-08-07
    - gpt-5-nano
    - gpt-audio-2025-08-28
    - gpt-realtime
    - gpt-realtime-2025-08-28
    - gpt-audio
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-5-pro-2025-10-06
    - gpt-5-pro
    - gpt-audio-mini
    - gpt-audio-mini-2025-10-06
    - gpt-5-search-api
    - gpt-realtime-mini
    - gpt-realtime-mini-2025-10-06
    - sora-2
    - sora-2-pro
    - gpt-5-search-api-2025-10-14
    - gpt-5.1-chat-latest
    - gpt-5.1-2025-11-13
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-5.1-codex-mini
    - gpt-5.1-codex-max
    - gpt-image-1.5
    - gpt-5.2-2025-12-11
    - gpt-5.2
    - gpt-5.2-pro-2025-12-11
    - gpt-5.2-pro
    - gpt-5.2-chat-latest
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-4o-mini-tts-2025-03-20
    - gpt-4o-mini-tts-2025-12-15
    - gpt-realtime-mini-2025-12-15
    - gpt-audio-mini-2025-12-15
    - chatgpt-image-latest
    - gpt-5.2-codex
    - gpt-5.3-codex
    - gpt-realtime-1.5
    - gpt-audio-1.5
    - gpt-4o-search-preview
    - gpt-4o-search-preview-2025-03-11
    - gpt-5.3-chat-latest
    - gpt-5.4-2026-03-05
    - gpt-5.4-pro
    - gpt-5.4-pro-2026-03-05
    - gpt-5.4
    - gpt-5.4-nano-2026-03-17
    - gpt-5.4-nano
    - gpt-5.4-mini-2026-03-17
    - gpt-5.4-mini
    - gpt-image-2
    - gpt-image-2-2026-04-21
    - gpt-5.5
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-5.5-pro-2026-04-23
    - chat-latest
    - gpt-realtime-translate
    - gpt-realtime-2
    - gpt-realtime-whisper
  anthropic:
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get OpenAI Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper

=== Get Anthropic Models ===
Anthropic models: claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

=== Chat Models Example Complete ===

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
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - User Functions Example ===

=== Create User Function ===
Created user function with ID: 3QRfA-G2ex485sl0Rsg9lOMwlTSkpw40oGqt_oFbPkBDb8wYoXjyvNOHLxzgWuxU1res6FTdfwfxrPq3JUYi1Q

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 11 user functions:
  - "fetch_user": "Fetch user by code"
  - "fetch_api_user": "Fetch User with Cache"
  - "fetch_slim_user": "Validate and slim down user"
  - "cache_api_call_js": "Cache External API Call"
  - "cache_api_call": "Cache External API Call"

=== List User Functions by Tag ===
Found 11 user functions with 'kotlin' tag:
  - "fetch_user"
  - "fetch_api_user"
  - "fetch_slim_user"
  - "cache_api_call_js"
  - "cache_api_call"
  - "get_active_users_kt"
  - "fetch_product_with_reviews"
  - "swr_user"
  - "validate_user"
  - "cache_api_call_py"
  - "fetch_and_store_user"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

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
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: EkoRecord(fields={id=StringValue(value=fmUocBmz-eux3-UCP3w9_YPLMpwOctZC41EpH5B70hrMuJZDduHcRBfIDUS7div4CQCqGGJ59F8BfZW22GTvJQ)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), id=StringValue(value=fmUocBmz-eux3-UCP3w9_YPLMpwOctZC41EpH5B70hrMuJZDduHcRBfIDUS7div4CQCqGGJ59F8BfZW22GTvJQ), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=bNTIz9bk2eEWT2AqeYRgoBfa5E1pZK0WttAWxSpQcVcTFODKOl4QgxYKSDRWZWOW4m25P8Y8OVJPQCrsNQKOLA)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=fmUocBmz-eux3-UCP3w9_YPLMpwOctZC41EpH5B70hrMuJZDduHcRBfIDUS7div4CQCqGGJ59F8BfZW22GTvJQ), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!

BUILD SUCCESSFUL in 11s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== Bypass Ripple Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
1. Basic insert (ripple enabled):
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=qgTG9Ua8tObf4nNzuA7Z22j6c4X4qIzoEWat94WuemMGS_vbhZECgb0sjvOeCZLwfuMhkDBirhoZDRU9R1nBdw)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=XcrADqX53hEydI1YSkmDDxNCBrMNV1NxtpSegYHo0qQeRIA9WthhLQNpWl-Hp2S2dwn8AiwOFTdJu4itbBvp3Q)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), id=StringValue(value=qgTG9Ua8tObf4nNzuA7Z22j6c4X4qIzoEWat94WuemMGS_vbhZECgb0sjvOeCZLwfuMhkDBirhoZDRU9R1nBdw)})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=qgTG9Ua8tObf4nNzuA7Z22j6c4X4qIzoEWat94WuemMGS_vbhZECgb0sjvOeCZLwfuMhkDBirhoZDRU9R1nBdw), price=ObjectValue(value={value=IntegerValue(value=500), type=StringValue(value=Integer)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)})})

✅ All bypass_ripple operations completed successfully!

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
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: true

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: Alice Johnson

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 3 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!

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
✓ Client created
✓ kt_users_register saved
✓ kt_users_login saved
✓ kt_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/kt_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions

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
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions

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
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions

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
✓ Client created
✓ kt_route_admin saved
✓ kt_route_user_by_id saved
✓ kt_route_user_posts saved
✓ kt_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: bwC3lLS379PlMeFvV1-tTNAXwxsaFH3hCxw_Ar-1BvwBehoyaufMjmWiG7qxLxiNsUXvb3qgKJdMt7LsQGbo6g
Created Bob: $500 - ID: laaZfryHZl-yyZxJZueQDZ-CL4V6l4By9Vn0onvLJdulOxOWPexep1U8ZMH_PYu4k_A2cSeSGfmuc2O3IUvgaQ

=== Example 1: Begin Transaction ===
Transaction ID: edd396fb-07b6-4392-9942-c347b8014737

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 212db9b0-db9c-4279-96e8-427aa565bf7f
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
