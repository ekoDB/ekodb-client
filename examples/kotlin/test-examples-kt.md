make test-examples-kt

🟣 Building Kotlin client library... To honour the JVM settings for this build a
single-use Daemon process will be forked. For more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :compileKotlin w:
> file:///Users/tek/Development/ekoDB/ekodb-client/ekodb-client-kt/src/main/kotlin/io/ekodb/client/QueryBuilder.kt:304:76
> Unchecked cast: Any to Map<String, Any> w:
> file:///Users/tek/Development/ekoDB/ekodb-client/ekodb-client-kt/src/main/kotlin/io/ekodb/client/QueryBuilder.kt:307:72
> Unchecked cast: Any? to Map<String, Any>

BUILD SUCCESSFUL in 6s 3 actionable tasks: 3 executed ✅ Kotlin client built! 🧪
Running Kotlin client library examples... To honour the JVM settings for this
build a single-use Daemon process will be forked. For more on this, please refer
to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :compileKotlin w:
> file:///Users/tek/Development/ekoDB/ekodb-client/examples/kotlin/examples/ClientJoins.kt:69:39
> Unnecessary non-null assertion (!!) on a non-null receiver of type String w:
> file:///Users/tek/Development/ekoDB/ekodb-client/examples/kotlin/examples/ClientJoins.kt:76:39
> Unnecessary non-null assertion (!!) on a non-null receiver of type String w:
> file:///Users/tek/Development/ekoDB/ekodb-client/examples/kotlin/examples/ClientJoins.kt:83:39
> Unnecessary non-null assertion (!!) on a non-null receiver of type String w:
> file:///Users/tek/Development/ekoDB/ekodb-client/examples/kotlin/examples/ClientJoins.kt:138:34
> Unnecessary non-null assertion (!!) on a non-null receiver of type String w:
> file:///Users/tek/Development/ekoDB/ekodb-client/examples/kotlin/examples/ClientJoins.kt:143:34
> Unnecessary non-null assertion (!!) on a non-null receiver of type String

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Simple CRUD Example ===

=== Create === ✓ Inserted user:
Record(fields={id=StringValue(value=cEdcRgfVg5QzS6R9N1Y96Oycg-XYNbUVeI_Xp81CedhrNUV2tgbKTM4q7Uky_LOdueRmSYnZLQKaENfnkLIZBw)})
User ID:
cEdcRgfVg5QzS6R9N1Y96Oycg-XYNbUVeI_Xp81CedhrNUV2tgbKTM4q7Uky_LOdueRmSYnZLQKaENfnkLIZBw

=== Read === ✓ Found user by ID:
Record(fields={active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
id=StringValue(value=cEdcRgfVg5QzS6R9N1Y96Oycg-XYNbUVeI_Xp81CedhrNUV2tgbKTM4q7Uky_LOdueRmSYnZLQKaENfnkLIZBw),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)})}) === Update === ✓ Updated user:
Record(fields={active=ObjectValue(value={value=BooleanValue(value=true),
type=StringValue(value=Boolean)}), city=StringValue(value=San Francisco),
email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)}), age=IntegerValue(value=29),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
id=StringValue(value=cEdcRgfVg5QzS6R9N1Y96Oycg-XYNbUVeI_Xp81CedhrNUV2tgbKTM4q7Uky_LOdueRmSYnZLQKaENfnkLIZBw)})
=== Query === ✓ Found 1 users matching query

- Record(fields={id=StringValue(value=cEdcRgfVg5QzS6R9N1Y96Oycg-XYNbUVeI_Xp81CedhrNUV2tgbKTM4q7Uky_LOdueRmSYnZLQKaENfnkLIZBw),
  active=ObjectValue(value={value=BooleanValue(value=true),
  type=StringValue(value=Boolean)}),
  name=ObjectValue(value={value=StringValue(value=Alice Johnson),
  type=StringValue(value=String)}),
  age=ObjectValue(value={value=IntegerValue(value=29),
  type=StringValue(value=Integer)}),
  email=ObjectValue(value={value=StringValue(value=alice@example.com),
  type=StringValue(value=String)}),
  city=ObjectValue(value={value=StringValue(value=San Francisco),
  type=StringValue(value=String)})}) === Delete === ✓ Deleted user with ID:
  cEdcRgfVg5QzS6R9N1Y96Oycg-XYNbUVeI_Xp81CedhrNUV2tgbKTM4q7Uky_LOdueRmSYnZLQKaENfnkLIZBw

✓ Confirmed user was deleted === Cleanup === ✓ Deleted collection:
kotlin_users_example === Example Complete === BUILD SUCCESSFUL in 8s 3
actionable tasks: 3 executed To honour the JVM settings for this build a
single-use Daemon process will be forked. For more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert === ✓ Inserted 5 records IDs:
PCKWX2IYppNeONDCK0W2j_vZwNqKjLp2-os205B9RipY3iQyath56m2RridIanZVuK9m9Qyh-S5YWQ6lsBbRCA,
qIubEGdqlbZ91RcorEGBrBOx61JvY2KYCxKFmcDUO_rFScNAvov7bdKt3RYBb7WvJotm942iYaXWeJNkhXvZ-A,
yiC7VQhSPtNihIw-PDS8TIQYntutmYq8XfF9Wu6N-NOKlRYhBPpC7li3SPJbsGgZnJJO3VuerpuWVzGaoBHpaw...

=== Batch Update === ✓ Updated 3 records

=== Batch Delete === ✓ Deleted 2 records === Cleanup === ✓ Deleted collection:
kotlin_batch_example === Example Complete ===

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Query Builder Example === === Inserting Sample Data ===
> ✓ Inserted 5 records

                                                                                                          === Query 1: Equality (city = 'NYC') ===

✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) === ✓ Found 3 records with age 25-31
=== Query 3: Sort by score (descending) === ✓ Top 3 scores: - Score:
ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=95)}) - Score:
ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=92)}) - Score:
ObjectValue(value={value=IntegerValue(value=88),
type=StringValue(value=Integer)}) === Query 4: Complex (score > 80 AND
age >= 25) === ✓ Found 4 high-scoring adults === Query 5: IN (city IN ['NYC',
'LA']) === ✓ Found 4 records in NYC or LA === Query 6: Pagination (skip 2,
limit 2) === ✓ Page 2 (2 records): -
ObjectValue(value={value=StringValue(value=Charlie),
type=StringValue(value=String)}) -
ObjectValue(value={value=StringValue(value=Diana),
type=StringValue(value=String)}) === Query 7: Contains (name contains 'a') === ✓
Found 2 names containing 'a'

=== Cleanup === ✓ Deleted collection: kotlin_query_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - KV Operations Example ===

=== KV Set === ✓ Set key: user:123

=== KV Get === ✓ Retrieved value:
{"role":"admin","name":"Alice","email":"alice@example.com"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"created_at":1760511901979,"user_id":"123"} (Will expire in 10 seconds)

=== Set Multiple Keys === ✓ Set 3 configuration keys

=== KV Delete === ✓ Deleted key: user:123

=== Verify Deletion === ✓ Confirmed key was deleted

=== Cleanup === ✓ Cleaned up test keys

=== Example Complete ===

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Collection Management Example ===

=== List Collections === ✓ Found 0 collections

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-10-15T07:05:05.872413Z","last_modified":"2025-10-15T07:05:05.872413Z","bypass_ripple":false}

=== Cleanup === ✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL === ✓ Inserted document with 10s TTL Document ID:
bxLe1Ep5FawFREyQvlAYv2DrcUloiT8hCCWmkPOIsONCC9MjAG873HCUXbJR2Zta32AOckshzHw7Qed0FE-yAA

=== Verify Document Exists === ✓ Document found: [id, created_at, ttl_duration,
user_id, session_id]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
ea4j7YrMuRtu1yuvc2xcDjRDEYJVKI7TrxCUAqLW1jZXNrE2XhVM0574jeJJB91eoEpSJIeBCZQAUCw3nkbMvg

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists:
[ttl_duration, value, id, cache_key]

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client === ✓ WebSocket client created

=== Connect to WebSocket === ✓ Connected to WebSocket

=== Insert Test Record === ✓ Inserted test record

=== Find All via WebSocket === ✓ WebSocket findAll result:
{"payload":{"data":[{"id":"XJAZewF57F_WrA0tPG3mpqSfNilkTIERJtEmpYDjcJpiZ07ofMSsIxVcw5WP7oQkTX2jD40XNOuDjwfeooySVQ","name":{"type":"String","value":"Test
User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

=== Close WebSocket === ✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema === ✓ Created collection
'kotlin_schema_example' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
StringValue(value=GlWLy5licKJ_qmfybdTewUzUu9gYv4UfUH0m67oex6KWxTQ7GIma_5z7VZUG755lZzvgm8BwJVDMojwy0KnJ0g)
✓ Inserted user 2:
StringValue(value=KYE_XxyGYpOx9TyMbBQdgMGKGSZVBe-ruuBajOOxIcgQHKrZ9rLdVuWLK4RsXLfw6-KzL-mGHNcsfkHafc10uQ)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 1 Sample:
[kotlin_schema_example]

=== Cleanup === ✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Search results for 'programming':
{"results":[{"record":{"tags":"programming,rust,tutorial","description":"Learn
Rust programming language with hands-on examples and best
practices.","title":"Rust
Programming","views":465,"id":"19ogLjcJPxayuCcnuhIIzt8jzo5DTJgCktaTVyScBbj3w-7I9fKGN30OwoSUSB9EiiMyMzvx0DDUCEa1EEko0w"},"score":19.8,"matched_fields":["description","title","tags"]},{"record":{"tags":"programming,python,data-science","views":735,"id":"N2u-Z82Vg8lWQvfPpUfq0RGMTcQNNJDqVytrZWxtgE6ssx94adUFo8VdyfN9tyy5QBL8bWKT-raJYymBwmrOSg","title":"Python
for Data Science","description":"Master Python for data analysis, machine
learning, and
visualization."},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":"Build
modern web applications using JavaScript, React, and
Node.js.","id":"j2bOAk65eSR3DIU9S0Sdhn11R12nIrQZX87-DNRNAhw2KCXnNmE_hR9DuK9xBxMPJbb4AtaKZpIurAqQmcnVAQ","views":512,"title":"JavaScript
Web
Development","tags":"programming,javascript,web"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"views":281,"description":"Introduction to machine
learning algorithms and neural
networks.","id":"B07cDhYBaReG7NQkQR-6QT-Ho1lfepmumRo_8mIYbg5bgUGIMYoAxpTAdAng0xg20gRXx3-OXGWnP1a2tjsB6w","tags":"ai,machine-learning,python","title":"Machine
Learning
Basics"},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"description":"Master
Python for data analysis, machine learning, and visualization.","title":"Python
for Data
Science","id":"N2u-Z82Vg8lWQvfPpUfq0RGMTcQNNJDqVytrZWxtgE6ssx94adUFo8VdyfN9tyy5QBL8bWKT-raJYymBwmrOSg","tags":"programming,python,data-science","views":735},"score":1.0,"matched_fields":["description"]},{"record":{"title":"Database
Design","tags":"database,design,sql","description":"Learn database design
principles, normalization, and query
optimization.","id":"iyZwGGXJB3qx5g2HgJe9mI0tA-6b2w7FsxiC7tZNLARveriSxdFqaPKyvYxBwhm_AiB8HmiVL90fGWuCsJdwPQ","views":512},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn
Rust programming language with hands-on examples and best
practices.","id":"19ogLjcJPxayuCcnuhIIzt8jzo5DTJgCktaTVyScBbj3w-7I9fKGN30OwoSUSB9EiiMyMzvx0DDUCEa1EEko0w","views":465,"tags":"programming,rust,tutorial","title":"Rust
Programming"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup === ✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
StringValue(value=6NakwV_oq8kF7E5imRzhc3FovGrFahkplEgP5AlrL1sn1qPvgbEpiNSFjNaaEJXxvwQZi9OVPdRWvrYdqFeGxA)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1760511925504},"id":"6NakwV_oq8kF7E5imRzhc3FovGrFahkplEgP5AlrL1sn1qPvgbEpiNSFjNaaEJXxvwQZi9OVPdRWvrYdqFeGxA","name":{"type":"String","value":"WebSocket
TTL
Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Join Operations Example ===

=== Setting up sample data === ✓ Sample data created

=== Example 1: Single collection join (users with departments) === ✓ Found 2
users with department data

- Alice Johnson
- Bob Smith

=== Example 2: Join with filtering === ✓ Found 2 users in Engineering

- Alice Johnson
- Bob Smith

=== Example 3: Multi-collection join (users with departments and profiles) === ✓
Found 2 users with department join

- Alice Johnson
- Bob Smith

=== Example 4: Join orders with user data === ✓ Found 3 completed orders

- Keyboard ($75)
- Mouse ($25)
- Laptop ($1200)

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com)
- Bob Smith (bob@example.com)

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully

BUILD SUCCESSFUL in 3s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
nE4JW5qmSaVqoq59BoBplNyvEXO5led_sm6_Vg1tyPw0rBeN168ZDd3Mo3E-UVUm_CqrZdeOdWhXHyRlYntq2g

=== Sending Chat Message === ✓ Chat response: Message ID:
"bMoP22EkaXq4oJgZPb54kk-hL1r7cAjcQTeTCeHb7iKLruG-sAuRBvkgMLGuK7rCJvHLLpR4infYL9NcNaHBog"
Responses: ["ekoDB is a high-performance database that comes with intelligent
caching, real-time capabilities, and AI integration. Some of its notable
features include an AI Chat Integration, which allows you to query your database
using natural language and receive AI-powered responses with relevant context.
Additionally, ekoDB supports various search functionalities inclusive of
full-text search, vector search, and hybrid search with automatic context
retrieval."]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 8s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
s5v-ilIyfpV7RMcaMRYFsOXyPh7gI1J-bokKG0vIskgd_TqHzR2FaObmh_YWRVDUiYvX88WZTXaiaObUfoKp_g

=== Sending Messages === ✓ Message 1 sent Responses: ["Based on the provided
context, one available product is ekoDB. This is a high-performance database
product with AI capabilities. It is priced at $99."]

✓ Message 2 sent Responses: ["As an assistant, I'm sorry but I can't provide the
information you're asking for because the context doesn't mention any specific
product or service. Could you please provide more details?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"s5v-ilIyfpV7RMcaMRYFsOXyPh7gI1J-bokKG0vIskgd_TqHzR2FaObmh_YWRVDUiYvX88WZTXaiaObUfoKp_g"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":{"type":"String","value":"Bx8eBoFONwvz_2xdv7zeFth5HoQk0JI0M6hZBMqugOe8-uZp3nDCdl21AcpwSBNPAG6HHBPN5xL5wakhp2dVeA"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-10-15T07:05:44.238276Z"},"id":"IHVUjf3TBRKGPTsKFnsMavCHlYbHLfPBBVMRZ4W9RYag4n1X5_TtJjXSHIYqmnlAg8I6ZIOCY0FVPBueNn0t1w","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":32},"prompt_tokens":{"type":"Integer","value":169},"total_tokens":{"type":"Integer","value":201}}},"updated_at":{"type":"DateTime","value":"2025-10-15T07:05:44.238276Z"}},{"chat_id":{"type":"String","value":"s5v-ilIyfpV7RMcaMRYFsOXyPh7gI1J-bokKG0vIskgd_TqHzR2FaObmh_YWRVDUiYvX88WZTXaiaObUfoKp_g"},"content":{"type":"String","value":"Based
on the provided context, one available product is ekoDB. This is a
high-performance database product with AI capabilities. It is priced at
$99."},"context_snippets":{"type":"Array","value":[{"type":"Object","value":{"collection":{"type":"String","value":"kotlin_chat_sessions_example"},"matched_fields":{"type":"Array","value":[{"type":"String","value":"description"}]},"record":{"type":"Object","value":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":{"type":"String","value":"Bx8eBoFONwvz_2xdv7zeFth5HoQk0JI0M6hZBMqugOe8-uZp3nDCdl21AcpwSBNPAG6HHBPN5xL5wakhp2dVeA"},"price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}}},"score":{"type":"Float","value":0.25}}}]},"created_at":{"type":"DateTime","value":"2025-10-15T07:05:44.240337Z"},"id":"w0IaosPICknf4h9Y8TlBnnjqFjOhBOXIXtSpNkshkEEbrJDFh1XA6V45df5gISReiCxqagpFZYnaQgZi0HF_Lg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":32},"prompt_tokens":{"type":"Integer","value":169},"total_tokens":{"type":"Integer","value":201}}},"updated_at":{"type":"DateTime","value":"2025-10-15T07:05:44.240337Z"}},{"chat_id":{"type":"String","value":"s5v-ilIyfpV7RMcaMRYFsOXyPh7gI1J-bokKG0vIskgd_TqHzR2FaObmh_YWRVDUiYvX88WZTXaiaObUfoKp_g"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-10-15T07:05:46.073621Z"},"id":"d69yZQ86kTdMNLGCZoNlQ2dTLdSNFHlKaaI5t_rbUQTRGSMoCluMUFUtUygo9c3aXHkTX8diDUOW09LDdZpvYw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":37},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":100}}},"updated_at":{"type":"DateTime","value":"2025-10-15T07:05:46.073621Z"}},{"chat_id":{"type":"String","value":"s5v-ilIyfpV7RMcaMRYFsOXyPh7gI1J-bokKG0vIskgd_TqHzR2FaObmh_YWRVDUiYvX88WZTXaiaObUfoKp_g"},"content":{"type":"String","value":"As
an assistant, I'm sorry but I can't provide the information you're asking for
because the context doesn't mention any specific product or service. Could you
please provide more
details?"},"created_at":{"type":"DateTime","value":"2025-10-15T07:05:46.075345Z"},"id":"XFvyrPLjjzDmnRG-XhLJVZP8D_wrOknFbSv3I7EWpwsUz0SnvqlpBa03LUPF5jmreDJroHSxCGX3GLf2tVEWIw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":{"type":"Integer","value":37},"prompt_tokens":{"type":"Integer","value":63},"total_tokens":{"type":"Integer","value":100}}},"updated_at":{"type":"DateTime","value":"2025-10-15T07:05:46.075345Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 1

=== Branching Session === ✓ Created branched session:
Rx7IA0B8niqK1mfUQKCY-loK6SxqSDG9c4uW-aBd2vOu4ZYelzR4bsFQ_cofs6D4-X1e55kt2NER2USkO5LQWw

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 6s 3 actionable tasks: 2 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.5/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :run SLF4J: Failed to load class "org.slf4j.impl.StaticLoggerBinder".
> SLF4J: Defaulting to no-operation (NOP) logger implementation SLF4J: See
> http://www.slf4j.org/codes.html#StaticLoggerBinder for further details. ===
> ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
\_onrSis7r4ZVo4TS2Sm3d45QK-YST_WdEEMWp9cP-BaktsoTZ3p0TPI92fwaRfsWI3DBGhtm2nP5zFdvJ3_Cvg

=== Sending Initial Message === ✓ Message sent Responses: ["Based on the context
provided, the available product is ekoDB. It's a high-performance database
product priced at $99."]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses:
["Based on the provided context, the available product is a high-performance
database product named ekoDB. It costs $99."]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
FOuha0tk0cItOoQhjUdtW0yFgsuXca1xHys2xOqCMtGybv6jH9X4jGEQhhnIPpfYpsm-y4NoLSF2fZHV-lHdYQ

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 9s 3 actionable tasks: 2 executed, 1 up-to-date ✅ Kotlin
client examples complete! ✅ All Kotlin integration tests complete!
