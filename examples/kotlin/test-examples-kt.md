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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=mRxEvOuim0W2UbeaR__q3x8HvYga3mi45kIIV5xJaHjoCvoKAd8AJqkGQwh89y21K66IRZoew_u3kh2DK5-3LQ)})
  User ID: mRxEvOuim0W2UbeaR__q3x8HvYga3mi45kIIV5xJaHjoCvoKAd8AJqkGQwh89y21K66IRZoew_u3kh2DK5-3LQ

=== Read ===
✓ Found user by ID: EkoRecord(fields={user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), id=StringValue(value=mRxEvOuim0W2UbeaR__q3x8HvYga3mi45kIIV5xJaHjoCvoKAd8AJqkGQwh89y21K66IRZoew_u3kh2DK5-3LQ), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Wed Jun 03 08:08:36 EDT 2026)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Wed Jun 03 08:08:36 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: user_id, email, categories, age, tags, active, data, name, id, price, metadata, embedding, created_at

=== Update ===
✓ Updated user: EkoRecord(fields={embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), created_at=ObjectValue(value={value=StringValue(value=Wed Jun 03 08:08:36 EDT 2026), type=StringValue(value=String)}), id=StringValue(value=mRxEvOuim0W2UbeaR__q3x8HvYga3mi45kIIV5xJaHjoCvoKAd8AJqkGQwh89y21K66IRZoew_u3kh2DK5-3LQ), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={id=StringValue(value=mRxEvOuim0W2UbeaR__q3x8HvYga3mi45kIIV5xJaHjoCvoKAd8AJqkGQwh89y21K66IRZoew_u3kh2DK5-3LQ), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), created_at=ObjectValue(value={value=StringValue(value=Wed Jun 03 08:08:36 EDT 2026), type=StringValue(value=String)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})

=== Delete ===
✓ Deleted user with ID: mRxEvOuim0W2UbeaR__q3x8HvYga3mi45kIIV5xJaHjoCvoKAd8AJqkGQwh89y21K66IRZoew_u3kh2DK5-3LQ

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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"data":[{"id":"53kIGoq01TmPC1QgSpu9MWXKdqVeIi5Kcx5w_K7nRuYuYpOL6e33CEU_GCSZEyNwd-ssTIij4RSMW_tUpS7vjg","name":{"value":"Test User","type":"String"},"status":{"value":"active","type":"String"}}]}

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
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: 4ajEWgFghYqcDL1Ht5vUYnoZkRG5CaImqw8b35JynKRhbNMs6HKnTDUcADdIGOEDq8welP1wqX0TX--3dej90g, 7BYc4QV2lMqhLjEmtOoO-Ti23nL79WBBA_IDQlZeiZRJcWvMKkY4oGqqE3AwHV5uP1K5LHUbaOQY_oZyQ22THQ, 4YYqucwklFE1J_oHnIw-JD8br6MKsshV9zt029J48ZkUs2DI9GH3uqDTT8XB5DfKZ3xVKd3feZ3FU5b7UNFg6A...

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
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 29 collections
  - schema_products_client_js
  - schema_employees_client_ts
  - batch_users
  - chat_messages__ek0_testing
  - test_accounts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-06-03T12:08:52.822893Z","last_modified":"2026-06-03T12:08:52.822894Z","bypass_ripple":false,"primary_key_alias":"id"}

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
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"created_at":1780488537048,"user_id":"123"}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  config:db: {"value":{"host":"localhost","port":5432}}
  config:cache: {"value":{"enabled":true,"ttl":3600}}
  config:api: {"value":{"retries":3,"timeout":30}}

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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: omj9K3iUUp5NRJRugKD1GSrbLOwRWRFndKFgiPEEF9465qBJzozxrc6rfVHqfeFNTRHCmQONNSCF0ejZsAjYUg
Created Bob: $500 - ID: z5EKnUHCYrlyK_73408NJ48pBQWE5VsWJ_sjRw1ThgUo8ovLjOC7-zIK-YUJmRnPLoW_oK-XVgTGOXC2YXRzBg

=== Example 1: Begin Transaction ===
Transaction ID: 6a4c20c8-9f5a-44ca-a0ef-83f56e961bc6

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6a355a82-546c-49b2-9e59-061053d8eb39
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed

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
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"tags":{"type":"String","value":"programming,rust,tutorial"},"views":{"value":480,"type":"Integer"},"description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"},"id":"buon7-m7jfu4ZAyQX3mrslegDYn_kA58f47c5ZQ6MJIyRM_P9mIsFLnqWDjAF9wF8DOQ-C7nRhvbWJirqDoDKQ","title":{"value":"Rust Programming","type":"String"}},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"description":{"value":"Build modern web applications using JavaScript, React, and Node.js.","type":"String"},"tags":{"value":"programming,javascript,web","type":"String"},"views":{"type":"Integer","value":952},"id":"YYuMMvAncahgzkyA-QSpvIgqfAxzMUuimtWJM97yCrnH1Q196Fd3Lnf3Ch2qM-KHgwanInLrHmFK5BJlbpir4w","title":{"type":"String","value":"JavaScript Web Development"}},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"title":{"type":"String","value":"Python for Data Science"},"tags":{"value":"programming,python,data-science","type":"String"},"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"views":{"value":619,"type":"Integer"},"id":"ZmvjB60mpPDxABqgU65X41h7YkEHKY1W8_NNM4iIHg7Q5gKnbF-0jQLH2-K6CIqp6TPA3GPxZD-F_7Vyc0Gxkg"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":9}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"title":{"value":"Machine Learning Basics","type":"String"},"id":"LvjHM1taU7SgUe7N3A20DEn2i7wEIdnbGaEATKnvKSfRcYXY194_EAxbMlkUAipcYaRjnP3baRGC0kv3IMxTww","tags":{"type":"String","value":"ai,machine-learning,python"},"views":{"type":"Integer","value":18},"description":{"value":"Introduction to machine learning algorithms and neural networks.","type":"String"}},"score":2.7,"matched_fields":["title","tags","description"]},{"record":{"views":{"type":"Integer","value":619},"description":{"value":"Master Python for data analysis, machine learning, and visualization.","type":"String"},"id":"ZmvjB60mpPDxABqgU65X41h7YkEHKY1W8_NNM4iIHg7Q5gKnbF-0jQLH2-K6CIqp6TPA3GPxZD-F_7Vyc0Gxkg","title":{"value":"Python for Data Science","type":"String"},"tags":{"value":"programming,python,data-science","type":"String"}},"score":1.0,"matched_fields":["description"]},{"record":{"title":{"value":"Database Design","type":"String"},"views":{"type":"Integer","value":525},"id":"x0mHctem6UZVlxHrTdJStt7P7kXIqQb4hUhMAmd9A6o76bfJl-N-TQ_b9jLB78tpsqtsKEm787wWHV1XlfyRdw","description":{"type":"String","value":"Learn database design principles, normalization, and query optimization."},"tags":{"type":"String","value":"database,design,sql"}},"score":0.5,"matched_fields":["description"]},{"record":{"views":{"value":480,"type":"Integer"},"description":{"type":"String","value":"Learn Rust programming language with hands-on examples and best practices."},"tags":{"value":"programming,rust,tutorial","type":"String"},"title":{"value":"Rust Programming","type":"String"},"id":"buon7-m7jfu4ZAyQX3mrslegDYn_kA58f47c5ZQ6MJIyRM_P9mIsFLnqWDjAF9wF8DOQ-C7nRhvbWJirqDoDKQ"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":5}

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
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=WzIRv3URhFkFzpnc8yyC5Lvn2l5Z3Cw0nDQZGax61VTB68mT_9nownk9SvXAL8vLLgQLJHQfXS0wdrkUpqEoQw)
✓ Inserted user 2: StringValue(value=dtGyJp12nmWSNFYRnSW7fT-Zz7tTk7I4AIayQrATktX9FiWaAm_hp6frtyTqp9P-kQwfQlJe8Pyd6hKw-tlnlg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 30
  Sample: [schema_products_client_js, schema_employees_client_ts, batch_users, chat_messages__ek0_testing, test_accounts]

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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: eu9W3oOCsaxzc5BjHuHbYddCaxySbho0FzyYAbOC3Yn1qTnQPKyT1vwGj4aLIcuPwCviQiYohOB8Q-KPcWYIhg

=== Verify Document Exists ===
✓ Document found: id, user_id, created_at, session_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: lOUqy8vU-hm6taxQq4WuEO7q249cZsepG01Bn4nhl6-v5RmRKQ46TLXMHIRZFsZ7i9ce3kcGHRjvel77iB6K6w

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: value, cache_key, id, ttl

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
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=c329MNK84kXVrnQ7E49XZgZeyNNWtP3qQS7jESAW3mMgexEyGoVE-JwGUIEIKAmlUAuLBhprA6azJBuqY05jCw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"id":"c329MNK84kXVrnQ7E49XZgZeyNNWtP3qQS7jESAW3mMgexEyGoVE-JwGUIEIKAmlUAuLBhprA6azJBuqY05jCw","name":{"type":"String","value":"WebSocket TTL Test"},"value":{"type":"Integer","value":42},"created_at":{"value":1780488570259,"type":"Integer"},"ttl":"2026-06-03T13:09:30.366349Z"}]}

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
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup function...
✓ Edge cache function created: 0iFhkmkvCxU0Xs6J53CqonovTZyg6pIr0FbG938jrgNWTOXSIUD94feyliHn1HpyI-qPVDNKiG0h3oRF5-9l6Q

Call 1: Cache lookup
Found 1 cached entries
Response time: 42ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 21ms

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
🚀 ekoDB Functions Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: PMX9M5reWc5xTw2fxgaHr0eFHZVihOPzjE7Y7ogxKU91kBMzsqbLLDCE8Bcs3i_4ouNgCOZAFkfACzCq6N16MQ
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 3 groups
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":60.0,"type":"Float"},"status":{"type":"String","value":"null"}}
   {"count":{"type":"Integer","value":5},"status":{"type":"String","value":"inactive"},"avg_score":{"type":"Float","value":50.0}}
   {"status":{"type":"String","value":"active"},"avg_score":{"type":"Float","value":60.0},"count":{"type":"Integer","value":5}}
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
🚀 ekoDB Kotlin Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: rhGS--OKkzYqTYHeyL76U-PBp3hqi-YcOxe1rz6EaZYi4K12md1nI1oDKU2hhIRhWXYAoBvdRa3GX0pNJrdoYg
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

✅ Inserted order: StringValue(value=tLGhyJ0xBKfGmcyF0IjT30jlV-CxbdfS1qkq77yjbjynjRk0vCC8wrJO3qruY0k2U4dLMrW4EVyI3kNLElM_rA)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: 3OpURpdWmlRGQqrjkybPL3Qj7MPFV2qB2tia85Lkh37s0xo6uzEcHist_PWprkDh-WZ7-YeK13L6BXgSMWWixA
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: w2F9w_RZiDHN3pilT7cNXCD7CyZbuMHURSrb7Xqdz82Ls3mqvIZWbDTWRXwRpVvz4vwCLEXSpyXP0_S6uRtWPg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: arQttjV0oVPg-U1QUrxFXsdtDmIV90eGnK_81b6Yl8HJ8S0A1ZdZtnNlLyRqnOT19k-t6hj8nhh4vF4xkOz-OA
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
✓ Created SWR function: swr_cache_lookup_kt (TE2kVnojALS4FMKnodaXgQh2LTzHlkHcUG2sgF61UV-3MJ2c4tRNib4NujmCAYqssghIZDFlZPUU3X5-sUO6gg)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 22ms (served from cache)
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
✓ Created native SWR function: github_user_native (LFmOMnslEPxc9wMJUw76-8X7XSQ9aa_a5ysTkI2ITRcWEkG9W1qaF7etgZ4FZnaba09sY98aEP2qFohNSfgtyg)

First call (cache miss - will fetch from GitHub API):
  Response time: 30ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 19ms
  Speedup: 1.6x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit (JorTRFKs_qu9AD_Z3tbaUkv2Zc80G2kH8LgDRt1EwiLrvsKUbO9hUxrsmBpDx2yqz063nKP8Y6SsuUBoF-BOVA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (l06oNejI8Rcy56K_nVeGVcHN4Gf1L4II3dEfM5VJQKh_IODoTI0JB0Dq57AA6JcSO_gioGcZQ72AHwTTPGlK0w)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache (hBEpny-bGHzlsGiKNOC60F9hbkN4Deq6vfjRKqpZIiUpM5EZ5ORj-aF27KNBy4fRd3nIpFSFbdOQAn2xmmQQ8Q)
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
   {"avg_price":{"type":"Float","value":367.0},"category":{"type":"String","value":"Electronics"},"count":{"value":5,"type":"Integer"}}
   {"category":{"value":"Furniture","type":"String"},"avg_price":{"value":365.6666666666667,"type":"Float"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

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
🚀 ekoDB Kotlin AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   {"tokens_used":{"value":216,"type":"Integer"},"response":{"type":"String","value":"Vector databases offer several benefits, including:\n\n1. **Efficient Similarity Search**: They enable fast retrieval of similar items based on vector representations, ideal for applications like recommendation systems and image search.\n\n2. **Handling High-Dimensional Data**: Vector databases are designed to manage and query high-dimensional data effectively.\n\n3. **Scalability**: They can scale to handle large datasets, maintaining performance as the volume of data grows.\n\n4. **Flexible Data Models**: They support various data types and structures, accommodating diverse applications.\n\n5. **Integration with Machine Learning**: Vector databases easily integrate with ML models, allowing for the storage and querying of embeddings generated by models.\n\n6. **Real-time Analytics**: They enable real-time data processing and retrieval, which is critical for applications needing immediate insights.\n\n7. **Improved User Experience**: By enabling personalized content and recommendations, they enhance user engagement and satisfaction."}}
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
   {"count":{"type":"Integer","value":7},"status":{"value":"active","type":"String"}}
   {"status":{"type":"String","value":"inactive"},"count":{"type":"Integer","value":3}}
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
   1. {"type":"String","value":"Introduction to Machine Learning"} ({"type":"String","value":"AI"})
   2. {"type":"String","value":"Vector Databases Explained"} ({"value":"Database","type":"String"})
   3. {"value":"Getting Started with ekoDB","type":"String"} ({"value":"Database","type":"String"})
   4. {"value":"Database Design Principles","type":"String"} ({"value":"Database","type":"String"})
   5. {"type":"String","value":"Natural Language Processing"} ({"value":"AI","type":"String"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"value":3,"type":"Integer"}}
   {"count":{"type":"Integer","value":2},"category":{"type":"String","value":"AI"}}
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
✓ Created session: 3K8lHzGPrJBHjBTFD-5bwrL2ZlOgBPGbD5lNUdiLSJaZas79xV-Dtx0O_kP-xSAeL-q8zulYagzNSpduR5_FRQ

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "Ss83sbFiMyaofcosdDAI8nZXM4II_EsAQb4D8q_dY3XHlxmsuFZiy1leMB27VuVgDwNQIvUcq9uICbv_mwyWsA"
  Responses: ["ekoDB is a high-performance database that offers several advanced features to enhance data management and retrieval. Here are some of its key features:\n\n1. **AI Chat Integration**: The chat feature allows users to query the database using natural language, receiving AI-powered responses that include relevant context. This facilitates a more intuitive interaction with the database.\n\n2. **Real-time Capabilities**: ekoDB is designed to support real-time updates and intelligent caching, ensuring quick access to data.\n\n3. **Full-text Search**: It supports full-text search capabilities, allowing for efficient text-based queries across the stored content.\n\n4. **Vector Search**: This feature enables semantic search using vector embeddings, which helps in retrieving records based on the meaning rather than exact word matches.\n\n5. **Hybrid Search**: ekoDB combines both full-text and vector search functionalities, providing a comprehensive search experience that leverages automatic context retrieval.\n\nOverall, ekoDB integrates AI capabilities with robust search and data management features to provide a powerful database solution."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 9s
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
✓ Created session: GPA40k6IiuM-PO0IV0O2tWgPthHN0j6ZUk2JFmpbBk9nY358HLprkZ_QmGO5vK5TS_nfdwf3iiSqBrgjs3nsLw

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The available product is:\n\n- **Name:** ekoDB\n- **Description:** High-performance database product\n- **Price:** $99"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Currently, there is one product available:\n\n- **Product Name:** ekoDB\n- **Description:** High-performance database product\n- **Price:** $99\n\nIf you need more information or additional products, feel free to ask!"]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: tjDYHWM9NpHQnegjjZVaZHW51oPyy0q_GoAUaC4PElmwumPRTsDU7X-A7Av3pd_eCSFmqjSsBN9TOBoserSikA

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 2

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 13s
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
✓ Created session: OvDupWIIWzwstdBdy_ExZ0FKwF9gFeZWhGyqD3U1a4guil_K6KxRbQ05vTSExPZxxlRHmt5qLTbPNCuuHsfyQA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99"]

✓ Message 2 sent
  Responses: ["The price of ekoDB is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"OvDupWIIWzwstdBdy_ExZ0FKwF9gFeZWhGyqD3U1a4guil_K6KxRbQ05vTSExPZxxlRHmt5qLTbPNCuuHsfyQA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"vI_MRSnKNly38F0D8DKwN3QxPPsWULGLtSpQ86PegKDsVJkQrPoY8h1ec44XFXSIpAqZKHknWUK7nHKbuHocUQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-06-03T12:10:58.536310Z"},"id":"LDiMs-0bdwwVfRNli4ftQyBvq8Rs__hBC8ayz0dNf-IuKEH2RyhSzWxGlQVR5QxyvxHdkWkCv1uLj49xbmy1tA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":143,"prompt_tokens":10403,"total_tokens":10546}},"updated_at":{"type":"DateTime","value":"2026-06-03T12:10:58.536310Z"}},{"chat_id":{"type":"String","value":"OvDupWIIWzwstdBdy_ExZ0FKwF9gFeZWhGyqD3U1a4guil_K6KxRbQ05vTSExPZxxlRHmt5qLTbPNCuuHsfyQA"},"content":{"type":"String","value":"The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"vI_MRSnKNly38F0D8DKwN3QxPPsWULGLtSpQ86PegKDsVJkQrPoY8h1ec44XFXSIpAqZKHknWUK7nHKbuHocUQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-06-03T12:10:58.540662Z"},"id":"lYzuwlM7rxegsKsJ2kOqP0yBy72CqH6zX7r5p9es_obKnTOgCEHW6SCMXrajDBrXydcWhYTD6Exv7jU1B2LiQA","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":143,"prompt_tokens":10403,"total_tokens":10546}},"tool_call_count":{"type":"Integer","value":3},"tool_call_history":{"type":"Object","value":{"iterations":3,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","filter":{"content":{"field":"product","operator":"Exists","value":true},"type":"Condition"}},"id":"call_qCw85tp15wgZ1zwc3JWY2rnl","name":"query_collection"},{"arguments":{"collection":"kotlin_chat_sessions_example","filter":{"content":{"expressions":[{"content":{"field":"product","operator":"Ne","value":null},"type":"Condition"},{"content":{"field":"price","operator":"Gt","value":0},"type":"Condition"}],"operator":"And"},"limit":10,"type":"Logical"},"limit":100},"id":"call_hb8SOepkUL6IS6CBz1QLLOLx","name":"query_collection"}],"tool_results":[{"error":"Unknown operator: Exists (unknown variant `Exists`, expected one of `Eq`, `Equals`, `Equal`, `Ne`, `NotEquals`, `NotEqual`, `Gt`, `GreaterThan`, `Lt`, `LessThan`, `Gte`, `GreaterThanOrEqual`, `Lte`, `LessThanOrEqual`, `In`, `NotIn`, `Contains`, `StartsWith`, `EndsWith`)","result":null,"success":false,"tool_call_id":"call_qCw85tp15wgZ1zwc3JWY2rnl","tool_name":"query_collection"},{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"vI_MRSnKNly38F0D8DKwN3QxPPsWULGLtSpQ86PegKDsVJkQrPoY8h1ec44XFXSIpAqZKHknWUK7nHKbuHocUQ","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_hb8SOepkUL6IS6CBz1QLLOLx","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-06-03T12:10:58.540662Z"}},{"chat_id":{"type":"String","value":"OvDupWIIWzwstdBdy_ExZ0FKwF9gFeZWhGyqD3U1a4guil_K6KxRbQ05vTSExPZxxlRHmt5qLTbPNCuuHsfyQA"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"vI_MRSnKNly38F0D8DKwN3QxPPsWULGLtSpQ86PegKDsVJkQrPoY8h1ec44XFXSIpAqZKHknWUK7nHKbuHocUQ","price":99,"product":"ekoDB"},"score":2.211111111111111}]},"created_at":{"type":"DateTime","value":"2026-06-03T12:10:59.572641Z"},"id":"2ecsGdWjKh_foqu5djA2zxsFmgPuMBzfsNn04WIqPKkGSb3iu4yRsvn0MiSl8L5VuYdByIagPMaDdYINPkNaxQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":6,"prompt_tokens":3422,"total_tokens":3428}},"updated_at":{"type":"DateTime","value":"2026-06-03T12:10:59.572641Z"}},{"chat_id":{"type":"String","value":"OvDupWIIWzwstdBdy_ExZ0FKwF9gFeZWhGyqD3U1a4guil_K6KxRbQ05vTSExPZxxlRHmt5qLTbPNCuuHsfyQA"},"content":{"type":"String","value":"The price of ekoDB is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"vI_MRSnKNly38F0D8DKwN3QxPPsWULGLtSpQ86PegKDsVJkQrPoY8h1ec44XFXSIpAqZKHknWUK7nHKbuHocUQ","price":99,"product":"ekoDB"},"score":2.211111111111111}]},"created_at":{"type":"DateTime","value":"2026-06-03T12:10:59.578554Z"},"id":"lSmZSNgVOzcmmVf9kiycMjYG2iJoj8h-7XblTqlTkbwf_2ZwENngsOwlu9DR27xIFNlqlvePvYj17ARcNfMVjQ","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":6,"prompt_tokens":3422,"total_tokens":3428}},"updated_at":{"type":"DateTime","value":"2026-06-03T12:10:59.578554Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: HZLzB6XkNzdIficV9uPZjekxdc6P7Q5ARZlAXdpfnskzlVaCBH3SHpXF7-vPAze6sT7K3uPYYE0eBmkGFddHrg

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

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
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
    - claude-opus-4-20250514
    - claude-sonnet-4-20250514
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get OpenAI Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper

=== Get Anthropic Models ===
Anthropic models: claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514

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
Created user function with ID: 6xCPOMKXfwZxVEIgF13U0GTXSZE9nQQBahFslaTUycgoFkALSVLg6aPsNahkIpoI9pfcQFh6p4RgtLfcEohTxQ

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 11 user functions:
  - "validate_user": "Check if user exists"
  - "cache_api_call_py": "Cache External API Call"
  - "cache_api_call_js": "Cache External API Call"
  - "fetch_user": "Fetch user by code"
  - "fetch_and_store_user": "Fetch user from API and cache in KV"

=== List User Functions by Tag ===
Found 11 user functions with 'kotlin' tag:
  - "validate_user"
  - "cache_api_call_py"
  - "cache_api_call_js"
  - "fetch_user"
  - "fetch_and_store_user"
  - "swr_user"
  - "get_active_users_kt"
  - "fetch_product_with_reviews"
  - "fetch_api_user"
  - "cache_api_call"
  - "fetch_slim_user"

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
✓ Created record: EkoRecord(fields={id=StringValue(value=fxBOlbW6bJwHcSIgowJtsU11tU6WZ-9rgunrnWkSJPx8MFmuzRb6C8v0tYBnvWlhFuQtMg_AvBcFwCGJzK_t0g)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), id=StringValue(value=fxBOlbW6bJwHcSIgowJtsU11tU6WZ-9rgunrnWkSJPx8MFmuzRb6C8v0tYBnvWlhFuQtMg_AvBcFwCGJzK_t0g), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=RTq2a7Z7cqaqvWRfsI7kfFsirnuiBan6L7gGE6EwPxGIxTp_7zdImRVIvAsYeeX3oOuxMeeOD6ecIUpuaeam8w)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), id=StringValue(value=fxBOlbW6bJwHcSIgowJtsU11tU6WZ-9rgunrnWkSJPx8MFmuzRb6C8v0tYBnvWlhFuQtMg_AvBcFwCGJzK_t0g)})
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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=JQEYpuNBsbPtKC1ZI2ySOispwmO0KVWkYF1FlMAp8OzYr7mK7MOi5e6OuduFRowiT_iQSnjeSc6phf7AqhpnQg)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=U_QRfYayoSo02gV55AS_owyETzs7ABHwRYAMd_Yh-5DlHiXmHIl7nmwKtz4aSeTyc8os6YcEYi2nlhPJK8zE9Q)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), id=StringValue(value=JQEYpuNBsbPtKC1ZI2ySOispwmO0KVWkYF1FlMAp8OzYr7mK7MOi5e6OuduFRowiT_iQSnjeSc6phf7AqhpnQg), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=150)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)}), id=StringValue(value=JQEYpuNBsbPtKC1ZI2ySOispwmO0KVWkYF1FlMAp8OzYr7mK7MOi5e6OuduFRowiT_iQSnjeSc6phf7AqhpnQg), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)})})

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

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: TECInvw-AzO44A9sZhFNGvHgMhn6YkOS2uemWvSLjb6B82JgVmAbFXq2bHnd9KyEYISVyXavo1sak4at3JaxrQ
Created Bob: $500 - ID: fj632G9vcn92J5M18Ee6MDxcc0buS8ToNk7OoYZ5C7xzXCtbrDP8gzA075-qad_C_nLNCLiMcGqTs3Z94tTaFQ

=== Example 1: Begin Transaction ===
Transaction ID: b1cfd517-53ee-4e0d-a7e8-adf046f8a9ac

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9c4e3a9f-59d8-4701-8438-15b7c126ba98
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
command -v java >/dev/null 2>&1 || [ -n "$JAVA_HOME" ] || { \
	echo "\033[31mNo Java runtime found — Gradle needs a JVM to launch.\033[0m"; \
	echo "\033[33m  Ubuntu/Debian: sudo apt install openjdk-17-jdk\033[0m"; \
	echo "\033[33m  macOS:         brew install openjdk@17\033[0m"; \
	exit 1; \
}
echo "🟣 \033[36mBuilding Kotlin client library...\033[0m"
cd ekodb-client-kt && { JH=$(/usr/libexec/java_home -v 17 2>/dev/null) && export JAVA_HOME=$JH && export PATH=$JH/bin:$PATH || true; } && ./gradlew build --no-daemon
echo "✅ \033[32mKotlin client built!\033[0m"
echo "🧪 \033[36mRunning Kotlin client library examples...\033[0m"
if [ -f .env ]; then \
	. ./.env && \
	{ JH=$(/usr/libexec/java_home -v 17 2>/dev/null) && export JAVA_HOME=$JH && export PATH=$JH/bin:$PATH || true; } && \
	cd examples/kotlin && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientSimpleCrudKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientSimpleWebsocketKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientBatchOperationsKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientCollectionManagementKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientKvOperationsKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientTransactionsKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientQueryBuilderKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientSearchKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientSchemaManagementKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientJoinsKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientDocumentTtlKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientWebsocketTtlKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientEdgeCacheKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsCompleteKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsKvWrappedKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientSwrPatternKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientSwrNativeKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsAdvancedKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsAiKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsCrudKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsSearchKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientChatBasicKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientChatAdvancedKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientChatSessionsKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientChatModelsKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientUserFunctionsKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientConvenienceMethodsKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.BypassRippleExampleKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientProjectionKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientJwtAuthFlowKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientCryptoStagesKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientConcurrencyStagesKt --no-daemon && \
	API_BASE_URL=$API_BASE_URL WS_BASE_URL=$WS_BASE_URL API_BASE_KEY=$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientPathRoutedFunctionKt --no-daemon; \
else \
	echo "\033[31m❌ .env file not found\033[0m"; \
	echo "\033[33m💡 Create .env file with API_BASE_URL, WS_BASE_URL, and API_BASE_KEY\033[0m"; \
	exit 1; \
fi
echo "✅ \033[32mKotlin client examples complete!\033[0m"
