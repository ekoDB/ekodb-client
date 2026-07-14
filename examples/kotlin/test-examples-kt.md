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
> Task :loadKtlintReporters UP-TO-DATE
> Task :runKtlintCheckOverKotlinScripts UP-TO-DATE
> Task :ktlintKotlinScriptCheck UP-TO-DATE
> Task :runKtlintCheckOverMainSourceSet UP-TO-DATE
> Task :ktlintMainSourceSetCheck UP-TO-DATE
> Task :runKtlintCheckOverTestSourceSet UP-TO-DATE
> Task :ktlintTestSourceSetCheck UP-TO-DATE
> Task :compileTestKotlin UP-TO-DATE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test UP-TO-DATE
> Task :check UP-TO-DATE
> Task :build UP-TO-DATE

BUILD SUCCESSFUL in 4s
11 actionable tasks: 11 up-to-date
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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=j88PAIf1KFoLlD5h5huYLK2EncRhoPO9EYpkjqdQ2G8qKIjN-CZd15wk2kwZBVKrA3V7Yq0bnZ9jykQE5UXfNg)})
  User ID: j88PAIf1KFoLlD5h5huYLK2EncRhoPO9EYpkjqdQ2G8qKIjN-CZd15wk2kwZBVKrA3V7Yq0bnZ9jykQE5UXfNg

=== Read ===
✓ Found user by ID: EkoRecord(fields={created_at=ObjectValue(value={value=StringValue(value=Tue Jul 14 09:53:24 EDT 2026), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), id=StringValue(value=j88PAIf1KFoLlD5h5huYLK2EncRhoPO9EYpkjqdQ2G8qKIjN-CZd15wk2kwZBVKrA3V7Yq0bnZ9jykQE5UXfNg), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Tue Jul 14 09:53:24 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {nested={deep=true}, key=value}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: created_at, active, embedding, email, user_id, id, price, metadata, data, categories, age, tags, name

=== Update ===
✓ Updated user: EkoRecord(fields={embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), created_at=ObjectValue(value={value=StringValue(value=Tue Jul 14 09:53:24 EDT 2026), type=StringValue(value=String)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), id=StringValue(value=j88PAIf1KFoLlD5h5huYLK2EncRhoPO9EYpkjqdQ2G8qKIjN-CZd15wk2kwZBVKrA3V7Yq0bnZ9jykQE5UXfNg), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Tue Jul 14 09:53:24 EDT 2026)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), id=StringValue(value=j88PAIf1KFoLlD5h5huYLK2EncRhoPO9EYpkjqdQ2G8qKIjN-CZd15wk2kwZBVKrA3V7Yq0bnZ9jykQE5UXfNg), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)})})

=== Delete ===
✓ Deleted user with ID: j88PAIf1KFoLlD5h5huYLK2EncRhoPO9EYpkjqdQ2G8qKIjN-CZd15wk2kwZBVKrA3V7Yq0bnZ9jykQE5UXfNg

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
  {"data":[{"id":"YUhq6uv_q7XgIphI693_4V3VtIveThr9s1nUCVcXJTqDVC2QWTqmLrhrUfELh4Ci2ORZPJkFp5x7GJB5c-ICMA","name":{"type":"String","value":"Test User"},"status":{"value":"active","type":"String"}}]}

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
  IDs: 1pdbqqCf_q_tzSofZhQGAiQQHsJHGqIhiyBPOIkm44Q0klX-iQ9UA19Z7NCvsUtk3sxPRaI-fUzwg6cxaSCLaw, uAj1Rm8wRSANqkVULdm40bHkl8Qs3d9eOWuHwJa3CCFzSBCv1bmQLyVELbIS5ZRAhFQQU0i51NLE4BIdEKcHeA, sHT6weCxefPql_Z2gOs2ciXLyy96O7UtAfdg1U7vjIT_UB6SSMJmVg93zxawpoNdwjJPYP35R5BbZaFcUx6KuQ...

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
  - schema_documents_client_ts
  - audit__ek0_testing
  - schema_employees_client_ts
  - functions__ek0_testing
  - schema_users_client_go

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-07-14T13:53:42.628210Z","last_modified":"2026-07-14T13:53:42.628210Z","bypass_ripple":false,"primary_key_alias":"id"}

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
✓ Retrieved value: {"type":"Object","value":{"role":"admin","name":"Alice","email":"alice@example.com"}}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"type":"Object","value":{"created_at":1784037227412,"user_id":"123"}}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  config:db: {"value":{"host":"localhost","port":5432}}
  config:cache: {"value":{"ttl":3600,"enabled":true}}
  config:api: {"value":{"timeout":30,"retries":3}}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 16

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
Created Alice: $1000 - ID: y6UbY66rlrtLhNNWODeWsaGo2fG_XzrYii5A9xUGi5snMF0ubFt0LUGB8WbXH7KO_pe2_MuBrhkyZkcaIjKFNA
Created Bob: $500 - ID: 7cz6fQv2QBlakGU1Ngi7e7plY6Vl9hKoyqamPA4qgfFt461GNR7HIg5u-9Im-kE8OSl1LA7hY8uuFYAnVc_aqA

=== Example 1: Begin Transaction ===
Transaction ID: 5f8033fb-d775-4047-a6d7-b957bb9737f4

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 8b511d22-e749-425e-b09c-91a71a6881ae
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
  - Score: ObjectValue(value={value=IntegerValue(value=95), type=StringValue(value=Integer)})
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
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"id":"NCernqvzZxGziAZXlxzwUodyUbiXHel1-Qlyz9t5dH6vqQVHGn1V7mub9yxvVYmExYo0kmE-dRc1cP0edv53-A","category":{"type":"String","value":"programming"},"title":{"type":"String","value":"Rust Programming"},"views":{"value":444,"type":"Integer"},"description":{"type":"String","value":"Learn Rust programming language with hands-on examples and best practices."},"tags":{"value":"programming,rust,tutorial","type":"String"}},"score":26.400000000000002,"matched_fields":["title","tags","description","category"]},{"record":{"description":{"value":"Build modern web applications using JavaScript, React, and Node.js.","type":"String"},"id":"ayFdIz0bIu0037m_p9G_FikztxRvjUYs2M81Php3l1JbYoEUwjbopX8qMbH2UXsP7raJ6W2LPYS4ppyylbzfaQ","category":{"type":"String","value":"programming"},"title":{"value":"JavaScript Web Development","type":"String"},"tags":{"type":"String","value":"programming,javascript,web"},"views":{"type":"Integer","value":378}},"score":13.200000000000001,"matched_fields":["tags","category"]},{"record":{"category":{"type":"String","value":"programming"},"title":{"value":"Python for Data Science","type":"String"},"tags":{"type":"String","value":"programming,python,data-science"},"id":"3o3a0g-Idt2rFR82AVg0FqnJCXARRFUT09FhFvD-9ByrzqXDH-ALV8wO-bGaIc1r70wxN06PFSSE4I65aybSWw","views":{"type":"Integer","value":635},"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."}},"score":13.200000000000001,"matched_fields":["tags","category"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"tags":{"type":"String","value":"ai,machine-learning,python"},"id":"OePcrYuIRVTtoGFnhJVDwKhn8nVGOCRk5hGywvmhvJlrbM9NBJgH62HF_9OxPm6PnRcWk4J_gSavxCcvPnHyOw","title":{"type":"String","value":"Machine Learning Basics"},"views":{"type":"Integer","value":107},"description":{"type":"String","value":"Introduction to machine learning algorithms and neural networks."},"category":{"value":"ai","type":"String"}},"score":2.7,"matched_fields":["description","tags","title"]},{"record":{"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"category":{"type":"String","value":"programming"},"id":"3o3a0g-Idt2rFR82AVg0FqnJCXARRFUT09FhFvD-9ByrzqXDH-ALV8wO-bGaIc1r70wxN06PFSSE4I65aybSWw","title":{"value":"Python for Data Science","type":"String"},"views":{"type":"Integer","value":635},"tags":{"type":"String","value":"programming,python,data-science"}},"score":1.0,"matched_fields":["description"]},{"record":{"description":{"value":"Learn database design principles, normalization, and query optimization.","type":"String"},"id":"-muwh96yE6bJXUwi1MBta9eLe_tRICTJgz0bF-e94zgtzFg7rP97fNpSSXmoBY6equ21r06Vlp4fuaeZbVJajw","title":{"type":"String","value":"Database Design"},"category":{"value":"database","type":"String"},"views":{"type":"Integer","value":239},"tags":{"type":"String","value":"database,design,sql"}},"score":0.5,"matched_fields":["description"]},{"record":{"category":{"value":"programming","type":"String"},"views":{"value":444,"type":"Integer"},"description":{"type":"String","value":"Learn Rust programming language with hands-on examples and best practices."},"id":"NCernqvzZxGziAZXlxzwUodyUbiXHel1-Qlyz9t5dH6vqQVHGn1V7mub9yxvVYmExYo0kmE-dRc1cP0edv53-A","title":{"type":"String","value":"Rust Programming"},"tags":{"value":"programming,rust,tutorial","type":"String"}},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Search with a metadata pre-filter (category = programming) ===
✓ Found results in category 'programming' (database/ai excluded)
  {"results":[{"record":{"tags":{"type":"String","value":"programming,rust,tutorial"},"id":"NCernqvzZxGziAZXlxzwUodyUbiXHel1-Qlyz9t5dH6vqQVHGn1V7mub9yxvVYmExYo0kmE-dRc1cP0edv53-A","category":{"value":"programming","type":"String"},"title":{"type":"String","value":"Rust Programming"},"views":{"type":"Integer","value":444},"description":{"type":"String","value":"Learn Rust programming language with hands-on examples and best practices."}},"score":6.6000000000000005,"matched_fields":["description"]},{"record":{"title":{"type":"String","value":"Python for Data Science"},"category":{"type":"String","value":"programming"},"views":{"type":"Integer","value":635},"id":"3o3a0g-Idt2rFR82AVg0FqnJCXARRFUT09FhFvD-9ByrzqXDH-ALV8wO-bGaIc1r70wxN06PFSSE4I65aybSWw","tags":{"type":"String","value":"programming,python,data-science"},"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."}},"score":6.6000000000000005,"matched_fields":["description"]}],"total":2,"execution_time_ms":1}

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
✓ Inserted user 1: StringValue(value=UKxnwwCNJER1Jmf3vUzapjqWV8272JanXMwLblX2iuNcQvU8igI_LK8nNJuEHbrfH8tbPXydaATVofIF6fJxig)
✓ Inserted user 2: StringValue(value=A-A486I4jOUDOx5j77BWPeY_qNYTQttJOsFWTan7mQBYu0xnoPaBmLQRfZ1bm_adVqGmjnn863MGga-iymq6EA)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 30
  Sample: [schema_documents_client_ts, audit__ek0_testing, schema_employees_client_ts, functions__ek0_testing, schema_users_client_go]

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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

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
  Document ID: HPYbuq3qY9dF91bP-yjSoXCR1ucD3b7fJtaO4iUCQ1XRfKsvP8x44mZjqTuHp8vJNPZ59MUBCOHZ8NR-VohXXA

=== Verify Document Exists ===
✓ Document found: created_at, id, session_id, user_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: 9HilBQJ-5cKD9rQK4BRbGGkQjxs0UHms5m28jMrR2JYb_u9VndqGtDfflN6ePuxwU4O0-c2BxDT6Dslpxz_0Mw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: id, value, cache_key, ttl

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
✓ Inserted document with TTL: StringValue(value=BnHHV7XGNZhYF1kaJqF7-k1FxT20jK79XjKSvC5qSnbTbVUnDWlN3j-TvuaszV80fKUre28BeJ7rrhgz5GN53w)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"name":{"type":"String","value":"WebSocket TTL Test"},"id":"BnHHV7XGNZhYF1kaJqF7-k1FxT20jK79XjKSvC5qSnbTbVUnDWlN3j-TvuaszV80fKUre28BeJ7rrhgz5GN53w","created_at":{"type":"Integer","value":1784037266027},"value":{"value":42,"type":"Integer"},"ttl":"2026-07-14T14:54:26.143567Z"}]}

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
✓ Edge cache function created: yjF5NYsNV1sio8ITpK-U4wEo7T3Vy8IE7DmtqsB_RxnTpZHnryj_Tea6-CgqYflfZD3jrBqxkgT1u1Np5NItog

Call 1: Cache lookup
Found 1 cached entries
Response time: 17ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 5ms

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

✅ Function saved: yW2U3g99cNAkdnoGXGxJDFy6YWqXw5VBL-dPBnnGw4B6d5vl55mIiU9TKDYcQSMJUJQjZDiLNwW6sRZtfAcqaw
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 3 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":60.0},"status":{"value":"null","type":"String"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"value":50.0,"type":"Float"},"status":{"value":"inactive","type":"String"}}
   {"avg_score":{"type":"Float","value":60.0},"status":{"value":"active","type":"String"},"count":{"type":"Integer","value":5}}
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
📊 Total user count: {"value":15,"type":"Integer"}
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

✅ Function saved: sd-3_GlXDXTh5d3yBPwJg5gdP2epL8SFZ-C-Pd3Jnd4Dwf1EStc4i7dTsgCaia7v7Qw_E-r7Xxssuz030UDrEQ
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

✅ Inserted order: StringValue(value=qmnxR0Mkht2Cie6uXeYFWAmgBG5eZucmyTihtqF38KnfEzYsh-M4cQiyiNofGtMY8YKXP8rbZ8Ulboce0rbC7A)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: 0Xh0RodgIJrRlzSMHZfHhZqA5hLHx4Yw7B96Hwtkb0sMzBBFfISeJvbuKj7f-qK-NwgbZqSXfMLxoqjXqnjeXQ
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: -gP6xcartDgZ8tHr3WPqvzFotxZeB3l7KxMvECaPWRlv-ElvO2yHBqnjC-EAiuOD_jlOvP2f1QTEP3K1k-7iWg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: LIMnmUh91uZICnqkF9WSPsD-86zMTEzKG5H5FZZdRA63YXu_Ag7OujskrqhV5UksTpuoz7NlBZBpL4UGbH66VA
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
✓ Created SWR function: swr_cache_lookup_kt (xsNkq_4O1eg-Aa7RwBXN1lHT0xum96Cmvc0f5FEfWGi-SxQyFuJVgAoKlHyJ-gaOaje7bjmAeQ2QBGwPVkgP9g)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 4ms (served from cache)
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
✓ Created native SWR function: github_user_native (ccgWIj2xau_06n5EJmO_0zNj8M0C7qk6amSgVC8QFZ8jpCQT53TNMgf4CEY713MjDGTTCvtBL8lNlfpwrZ4xlQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 17ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 3.4x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit (0VV1APehhy1M1JJTkiI2RcYImMR-Nm4veAsb6gGhe3E3r5ewRsbv9cNYCZWqKJwT19x-qmGMoy6U8q0e-7uPkg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (3Wh_YeMWJlHUqcUtMhitA_nx5P_J-9kOcg1WO69z0Rb6NiusgF--LVRIFZz95cZEhl_dTTpOySl0mCtckaVk2g)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache (IsN3MWdAUd_zqCK5-8GXuUk-FOMlOl98k4QFRUwG-FASwvAyzbMYEsBhyM-f-ycAN09viC5GWBbJXuQZLsXJ3Q)
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
   {"avg_price":{"value":367.0,"type":"Float"},"count":{"value":5,"type":"Integer"},"category":{"type":"String","value":"Electronics"}}
   {"avg_price":{"type":"Float","value":365.6666666666667},"category":{"type":"String","value":"Furniture"},"count":{"value":3,"type":"Integer"}}
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
   {"response":{"type":"String","value":"Vector databases offer several benefits:\n\n1. **Efficient Similarity Search**: They enable quick retrieval of similar items based on vector embeddings, which is crucial for tasks like image and text similarity.\n\n2. **High-Dimensional Data Handling**: Designed to manage and search high-dimensional data efficiently, making them suitable for applications in machine learning and AI.\n\n3. **Scalability**: Vector databases can scale to accommodate large datasets while maintaining performance.\n\n4. **Advanced Query Capabilities**: Support for complex queries, including nearest neighbor searches and range queries, enhancing data exploration.\n\n5. **Integration with ML Workflows**: Seamlessly integrates with machine learning pipelines, allowing easy storage and retrieval of feature vectors.\n\n6. **Real-Time Processing**: Many vector databases are optimized for real-time processing, which is beneficial for applications requiring immediate results.\n\n7. **Rich Metadata Support**: They can store and manage metadata associated with vectors, facilitating better organization and retrieval.\n\n8. **Cross-Modal Search**: Enables searching across different data types (e.g., images and text) using a unified approach.\n\nThese advantages make vector databases particularly suitable for modern applications in AI, natural language processing, computer vision, and recommendation systems."},"tokens_used":{"type":"Integer","value":276}}
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
   {"count":{"type":"Integer","value":7},"status":{"type":"String","value":"active"}}
   {"status":{"type":"String","value":"inactive"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

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
🚀 ekoDB Kotlin Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. {"type":"String","value":"Getting Started with ekoDB"} ({"value":"Database","type":"String"})
   2. {"value":"Database Design Principles","type":"String"} ({"type":"String","value":"Database"})
   3. {"type":"String","value":"Introduction to Machine Learning"} ({"type":"String","value":"AI"})
   4. {"value":"Vector Databases Explained","type":"String"} ({"value":"Database","type":"String"})
   5. {"type":"String","value":"Natural Language Processing"} ({"value":"AI","type":"String"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"type":"Integer","value":2},"category":{"value":"AI","type":"String"}}
   {"count":{"type":"Integer","value":3},"category":{"value":"Database","type":"String"}}
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
✓ Created session: HIrtbRjj8G1_KMTM5JUCc-NBioJirvXjCVb6_PIi1vqVExK3UiFNLCYqGf-pU9T5wD7Ph253Q8yqPN_tYU8GhA

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "7JEOc-ge9Zu3tLKnrm0Dhcv-FufVPjp6YV1CbHIMt9ooRfNntD92IaLpAfUGnB_jmjDI4exdwVg3-LOG-H2oqw"
  Responses: ["ekoDB is a high-performance database that offers several advanced features. Here’s a summary of what it is and its key features:\n\n### What is ekoDB?\nekoDB is a cutting-edge database that incorporates intelligent caching, real-time capabilities, and seamless AI integration, making it suitable for various applications that require fast and efficient data handling.\n\n### Key Features of ekoDB:\n1. **AI Chat Integration**: The chat feature allows you to query your database using natural language, providing AI-powered responses along with relevant context.\n   \n2. **Search Capabilities**:\n   - **Full-Text Search**: Enables searching through text data effectively.\n   - **Vector Search**: Supports searching using vector embeddings to find semantically similar records.\n   - **Hybrid Search**: Combines both keyword matching and semantic similarities, allowing comprehensive search functionalities with automatic context retrieval.\n\nThese features highlight ekoDB's capabilities in providing a flexible and intelligent database solution suitable for modern application needs."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 16s
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
✓ Created session: -jJwAnIi1AZSGjyMGO1btiCfbjF_SbOMq6i_0gl0qkiTViwbEcqvcjNEbRPh3TwuYaZ6jhE8P1y-bP9iTaVzFQ

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The available product is:\n\n- **Name**: ekoDB\n- **Description**: High-performance database product\n- **Price**: $99\n\nIf you need more information or details about additional products, feel free to ask!"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The price of ekoDB is **$99**."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: 7jBMjzIN-NBjOs_F8ccm2kauSUUiEno-A_nYY0le8bvTwrEbpNawN6OOC_vnoJ3oZwraSIVY1MH3pQXhfna68Q

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 23s
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
✓ Created session: jXTuTQRG3cNIElfrhhYl40aMsfkBFoJfPT_v4dkwmlxWRc1Nid44e-Y-CMbzCWqvm9wx-lpbH-tuqYVqDg6K_g

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product**: ekoDB\n  - **Description**: A high-performance database product with AI capabilities\n  - **Price**: $99\n\nIf you need more information or have any other questions, feel free to ask!"]

✓ Message 2 sent
  Responses: ["The price of the product ekoDB is **$99**."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"jXTuTQRG3cNIElfrhhYl40aMsfkBFoJfPT_v4dkwmlxWRc1Nid44e-Y-CMbzCWqvm9wx-lpbH-tuqYVqDg6K_g"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"OX_Gk3g1YanMHiRXETVNMFykXrNPCYw31VZ3aTT8Y7UOXJHPYcGW7VDZVIQd7mpvIkjJB95cgXQE3yLsiv6Mzw","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-07-14T13:56:23.132806+00:00"},"id":"G_Q7kREHvQACT4IE_1yjuyaGkRnp2uHxkvw4aaXq9Cx7td1QV5h3vg4FjS-Il9Eh2eA6_CFMoulJYTrEbdTFyg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":109,"prompt_tokens":10262,"total_tokens":10371}},"updated_at":{"type":"DateTime","value":"2026-07-14T13:56:23.132806+00:00"}},{"chat_id":{"type":"String","value":"jXTuTQRG3cNIElfrhhYl40aMsfkBFoJfPT_v4dkwmlxWRc1Nid44e-Y-CMbzCWqvm9wx-lpbH-tuqYVqDg6K_g"},"content":{"type":"String","value":"The available product is:\n\n- **Product**: ekoDB\n  - **Description**: A high-performance database product with AI capabilities\n  - **Price**: $99\n\nIf you need more information or have any other questions, feel free to ask!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"OX_Gk3g1YanMHiRXETVNMFykXrNPCYw31VZ3aTT8Y7UOXJHPYcGW7VDZVIQd7mpvIkjJB95cgXQE3yLsiv6Mzw","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-07-14T13:56:23.148017+00:00"},"id":"b1ZesUkR_vwQbbpdkzixAsHDNd0bkcEQQ8tc81pw-mMe8Mynq9NsK77LORZjscvHv5sPYG_1zlFUZYxc4mgoTg","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":109,"prompt_tokens":10262,"total_tokens":10371}},"tool_call_count":{"type":"Number","value":3},"tool_call_history":{"type":"Object","value":{"iterations":3,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","filter":{},"sort":[{"ascending":true,"field":"price"}]},"id":"call_gRZfFvccN9nuB3xAYvriJ8NV","name":"query_collection"},{"arguments":{"collection":"kotlin_chat_sessions_example","filter":{"content":{"expressions":[],"operator":"And"},"type":"Logical"}},"id":"call_MWNLnLrOWTPaaU8sJ7DavGf2","name":"query_collection"}],"tool_results":[{"error":"Invalid filter format. Canonical (preferred): {\"type\": \"Condition\", \"content\": {\"field\":..., \"operator\":..., \"value\":...}} OR {\"type\": \"Logical\", \"content\": {\"operator\": \"And|Or|Not\", \"expressions\": [...]}}. Flat (back-compat): {\"field\":..., \"operator\":..., \"value\":...} OR {\"and\":[...]} OR {\"or\":[...]} OR {\"not\":{...}}.","result":null,"success":false,"tool_call_id":"call_gRZfFvccN9nuB3xAYvriJ8NV","tool_name":"query_collection"},{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"OX_Gk3g1YanMHiRXETVNMFykXrNPCYw31VZ3aTT8Y7UOXJHPYcGW7VDZVIQd7mpvIkjJB95cgXQE3yLsiv6Mzw","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_MWNLnLrOWTPaaU8sJ7DavGf2","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-07-14T13:56:23.148017+00:00"}},{"chat_id":{"type":"String","value":"jXTuTQRG3cNIElfrhhYl40aMsfkBFoJfPT_v4dkwmlxWRc1Nid44e-Y-CMbzCWqvm9wx-lpbH-tuqYVqDg6K_g"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"OX_Gk3g1YanMHiRXETVNMFykXrNPCYw31VZ3aTT8Y7UOXJHPYcGW7VDZVIQd7mpvIkjJB95cgXQE3yLsiv6Mzw","price":99,"product":"ekoDB"},"score":2.0999999999999996}]},"created_at":{"type":"DateTime","value":"2026-07-14T13:56:24.147273+00:00"},"id":"xSqkHOFaIKrrEXc9-_ynEVa-7AG3oh3pWipfp9PkcXkJcZenfPsBdNkHjXCM3_tPrdvX6nNDTLPSuPeqYOdswA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":10,"prompt_tokens":3387,"total_tokens":3397}},"updated_at":{"type":"DateTime","value":"2026-07-14T13:56:24.147273+00:00"}},{"chat_id":{"type":"String","value":"jXTuTQRG3cNIElfrhhYl40aMsfkBFoJfPT_v4dkwmlxWRc1Nid44e-Y-CMbzCWqvm9wx-lpbH-tuqYVqDg6K_g"},"content":{"type":"String","value":"The price of the product ekoDB is **$99**."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"OX_Gk3g1YanMHiRXETVNMFykXrNPCYw31VZ3aTT8Y7UOXJHPYcGW7VDZVIQd7mpvIkjJB95cgXQE3yLsiv6Mzw","price":99,"product":"ekoDB"},"score":2.0999999999999996}]},"created_at":{"type":"DateTime","value":"2026-07-14T13:56:24.160562+00:00"},"id":"sAnvRoeTHuYQSFgGXd8OmNy976N5pw9n8g9ziZNeHZw2h25ETjPmUA4ZtED5iGOU4mDr-UKUXUm5OhZUmaha5Q","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":10,"prompt_tokens":3387,"total_tokens":3397}},"updated_at":{"type":"DateTime","value":"2026-07-14T13:56:24.160562+00:00"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: DE-Odvp0zKC2aa8gjDHU2fv9hql5dLY88z9wyLxIG2JmPR8UeOce6ckITCPZBPB_m6MH8wJwnyXTMC0a9JvPnA

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 17s
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
    - gpt-5.6-sol
    - gpt-5.6-terra
    - gpt-5.6-luna
    - gpt-realtime-2.1
    - gpt-realtime-2.1-mini
  anthropic:
    - claude-sonnet-5
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805

=== Get OpenAI Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper, gpt-5.6-sol, gpt-5.6-terra, gpt-5.6-luna, gpt-realtime-2.1, gpt-realtime-2.1-mini

=== Get Anthropic Models ===
Anthropic models: claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805

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
Created user function with ID: gJAbG_17zdg1qHfhrU8DCIB4oR1uYmzTUToM48uVRzEvILnj2Usq2chnRHHbgMN7e0hfPZmnWXPWBTTK380ffw

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 11 user functions:
  - "swr_user": "SWR pattern for user data (KV-based)"
  - "validate_user": "Check if user exists"
  - "cache_api_call": "Cache External API Call"
  - "fetch_and_store_user": "Fetch user from API and cache in KV"
  - "get_active_users_kt": "Get Active Users (Kotlin)"

=== List User Functions by Tag ===
Found 11 user functions with 'kotlin' tag:
  - "swr_user"
  - "validate_user"
  - "cache_api_call"
  - "fetch_and_store_user"
  - "get_active_users_kt"
  - "cache_api_call_js"
  - "cache_api_call_py"
  - "fetch_slim_user"
  - "fetch_api_user"
  - "fetch_user"
  - "fetch_product_with_reviews"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

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
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: EkoRecord(fields={id=StringValue(value=CgxcJUSynirBafbd_x5BFDRsMvQN4Xi91TSOStF5ew-75gAPAVyPyHTKAcMFWDG56a5aNaLYPJh2_xcQdvRQgw)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), id=StringValue(value=CgxcJUSynirBafbd_x5BFDRsMvQN4Xi91TSOStF5ew-75gAPAVyPyHTKAcMFWDG56a5aNaLYPJh2_xcQdvRQgw), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=xhOPp0r95DvhR38JrxECztX0w9RVdvvGOVxhupkqOPNRfkA8gZMzzxxTiJAg73z9ce9OnQrK60AvTLWVtZVb_g)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=CgxcJUSynirBafbd_x5BFDRsMvQN4Xi91TSOStF5ew-75gAPAVyPyHTKAcMFWDG56a5aNaLYPJh2_xcQdvRQgw), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})
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

BUILD SUCCESSFUL in 10s
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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=S17g1FG7VWmChhynvNIDYLmVZeZZR2ibXa4llMZ_7-LRnQsw9O7xUX8xPCwJVmdK46xS0wAJopCZUppAjEoGjQ)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=cJqo149frqxoaJn9c6stNp871RYRzkt3G8eB3COuxXVDzA5MJ_Iok_POlsnybgEYPp_sDdsd2jU5W8YuKP6-8w)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), id=StringValue(value=S17g1FG7VWmChhynvNIDYLmVZeZZR2ibXa4llMZ_7-LRnQsw9O7xUX8xPCwJVmdK46xS0wAJopCZUppAjEoGjQ)})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)}), price=ObjectValue(value={value=IntegerValue(value=500), type=StringValue(value=Integer)}), id=StringValue(value=S17g1FG7VWmChhynvNIDYLmVZeZZR2ibXa4llMZ_7-LRnQsw9O7xUX8xPCwJVmdK46xS0wAJopCZUppAjEoGjQ)})

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
Created Alice: $1000 - ID: OVk_WuX3896wUoA7WI0XLWXzlsG-33QdpWb8ZXs-TtLTX51hJPg4wp5sy--94RAnjEyDepTY5w383HJ3mfmMxw
Created Bob: $500 - ID: yaBnbSJFUqRSIiCpQRD6YdAU9JN0gkocNA-IvOOPtJ-ovB3VX7oPmou7wfHq4e6OkFQ2j4p0Jr_sSKVm4Zqq0A

=== Example 1: Begin Transaction ===
Transaction ID: 2477bde2-fba6-40f3-ae71-d451053b7eea

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 210bb20c-3f3f-4b1b-84b5-4146ce909016
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
