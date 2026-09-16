make test-examples-kotlin
🟣 Building Kotlin client library...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
> Task :compileTestKotlin UP-TO-DATE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test
> Task :runKtlintCheckOverTestSourceSet
> Task :ktlintTestSourceSetCheck
> Task :check
> Task :build

[Incubating] Problems report is available at: file://ekoDB/ekodb-client/ekodb-client-kt/build/reports/problems/problems-report.html

Deprecated Gradle features were used in this build, making it incompatible with Gradle 10.

You can use '--warning-mode all' to show the individual deprecation warnings and determine if they come from your own scripts or plugins.

For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/command_line_interface.html#sec:command_line_warnings in the Gradle documentation.

BUILD SUCCESSFUL in 13s
11 actionable tasks: 3 executed, 8 up-to-date
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
✅ Kotlin client built!
bash: warning: setlocale: LC_ALL: cannot change locale (C.UTF-8): No such file or directory
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - Advanced CRUD Example ===

--- Inserting base record ---
Inserted: 8UbsmV80IkDd1tN1BM-oZq7BZQfMOrzIleI2iS5tSVqPXAq3dz8MvRDZNoFZpO0PLjiJ8oVI0-7f2bku6SatHA

--- updateWithAction: increment views ---
After increment: EkoRecord(fields={views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example)])}), id=StringValue(value=8UbsmV80IkDd1tN1BM-oZq7BZQfMOrzIleI2iS5tSVqPXAq3dz8MvRDZNoFZpO0PLjiJ8oVI0-7f2bku6SatHA), name=ObjectValue(value={value=StringValue(value=Counter Record), type=StringValue(value=String)}), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)})})

--- updateWithAction: push to tags ---
After push: EkoRecord(fields={score=ObjectValue(value={value=FloatValue(value=100.0), type=StringValue(value=Float)}), views=ObjectValue(value={value=IntegerValue(value=15), type=StringValue(value=Integer)}), id=StringValue(value=8UbsmV80IkDd1tN1BM-oZq7BZQfMOrzIleI2iS5tSVqPXAq3dz8MvRDZNoFZpO0PLjiJ8oVI0-7f2bku6SatHA), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced)]), type=StringValue(value=Array)})})

--- updateWithActionSequence: multiple operations ---
After sequence: EkoRecord(fields={score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=75.0)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced), StringValue(value=sequenced)]), type=StringValue(value=Array)}), views=ObjectValue(value={value=IntegerValue(value=115), type=StringValue(value=Integer)}), id=StringValue(value=8UbsmV80IkDd1tN1BM-oZq7BZQfMOrzIleI2iS5tSVqPXAq3dz8MvRDZNoFZpO0PLjiJ8oVI0-7f2bku6SatHA)})

--- Cleanup ---
Deleted collection: kotlin_advanced_crud_example

=== Example Complete ===

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
? Inserted 5 records
  IDs: J7u8ChqppBfuKEBeYqzNDUe3ek-0pPIgxdRUR770NB0VlhyNEP_dLo2DyDTiVGdydETM0QxZJ-UcwIvFJiTBOw, KTcWmXfGA9kbbetmJ25pMJl04btfIBSXYBD0JDLhx-uygL5K3y9XbsXNM9ITpBi4Fbh5ht9qI0IV6xePTS4N5g, gyjNbyqJCizfh0OU9PdldARIQKw994Rv4hcyK7WKTarxbXau3As08sfDefS0wf6AmImyzfXkXBTLNRGZqT17mg...

=== Batch Update ===
? Updated 3 records

=== Batch Delete ===
? Deleted 2 records

=== Cleanup ===
? Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
? Inserted sample product

=== Creating Chat Session ===
? Created session: 4I0UdfqNcdqtoUkD2pTT0Lg80yq8iMn3Fi0yuonQzTaTnn8nqG_cBsb_jku4YXLB2uVhrZhYqtsUPYR1gqQHng

=== Sending Initial Message ===
? Message sent
  Responses: ["The available product is:\n\n- **Name:** ekoDB\n- **Description:** High-performance database product\n- **Price:** $99\n\nIf you need more information or have other questions, feel free to ask!"]

? Second message sent

=== Regenerating AI Response ===
? AI response regenerated
  New responses: ["The price of ekoDB is $99. If you have any other questions or need more information, feel free to ask!"]

=== Updating Message ===
? Updated message content

=== Toggling Forgotten Status ===
? Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
? Created second session: kyZrtql3xoKunqJ4L8bITuiX46MMo9wmXuXx7R01Cn07BtB259BtnQ-rxfIzYzOmVi6Ddi2Yn1_8ZEIjz_gfrg

=== Merging Sessions ===
? Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
? Deleted message

=== Cleanup ===
? Deleted chat session: kyZrtql3xoKunqJ4L8bITuiX46MMo9wmXuXx7R01Cn07BtB259BtnQ-rxfIzYzOmVi6Ddi2Yn1_8ZEIjz_gfrg
? Deleted chat session: 4I0UdfqNcdqtoUkD2pTT0Lg80yq8iMn3Fi0yuonQzTaTnn8nqG_cBsb_jku4YXLB2uVhrZhYqtsUPYR1gqQHng
? Deleted collection: kotlin_chat_advanced_example

? Advanced chat features example completed successfully

BUILD SUCCESSFUL in 14s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
? Inserted 3 sample documents

=== Creating Chat Session ===
? Created session: kIsbXtyCWRFJtyKoIXIXv6AmzmyH5frO9CjGaNbf74cLZX-xknROnusfswvCLzVIN3LDDNTkWsIFJEe-UAbKow

=== Sending Chat Message ===
? Chat response:
  Message ID: "ZZSaE2WgA-t6zm3XjcAAWhpkGyRwW-4pjuy4aDe-9WZKYu7_Zgeu5emvEBM_xQlLMn1zLGhbunWDbhsjKMvGQg"
  Responses: ["ekoDB is a high-performance database that integrates intelligent caching and real-time capabilities, along with AI integration. It allows users to query the database using natural language, providing AI-powered responses enriched with relevant context. \n\nKey features of ekoDB include:\n\n1. **AI Chat Integration**: Users can interact with the database through a chat feature that facilitates querying using natural language.\n   \n2. **Search Capabilities**: ekoDB supports multiple search types, including full-text search, vector search, and hybrid search, which automatically retrieves context relevant to the queries.\n\nIn summary, ekoDB offers a user-friendly interface powered by AI, along with robust search functionalities that enhance the database querying experience."]

=== Cleanup ===
? Deleted chat session
? Deleted collection: kotlin_chat_basic_example

? Basic chat example completed successfully

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== ekoDB Chat Message Stream (SSE) Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
Created session: hLMzYUbPX-WRBVW9QJs37_qky1BqyGJxS1odkg48aGebdrFOLdj3EUK9qux99C44n9-JdrBD191FnMLEdBpVLw

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a high-performance, in-memory key-value database designed for fast data retrieval and manipulation. It is often used in environments requiring low-latency access to data, such as real-time analytics, gaming, or caching scenarios.

EkoDB typically emphasizes ease of use, scalability, and robust performance, allowing developers to efficiently manage data without the overhead associated with traditional disk-based databases. It may support various data structures and provide features like persistence, replication, and transactions, depending on the specific implementation.

However, as of my last knowledge update in October 2023, EkoDB does not have widespread recognition compared to other databases like Redis, Memcached, or traditional relational databases. If you are looking for more specific information or have a particular context in mind about EkoDB, feel free to provide additional details!

--- Stream complete ---
Message ID: UG9V73G6O8rkeGFmUh2-pD8lBW1y1V6hcAsubmZ-EDTHUVcWHIGwpS4nzlXrOWs8cPj6B_cUu6UYx13BV5IAJQ
Execution time: 1977ms
Context window: 128000 tokens

? Chat message stream example completed

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
    - gpt-transcribe
    - gpt-live-transcribe
    - gpt-6-astra
    - gpt-image-2.5-flare
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-live-1
  anthropic:
    - claude-fable-5-1
    - claude-opus-5
    - claude-sonnet-5
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
Provider status:
  anthropic: ok 11 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 132 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper, gpt-5.6-sol, gpt-5.6-terra, gpt-5.6-luna, gpt-realtime-2.1, gpt-realtime-2.1-mini, gpt-transcribe, gpt-live-transcribe, gpt-6-astra, gpt-image-2.5-flare, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.5-sunburst-2026-09-08, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

=== Chat Models Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
? Inserted sample product

=== Creating Chat Session ===
? Created session: kt47Tl9LW95YFmu0_P_Q2lWT2XBjMwM512abXuUtxnRyNR1YQw18hcS3g1loxGE7RjmWjdTD4HL8N5olIiAh7Q

=== Sending Messages ===
? Message 1 sent
  Responses: ["The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you need more information or have any other questions, feel free to ask!"]

? Message 2 sent
  Responses: ["The price of the product ekoDB is $99."]

=== Getting Message History ===
? Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"kt47Tl9LW95YFmu0_P_Q2lWT2XBjMwM512abXuUtxnRyNR1YQw18hcS3g1loxGE7RjmWjdTD4HL8N5olIiAh7Q"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"J-QZq3RjgoPld842BzRy3A3h1DYLUp3pzDM-uAPlrv_KOwtxW3ct0sCZazA1TOVogwmFiYlbkIMazuPG_DBhlQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-09-15T18:47:31.268160+00:00"},"id":"1RkW57DsL52uIH78R8rvB03lhRW-JPIZm1Wt-pCOQirLu86lniSfdOQz2Myezk_gYElA4wbSzZWWF72M2zLqnQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":87,"prompt_tokens":6740,"total_tokens":6827}},"updated_at":{"type":"DateTime","value":"2026-09-15T18:47:31.268160+00:00"}},{"chat_id":{"type":"String","value":"kt47Tl9LW95YFmu0_P_Q2lWT2XBjMwM512abXuUtxnRyNR1YQw18hcS3g1loxGE7RjmWjdTD4HL8N5olIiAh7Q"},"content":{"type":"String","value":"The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you need more information or have any other questions, feel free to ask!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"J-QZq3RjgoPld842BzRy3A3h1DYLUp3pzDM-uAPlrv_KOwtxW3ct0sCZazA1TOVogwmFiYlbkIMazuPG_DBhlQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-09-15T18:47:31.286+00:00"},"id":"OORWYCZwiY2JbR8igbUYNARC6bJ0Xj63uTfCF9PFNrSbafqOqUs4BAAfbfbNy7aY8j4Pk3_mS-p0N3NWdVMoaA","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":87,"prompt_tokens":6740,"total_tokens":6827}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","filter":{"content":{"field":"product","operator":"Ne","value":""},"type":"Condition"}},"id":"call_QeGb0sRTrxyy0bJnpYPdUOGe","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"J-QZq3RjgoPld842BzRy3A3h1DYLUp3pzDM-uAPlrv_KOwtxW3ct0sCZazA1TOVogwmFiYlbkIMazuPG_DBhlQ","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_QeGb0sRTrxyy0bJnpYPdUOGe","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-09-15T18:47:31.286+00:00"}},{"chat_id":{"type":"String","value":"kt47Tl9LW95YFmu0_P_Q2lWT2XBjMwM512abXuUtxnRyNR1YQw18hcS3g1loxGE7RjmWjdTD4HL8N5olIiAh7Q"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"J-QZq3RjgoPld842BzRy3A3h1DYLUp3pzDM-uAPlrv_KOwtxW3ct0sCZazA1TOVogwmFiYlbkIMazuPG_DBhlQ","price":99,"product":"ekoDB"},"score":2.0999999999999996}]},"created_at":{"type":"DateTime","value":"2026-09-15T18:47:32.480620+00:00"},"id":"eL1iyirvuDp5XuUcDOCqSS62mo-En1KrQpojhgbkVUU0Uu7ge4kSrcMujOMHsayhGMe0CL2qesYJ75LclseC0w","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":9,"prompt_tokens":3386,"total_tokens":3395}},"updated_at":{"type":"DateTime","value":"2026-09-15T18:47:32.480620+00:00"}},{"chat_id":{"type":"String","value":"kt47Tl9LW95YFmu0_P_Q2lWT2XBjMwM512abXuUtxnRyNR1YQw18hcS3g1loxGE7RjmWjdTD4HL8N5olIiAh7Q"},"content":{"type":"String","value":"The price of the product ekoDB is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"J-QZq3RjgoPld842BzRy3A3h1DYLUp3pzDM-uAPlrv_KOwtxW3ct0sCZazA1TOVogwmFiYlbkIMazuPG_DBhlQ","price":99,"product":"ekoDB"},"score":2.0999999999999996}]},"created_at":{"type":"DateTime","value":"2026-09-15T18:47:32.508340+00:00"},"id":"I8wFL6UVDy5INpDj-lbTDBZEFnbJdRG2vOy_LiMuVB64-SpsDBxwMi3hKENgTYk7_uZ6FYSEQxcz15NT0muIFA","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":9,"prompt_tokens":3386,"total_tokens":3395}},"updated_at":{"type":"DateTime","value":"2026-09-15T18:47:32.508340+00:00"}}]

=== Updating Session ===
? Updated session system prompt

=== Listing Sessions ===
? Total sessions: 7

=== Branching Session ===
? Created branched session: x1MfXtI-2NL_d5CAuzXjhHVxvb1gpFPZlcxW_P7BL4Sn-Ua7SHYi5fJyzok7LRt3fAg6WAuLgc7e0X0oYvoQ9Q

=== Cleanup ===
? Deleted chat sessions
? Deleted collection: kotlin_chat_sessions_example

? Chat session management example completed successfully

BUILD SUCCESSFUL in 14s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
? Found 36 collections
  - schema_employees_client_ts
  - schema_products_client_go
  - agent_function_versions__ek0_testing
  - chat_agent_configs__ek0_testing
  - chat_goal_templates__ek0_testing

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
? Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
? Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-09-15T18:47:42.018463Z","last_modified":"2026-09-15T18:47:42.018464Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
? Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - Collection Utilities Example ===

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_kt' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_kt' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_kt': 5

=== Get Collection Metadata ===
Collection metadata: {"analytics":["collection_utils_test_kt",{"manifest_backed":false,"record_count":5,"resident_record_ids":5,"total_size":675}],"collection":{"bypass_ripple":false,"created_at":"2026-09-15T18:47:51.727908Z","fields":{"index":{"default":null,"enums":[],"field_type":"Integer","max":null,"min":null,"regex":null,"required":false,"unique":false},"name":{"default":null,"enums":[],"field_type":"String","max":null,"min":null,"regex":null,"required":false,"unique":false}},"last_modified":"2026-09-15T18:47:51.728757Z","primary_key_alias":"id","version":1}}

=== List Collections ===
All collections (37):
  - schema_employees_client_ts
  - schema_products_client_go
  - agent_function_versions__ek0_testing
  - chat_agent_configs__ek0_testing
  - chat_goal_templates__ek0_testing
  - schedules__ek0_testing
  - schema_users_client_py
  - schema_employees_client_py
  - schema_products_client_js
  - test_accounts
  - users
  - ws_subscribe_example_rs
  - schema_products_client_ts
  - ws_subscribe_example_go
  - chat_messages__ek0_testing
  - schema_products_client_py
  - chat_raw_completions__ek0_testing
  - chat_tasks__ek0_testing
  - schema_documents_client_ts
  - ws_subscribe_example_ts
  - chat_goals__ek0_testing
  - schema_users_client_ts
  - collection_utils_test_kt
  - schema_documents_client_go
  - schema_users_client_js
  - schema_employees_client_js
  - test_collection
  - chat_configurations__ek0_testing
  - schema_documents_client_js
  - audit__ek0_testing
  - chat_turns__ek0_testing
  - ttl_cache
  - functions__ek0_testing
  - schema_documents_client_py
  - ws_subscribe_example_py
  - schema_users_client_go
  - schema_employees_client_go

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz_304132701624750' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_kt'

=== Collection Utilities Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Client created
? conc_demo_pay saved
? conc_demo_rl_fail saved
? conc_demo_rl_skip saved
? conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

? Cleaned up demo functions

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
? Created record: EkoRecord(fields={id=StringValue(value=dnEhhLEAo7Yn8V9-ewQZYd3hd06vHkRCdMMhodUdnKoTpa29Ljm_uAkFNFyJXrIZQysluXYWFHwTtPr_NwItMQ)})

=== Upsert Operation ===
? First upsert (update): EkoRecord(fields={age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), id=StringValue(value=dnEhhLEAo7Yn8V9-ewQZYd3hd06vHkRCdMMhodUdnKoTpa29Ljm_uAkFNFyJXrIZQysluXYWFHwTtPr_NwItMQ), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})
? Second upsert (insert): EkoRecord(fields={id=StringValue(value=0RhSR5nksqnLzx9oIancnBdAlL6AZY-_aJ64PmcOO4nq4hEz4xVKrU3T4OLl5DhL-rPdj6HNvMklV8xQ195QFg)})

=== Find One Operation ===
? Found user by email: EkoRecord(fields={age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), id=StringValue(value=dnEhhLEAo7Yn8V9-ewQZYd3hd06vHkRCdMMhodUdnKoTpa29Ljm_uAkFNFyJXrIZQysluXYWFHwTtPr_NwItMQ)})
? User not found (as expected)

=== Exists Check ===
? Record exists: true
? Fake record exists: false (should be false)

=== Pagination ===
? Inserted 25 records for pagination
? Page 1: 10 records (expected 10)
? Page 2: 10 records (expected 10)
? Page 3: 7 records (expected ~7)

=== Cleanup ===
? Deleted collection

? All convenience methods demonstrated successfully!

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Client created
? crypto_demo_hmac saved
? crypto_demo_aes saved
? crypto_demo_uuid saved
? crypto_demo_totp saved
? crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "H?llo World" }

? Cleaned up demo functions

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Kotlin distinct-values example passed

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
? Inserted document with 10s TTL
  Document ID: yjmuX9E4nl2YPHzvQ6_pS4IJbU8ppfn8kkzyXb6F2k8qPjiXYQWlH368eRYOk4ngTWt4-sio_Lt7dF5wo-Jpew

=== Verify Document Exists ===
? Document found: user_id, created_at, session_id, id, ttl

=== Insert with Longer TTL ===
? Inserted document with 1h TTL
  Document ID: 1J7UPQFGNoO0QgBMyQkZsNZjl7hR-30TcVkiST3xf3-V2-AleYV_Nr8dPTg2YVDoQbTbok88lZVYaqJGmSN9Ew

=== TTL Expiration ===
? Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
? Long TTL document still exists: id, value, cache_key, ttl

=== Delete Document ===
? Deleted document

=== Cleanup ===
? Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
? Cache entry created

Creating edge cache lookup function...
? Edge cache function created: JLsCg7zHjtA4y3S2cCnG_b1T6kQAsVZCRQAeef6LHJ5dEBkhPFswW3BkiW6Loth6G6UHoU8q2dRHVHxKXcpydg

Call 1: Cache lookup
Found 1 cached entries
Response time: 24ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 14ms

? Cleaning up...
? Cleanup complete

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

? Example complete!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Function Composition Examples ===

? Setting up test data...

? Test data ready

? Example 1: Basic Function Composition

Building reusable functions that call each other...

? Saved reusable function: fetch_user
? Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

? Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

? Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

? Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

? Saved reusable function: fetch_and_store_user (uses KV)
? Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ??  Duration: 118ms
   ? Records: 1

Second call (cache hit - from cache):
   ??  Duration: 14ms
   ? Records: 1
   ? Cache speedup: 8.4x faster!

? Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

? Level 1 function: validate_user
? Level 2 function: fetch_slim_user (calls validate_user)
? Level 3 function: get_verified_user (calls fetch_slim_user)

? Result from 3-level nested composition:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

? Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow

? Cleaning up...
? Cleanup complete

? All composition examples completed!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
client_function_contract: ok

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
? ekoDB Functions Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
? Client initialized

? Setting up test data...
? Test data ready

? Example 1: Simple Query Function

? Function saved: 8buUeI05rP5_2OgeBRt97ZyW6plRfeLVgWy7jX8xYpZyl2Yop00jRGsU_UCT8OQevxC99drW8SZkdoHak183kg
? Found 5 records
??  Execution time: 0ms

? Example 2: Parameterized Function

? Function saved
? Found 3 users (limited)
??  Execution time: 0ms

? Example 3: Aggregation Function

? Function saved
? Statistics: 2 groups
   {"avg_score":{"value":60.0,"type":"Float"},"status":{"value":"active","type":"String"},"count":{"value":5,"type":"Integer"}}
   {"avg_score":{"type":"Float","value":50.0},"count":{"value":5,"type":"Integer"},"status":{"value":"inactive","type":"String"}}
??  Execution time: 0ms

? Example 4: function Management

? Total functions: 15
? Retrieved function: Get Active Users
??  function updated
??  function deleted

??  Note: GET/UPDATE/DELETE operations require the encrypted ID
??  Only CALL can use either ID or label

? Example 5: Multi-Stage Pipeline

? Multi-stage function saved
? Pipeline executed 2 stages
??  Total execution time: 0ms
? Stage breakdown:

? Example 6: Count Users

? Count function saved
? Total user count: {"type":"Integer","value":10}
??  Execution time: 0ms

? Cleaning up...
? Deleted test functions
? Deleted collection

? All examples completed successfully!

? Key Advantages of Using the Client:
   ? Automatic token management
   ? Type-safe Stage builders
   ? Built-in error handling

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? ekoDB Kotlin Advanced Functions Example

? Setting up test data...
? Created 8 products

? Example 1: List All Products

? Function saved
? Found 8 products
??  Execution time: 0ms

? Example 2: Group Products by Category

? Function saved
? Category breakdown:
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":5},"avg_price":{"type":"Float","value":367.0}}
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":365.6666666666667}}
??  Execution time: 0ms

? Cleaning up...
? Cleanup complete

? All advanced function examples finished!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? ekoDB Kotlin AI Functions Example

? Setting up test data...
? Created 2 articles

? Example 1: Simple Chat Completion

? Chat function saved
? AI Response:
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast retrieval of similar items through vector representations, making them ideal for applications like recommendation systems and image searches.

2. **Handling High-Dimensional Data**: They effectively manage and query high-dimensional data, which is common in machine learning and AI applications.

3. **Scalability**: Vector databases can scale to handle large datasets, accommodating growth in data volume and complexity.

4. **Real-time Querying**: They support real-time data processing and querying, essential for dynamic applications.

5. **Integration with AI/ML Models**: Vector databases can efficiently store and query embeddings from deep learning models, facilitating AI-driven applications.

6. **Complex Data Understanding**: They allow for capturing and querying complex relationships in data, improving the understanding of contextual information.

7. **Reduced Latency**: By optimizing data retrieval through indexing techniques, they significantly reduce query response times.

8. **Versatile Use Cases**: Suitable for a wide range of applications, including natural language processing, computer vision, and anomaly detection.

Overall, vector databases enhance performance and usability for modern applications relying on data relationships and similarity.
??  Execution time: 0ms

? Example 2: Generate Embeddings

? Embed function saved
? Generated 2 embeddings
   Dimensions: 1536
??  Execution time: 0ms

? Cleaning up...
? Cleanup complete

? All AI function examples finished!

? This example demonstrates:
   ? Chat completions with system/user messages
   ? Embedding generation for text

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? ekoDB Kotlin Complete Functions Example

? Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

? Setting up complete test data...
? Created 5 products

? Example 1: FindAll + Group (Simple Aggregation)

? Function saved: hZuSe1cn1dvdYzth2CXcF_KmiFlmNhJtIasRFhEafLS_sA5K0JCOnPEf--F6N3Mn97IsFRyeFTBMuRyo1nmcuw
? Found 2 category groups

? Example 2: Simple Product Listing

? Function saved
? Found 5 products

? Example 3: Count by Category

? Function saved
? Found 2 categories

? Example 4: Multi-Stage Pipeline (FindAll ? Group ? Count)

? Function saved
? Pipeline executed with 1 results

? Cleaning up...
? Cleanup complete

? All complete function examples finished!

? This example demonstrates ekoDB's function system:
   ? FindAll operations
   ? Group aggregations (Count, Average)
   ? Multi-stage pipelines (FindAll ? Group ? Count)
   ? Function management (save, call, delete)

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? ekoDB Kotlin CRUD Functions Example

? Setting up test data...
? Created 10 test users

? Example 1: List All Users

? Function saved
? Found 10 users
??  Execution time: 0ms

? Example 2: Count Users by Status

? Function saved
? User counts by status:
   {"count":{"type":"Integer","value":7},"status":{"type":"String","value":"active"}}
   {"status":{"value":"inactive","type":"String"},"count":{"value":3,"type":"Integer"}}
??  Execution time: 0ms

? Cleaning up...
? Cleanup complete

? All CRUD function examples finished!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
? ekoDB Kotlin KV Store & Wrapped Types Example

? Demonstrates:
   ? Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   ? KV store operations (get, set, delete, exists, query)
   ? KV operations within functions
   ? Combined wrapped types + KV workflows

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
? Client initialized

? Example 1: Inserting Records with Wrapped Types

? Inserted order: StringValue(value=bBff1zdFPvHAK9e-hj0POymIgKBv75zOe1d1qFZoinVxm0jhJqSichZEj7hZnvdXXMvA6BvxWleWGyceDNM4HA)
? Inserted 2 products

? Example 2: function with Wrapped Type Parameters

? Function saved: TOTtKjeNUez1WupoE_2BCdwRrTR4sxQ2q4BM4BgRSOgMxoDBO60d4H7LAFW2zbxplVsWzLUrfKsb4KvFwbsVYQ
? Created order via function
??  Execution time: 0ms

? Example 3: Basic KV Store Operations

? Set session data
? Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
? Set cached data with 1 hour TTL
??  Deleted session

? Example 4: KV Operations in Functions

? Function saved: jrwKLNg89_-d5EulebV86tzUHLwO6LaIHhhs14pPAcLr9bT3QqkiLdwfR5RceD3PhDxRnVK21iz3-FxJxtRxXg
? Cached and retrieved product data
??  Execution time: 0ms

? Example 5: Combined Wrapped Types + KV Function

? Function saved: Ue2UStGQXyz_1x1OZ5oMcuB7oZDFdh3_xonDH4UT7upL56aKqRiOkalYvEDrnqbPUmkT7EBggIFsk3jQOdTt5w
? Processed order with caching
??  Stages executed: 3
??  Execution time: 0ms

? Cleaning up...
? Cleanup complete

? All KV & Wrapped Types examples completed!

? Key takeaways:
   ? Use field* helpers for type-safe wrapped values
   ? fieldDecimal() preserves precision (no floating point errors)
   ? KV store is great for caching and quick lookups
   ? FunctionStageConfig.Kv* classes work within functions

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? ekoDB Kotlin Search Functions Example

? Setting up test data...
? Inserted 5 documents

? Example 1: List All Documents

? Function saved
? Found 5 documents
   1. {"value":"Vector Databases Explained","type":"String"} ({"type":"String","value":"Database"})
   2. {"value":"Database Design Principles","type":"String"} ({"type":"String","value":"Database"})
   3. {"type":"String","value":"Natural Language Processing"} ({"type":"String","value":"AI"})
   4. {"type":"String","value":"Getting Started with ekoDB"} ({"type":"String","value":"Database"})
   5. {"type":"String","value":"Introduction to Machine Learning"} ({"value":"AI","type":"String"})
??  Execution time: 0ms

? Example 2: Count Documents by Category

? Function saved
? Documents by category:
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Database"}}
   {"category":{"type":"String","value":"AI"},"count":{"value":2,"type":"Integer"}}
??  Execution time: 0ms

? Cleaning up...
? Cleanup complete

? All search function examples finished!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== ekoDB Goal Template CRUD Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
--- Creating goal template ---
Created template: Data Migration (id: W67MA6Yphp4XmBUV_FocBA-TXt4bwLIfTzufpqZbQUYv1Qxv6XO_SQeWlRcD7XJIEclFxrGMqro_9yMjb7DiOg)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"W67MA6Yphp4XmBUV_FocBA-TXt4bwLIfTzufpqZbQUYv1Qxv6XO_SQeWlRcD7XJIEclFxrGMqro_9yMjb7DiOg","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

? Goal template CRUD example completed

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - Goals, Tasks & Agents Example ===

--- Creating goal ---
Created goal: 80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA

--- Listing goals ---
Goals: {"count":1,"goals":[{"created_at":"2026-09-15T18:50:13.044861+00:00","description":"Ship the next major release","id":"80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA","status":"pending","steps":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0","updated_at":"2026-09-15T18:50:13.044861+00:00"}]}

--- Getting goal ---
Fetched: {"type":"String","value":"Deploy v2.0"}

--- Updating goal ---
Updated description: {"type":"String","value":"Ship v2.0 with full test coverage"}

--- Searching goals ---
Search results: {"count":1,"items":[{"_score":12.870000000000001,"created_at":{"type":"DateTime","value":"2026-09-15T18:50:13.044861+00:00"},"description":{"type":"String","value":"Ship v2.0 with full test coverage"},"id":"80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0"},"updated_at":{"type":"DateTime","value":"2026-09-15T18:50:13.101196+00:00"}}]}

--- Goal step lifecycle ---
Step 0 started: "80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA"
Step 0 completed: "80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA"
Step 1 started: "80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA"
Step 1 failed: "80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA"

--- Completing goal ---
Goal status after complete: {"type":"String","value":"pending_review"}

--- Approving goal ---
Goal status after approve: {"type":"String","value":"in_progress"}

--- Creating goal for rejection ---
Goal status after reject: {"type":"String","value":"failed"}

--- Creating task ---
Created task: CqXNegRwOpwwtYyhQPukbSOcnCYSfRhS0_h9zHm6nGY6jpLCNVtQZLTC9cCIPccQK6BTy1AtECzS7lGRZqqnag

--- Listing tasks ---
Tasks: {"count":1,"items":[{"cron":{"type":"String","value":"0 2 * * *"},"description":{"type":"String","value":"Full database backup every night at 2 AM"},"id":"CqXNegRwOpwwtYyhQPukbSOcnCYSfRhS0_h9zHm6nGY6jpLCNVtQZLTC9cCIPccQK6BTy1AtECzS7lGRZqqnag","name":{"type":"String","value":"Nightly Backup"}}]}

--- Getting task ---
Task name: {"type":"String","value":"Nightly Backup"}

--- Starting task ---
Task status: {"type":"String","value":"running"}

--- Succeeding task ---
Task status after succeed: {"type":"String","value":"active"}

--- Pausing task ---
Task status: {"type":"String","value":"paused"}

--- Resuming task ---
Task status: {"type":"String","value":"active"}

--- Failing task ---
Task marked as failed

--- Checking due tasks ---
Due tasks: {"count":0,"items":[]}

--- Deleting tasks ---
Tasks deleted

--- Creating agent ---
Created agent: l-KzWeoKPFHpOY662VLZuJPyMnwJOqU8hOzgdW5KHHIyMDAIrLapTTpXthE_xN2ffacDNv6VHdRpd0vha0G4KA ? null

--- Listing agents ---
Agents: {"count":1,"items":[{"deployment_id":{"type":"String","value":"deploy_kt_example"},"id":"l-KzWeoKPFHpOY662VLZuJPyMnwJOqU8hOzgdW5KHHIyMDAIrLapTTpXthE_xN2ffacDNv6VHdRpd0vha0G4KA","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"DataBot"},"system_prompt":{"type":"String","value":"You are a data analysis assistant."}}]}

--- Getting agent by ID ---
Agent: {"type":"String","value":"DataBot"}

--- Getting agent by name ---
By name: {"type":"String","value":"DataBot"}

--- Updating agent ---
Updated agent system_prompt

--- Agents by deployment ---
Agents for deployment: {"count":0,"items":[]}

--- Deleting agent ---
Agent deleted

--- Cleanup: deleting goals ---
Goals deleted

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - Join Operations Example ===

=== Setting up sample data ===
? Sample data created

=== Example 1: Single collection join (users with departments) ===
? Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

=== Example 2: Join with filtering ===
? Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
? Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data ===
? Found 2 completed orders
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
? Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
? Deleted test collections

? Join operations example completed successfully

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Client created
? kt_users_register saved
? kt_users_login saved
? kt_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/kt_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_verify_token { "token": "<jwt>" }

? Cleaned up demo functions

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - KV Links Example ===

--- Setting KV entry ---
Set key: user:alice

--- Linking documents ---
Linked order_001: null
Linked order_002: null
Linked inv_100: null

--- Getting links ---
Links for user:alice: [{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-09-15T18:50:38.896396Z","last_accessed":"2026-09-15T18:50:38.902537Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-09-15T18:50:38.877175Z","last_accessed":"2026-09-15T18:50:38.902537Z","metadata":{}},{"collection":"orders","document_id":"order_002","field_path":null,"created_at":"2026-09-15T18:50:38.886981Z","last_accessed":"2026-09-15T18:50:38.902537Z","metadata":{}}]

--- Unlinking document ---
Unlinked order_002: null

--- Verifying remaining links ---
Remaining links: [{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-09-15T18:50:38.896396Z","last_accessed":"2026-09-15T18:50:38.915969Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-09-15T18:50:38.877175Z","last_accessed":"2026-09-15T18:50:38.915969Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining documents
Deleted key: user:alice

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
? Set key: user:123

=== KV Get ===
? Retrieved value: {"type":"Object","value":{"name":"Alice","email":"alice@example.com","role":"admin"}}

=== KV Set with TTL ===
? Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
? Session value: {"value":{"user_id":"123","created_at":1789498246239},"type":"Object"}
  (Will expire in 10 seconds)

=== KV Batch Set ===
? Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
? Batch retrieved 3 values
  config:db: {"value":{"host":"localhost","port":5432}}
  config:cache: {"value":{"ttl":3600,"enabled":true}}
  config:api: {"value":{"retries":3,"timeout":30}}

=== KV Exists ===
? Key exists: true

=== KV Find (Pattern Query) ===
? Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
? Total keys in store: 5

=== KV Delete ===
? Deleted key: user:123

=== Verify Deletion ===
? Key exists after delete: false

=== KV Batch Delete ===
? Batch deleted 3 keys
  config:db: deleted
  config:cache: deleted
  config:api: deleted

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
=== ekoDB Kotlin Client - KV Precision: Float vs Decimal ===

=== Test 1: Using Kotlin Doubles (LOSES PRECISION) ===
Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) MATCH
  Widget B: $39.99 (expected $39.99) MATCH
  Widget C: $49.99 (expected $49.99) MATCH

=== Test 2: Using fieldDecimal() (PRESERVES PRECISION) ===
Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99)
  Widget B: $39.99 (expected $39.99)
  Widget C: $49.99 (expected $49.99)

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
Cleaned up 8 test keys

=== Summary ===
Use fieldDecimal() for monetary values, percentages, and
any case where floating-point errors are unacceptable.
fieldDecimal() stores values as strings internally,
preserving exact precision across all operations.

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
