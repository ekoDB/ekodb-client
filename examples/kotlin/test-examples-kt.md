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
✓ Inserted user: Record(fields={id=StringValue(value=dD5W57xWkershpUI_DafE9-X9Tv2czrwD8oBPT_nNb-kn8SHfHq5WrU_rl3L7z16LZyVAsuEeMMrFs1HoYbV-Q)})
  User ID: dD5W57xWkershpUI_DafE9-X9Tv2czrwD8oBPT_nNb-kn8SHfHq5WrU_rl3L7z16LZyVAsuEeMMrFs1HoYbV-Q

=== Read ===
✓ Found user by ID: Record(fields={created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sat Jan 03 14:53:20 EST 2026)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), metadata=ObjectValue(value={value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)}), type=StringValue(value=Object)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), id=StringValue(value=dD5W57xWkershpUI_DafE9-X9Tv2czrwD8oBPT_nNb-kn8SHfHq5WrU_rl3L7z16LZyVAsuEeMMrFs1HoYbV-Q), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sat Jan 03 14:53:20 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {nested={deep=true}, key=value}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [created_at, age, name, metadata, categories, active, user_id, data, price, id, email, embedding, tags]

=== Update ===
✓ Updated user: Record(fields={active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sat Jan 03 14:53:20 EST 2026)}), id=StringValue(value=dD5W57xWkershpUI_DafE9-X9Tv2czrwD8oBPT_nNb-kn8SHfHq5WrU_rl3L7z16LZyVAsuEeMMrFs1HoYbV-Q)})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), id=StringValue(value=dD5W57xWkershpUI_DafE9-X9Tv2czrwD8oBPT_nNb-kn8SHfHq5WrU_rl3L7z16LZyVAsuEeMMrFs1HoYbV-Q), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sat Jan 03 14:53:20 EST 2026)}), metadata=ObjectValue(value={value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)}), type=StringValue(value=Object)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)})})

=== Delete ===
✓ Deleted user with ID: dD5W57xWkershpUI_DafE9-X9Tv2czrwD8oBPT_nNb-kn8SHfHq5WrU_rl3L7z16LZyVAsuEeMMrFs1HoYbV-Q

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 6s
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
  IDs: Oe8g0HzWGQO_iiyxMwp3A3mHTNbl4rDjlXMi3NK82xAn4s1qqc_vYnJ_gSqQ24oKhQ0RklKOo9KHD9Pg7b7zRw, df3AQkUkFxfy5r7IPdBoSDvgknv7XKU9dSWOy6wJIHyGxjUOG7wpBz1Qkdr6bvPCTfjFm3VKxV1BLctl6BwxQQ, 3YgeropedcagvLjzRBqDr4MUDaS733dHhqjvlHVkbnqXHVyybdOwnPRQ24ARN53OYAe4Wkmq6kaqhX8XCzFzxA...

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
✓ Session value: {"user_id":"123","created_at":1767470013383}
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
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-03T19:53:37.605629Z","last_modified":"2026-01-03T19:53:37.605629Z","bypass_ripple":false,"primary_key_alias":"id"}

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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: hb8el-CfPIgjBDyD3JZerWseEdwsdIvgkS0Lz-4u9MghRKajB7LH4Na5Duif2Lxcj_zBJksyUgA41W0l7gitEg

=== Verify Document Exists ===
✓ Document found: [id, user_id, session_id, created_at, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: OQQ_b9PTS4VwrCzy2J-ZbCMMy4cQZS2KovXHebRzSd9P3SqrdGTAiWcz1El_o7o9IL1iQWtge9JPvnWR2zPVwQ

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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"payload":{"data":[{"id":"xg_IW7Atq5VnJ5Q3hIJzga9se5AuKuqmS-FAhWmPl1yRv03jq1EtN7472knF7OZ9VxxgGUz4AcVKCuB-a_bdsw","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=43ROyA8HBr3-QfIlgvsh1ItLeejwFxeYljS1t6Sr0XUKEQqm2peCIS5Jc8kWs3BOUQKGbJEi9Ioj-Kgi6WJgcw)
✓ Inserted user 2: StringValue(value=Fflx20w9EchOCYHnCQt7TbZaXNEcPtYZvFYrWvtkqnWgJUshUsuevi4K3g2LyZ2jjvHSXOXl-Nc1YpiJbrj_nw)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

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
  {"results":[{"record":{"views":523,"title":"Rust Programming","id":"8z49qgvWlXvFDMFnU2IQzo62_hoGOpny5K3tvvLOH0uPRpEmEzNdoWPG9LKU4YZS_nEfjUbjxnCGHWrAdxIg_A","tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices."},"score":19.8,"matched_fields":["description","tags","title"]},{"record":{"description":"Build modern web applications using JavaScript, React, and Node.js.","tags":"programming,javascript,web","id":"iU64G-CX6qiLdrCRhpJIBoovhzY2sSIGbBoGnxLhuaITu9sm2GIGDT0ff-7dMD6W_ChGBO3W4Rro_22Alb8FNg","title":"JavaScript Web Development","views":252},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization.","title":"Python for Data Science","views":43,"id":"m40XPD7tuIQ2DSBEYlDuFTQgRNCG7UcRJ6-GYZk-mL0_o74MDQ9zRvZo2UIK-MB85Ez1UWPHK16h18Ck6n39_A"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"id":"i_oL-VyqZG2t-eZSgkJlqquzdVrKrucNCyMhuw4Ohk_s-pNDftPIRSiNsNbxA85CTtev0j5AjQn2Z1im26hgFg","views":954,"title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","tags":"ai,machine-learning,python"},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"tags":"programming,python,data-science","title":"Python for Data Science","id":"m40XPD7tuIQ2DSBEYlDuFTQgRNCG7UcRJ6-GYZk-mL0_o74MDQ9zRvZo2UIK-MB85Ez1UWPHK16h18Ck6n39_A","views":43,"description":"Master Python for data analysis, machine learning, and visualization."},"score":1.0,"matched_fields":["description"]},{"record":{"views":301,"description":"Learn database design principles, normalization, and query optimization.","id":"iyDixdeQ0gTj-AcZCg3gkFY5GV3sKFar2IhvhtjN8X1TLj_xv1TPoU3yVCcrrOh6ClnnoNwOcbm1_kG0kzzkWA","title":"Database Design","tags":"database,design,sql"},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","tags":"programming,rust,tutorial","views":523,"id":"8z49qgvWlXvFDMFnU2IQzo62_hoGOpny5K3tvvLOH0uPRpEmEzNdoWPG9LKU4YZS_nEfjUbjxnCGHWrAdxIg_A","title":"Rust Programming"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=pF6f_o-b8-PG7ScvwUs_pv-gbcSMQHK9uPAv4ciKMm3cAKrqdl3x9AMx9s3QX0gn5bBWc4MkoGE_P8n-1txLKw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767470038940},"id":"pF6f_o-b8-PG7ScvwUs_pv-gbcSMQHK9uPAv4ciKMm3cAKrqdl3x9AMx9s3QX0gn5bBWc4MkoGE_P8n-1txLKw","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-03T20:53:58.067813Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: IgHgYOLFOdKDRM2htps2EVv793-E00uXfJrfxKBo-YPx5isQYHNdUsImbdnvfDCnP-oTlWKz6raZlNOB_jem5g

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "CRlqjUuN5qAE3aW9R8KzeTH_lVsciX94daRCrf-wwAsoh78B8TD-IzTELMsE2zxxvj1AlNryp84WRLEecQytbw"
  Responses: ["ekoDB is a high-performance database that is designed with intelligent caching, real-time capabilities, and AI integration. It offers a variety of features aimed at improving user interaction and data retrieval. \n\nOne of the key features is the AI Chat Integration, which allows users to query their database using natural language. The responses to these queries are powered by AI and provide relevant context. \n\nAnother significant feature is the advanced search capabilities. ekoDB supports full-text search, vector search, and hybrid search. This makes it versatile and adaptable to different search needs. The search feature also comes equipped with automatic context retrieval, enhancing the results' relevance and accuracy."]

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
✓ Created session: lgweY_lRrzw1y_qHeMVe_w6HzFViVRnCQw8gZAsvZBdbRLawOSEfPiK38u3TDE6O2tYCIRMmy6DWVrE29s467w

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The product available based on the provided context is ekoDB. It's a high-performance database product with AI capabilities. The price for this product is $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"lgweY_lRrzw1y_qHeMVe_w6HzFViVRnCQw8gZAsvZBdbRLawOSEfPiK38u3TDE6O2tYCIRMmy6DWVrE29s467w"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"YjjekUMSGCvSydVNKy3LYaDbYe3kUXc3SU1YlpXrswcPHkwhzH2mXK1_C4xMBhk4bZsHWnB1VTqBUcSImh76bQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-03T19:54:16.969884Z"},"id":"JUxljHeO5akBnd0GpHethEToSljGylY7IwFmqZJcvy7A_ERi3Lw8J9bJwDOhi0enLDdbZCl1VuLOluzx8TRcnw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":240,"total_tokens":273}},"updated_at":{"type":"DateTime","value":"2026-01-03T19:54:16.969884Z"}},{"chat_id":{"type":"String","value":"lgweY_lRrzw1y_qHeMVe_w6HzFViVRnCQw8gZAsvZBdbRLawOSEfPiK38u3TDE6O2tYCIRMmy6DWVrE29s467w"},"content":{"type":"String","value":"The product available based on the provided context is ekoDB. It's a high-performance database product with AI capabilities. The price for this product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"YjjekUMSGCvSydVNKy3LYaDbYe3kUXc3SU1YlpXrswcPHkwhzH2mXK1_C4xMBhk4bZsHWnB1VTqBUcSImh76bQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-03T19:54:16.973139Z"},"id":"RY_DbmIwQ4Lbg0zF17tgRY_GY3pJE5Q3reWtZE4Idqvpl3KBgU5Lm5RmsJT9BuzvJcYvwryorzAdz9sPINhZfQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":240,"total_tokens":273}},"updated_at":{"type":"DateTime","value":"2026-01-03T19:54:16.973139Z"}},{"chat_id":{"type":"String","value":"lgweY_lRrzw1y_qHeMVe_w6HzFViVRnCQw8gZAsvZBdbRLawOSEfPiK38u3TDE6O2tYCIRMmy6DWVrE29s467w"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"YjjekUMSGCvSydVNKy3LYaDbYe3kUXc3SU1YlpXrswcPHkwhzH2mXK1_C4xMBhk4bZsHWnB1VTqBUcSImh76bQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-03T19:54:18.199235Z"},"id":"xhGmfrWorCyNErd8dzu9JvMVpdiNJNdIe6ZvElzTQFhRkk1jdI9vNkW6e6m8PUve8qVCCTgk_ixdN4iA_mjulw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":240,"total_tokens":252}},"updated_at":{"type":"DateTime","value":"2026-01-03T19:54:18.199235Z"}},{"chat_id":{"type":"String","value":"lgweY_lRrzw1y_qHeMVe_w6HzFViVRnCQw8gZAsvZBdbRLawOSEfPiK38u3TDE6O2tYCIRMmy6DWVrE29s467w"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"YjjekUMSGCvSydVNKy3LYaDbYe3kUXc3SU1YlpXrswcPHkwhzH2mXK1_C4xMBhk4bZsHWnB1VTqBUcSImh76bQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-03T19:54:18.202386Z"},"id":"mFVs0I7gCZGECMrElErv_bT9sYeYUDhy7bO9fY5MUJzNn4rIOpbcowAvHpTpjYmt23Rgp_Fw_CX97wIdwjIViw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":240,"total_tokens":252}},"updated_at":{"type":"DateTime","value":"2026-01-03T19:54:18.202386Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 1

=== Branching Session ===
✓ Created branched session: ps4eNh1dTRkuIlBT26C38J5VIg1iaOXBRgeFv6sKZp7ObGkgHG-Bwn6IGGvDZNbEss3D1PlizAWVVdd-JThb9g

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
✓ Created session: oEhU-ZFISDKbDCLfyMDU8t0tq2QODfzvTSCsR9jBnXTycLyCjEEhapAPXGsti_u9bY0-xOtNSN7yd5Kh_VapsA

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information available, one product that is available is \"ekoDB.\" This is a high-performance database product."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the information I have, the available product is \"ekoDB\". This is a high-performance database product and it is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: 9__wf3X8y_k_SgMmPGF1jR_UuPKerGKsEags-UPheij0BVaMUh_C4HLC5-IA5H1_9etXaBEPFPyyeNyMOQGUpw

=== Merging Sessions ===
✓ Merged sessions
  Merged session ID: null

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

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
🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: FQJqH9srq1zZFdfcsuUUaApwi6R_0ClZOFWsnBHMkNY1uUrasOu1pOVnu_fUPA_3TEKhbhaJTDIRi0k6qDHwdA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"avg_score":50.0,"status":"inactive"}
   {"avg_score":60.0,"status":"active","count":5}
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
Created Alice: $1000 - ID: H_cfhZszV1L_h-8aWm5fR_x_-S87OB1hshgWMebdujliPJR-a8HKdVZEn_sH64sGLLcYUIxfiPWwN-sdWKYSxw
Created Bob: $500 - ID: FJgVZ4nZ67bYAkKaQgVR_99Zdb_eZgW8ktQe7vzmYcwE5xJxGIeY9BEPEAOWsYe-vkyKVczxNtRiXS9CnHOkQw

=== Example 1: Begin Transaction ===
Transaction ID: f6a83ea5-8448-4c2b-bf61-7b6500fc5220

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 07ca2623-c0a6-44e1-b1f7-5c143e19ad47
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
