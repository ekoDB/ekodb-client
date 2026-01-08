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
✓ Inserted user: Record(fields={id=StringValue(value=jSQ51ytpHGmyHyW-NOQTBa9m9xhnHqN1_YwYZyptcYRmxCdoNzy57bgm6yMrTqA6UXHKd25UcmJIEQCA-KC2Pw)})
  User ID: jSQ51ytpHGmyHyW-NOQTBa9m9xhnHqN1_YwYZyptcYRmxCdoNzy57bgm6yMrTqA6UXHKd25UcmJIEQCA-KC2Pw

=== Read ===
✓ Found user by ID: Record(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), id=StringValue(value=jSQ51ytpHGmyHyW-NOQTBa9m9xhnHqN1_YwYZyptcYRmxCdoNzy57bgm6yMrTqA6UXHKd25UcmJIEQCA-KC2Pw), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Wed Jan 07 20:19:33 EST 2026)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Wed Jan 07 20:19:33 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {nested={deep=true}, key=value}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [email, data, embedding, id, tags, age, metadata, created_at, active, name, categories, user_id, price]

=== Update ===
✓ Updated user: Record(fields={created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Wed Jan 07 20:19:33 EST 2026)}), id=StringValue(value=jSQ51ytpHGmyHyW-NOQTBa9m9xhnHqN1_YwYZyptcYRmxCdoNzy57bgm6yMrTqA6UXHKd25UcmJIEQCA-KC2Pw), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={created_at=ObjectValue(value={value=StringValue(value=Wed Jan 07 20:19:33 EST 2026), type=StringValue(value=String)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), id=StringValue(value=jSQ51ytpHGmyHyW-NOQTBa9m9xhnHqN1_YwYZyptcYRmxCdoNzy57bgm6yMrTqA6UXHKd25UcmJIEQCA-KC2Pw), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)})})

=== Delete ===
✓ Deleted user with ID: jSQ51ytpHGmyHyW-NOQTBa9m9xhnHqN1_YwYZyptcYRmxCdoNzy57bgm6yMrTqA6UXHKd25UcmJIEQCA-KC2Pw

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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"payload":{"data":[{"id":"xrTg0u3PEldQMyQHvwaHY9DO1eXI0Kgev1c0Nb3Nq-dKPKxBLrTISsqUBRxLxtPRJEoICSpMmMhpTOFnLvJZEQ","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
  IDs: uNeaKfjEGdwVQxE1xc2hgfRn_Us2P8tGHLkuypo6NAOqJQxJXyDLYym2hexjTitmKxBOpznRSp_VFrwK33Ls-A, GPBh2ZPLX9sey1H2v2nEbwtgS0ueiEimY3cntCiYzbwgCaFzqA-q43ZVVwJJ9YOwvr4uwWVBcJx35YjoGvOheQ, AtjGsx9lh7uh6dfeg_azwlT4pqTEO-bWd0Hav5Kicv5Y_IqIDXogdIyx5DVnh0IDFn_tPDhzbk1ITdsqQDRl8A...

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
✓ Found 27 collections
  - websocket_test
  - chat_configurations__ek0_testing
  - test_accounts
  - test_collection
  - schema_users_client_ts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-08T01:19:46.419855Z","last_modified":"2026-01-08T01:19:46.419856Z","bypass_ripple":false,"primary_key_alias":"id"}

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
✓ Retrieved value: {"email":"alice@example.com","role":"admin","name":"Alice"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"created_at":1767835190600,"user_id":"123"}
  (Will expire in 10 seconds)

=== Set Multiple Keys ===
✓ Set 3 configuration keys

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 11

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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: oSbdVYs90vQLZzDQ3VZZLgZ5dhw8-jrMGONBW30HaTVjo1IfoDb4EG5P-rxsttSYZlo5tAISTJSPUqFd20UWWw
Created Bob: $500 - ID: yBkZ7rXlgG8FZmO0w_rDWlyNH7tlH4mfYeuWT7YHZFlCMivNbeXTHJEn31TzzE_4UEXXsbJIz9JTjN7k0e3sgQ

=== Example 1: Begin Transaction ===
Transaction ID: d5fca50b-24c5-423e-a73d-dec3388a8822

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: fe993992-1d89-42f6-93b7-2f65fffaf15d
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
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=95)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=92)})
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
  {"results":[{"record":{"tags":"programming,rust,tutorial","title":"Rust Programming","id":"YnYbSNAKH1heGx3NSgRBwv_alAA56tBnM2eGC1BJhirC-uFk4C6FFONudIJMe-10Q5clFpddGf1u2yWmftt0yQ","views":209,"description":"Learn Rust programming language with hands-on examples and best practices."},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"id":"XGGXJyCLG7OTgoF4Idw9RVdAMaS-ekp49pQchnmafgKPVf7Apk5QPILBaGHXZRpWT7sJ7BW0-vTsv18pTHkhpw","description":"Master Python for data analysis, machine learning, and visualization.","title":"Python for Data Science","views":566,"tags":"programming,python,data-science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":"Build modern web applications using JavaScript, React, and Node.js.","tags":"programming,javascript,web","title":"JavaScript Web Development","id":"YmQTjWsWAhgNCkOtAM77WeZbaQSucykiFbrlstIqYdK1gg3sbOiqROGAaKwEZW3kD65l5nsRqXY4WOnu4fSe7w","views":297},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"tags":"ai,machine-learning,python","id":"zFEpnLa2Hd9oNwQ8M6BYQYgAAAXV7AZFLSKIkYwGp7bnaFPwfpUk3-USMkVsLxcab4w03P2MlLMeaLqDtTOQfA","views":483,"description":"Introduction to machine learning algorithms and neural networks.","title":"Machine Learning Basics"},"score":2.7,"matched_fields":["title","tags","description"]},{"record":{"id":"XGGXJyCLG7OTgoF4Idw9RVdAMaS-ekp49pQchnmafgKPVf7Apk5QPILBaGHXZRpWT7sJ7BW0-vTsv18pTHkhpw","description":"Master Python for data analysis, machine learning, and visualization.","tags":"programming,python,data-science","title":"Python for Data Science","views":566},"score":1.0,"matched_fields":["description"]},{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","tags":"programming,rust,tutorial","views":209,"id":"YnYbSNAKH1heGx3NSgRBwv_alAA56tBnM2eGC1BJhirC-uFk4C6FFONudIJMe-10Q5clFpddGf1u2yWmftt0yQ","title":"Rust Programming"},"score":0.5,"matched_fields":["description"]},{"record":{"tags":"database,design,sql","description":"Learn database design principles, normalization, and query optimization.","title":"Database Design","id":"ZJ5eY6kfDq08J0WpPB2gQ4571tl9emii4XmOu6NzkElLbPmrzfSDitFOO3EAHV_zORVZf_IObY3CB3hzU2wEvA","views":808},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted user 1: StringValue(value=KbTECAyFFW6TdD1A_ADi3IDLHdzkOCHw_8c2L9rpyoTP9mjza8CjC1dvm4hilo-B6m5X49iMfFqrnVqEAXJ3Yw)
✓ Inserted user 2: StringValue(value=dg486BiSOOiXl91oyHGl9eQZM9yC2OBgs2shFRFV1LapgMRg4S51ffxN736gMzYKimtbqd_oyCMwpYKwSGVUOA)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 28
  Sample: [websocket_test, chat_configurations__ek0_testing, test_accounts, test_collection, schema_users_client_ts]

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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: hfAvbCyAKATqNoQfhWtqm8-p4KdUArkAAYp22C3ekEmedkxnwCwJP2x8DcbnAtdnv5sy5eSPrYfCxCDPT1sDsA

=== Verify Document Exists ===
✓ Document found: [session_id, id, created_at, user_id, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: _P8knkDrwH0F9vZEWOeZTLnsAzx5WWtqs1svx6OEHyECiJ5vCkM0nX9KXCb2zC0KyNtYGG3qLR5PKU7kaFRe3A

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [cache_key, value, id, ttl]

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
✓ Inserted document with TTL: StringValue(value=77S86y29URILqbm_2Jc8imNUuLY7SoJ4CHx0zTSefCIKxxh-2ccBkg8yUVa6Wi70BACOrtqlXDH3BQSi9t3Cvw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767835220879},"id":"77S86y29URILqbm_2Jc8imNUuLY7SoJ4CHx0zTSefCIKxxh-2ccBkg8yUVa6Wi70BACOrtqlXDH3BQSi9t3Cvw","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-08T02:20:21.052318Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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

Creating edge cache lookup script...
✓ Edge cache script created: x8GGOoDr4eUupE_YXLMAmQ4jQ22QHksL2Q2uSY_8qjop0EeHi15kXOUOc2_2P7DYTTIrNurTVe0IbZh4adQQWw

Call 1: Cache lookup
Found 1 cached entries
Response time: 16ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 7ms

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
🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: xMnD-1yufjVoieVRBi8XMzV0xmADhg4pDB5gb9GRXHfw9yMkKAH042_nAMKtqUvCjD3ddpAw_8FdwffPHieGtw
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 3 groups
   {"avg_score":50.0,"status":"inactive","count":5}
   {"status":"active","avg_score":60.0,"count":5}
   {"count":5,"status":"null","avg_score":60.0}
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
📊 Total user count: 15
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
🚀 ekoDB Kotlin Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 21StBcvXp86U3wjN1x5Xxz4we50DtffMKbQ4ocLy7UHGpcUXeV_f3-KnPixmlvojfIx-yF8K85HysJ7ezaPCgg
📊 Found 2 category groups

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed with 1 results

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Script management (save, call, delete)

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
🚀 ekoDB Kotlin KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: StringValue(value=OtzTsel0mZIx_96sRa4ueeKcv0Nbh11lNrW3vER8OR0qlqfZWymHFpRIo0ijohOS1EPwRQu5bTff9UoWASRhtA)
✅ Inserted 2 products

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: jtBrIt5MlLhIfKDxc0dpjMLJH0zHVOGJjJV25Tvt4YmSIhvd68fGEza3VPX99gTw5POvGDhx7r1sXDGMpK34DQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 9rzrs9JtA1qoj3d3Prw5dGocv46FcsVesIGp4TjeBIhJU9s4Q5nPzCUGMmKuwvCgXVx8_clfiG-2ac3yIiAF8Q
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: G7wZpZabo3WOVtYpAE1LYMKoUrCIRmLf0baOLCimYP2mFej8XuWNkdBqZwap92kqVj0ewQcnmyMPQJrDOvnNiA
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
   ✅ FunctionStageConfig.Kv* classes work within scripts

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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Setting up cache collection...
✓ Cache entry created

Step 2: Create SWR cache lookup script
✓ Created SWR script: swr_cache_lookup_kt (ZTI4AVPExfLeONG4iwsgD_X-CGE2EnF3FlD1h8myWOZtxrXH3RYx34IMzLxWBWt8U7Lzw35tnu7Od3CTWrf8VQ)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 6ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation

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
🚀 ekoDB Kotlin Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   {"category":"Furniture","avg_price":365.6666666666667,"count":3}
   {"category":"Electronics","count":5,"avg_price":367.0}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!

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
🚀 ekoDB Kotlin AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   {"response":"Vector databases have several benefits including: high precision in storing data, ability to retain complex spatial relationships between objects, less data redundancy, support for a wide range of spatial operations like proximity analysis and network analysis, and they are ideal for applications requiring complex, contiguous geographic detail such as the design of a new highway or river systems.","tokens_used":110}
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text

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
🚀 ekoDB Kotlin CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   {"status":"inactive","count":3}
   {"count":7,"status":"active"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!

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
🚀 ekoDB Kotlin Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. {"type":"String","value":"Getting Started with ekoDB"} ({"type":"String","value":"Database"})
   2. {"type":"String","value":"Vector Databases Explained"} ({"value":"Database","type":"String"})
   3. {"type":"String","value":"Natural Language Processing"} ({"value":"AI","type":"String"})
   4. {"value":"Database Design Principles","type":"String"} ({"value":"Database","type":"String"})
   5. {"type":"String","value":"Introduction to Machine Learning"} ({"type":"String","value":"AI"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"category":"Database","count":3}
   {"count":2,"category":"AI"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!

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
✓ Created session: 9pDeN-aJBqqQ0MnDd2WXCIMivt8945_I7TgkFz2hqg-AzpQpglY-MsxcdXgk_0YtqcVdV90naWoh4_n_ibrnkw

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "ZVVjxAUs4XESaqJ1yX2Udlun8nFDPzHJO3wj-R_WTUA1hLM_VUV1e_QvEuA_U-eOd7bV-O39gEBmGVzrZXnnnw"
  Responses: ["ekoDB is a high-performance database that comes with intelligent caching, real-time capabilities, and AI integration. It has a variety of features to meet diverse needs. \n\nOne of the key features is the AI Chat Integration which lets you query your database using natural language and returns AI-powered responses with the relevant context.\n\nekoDB also supports a variety of search features. It offers full-text search, vector search, and hybrid search, all with automatic context retrieval, making it easier for you to find the exact data you're looking for in your database."]

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
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: -2Hqp2zrSEPlojLoPn4u1_ObO19_mLxkunBVzCGFB4SXEdttQVr8LQV5yDQhNA-hWyFtctMJTjrFp5p0U7XvxA

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information provided, the available product is a high-performance database product named \"ekoDB\". It is priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the information provided, the available product is ekoDB. It's a high-performance database product priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: ZWJRY5wLZJpCvR9n599veXL8MvvWacYMpD40zlRUwBTwKHoww7Pl_XZFNZjUEBAd8-cWVjsL3zuo4P9XJ4Stbg

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
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: wPA2e2w8XnLkWaGUDknkWZmkFgDShY4xbTUUdQKmEg1KrYuWt3o2fco4w2KAg5Rk4KUdGeTg0MJxH3lzjLJanA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The product available is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"wPA2e2w8XnLkWaGUDknkWZmkFgDShY4xbTUUdQKmEg1KrYuWt3o2fco4w2KAg5Rk4KUdGeTg0MJxH3lzjLJanA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"p49ZvGcq6bp4GpiJxL-Ck0EjDGp6FpWzxlBJSv_pKcVjgKAdCGLBzkrahPa8RxpABgsxiyIxDIidZSj-7cGqrA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-08T01:21:25.319857Z"},"id":"gQf5U6l8isduaXrw-xzPByafTA5T7jdfcLdNHfz6a63iAjuzNMQmKDzhCBZ4NNx3-79NJcMtd8dpgqmZ4GYYzQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":24,"prompt_tokens":175,"total_tokens":199}},"updated_at":{"type":"DateTime","value":"2026-01-08T01:21:25.319857Z"}},{"chat_id":{"type":"String","value":"wPA2e2w8XnLkWaGUDknkWZmkFgDShY4xbTUUdQKmEg1KrYuWt3o2fco4w2KAg5Rk4KUdGeTg0MJxH3lzjLJanA"},"content":{"type":"String","value":"The product available is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"p49ZvGcq6bp4GpiJxL-Ck0EjDGp6FpWzxlBJSv_pKcVjgKAdCGLBzkrahPa8RxpABgsxiyIxDIidZSj-7cGqrA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-08T01:21:25.325206Z"},"id":"Lt6XYJtjyOvzInuBbQjJice_gZyDfRB5_FhWiF8b3myEMvsdknd_wGgedw_x8luOJbkEv04lfucLMIrzNI4SkQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":24,"prompt_tokens":175,"total_tokens":199}},"updated_at":{"type":"DateTime","value":"2026-01-08T01:21:25.325206Z"}},{"chat_id":{"type":"String","value":"wPA2e2w8XnLkWaGUDknkWZmkFgDShY4xbTUUdQKmEg1KrYuWt3o2fco4w2KAg5Rk4KUdGeTg0MJxH3lzjLJanA"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"p49ZvGcq6bp4GpiJxL-Ck0EjDGp6FpWzxlBJSv_pKcVjgKAdCGLBzkrahPa8RxpABgsxiyIxDIidZSj-7cGqrA","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-08T01:21:26.660451Z"},"id":"mgxKafBGm7snV3bNJdQxHO_mQ7oa7gB182B-Ix7EcNHgKHG35nJtlmlSMjZlpMKvRAoQ1tgDf_O9a0yDO1-6fQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":175,"total_tokens":187}},"updated_at":{"type":"DateTime","value":"2026-01-08T01:21:26.660451Z"}},{"chat_id":{"type":"String","value":"wPA2e2w8XnLkWaGUDknkWZmkFgDShY4xbTUUdQKmEg1KrYuWt3o2fco4w2KAg5Rk4KUdGeTg0MJxH3lzjLJanA"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"p49ZvGcq6bp4GpiJxL-Ck0EjDGp6FpWzxlBJSv_pKcVjgKAdCGLBzkrahPa8RxpABgsxiyIxDIidZSj-7cGqrA","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-08T01:21:26.663893Z"},"id":"8rG5fVjXxSQP6xoncnnlvJXFdAem2KV_45hq3hLimCBrSG23_YlL_y2Tk3UdQnAcDcr412ErGsFF7eHCXA9NBQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":175,"total_tokens":187}},"updated_at":{"type":"DateTime","value":"2026-01-08T01:21:26.663893Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: TVp7I6fdK87G0_a5exij4U9vAq4g9O3qcl1TufNEnsYD6yp1hwzHVDmusK7Iw77j8P6xDg4vl0xrSa2X4iarBA

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
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: Record(fields={id=StringValue(value=WJHfDLLR5cI-DuuMe5UI7aDtUHBmbS87ZTwlZLQk-_3ohVFSfNRGHISDNQz21KFwPiAghc4EeXOf8RJgC8mdSw)})

=== Upsert Operation ===
✓ First upsert (update): Record(fields={error=StringValue(value=Record not found)})
✓ Second upsert (insert): Record(fields={error=StringValue(value=Record not found)})

=== Find One Operation ===
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: false
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 6 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!

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
=== Bypass Ripple Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
1. Basic insert (ripple enabled):
   Inserted with ripple: Record(fields={id=StringValue(value=nANwh55RHDfFM4FdCXubwdpZajeuoLyeIi2ARbWt8BFx2hqRDYHEfEE7-Tk3hXZDMmuw72tVjsOrNQTjmITUZQ)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record(fields={id=StringValue(value=Vj3VJXoVnKUMhZQ_APmbsGuzCFC2P-LXJZwLJyc8_IPReex5QETNiRE2FuzvMe7evQDr3iZ8CZSLNrW5eimrUw)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=150)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), id=StringValue(value=nANwh55RHDfFM4FdCXubwdpZajeuoLyeIi2ARbWt8BFx2hqRDYHEfEE7-Tk3hXZDMmuw72tVjsOrNQTjmITUZQ)})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), id=StringValue(value=nANwh55RHDfFM4FdCXubwdpZajeuoLyeIi2ARbWt8BFx2hqRDYHEfEE7-Tk3hXZDMmuw72tVjsOrNQTjmITUZQ), name=ObjectValue(value={value=StringValue(value=Upsert Product), type=StringValue(value=String)})})

✅ All bypass_ripple operations completed successfully!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: k8NbTs8YT5XcRdxJM6BA7Drl8N1c2u58MUL05OlHH9YRWrCKwv0ugvs0wbb5FLEX_2TdAscf70oT7KJWwXwG6g
Created Bob: $500 - ID: _rUWj4sWc5ieYIYURGq4tcp7owm6oBnbbxeI0CbGheFTS8dRQntweIAxGwx24rmSwbnntgdBedGGSkMIzSATlw

=== Example 1: Begin Transaction ===
Transaction ID: bec6f926-af03-4d19-aa12-2cb1d8cde8ae

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9ae552a8-22a3-41e0-9f82-1126d723d2e2
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
