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
✓ Inserted user: Record(fields={id=StringValue(value=9avcwb2WzqPOoZNsWji59MZP3PVfTw6fJE695jJKXgyukmB8riRZGc27HrWJGrrQ48F9hTQadIBuxjBb8QCppw)})
  User ID: 9avcwb2WzqPOoZNsWji59MZP3PVfTw6fJE695jJKXgyukmB8riRZGc27HrWJGrrQ48F9hTQadIBuxjBb8QCppw

=== Read ===
✓ Found user by ID: Record(fields={price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), id=StringValue(value=9avcwb2WzqPOoZNsWji59MZP3PVfTw6fJE695jJKXgyukmB8riRZGc27HrWJGrrQ48F9hTQadIBuxjBb8QCppw), created_at=ObjectValue(value={value=StringValue(value=Sat Jan 03 18:56:27 EST 2026), type=StringValue(value=String)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sat Jan 03 18:56:27 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [price, embedding, data, id, created_at, email, name, tags, categories, active, metadata, user_id, age]

=== Update ===
✓ Updated user: Record(fields={age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), created_at=ObjectValue(value={value=StringValue(value=Sat Jan 03 18:56:27 EST 2026), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), id=StringValue(value=9avcwb2WzqPOoZNsWji59MZP3PVfTw6fJE695jJKXgyukmB8riRZGc27HrWJGrrQ48F9hTQadIBuxjBb8QCppw), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sat Jan 03 18:56:27 EST 2026)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), id=StringValue(value=9avcwb2WzqPOoZNsWji59MZP3PVfTw6fJE695jJKXgyukmB8riRZGc27HrWJGrrQ48F9hTQadIBuxjBb8QCppw)})

=== Delete ===
✓ Deleted user with ID: 9avcwb2WzqPOoZNsWji59MZP3PVfTw6fJE695jJKXgyukmB8riRZGc27HrWJGrrQ48F9hTQadIBuxjBb8QCppw

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
  IDs: Sv7fRPoWskaSh7MOY8DeRKZq1zvKqc5eCeQPTV7mq4Tdag2NiZznQgOeuaZnWisxMSrQ3dG34EkoDDKfsAKyZw, SyhLpjHsT4e09N9k_Z2V4PCv0HAIwz2ktPdw3p1zeokxzFaHv8AxKnYlOwuuQOC7kBSYIynAt43eFd-H2i3-ZA, 6mqmWGBbEZKa3HecFT181IFqkgPLh8I2J7o32jOaj5mEcSeesU1u4tCNVnhqwDRKhOjcUdwODDEd_mvSYgHKng...

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
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"created_at":1767484600356,"user_id":"123"}
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
  - chat_configurations__ek0_testing
  - ttl_cache
  - schema_documents_client_py
  - schema_users_client_go
  - schema_products_client_go

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-03T23:56:44.601002Z","last_modified":"2026-01-03T23:56:44.601002Z","bypass_ripple":false,"primary_key_alias":"id"}

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
  Document ID: f6x8ipn2I5sVEOKeviOtTWey53RnWoFgeCkfDiJg23sfuTVyA6aoXhx9GHvnwiq1JmIXnYrb2Sd-B1C1pOMnvA

=== Verify Document Exists ===
✓ Document found: [session_id, user_id, id, created_at, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: YPbOOfyf1hb8aeS82knXRdcLOFp1iEqMtnibc3HZI_Pqhmv0kaHDXb6PGxnTXVvNTTUMcajEhpaApK1bihgJaA

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
  {"payload":{"data":[{"id":"XyWZtPLmGvgb9g6C12H9eKS-T4d2uLphbqDNsYKptbVo-OEqa6CgSBqhjGKGlRIlrgQJ08N6Zvr4Gf4DNXpNdg","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
✓ Inserted user 1: StringValue(value=1kTn_oK6KiXvCAWB4OPplxlvnweFNEeEFeHuInZu1z4wUOIw6SG3UCJKsufTnVVSuLEommw5evP47FkQwJ7BNQ)
✓ Inserted user 2: StringValue(value=WJT38RqAihJSmu0HEZHOMP86bz-7FSkWF3RlCXPfgM6XYTBXPcLISq2Cq1D99B3RKp2Wsx0bA9urV8Bs7sXL1Q)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [chat_configurations__ek0_testing, ttl_cache, schema_documents_client_py, schema_users_client_go, schema_products_client_go]

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
  {"results":[{"record":{"id":"dMCXNBQC4VAcp61_2b82NoJtnJZKaPF2wtbVaCUNQtLkhDmS22POHf8jJfWfmwXeDcyH-EJgBRnZgiW4Blu8aw","views":811,"description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","tags":"programming,rust,tutorial"},"score":19.8,"matched_fields":["description","tags","title"]},{"record":{"title":"JavaScript Web Development","id":"UX-DJtsX-7NlzrSaIt88EBzR92dP6ouoG_5VLh27UOzgda9NrevXhUtm6W1ysDTh-qD76hli_jY5Nga7lztk_A","views":98,"description":"Build modern web applications using JavaScript, React, and Node.js.","tags":"programming,javascript,web"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,python,data-science","title":"Python for Data Science","id":"9ZFsUa7nwEJRbx3Kk2MnvZ_YX7GNGS3i5yMk6F79L8tYYZ2aYVNtgEWiIbWctGOQhDWKQaQclVN7VI07quKmdA","views":534,"description":"Master Python for data analysis, machine learning, and visualization."},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"views":363,"tags":"ai,machine-learning,python","description":"Introduction to machine learning algorithms and neural networks.","id":"eXGkyzXIbd_i8uVtyEMb3ihrHOrdBYkAr0yk0G6BBCjLpAfB-MVkqm_fL9oNkfFA-FcvuwieM_TR9O9oOy6ddQ","title":"Machine Learning Basics"},"score":2.7,"matched_fields":["title","tags","description"]},{"record":{"id":"9ZFsUa7nwEJRbx3Kk2MnvZ_YX7GNGS3i5yMk6F79L8tYYZ2aYVNtgEWiIbWctGOQhDWKQaQclVN7VI07quKmdA","tags":"programming,python,data-science","views":534,"title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization."},"score":1.0,"matched_fields":["description"]},{"record":{"views":798,"tags":"database,design,sql","id":"31jK_X2y-2LE5AIl2ebh5yzRGf_2MeZOq-0QKWkkZUcHD0lH8jW25ysVNz7KtYoL_F5lVPK9PPDx9BZkvQMsVQ","description":"Learn database design principles, normalization, and query optimization.","title":"Database Design"},"score":0.5,"matched_fields":["description"]},{"record":{"views":811,"description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","tags":"programming,rust,tutorial","id":"dMCXNBQC4VAcp61_2b82NoJtnJZKaPF2wtbVaCUNQtLkhDmS22POHf8jJfWfmwXeDcyH-EJgBRnZgiW4Blu8aw"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted document with TTL: StringValue(value=RoBNWzKKjSRbVQfAPQLCVLQNBFftOO_co95MnieYIGDAUCHI0JVDWVlsR8B0DEjE9zDAXU4CfbxBfmfB0NEO_g)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767484625727},"id":"RoBNWzKKjSRbVQfAPQLCVLQNBFftOO_co95MnieYIGDAUCHI0JVDWVlsR8B0DEjE9zDAXU4CfbxBfmfB0NEO_g","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-04T00:57:04.859161Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Created session: 8SpPtv3cwJSd3xsRpUIzj0mNUTDsgG8nI5wxfWokyjmtYUSb6sDYkuBl-kCq6_RB7nXNspuQusgJJ9RWuMwgUQ

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "a2XTWZ_gGnOfdroVp00eujhEG9Z1gll7Z6mcL561kPo65YzaEOMFQeTW4PBJVQr2edC2SMokNjd4hPhaYdkNrQ"
  Responses: ["ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. Some of its main features include:\n\n1. AI Chat Integration: This feature allows you to query your database using natural language and get AI-powered responses with relevant context. \n\n2. Search Features: ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval. These features make it easier to find specific information within the database."]

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
✓ Created session: JDHytImPeD8nBwBD7hlrBnigPa4pqsO-0nzCx62tmiKhWg1MHBfSICSYcaG_X3eTzyoScgk6o2GQwyYNV-dMLQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Based on the information provided, the available product is ekoDB, a high-performance database product with AI capabilities. The price for this product is $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"JDHytImPeD8nBwBD7hlrBnigPa4pqsO-0nzCx62tmiKhWg1MHBfSICSYcaG_X3eTzyoScgk6o2GQwyYNV-dMLQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"S1yEU3LWbs1t7t9Tx19Xj36RoP5yHZl_K7KeYpq-NuuPnEks8GzDvO444PheKC0il_8c5VOdorSftf9ELwN8iA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-03T23:57:22.726197Z"},"id":"09zruanWWuyP522Ejqp6HG6Q6THP_YPM0bABoakhZ4BRanBYpQign7tdxneSYpl96i2-RgHe3JU-qDAAhSPLjQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":32,"prompt_tokens":242,"total_tokens":274}},"updated_at":{"type":"DateTime","value":"2026-01-03T23:57:22.726197Z"}},{"chat_id":{"type":"String","value":"JDHytImPeD8nBwBD7hlrBnigPa4pqsO-0nzCx62tmiKhWg1MHBfSICSYcaG_X3eTzyoScgk6o2GQwyYNV-dMLQ"},"content":{"type":"String","value":"Based on the information provided, the available product is ekoDB, a high-performance database product with AI capabilities. The price for this product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"S1yEU3LWbs1t7t9Tx19Xj36RoP5yHZl_K7KeYpq-NuuPnEks8GzDvO444PheKC0il_8c5VOdorSftf9ELwN8iA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-03T23:57:22.730324Z"},"id":"RWOpaoIaQlQDRlbdaKZhAo9EoYEf1D8-noPieH6zGXLNvLI05qLMZ5xLnlHHsrgoa5i0R30VNgDR118ApwuuMQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":32,"prompt_tokens":242,"total_tokens":274}},"updated_at":{"type":"DateTime","value":"2026-01-03T23:57:22.730324Z"}},{"chat_id":{"type":"String","value":"JDHytImPeD8nBwBD7hlrBnigPa4pqsO-0nzCx62tmiKhWg1MHBfSICSYcaG_X3eTzyoScgk6o2GQwyYNV-dMLQ"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"S1yEU3LWbs1t7t9Tx19Xj36RoP5yHZl_K7KeYpq-NuuPnEks8GzDvO444PheKC0il_8c5VOdorSftf9ELwN8iA","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-03T23:57:23.513431Z"},"id":"6oKRRjewuTuCjhMaIwj7MhxW_oBDWdN9fBi2wKg4E7u-BbUbKbTnIPieBZ2aXfFHYUTqz8H22YFV_b3efuyWzg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":242,"total_tokens":254}},"updated_at":{"type":"DateTime","value":"2026-01-03T23:57:23.513431Z"}},{"chat_id":{"type":"String","value":"JDHytImPeD8nBwBD7hlrBnigPa4pqsO-0nzCx62tmiKhWg1MHBfSICSYcaG_X3eTzyoScgk6o2GQwyYNV-dMLQ"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"S1yEU3LWbs1t7t9Tx19Xj36RoP5yHZl_K7KeYpq-NuuPnEks8GzDvO444PheKC0il_8c5VOdorSftf9ELwN8iA","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-03T23:57:23.517534Z"},"id":"_qb_7dLn9MAROm_JYSG6RHKlDrdz9im_vQM9_wLe3vyEgganAckM_RbdCJHVaiL6aeeeuEcQA9aUDyvhlvEgEA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":242,"total_tokens":254}},"updated_at":{"type":"DateTime","value":"2026-01-03T23:57:23.517534Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: Ly1oztPgN77vzMgmT6VOakL7wqhN6nEdmbq6kL3QIY3f-2DrlgqPvJKel4Cs9FNYHWDryGeDnbZL0bactAa9kw

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
✓ Created session: 1xn6wLlYnWZvsIsaGcBNTfuomCNL9ttoRtiBfVpFNoSuTnMUVlUJ7pvsbP8CPCAsf49h0d6WvIl52xzJYpKO-w

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information provided, the available product is ekoDB, which is a high-performance database product."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The available product based on the provided context is ekoDB, a high-performance database product. It is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: icKUeFOq6a-1Zrapw-ZR_uHoCXvyDcoi_yZRN-i83ZUh0cPKWdspPX892bXdhV_XtMolJ_Qv-2ILQd7UpxUj8g

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

✅ Script saved: MWCSWskkBe0mi9-OAGzx7RjDcJ1xHlP6jZlRzJKMNLHKSZLa2Bb0j0ukhdGyalyYEpvbXk9lB2R0pL0CGzeIbg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","avg_score":60.0,"count":5}
   {"count":5,"avg_score":50.0,"status":"inactive"}
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
Created Alice: $1000 - ID: anP3HCboqjozxp5ySPVacF0WKg4vKrhpSnU7FWGFcAMm_nDKQ-_gEvChb2AtbTtRii5QG7oz4py665uyqy9qXg
Created Bob: $500 - ID: zzG1JPdbcZE91shIwUl4-iaX4ysx3o6o1jWsVhS-SylnSB53IBGf9O5xxdrYFV9zvynqQRd9iBeqEBo8AcHXow

=== Example 1: Begin Transaction ===
Transaction ID: 03e71918-3873-4fe7-bfb3-54c244d3f1ee

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 83fb6ed5-ce2b-418b-92ab-09e668dfed8a
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
