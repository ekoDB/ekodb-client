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

BUILD SUCCESSFUL in 3s
4 actionable tasks: 4 up-to-date
✅ [32mKotlin client built![0m
🧪 [36mRunning Kotlin client library examples...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple CRUD Example ===

=== Create ===
✓ Inserted user: Record(fields={id=StringValue(value=fJi3RDyc4uo6Lg4kcbj69--JCiXwKrWd2Jmpev-orc0pY9XyFIvwb3AfYvawtZIaZS2hNFyKtXi_AAi-2FaYGA)})
  User ID: fJi3RDyc4uo6Lg4kcbj69--JCiXwKrWd2Jmpev-orc0pY9XyFIvwb3AfYvawtZIaZS2hNFyKtXi_AAi-2FaYGA

=== Read ===
✓ Found user by ID: Record(fields={embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), id=StringValue(value=fJi3RDyc4uo6Lg4kcbj69--JCiXwKrWd2Jmpev-orc0pY9XyFIvwb3AfYvawtZIaZS2hNFyKtXi_AAi-2FaYGA), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sun Jan 04 14:39:10 EST 2026)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sun Jan 04 14:39:10 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [embedding, data, active, metadata, user_id, categories, id, age, created_at, tags, email, price, name]

=== Update ===
✓ Updated user: Record(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), created_at=ObjectValue(value={value=StringValue(value=Sun Jan 04 14:39:10 EST 2026), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=fJi3RDyc4uo6Lg4kcbj69--JCiXwKrWd2Jmpev-orc0pY9XyFIvwb3AfYvawtZIaZS2hNFyKtXi_AAi-2FaYGA), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), created_at=ObjectValue(value={value=StringValue(value=Sun Jan 04 14:39:10 EST 2026), type=StringValue(value=String)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), id=StringValue(value=fJi3RDyc4uo6Lg4kcbj69--JCiXwKrWd2Jmpev-orc0pY9XyFIvwb3AfYvawtZIaZS2hNFyKtXi_AAi-2FaYGA), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})

=== Delete ===
✓ Deleted user with ID: fJi3RDyc4uo6Lg4kcbj69--JCiXwKrWd2Jmpev-orc0pY9XyFIvwb3AfYvawtZIaZS2hNFyKtXi_AAi-2FaYGA

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
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
  IDs: NI5-q-JWYZEcgBt3usBylsRJJQa9-UHKc-q1XtsHyxktYcL3j153De4Bh4RrZV8B6BdRwFgiV-szTkJ6Mvb2Fw, 5-0DBd6FrZtskj9rrWTAc33QSAzZEJtxrO5cP-yaGikG8huweFHes9x2leBdlTy-O4SEbjcoRl3KiWP79geSWw, w5IyqyYk3IftqZ8QwEVEuQzx3wp38H3cXpxSk_bG49yYApjHDF-aQx07IY3mK4DZvS7EpE46b5bk9UNF-gT4WA...

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
✓ Session value: {"user_id":"123","created_at":1767555564170}
  (Will expire in 10 seconds)

=== Set Multiple Keys ===
✓ Set 3 configuration keys

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 5

=== KV Delete ===
✓ Deleted key: user:123

=== Verify Deletion ===
✓ Key exists after delete: false

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
  - schema_employees_client_ts
  - ttl_cache
  - schema_products_client_py
  - test_accounts
  - schema_users_client_go

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-04T19:39:28.465115Z","last_modified":"2026-01-04T19:39:28.465115Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: STOIja0m3pBmA8ueNF-ttDyQHW_sZODcHFqGKWbQSGQ_9ttDSGDg2biZqUqg57hNVHZSQoBnWaYo_Bqi2N24lg

=== Verify Document Exists ===
✓ Document found: [session_id, created_at, user_id, id, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: RxK2Wv0J3GW7mllXuwz9reshsIX5L4OMA2AZshW_AmLByoLoogDQ1IG9c2EIkim0PGfx1r2DI3KQpbRxCglDUA

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [value, id, cache_key, ttl]

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
  {"payload":{"data":[{"id":"a5XPfPtETyg2z3nS7gfVpNaVrXPRkpAZC2IZ3I3u7dosAucC3zTCVtqzbkfKGv1aLJhbwNNNgpJlJikKcJqzRQ","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=6AachGyt_8dA-_ghnfusBNdnDutC_eDj_0G0ka_uHv03zS_l78BdAganX-S9tinxvPVNIotMvdnDsCtwZo4G2Q)
✓ Inserted user 2: StringValue(value=38ZxG1FY8yiNviJeeKHWVk9x8-MgCVQxfS9fM4KR6MIe2C7o5qars3lWjTzos_Ywk8hAhZlsSV0cBzrumDfzFQ)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_employees_client_ts, ttl_cache, schema_products_client_py, test_accounts, schema_users_client_go]

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
  {"results":[{"record":{"tags":"programming,rust,tutorial","title":"Rust Programming","description":"Learn Rust programming language with hands-on examples and best practices.","views":762,"id":"tmT80J3B0Ee_ijb_OyzfU-pt8cRQRZZ-qgYygm5rnaCS8j5GFJ01tSQENBgE1D9wK0tMWXI-wBze6GuVS9TVgA"},"score":19.8,"matched_fields":["title","description","tags"]},{"record":{"description":"Master Python for data analysis, machine learning, and visualization.","views":398,"tags":"programming,python,data-science","id":"1WDOLOTbZgRZbjs2mJNdpJhHNfMmUnJWMx-stDBge3FQ-BKFa03odlQ6ZMcsWWr0FgTKYAea-fHZzZTBSxFMkA","title":"Python for Data Science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":"Build modern web applications using JavaScript, React, and Node.js.","id":"mPnAMR-iLKpRahGXNVzLt0KPUWszpt-XSpqQZCJ29E1HFLZHvNOwelJ3XopSm3nEmbF7pUup1V-BcmiGNAlZXw","tags":"programming,javascript,web","title":"JavaScript Web Development","views":759},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"tags":"ai,machine-learning,python","views":853,"title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","id":"CahleZ-sZxB-8LUlHcFBC5I5dJ0Kf8tfuilUkmD9fISifjmoyh_uxW6TbatMRJH5zIH2MmlWHCKdEbHWmGPGMg"},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization.","views":398,"id":"1WDOLOTbZgRZbjs2mJNdpJhHNfMmUnJWMx-stDBge3FQ-BKFa03odlQ6ZMcsWWr0FgTKYAea-fHZzZTBSxFMkA","tags":"programming,python,data-science"},"score":1.0,"matched_fields":["description"]},{"record":{"id":"l_e3rTmfJVfzAmLfoXPmFFnYEG1xOieASooZNUxnNgYvMpx6BwF9ps2PcBPX4L_J_9SmVCX2Y7fYb5QpOQoSQw","title":"Database Design","views":805,"tags":"database,design,sql","description":"Learn database design principles, normalization, and query optimization."},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","tags":"programming,rust,tutorial","views":762,"id":"tmT80J3B0Ee_ijb_OyzfU-pt8cRQRZZ-qgYygm5rnaCS8j5GFJ01tSQENBgE1D9wK0tMWXI-wBze6GuVS9TVgA","title":"Rust Programming"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=QAHuUBCO4FK_aSITRshWon95aFLswnGmM8rFneBfA7TDn92FGtULzAR0HKrVNAJ5Z7yukKme1Sj1wxrVwshvAw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767555589870},"id":"QAHuUBCO4FK_aSITRshWon95aFLswnGmM8rFneBfA7TDn92FGtULzAR0HKrVNAJ5Z7yukKme1Sj1wxrVwshvAw","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-04T20:39:49.011800Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Created session: BupP1oojiZ-6M5vOKcpzBsuDrwlLYn7GYJ21z_O5OuuyolTje7eVwv2T23z0DM1ss8DC1NT2WEkbNFglVfEspg

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "RMtmkTNycMNaaf83_P9cm0lZWG3FzuvHYln_-F36DT3jZn6JBCgp65DqaDm_ogxI4T14Gq6DUNc2aDmNq989uA"
  Responses: ["ekoDB is a high-performance database with intelligent caching and real-time capabilities. It has integrated AI features that contribute to its robust functionality.\n\nOne of its key features is the AI Chat Integration. This feature allows you to query your database using natural language and get AI-powered responses with relevant context, enhancing the user's interaction with the database.\n\nekoDB also supports various types of searches including full-text search, vector search, and hybrid search. These capabilities, combined with automatic context retrieval, make it a powerful tool for finding and managing data.\n\nThese are just some of the features of ekoDB, and more details can be found in the rest of the documentation."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: px_cTD1SFKmyV4URD7AphoRLpOiw_nxoeTZSBv5h2UtJe_eyOdy_mcGmLmmipoN1MA6VStTY9prwuY6eQx8jDA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Answer: The product available is ekoDB, a high-performance database product with AI capabilities. The price of this product is $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB, a high-performance database product with AI capabilities, is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"px_cTD1SFKmyV4URD7AphoRLpOiw_nxoeTZSBv5h2UtJe_eyOdy_mcGmLmmipoN1MA6VStTY9prwuY6eQx8jDA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"QNwXRJb6uu7HJUNKqhw5bFVl6EH7CrkOIn-vZIpy8IkfNezXcJlGRqybeQE2KRBdQ0hGO1hdNzSQAHz6oGGwYQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-04T19:40:08.548206Z"},"id":"RSrJRxnPdhC04bW7IResK47om1BS5RN28uaGcddclmnovYjI69inR-hADcCgBmb6KDd0x5ss5Fvtpq2SiSyL0A","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":28,"prompt_tokens":242,"total_tokens":270}},"updated_at":{"type":"DateTime","value":"2026-01-04T19:40:08.548206Z"}},{"chat_id":{"type":"String","value":"px_cTD1SFKmyV4URD7AphoRLpOiw_nxoeTZSBv5h2UtJe_eyOdy_mcGmLmmipoN1MA6VStTY9prwuY6eQx8jDA"},"content":{"type":"String","value":"Answer: The product available is ekoDB, a high-performance database product with AI capabilities. The price of this product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"QNwXRJb6uu7HJUNKqhw5bFVl6EH7CrkOIn-vZIpy8IkfNezXcJlGRqybeQE2KRBdQ0hGO1hdNzSQAHz6oGGwYQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-04T19:40:08.553113Z"},"id":"ezzP5pbW0pJIzv3RSF5teDxG4oCeRwwy6KSVW4qnHEXfAAOBwQ0Ypyw751PveK-NZPK_6PjqItNOyHXfD7XyIQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":28,"prompt_tokens":242,"total_tokens":270}},"updated_at":{"type":"DateTime","value":"2026-01-04T19:40:08.553113Z"}},{"chat_id":{"type":"String","value":"px_cTD1SFKmyV4URD7AphoRLpOiw_nxoeTZSBv5h2UtJe_eyOdy_mcGmLmmipoN1MA6VStTY9prwuY6eQx8jDA"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"QNwXRJb6uu7HJUNKqhw5bFVl6EH7CrkOIn-vZIpy8IkfNezXcJlGRqybeQE2KRBdQ0hGO1hdNzSQAHz6oGGwYQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-04T19:40:09.471904Z"},"id":"eI6xLJnyjNNvyLFKnre8PDj03UP2XxDOXm7UIoLeLP1VcHmnElwfYyFogpFTJgUFk3M4wbx9higqF5POxz7zIQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":21,"prompt_tokens":242,"total_tokens":263}},"updated_at":{"type":"DateTime","value":"2026-01-04T19:40:09.471904Z"}},{"chat_id":{"type":"String","value":"px_cTD1SFKmyV4URD7AphoRLpOiw_nxoeTZSBv5h2UtJe_eyOdy_mcGmLmmipoN1MA6VStTY9prwuY6eQx8jDA"},"content":{"type":"String","value":"The price of the ekoDB, a high-performance database product with AI capabilities, is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"QNwXRJb6uu7HJUNKqhw5bFVl6EH7CrkOIn-vZIpy8IkfNezXcJlGRqybeQE2KRBdQ0hGO1hdNzSQAHz6oGGwYQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-04T19:40:09.476876Z"},"id":"BjhQtYPbFz_BpfUPlGk2aPWH5WSIzP_H464n1GyHmtu-q4bemTbn1rwmBqOHfILd0_nyP8GlKOlK6bLTQ7gKVQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":21,"prompt_tokens":242,"total_tokens":263}},"updated_at":{"type":"DateTime","value":"2026-01-04T19:40:09.476876Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: b-s3tzB_uaG3KEe2a-nC7BENoeubDh97wg_QdryhpYt4twP5aU690aV16s6gtsz95SnsweLPuwQ6GeF4--7jvg

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
✓ Created session: 6IKFI8iGciI54PKtHMilQ3RPlJNStPlbN0HCWe8emU7rsZMmFoa_QXoQol4F0YJ-RLv4AeuK_hH-G760sNrCLg

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information provided, one product that is available is ekoDB. It is a high-performance database product and it costs $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the information available, we have a high-performance database product named \"ekoDB\". It is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: gs9xkjzev8zBeINr5Ejlcb7ZZ3KmpZjJ__9DJavWK8I-kOmgjd9C8Fwzgwch1l1UG1PWrq30YN10D1pNtpnRPw

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

✅ Script saved: BWINm5TAKq9RmjslMExUsBWnMXc0JzXXrEXh8rD4magbzOnC9sK8yQFeCuNhmldQcCMHSPQzIBTtlQ53W3xzww
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"inactive","count":5,"avg_score":50.0}
   {"count":5,"status":"active","avg_score":60.0}
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
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: bePc7rbwK-_Ibw8ELCXXUDt3ctitVysEA9uY8YBWclkEiDHbGLkKp5mTTKow1wBe6bbF22NXvWm64EQVjdkAoQ
Created Bob: $500 - ID: 2ywLNSmu5625eNJurWpW2nVHlz4MKoXWuLCXC7-voFt1sgeXukeXWZQrR8AULc_O8Jp5Q8sLgrQj8Fcg-kFawQ

=== Example 1: Begin Transaction ===
Transaction ID: 055d36a6-2b9c-4121-aadc-b8f23810abfc

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b516b8c9-f34c-4c2f-9921-5519aef3d0e5
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
