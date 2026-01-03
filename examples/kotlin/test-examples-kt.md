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
✓ Inserted user: Record(fields={id=StringValue(value=pg1zuFAm_mZlY9zHw5xG5QQ0EmbSaUIgdlLX_iMnNTFlWhppIFVwRo-4JPWUn70jgDI0XD5EaWv2D2ZzZA7XoQ)})
  User ID: pg1zuFAm_mZlY9zHw5xG5QQ0EmbSaUIgdlLX_iMnNTFlWhppIFVwRo-4JPWUn70jgDI0XD5EaWv2D2ZzZA7XoQ

=== Read ===
✓ Found user by ID: Record(fields={embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), id=StringValue(value=pg1zuFAm_mZlY9zHw5xG5QQ0EmbSaUIgdlLX_iMnNTFlWhppIFVwRo-4JPWUn70jgDI0XD5EaWv2D2ZzZA7XoQ), created_at=ObjectValue(value={value=StringValue(value=Sat Jan 03 15:11:30 EST 2026), type=StringValue(value=String)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sat Jan 03 15:11:30 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [embedding, tags, name, age, metadata, price, categories, active, email, user_id, id, created_at, data]

=== Update ===
✓ Updated user: Record(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), created_at=ObjectValue(value={value=StringValue(value=Sat Jan 03 15:11:30 EST 2026), type=StringValue(value=String)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), id=StringValue(value=pg1zuFAm_mZlY9zHw5xG5QQ0EmbSaUIgdlLX_iMnNTFlWhppIFVwRo-4JPWUn70jgDI0XD5EaWv2D2ZzZA7XoQ), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sat Jan 03 15:11:30 EST 2026)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), id=StringValue(value=pg1zuFAm_mZlY9zHw5xG5QQ0EmbSaUIgdlLX_iMnNTFlWhppIFVwRo-4JPWUn70jgDI0XD5EaWv2D2ZzZA7XoQ), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)})})

=== Delete ===
✓ Deleted user with ID: pg1zuFAm_mZlY9zHw5xG5QQ0EmbSaUIgdlLX_iMnNTFlWhppIFVwRo-4JPWUn70jgDI0XD5EaWv2D2ZzZA7XoQ

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 5s
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
  IDs: iS8lMn3IejAKRcnN-0T0BNAkMEQTJ-QLwltmjhix4qhHrPNRVigkBqVKAzpJo0mZ9VtAfv58RfC-TjYqfiV4Dw, B4xhGf75xup3YmveNZDUAFXuKvV_fMTryEgHGEC2mtL5Dqjt9RvjdcInoCADp21lB37GsVsw2xUnH1urLxwFyg, KAFFq5wmYszBvypMV4S2_AseXVtzURcQv1cSufcPlJKiQztNd8kKdYbhl-cdx7c6QB2m7oQOWCDj_GCpD3n9SQ...

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
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"created_at":1767471103220,"user_id":"123"}
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
✓ Found 0 collections

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-03T20:11:47.475469Z","last_modified":"2026-01-03T20:11:47.475470Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: 8bvH8MHtZfSfjMBCrGnuzPdSP2QJ7miFPZ771WlJikWCTokd3OMMddYAihvc9tSmhj0_RtQhjm8yL7ZvyFJ5CQ

=== Verify Document Exists ===
✓ Document found: [created_at, user_id, id, session_id, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: WZeavo2umlo7cExk_oL67eG-EYKSFFOer3HNC6jg62h2YKpEqBeD4hdkVJdmk8v5HnNJPwaXhfhHKJS_rIUR6w

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [id, cache_key, value, ttl]

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
  {"payload":{"data":[{"id":"W39RopfBduORmrsoyubTCOVeoBnaqd9QRjPha7RZmGuzlzFol2xW4woEYRaKXiH7z4LbpH_PxEqmWtxW41CmMg","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=J0kRbB_7JF6YCk1OqJOUJy-VQxHtiKOYcweu8RwreEpWX5aQFRdZYlLXnRIVjuyWXLrKKMiLhoT8dhngnhcyXw)
✓ Inserted user 2: StringValue(value=EX8tzvdneFl7iWRPfrxLSFouhaQn3fUcBJ_r4NTQf1Vx3Alg6KJOKW7ZsRe-qPa0eZ7c0_Y2RY8vDIGhQc4tJw)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 1
  Sample: [kotlin_schema_example]

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
  {"results":[{"record":{"tags":"programming,rust,tutorial","title":"Rust Programming","views":859,"description":"Learn Rust programming language with hands-on examples and best practices.","id":"-q_xLmYtYbPyR4yGGW3mJBdx6Qx8t4wFYF1CxDlYjF8MwcAEtt77VoGfDBKP6JlYCU1yUXPji9YrfE0c5KLR8Q"},"score":19.8,"matched_fields":["title","description","tags"]},{"record":{"views":407,"description":"Master Python for data analysis, machine learning, and visualization.","id":"Il2pPjJ3GvZARc_kUV7dowdz0NG-163v7aPCJnRmIEtMNoJ0WxUxmpQ3JTFE8BvJspaDoI9Lh6uE9TSWgplVlg","tags":"programming,python,data-science","title":"Python for Data Science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"title":"JavaScript Web Development","id":"uiq7Ar6QvdVfIdW_GWyniJxIs-4eUyxxyc4mhhDKvAen0uwwKQyjwtIXXH0V1YUBnWMl0hmUojd5oKSsccwspA","description":"Build modern web applications using JavaScript, React, and Node.js.","views":980,"tags":"programming,javascript,web"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"title":"Machine Learning Basics","id":"TRzwxwCItEXWJVAYJc4C8HtTDc1mGmscej51-fLoSnQisZdGcM5z8CANrxXDWAfxKrAh9z86vQPBGIaPupKNhA","views":792,"tags":"ai,machine-learning,python","description":"Introduction to machine learning algorithms and neural networks."},"score":2.7,"matched_fields":["title","description","tags"]},{"record":{"views":407,"title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization.","tags":"programming,python,data-science","id":"Il2pPjJ3GvZARc_kUV7dowdz0NG-163v7aPCJnRmIEtMNoJ0WxUxmpQ3JTFE8BvJspaDoI9Lh6uE9TSWgplVlg"},"score":1.0,"matched_fields":["description"]},{"record":{"id":"-q_xLmYtYbPyR4yGGW3mJBdx6Qx8t4wFYF1CxDlYjF8MwcAEtt77VoGfDBKP6JlYCU1yUXPji9YrfE0c5KLR8Q","tags":"programming,rust,tutorial","views":859,"title":"Rust Programming","description":"Learn Rust programming language with hands-on examples and best practices."},"score":0.5,"matched_fields":["description"]},{"record":{"views":549,"title":"Database Design","description":"Learn database design principles, normalization, and query optimization.","tags":"database,design,sql","id":"TuQzXIZJU3WouBgmkjcB4hXyKR4PS7yaObD08PkHglh22QtruUxsEGGUAzYFME4yYQKC9QSpXJKGEi8mwlEOWQ"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=76a5E9qvJBcBOko4UrUwwvFOD6gaVSZoitTkYDEZbeT17CRUJ6YOUKna20WlXa5cfM3S4v9v8dLaJ6t0KEzQCA)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767471128525},"id":"76a5E9qvJBcBOko4UrUwwvFOD6gaVSZoitTkYDEZbeT17CRUJ6YOUKna20WlXa5cfM3S4v9v8dLaJ6t0KEzQCA","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-03T21:12:07.651935Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Created session: n-_-aKSE6Mda_6HS6aIXCqk7Xr1AsCf2_r88lZQKfPcpvZQRFfiOg-1R-WhctyZCmwrp7XcYiOE-81ibTix6Bg

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "TnKQDDKW1lL32QjONTDfGAa3eDsSxaGI9NRwbOb86z0h5l0LzJh6zPv6pzKppQJfYnM24MLzVJJEzsMM9Y3_7A"
  Responses: ["ekoDB is a high-performance database that is known for its intelligent caching, real-time capabilities, and AI integration. It comes with a variety of features that make it a powerful tool. One of its unique features is the AI Chat Integration which allows you to query your database using natural language and get AI-powered responses with relevant context. In addition, ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval, providing you with a versatile search functionality."]

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
✓ Created session: pcYVxE1c6_cwr5D-u1qvChMtzHomYoNnKcehoL-syKUbCNvxYqwLDqolixEDiGZu-T7oiDu8ql9j8lqVUBA30A

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The product available is \"ekoDB\", which is a high-performance database product with AI capabilities. The price of this product is 99 units."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"pcYVxE1c6_cwr5D-u1qvChMtzHomYoNnKcehoL-syKUbCNvxYqwLDqolixEDiGZu-T7oiDu8ql9j8lqVUBA30A"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"-blwtDDhLNynfsE10oQ-XldV-J6FQW4whNwm0zlfVm0JzVgcNYy4xXNcc_ZYDxwlzl4R4IxUAL2DqLwYQHg2hQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-03T20:12:25.246971Z"},"id":"z6ay2h1SFqo2eCZgGWy8dTPVDV0apGN_Q8tbcZhjM5nvCeqiFSduFdpC1yun-S7KzVHjZ_t70BEAVQlCNEBO9g","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":29,"prompt_tokens":240,"total_tokens":269}},"updated_at":{"type":"DateTime","value":"2026-01-03T20:12:25.246971Z"}},{"chat_id":{"type":"String","value":"pcYVxE1c6_cwr5D-u1qvChMtzHomYoNnKcehoL-syKUbCNvxYqwLDqolixEDiGZu-T7oiDu8ql9j8lqVUBA30A"},"content":{"type":"String","value":"The product available is \"ekoDB\", which is a high-performance database product with AI capabilities. The price of this product is 99 units."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"-blwtDDhLNynfsE10oQ-XldV-J6FQW4whNwm0zlfVm0JzVgcNYy4xXNcc_ZYDxwlzl4R4IxUAL2DqLwYQHg2hQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-03T20:12:25.248486Z"},"id":"qWe6JzwoFLaCeYZVuhpv6jpMtZJKFABJlPGsuD0S-janOsv177c0izSC-zgbs3ZA3j6zzDYpKi7wi1AJyjv-Xw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":29,"prompt_tokens":240,"total_tokens":269}},"updated_at":{"type":"DateTime","value":"2026-01-03T20:12:25.248486Z"}},{"chat_id":{"type":"String","value":"pcYVxE1c6_cwr5D-u1qvChMtzHomYoNnKcehoL-syKUbCNvxYqwLDqolixEDiGZu-T7oiDu8ql9j8lqVUBA30A"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"-blwtDDhLNynfsE10oQ-XldV-J6FQW4whNwm0zlfVm0JzVgcNYy4xXNcc_ZYDxwlzl4R4IxUAL2DqLwYQHg2hQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-03T20:12:26.194503Z"},"id":"ruhg3gArLZSQ_3o1XGBLsuGPNjeVLg7HM1gYUgOg5vkqs4uy9HQDnD1RL507-u8SoQWzhVWjTyKj0mRK-b5YSQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":240,"total_tokens":252}},"updated_at":{"type":"DateTime","value":"2026-01-03T20:12:26.194503Z"}},{"chat_id":{"type":"String","value":"pcYVxE1c6_cwr5D-u1qvChMtzHomYoNnKcehoL-syKUbCNvxYqwLDqolixEDiGZu-T7oiDu8ql9j8lqVUBA30A"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"-blwtDDhLNynfsE10oQ-XldV-J6FQW4whNwm0zlfVm0JzVgcNYy4xXNcc_ZYDxwlzl4R4IxUAL2DqLwYQHg2hQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-03T20:12:26.198233Z"},"id":"3vFN6rY34wfjeohf-OuwBhlErYybwwArI-grHxkXhRisvk2--1MFHqOIipwQkwbIjP9-ZdNawwIcSUNWE5pTQw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":240,"total_tokens":252}},"updated_at":{"type":"DateTime","value":"2026-01-03T20:12:26.198233Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 1

=== Branching Session ===
✓ Created branched session: 0jsxbA3lzYYSsShqHzS9oCCXWHbtB9hFQyTnaBsOSuaqrZvSePAxCKxP4pcQhQcBGAQulorqSMZjQmErSl22fw

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
✓ Created session: sUIbgcB5j4TOB2U3imXaEnbZgS2NjLY2fjJ8BnUgLdMkfDexFChTogF--oqFn-BTyhpNYVVLUUfES5OGwMfq4A

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information available, we have a high-performance database product named \"ekoDB\" available. The price for this product is $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the provided context, the available product is ekoDB, a high-performance database product. It is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: SrE4znvIn5jNcLLUoG9odxLYqYwStAxrq5oadznrCoRtX0Ta1Y0-COxcvUHHsECadBPmQJSlFFui8scwIABHmA

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

✅ Script saved: 7u-SJLoErBycWQJvf0OsPKf-rtNZCweCN1km_keDXM3WrWIGVGEUiDgvguBGMMPe2H4iljMlnUfLYcxHA7gM3g
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
   {"count":5,"avg_score":60.0,"status":"active"}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 3
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
✅ Deleted test scripts

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
Created Alice: $1000 - ID: jxVcpvOCihkBte_iORejOsZouchdNDA4cQRWQW0klmgYHbsbin4EUOK3uRFuD45BNaA1VchqkbPZby2TZ5-DpA
Created Bob: $500 - ID: kfh_VO-G2RvODERhaDGceXE0kMgpgG9zK26l5HudwSZmMxrxCmXB86xThmoDxgKc-NvLQ4RLKYOcw_7boKh9ig

=== Example 1: Begin Transaction ===
Transaction ID: 7abde7ba-6ca1-4115-8c98-e523ab5acf2d

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 79cf6d7e-c1d5-437f-a05b-f49636f137ee
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
