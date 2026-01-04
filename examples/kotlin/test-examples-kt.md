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
✓ Inserted user: Record(fields={id=StringValue(value=HsW7ubaL7_cQSOC8rK2KJthdj_HrOukDPta1uQcxuw3Uc7OI7yUsjinkrs0c3A6h59F8drG1lPbNcteyHP0U6w)})
  User ID: HsW7ubaL7_cQSOC8rK2KJthdj_HrOukDPta1uQcxuw3Uc7OI7yUsjinkrs0c3A6h59F8drG1lPbNcteyHP0U6w

=== Read ===
✓ Found user by ID: Record(fields={id=StringValue(value=HsW7ubaL7_cQSOC8rK2KJthdj_HrOukDPta1uQcxuw3Uc7OI7yUsjinkrs0c3A6h59F8drG1lPbNcteyHP0U6w), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={value=StringValue(value=Sun Jan 04 01:33:58 EST 2026), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sun Jan 04 01:33:58 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [id, user_id, active, email, embedding, name, price, data, tags, created_at, age, metadata, categories]

=== Update ===
✓ Updated user: Record(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), created_at=ObjectValue(value={value=StringValue(value=Sun Jan 04 01:33:58 EST 2026), type=StringValue(value=String)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), id=StringValue(value=HsW7ubaL7_cQSOC8rK2KJthdj_HrOukDPta1uQcxuw3Uc7OI7yUsjinkrs0c3A6h59F8drG1lPbNcteyHP0U6w), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), id=StringValue(value=HsW7ubaL7_cQSOC8rK2KJthdj_HrOukDPta1uQcxuw3Uc7OI7yUsjinkrs0c3A6h59F8drG1lPbNcteyHP0U6w), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sun Jan 04 01:33:58 EST 2026)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)})})

=== Delete ===
✓ Deleted user with ID: HsW7ubaL7_cQSOC8rK2KJthdj_HrOukDPta1uQcxuw3Uc7OI7yUsjinkrs0c3A6h59F8drG1lPbNcteyHP0U6w

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
  IDs: JJPY3XVWSUKQb_RnLEvSLHR3vKh-CPSjJ6udTG1nfFv21h5wmADU8zMIfm8EocRcn538EcWJy-3F3S1G5KZ40w, fzg9MUtJZ7ZWTGrwys3yE9X5eZlIi2vdvSGsSP6YEIopQwDxk9vA3qjccfIJpGSzFL1cRAzV8Ll7u3fWXyTlwg, U7tSjj1Ay2W2F2rtqkc9jKcav_DyEANVxZE_MyT4vu9jExj3BFrrE6RzqJ_M_cXqVU0vxovkK9WUkWb5YITFIg...

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
✓ Retrieved value: {"name":"Alice","role":"admin","email":"alice@example.com"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1767508452075}
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
  - schema_documents_client_py
  - schema_products_client_ts
  - schema_products_client_go
  - batch_users
  - schema_employees_client_ts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-04T06:34:16.322583Z","last_modified":"2026-01-04T06:34:16.322584Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: t6pYjAzKbL9arxFRk2ZiQUG2uhd4bXu53g6fbgEDoaaENnaJJFo46tRtPKubYqEiQvMA9--OeLggMZpbLwi2_w

=== Verify Document Exists ===
✓ Document found: [session_id, created_at, user_id, id, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: JxhPeZgRm5_CuqjWAEQY9UF7yqXzOqW9fvlbu964ANcTU2AOIVUb8f2B9cU2D9_xnvsRR_dmcmYBEmYYYz7RKQ

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [value, cache_key, id, ttl]

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
  {"payload":{"data":[{"id":"42FfHcn4lCSxf502tM2axQH2TW_snYd4_pcTBNobjp9t2SyTyx45ESVGHRG7YGfWa3ZctXpMG5EVWH7InSCcIA","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=xiDIgIFaLNtwS1vKuKJpt1_yRuVhAredVdSow3uSANsK_H1_GpnfjJo3WLkTf_K6WA1yN0UMY0GlsrZtwV3NXA)
✓ Inserted user 2: StringValue(value=x1-E2ZapXftCRS0hhGsfAoMcwJ_GzBMaeBUIL1cgohfjJR1UxBhl4is1oaqTeBmBTetTRvXrskuss9wW8buYXw)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_documents_client_py, schema_products_client_ts, schema_products_client_go, batch_users, schema_employees_client_ts]

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
  {"results":[{"record":{"id":"rkQ0Ex_Hn-rHV1VVpqAVA-gbYg8wa-5zTVgvdJCUS7qrPkKiTl5vAPmJuwc6z64OFIf_4MKDVaxNKU1vCMsHrA","description":"Learn Rust programming language with hands-on examples and best practices.","views":297,"tags":"programming,rust,tutorial","title":"Rust Programming"},"score":19.8,"matched_fields":["description","tags","title"]},{"record":{"views":320,"description":"Build modern web applications using JavaScript, React, and Node.js.","title":"JavaScript Web Development","id":"-j33icJkVjXbbTM407Jc3rdoSVuO3jHuqYI5Fce6a2NfqlwG0IMR_Wgnug4DjgT8neH5teQof-XDd9H6wqTm5Q","tags":"programming,javascript,web"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,python,data-science","title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization.","id":"6Dz7jzwgPjD5ZEk84iHZwQPZbdLyQ70xo7pfRTUv5Y7Eys4dMlLfyGBPu3yCoHE9khDwprKMiBr-8n7i_T04jw","views":339},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"id":"Dx8kEOeSwkxerYSFtsV3qRPcfbaNwJfAKkSxSoc7J3UG2hTZcRkERGYQD0mZRF-tMidY2tBVL1tLhcbeUtgoZw","views":622,"title":"Machine Learning Basics","tags":"ai,machine-learning,python","description":"Introduction to machine learning algorithms and neural networks."},"score":2.7,"matched_fields":["title","tags","description"]},{"record":{"id":"6Dz7jzwgPjD5ZEk84iHZwQPZbdLyQ70xo7pfRTUv5Y7Eys4dMlLfyGBPu3yCoHE9khDwprKMiBr-8n7i_T04jw","title":"Python for Data Science","tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization.","views":339},"score":1.0,"matched_fields":["description"]},{"record":{"id":"HH90TGXY7Jo3Dy_2N10Qfdz-G97GbWGqKdvHcGHk1wFB8G3m_8ZUboVM9ZWnhidOaaJ04Av6LV1Kls0c8Zu5qA","title":"Database Design","views":74,"tags":"database,design,sql","description":"Learn database design principles, normalization, and query optimization."},"score":0.5,"matched_fields":["description"]},{"record":{"views":297,"id":"rkQ0Ex_Hn-rHV1VVpqAVA-gbYg8wa-5zTVgvdJCUS7qrPkKiTl5vAPmJuwc6z64OFIf_4MKDVaxNKU1vCMsHrA","tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=s-lBh7OMpqCAbY-jcKrtenGk_PtpBaD5aecDed1gsCwnY5Fz3DfHPO1ZufJuqlXNUdpi8V_NMrTQWbvFRYVhCQ)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767508477870},"id":"s-lBh7OMpqCAbY-jcKrtenGk_PtpBaD5aecDed1gsCwnY5Fz3DfHPO1ZufJuqlXNUdpi8V_NMrTQWbvFRYVhCQ","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-04T07:34:37.012318Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Created session: AoGV2ETq9ih_wHDgmEjg_EgzNgujHFIKDL8uN3J9rzARdCJa5tJMwlD7OuncU176W7jsmB-2x1lKf55SS8OJSw

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "RnZ8sCr2cXX4AwOe1uniXjQZsmvHelfMcl5Udw0IqAK80PSR8r4_pSloE41IiNwR-b2b8X48jxxOY4BFsocYBw"
  Responses: ["ekoDB is a high-performance database known for its intelligent caching, real-time capabilities, and AI integration. It provides several unique features to enhance the user experience and efficiency. One of the features is the AI Chat Integration, which allows you to query your database using natural language and receive AI-powered responses with relevant context. This integration makes it easier to interact with the database while ensuring accurate results. Additionally, ekoDB supports various search features, including full-text search, vector search, and hybrid search with automatic context retrieval. These features facilitate efficient and precise data retrieval from the database."]

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
✓ Created session: hT1lAcig8obFH6P8SazVCEQEthVjaOwrnLF_uMnrQ1wx3RSRGB4020Q_MYyS_ISMkfTQkdjLs02i3ri6cDqSCw

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The product available is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"hT1lAcig8obFH6P8SazVCEQEthVjaOwrnLF_uMnrQ1wx3RSRGB4020Q_MYyS_ISMkfTQkdjLs02i3ri6cDqSCw"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"saZp9sqjExiwthQgIGaqXKIof5DRyUeKziz6jsK5Pz2Fb34fqQg55rEEr03G2c8fIwisARqCywYvrbytBiZGsA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-04T06:34:55.918241Z"},"id":"e1_FEZJLxgghWcxjAlQJEdHZiq6DTUnNwyf9OIqu86Pv5Y4e3YzzFWZgu1QtPvWMB5y3cSgNX97n9PygnWoaUg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":24,"prompt_tokens":239,"total_tokens":263}},"updated_at":{"type":"DateTime","value":"2026-01-04T06:34:55.918241Z"}},{"chat_id":{"type":"String","value":"hT1lAcig8obFH6P8SazVCEQEthVjaOwrnLF_uMnrQ1wx3RSRGB4020Q_MYyS_ISMkfTQkdjLs02i3ri6cDqSCw"},"content":{"type":"String","value":"The product available is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"saZp9sqjExiwthQgIGaqXKIof5DRyUeKziz6jsK5Pz2Fb34fqQg55rEEr03G2c8fIwisARqCywYvrbytBiZGsA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-04T06:34:55.920772Z"},"id":"au04CoJKShHyHo9bfdk-7xlBeHD7lUVQxAbPobVCBEmmh-zCUniLCShURBItTY2jguBoY5Tj0nJECwTLyhKD6Q","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":24,"prompt_tokens":239,"total_tokens":263}},"updated_at":{"type":"DateTime","value":"2026-01-04T06:34:55.920772Z"}},{"chat_id":{"type":"String","value":"hT1lAcig8obFH6P8SazVCEQEthVjaOwrnLF_uMnrQ1wx3RSRGB4020Q_MYyS_ISMkfTQkdjLs02i3ri6cDqSCw"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"saZp9sqjExiwthQgIGaqXKIof5DRyUeKziz6jsK5Pz2Fb34fqQg55rEEr03G2c8fIwisARqCywYvrbytBiZGsA","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-04T06:34:56.943074Z"},"id":"PvlEiUjSeMCyTBumjxx4CAcwth41SbsLGJMrgUqvcuKUdEpfpsWln4yLd4IxBUc5MV3DIyUALExnE3IchXO0VQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":239,"total_tokens":251}},"updated_at":{"type":"DateTime","value":"2026-01-04T06:34:56.943074Z"}},{"chat_id":{"type":"String","value":"hT1lAcig8obFH6P8SazVCEQEthVjaOwrnLF_uMnrQ1wx3RSRGB4020Q_MYyS_ISMkfTQkdjLs02i3ri6cDqSCw"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"saZp9sqjExiwthQgIGaqXKIof5DRyUeKziz6jsK5Pz2Fb34fqQg55rEEr03G2c8fIwisARqCywYvrbytBiZGsA","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-04T06:34:56.945827Z"},"id":"unUhf1xQ2zmo8HdHC-PrNuBkZQYRpTXHw3o_gzt6KCRKfBEAiLe-8-jPk3sgbto12KyBwgW9DB7bS2lpwcxWqw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":239,"total_tokens":251}},"updated_at":{"type":"DateTime","value":"2026-01-04T06:34:56.945827Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: -qOqrDapUmDpvxL40unnfIqVK_dRWujHqNqwUwprl3EDaHj4U86WA1NM4TQejZsve3vQHw20BuDYiO8eUh99XA

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
✓ Created session: oy0Lo2TqtLhfKIlRzvnhS2xo3LAY5e-92LJWKt4H2JnqDyNaUFN7nLEP5__4o6D6GZECMofZVMHoDEHJsq28Zg

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the provided context, the available product is \"ekoDB\", a high-performance database product. It is priced at 99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the context provided, the available product is \"ekoDB\", which is a high-performance database product. It costs $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: QF_Zw_2Yu6RRp-Ricv76ErmMbErtehEj-G8178313rJ6PelLuImC-4S1U_J-paVvS2orrGleiVJ3Bm94KhHg8w

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

✅ Script saved: 9Pp6l4RX_kIji9rYWKkBa0fkfZ_EBgBkSScAJ59oScJuuILq9BfxDcuhdVfxetCuDNnGDSn2PuZKobFsFFBajw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"avg_score":60.0,"status":"active"}
   {"status":"inactive","avg_score":50.0,"count":5}
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
Created Alice: $1000 - ID: EZDL1oLbDdDbTGSOjcba_hzrXxHlvevz0ROSP_KQ_VCdCxWk0rsDmQAz6PHaKgXqeNCSYfwjP2I2fRU22MCayg
Created Bob: $500 - ID: iiF7sQFbuyslaO_m4PJ4OpeORH5WnDEfstbUKZLXdCVRGIry3SbHuPmBBfiiIW54c1I2Pd2JQMubTZKEaBZuEw

=== Example 1: Begin Transaction ===
Transaction ID: e37789f3-3c13-45a9-8e8d-a019d20024e1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: fffa2fa3-a93d-4cf7-878c-fdf13c07805d
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
