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
> Task :runKtlintCheckOverTestSourceSet UP-TO-DATE
> Task :ktlintTestSourceSetCheck UP-TO-DATE
> Task :compileTestKotlin UP-TO-DATE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test UP-TO-DATE
> Task :check UP-TO-DATE
> Task :build UP-TO-DATE

[Incubating] Problems report is available at: file://ekoDB/ekodb-client/ekodb-client-kt/build/reports/problems/problems-report.html

Deprecated Gradle features were used in this build, making it incompatible with Gradle 10.

You can use '--warning-mode all' to show the individual deprecation warnings and determine if they come from your own scripts or plugins.

For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/command_line_interface.html#sec:command_line_warnings in the Gradle documentation.

BUILD SUCCESSFUL in 3s
11 actionable tasks: 11 up-to-date
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
✅ Kotlin client built!
=== Running Kotlin example: ClientAdvancedCrud.kt ===
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
Inserted: 49DwLGFXGhzI55pk7CR8FZ9Y0f3JjDxxrleA9k5e9mmdFzdAS4uFgwOhb8MP3rHddoZfzOmNDz_NjXoMNFJlbQ

--- updateWithAction: increment views ---
After increment: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)}), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)}), score=ObjectValue(value={value=FloatValue(value=100.0), type=StringValue(value=Float)}), id=StringValue(value=49DwLGFXGhzI55pk7CR8FZ9Y0f3JjDxxrleA9k5e9mmdFzdAS4uFgwOhb8MP3rHddoZfzOmNDz_NjXoMNFJlbQ), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example)]), type=StringValue(value=Array)})})

--- updateWithAction: push to tags ---
After push: EkoRecord(fields={score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)}), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)}), id=StringValue(value=49DwLGFXGhzI55pk7CR8FZ9Y0f3JjDxxrleA9k5e9mmdFzdAS4uFgwOhb8MP3rHddoZfzOmNDz_NjXoMNFJlbQ), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced)])}), name=ObjectValue(value={value=StringValue(value=Counter Record), type=StringValue(value=String)})})

--- updateWithActionSequence: multiple operations ---
After sequence: EkoRecord(fields={id=StringValue(value=49DwLGFXGhzI55pk7CR8FZ9Y0f3JjDxxrleA9k5e9mmdFzdAS4uFgwOhb8MP3rHddoZfzOmNDz_NjXoMNFJlbQ), score=ObjectValue(value={value=FloatValue(value=75.0), type=StringValue(value=Float)}), name=ObjectValue(value={value=StringValue(value=Counter Record), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced), StringValue(value=sequenced)])}), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=115)})})

--- Cleanup ---
Deleted collection: kotlin_advanced_crud_example

=== Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientBatchOperations.kt ===
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
✓ Inserted 5 records
  IDs: QPqSyXpXret2dXdYUbSKUjjTvstbBQAS03jicFf8B0zXfR3yZHdGPetTOkme3wXlUNK-QErC1Q4UGcOvimEFIw, 9VypQIpaWwvuwWLR7IrZ3WQ4Genrt8s0Bxgh0p_a0DXbC70bql_qqRNRlnUoU80GLlM-jLzHOCxgLWgJMke3KA, nULAkIDEofg7KcdEArYQqVxfiG65_WHxJ7C6_TGNycdWm7zTC3NQkwYL-w-7dUTgJkecv5BwBviRnabcv17EgA...

=== Batch Update ===
✓ Updated 3 records

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatAdvanced.kt ===
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
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fZCjHdz6GGctm2deeJTM98-jc6Fw43IUzepT6lVxVQLO99HUQ7SjiCxv_ZYmB8MT0aiUZuYvbnWyVznNApPTzw

=== Sending Initial Message ===
✓ Message sent
  Responses: ["It seems there are currently no active products available in the database. Would you like to check for any specific product names or details?"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The price of ekoDB is $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: s-I5FIXph2OxFiIKu1sB5ed9TuJF56E4SmKpmkUCd-hTqnw6nuCUCj8ok2Pkg2C1kQV5p2_9D2vz_Fl0uLpUpQ

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat session: s-I5FIXph2OxFiIKu1sB5ed9TuJF56E4SmKpmkUCd-hTqnw6nuCUCj8ok2Pkg2C1kQV5p2_9D2vz_Fl0uLpUpQ
✓ Deleted chat session: fZCjHdz6GGctm2deeJTM98-jc6Fw43IUzepT6lVxVQLO99HUQ7SjiCxv_ZYmB8MT0aiUZuYvbnWyVznNApPTzw
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatBasic.kt ===
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
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 0w5HD8JD_wEKAw5sHWmk9pgvNnHpyETTx3AhwbJ0fdzNatXR-FLdmhnEDTioyKnLQKxaLddz_2BaZkG1zFR3FQ

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "tVub5Rj0tK0H1v5dGUAE613u11WeodIpQnwLzRJ9v8478o_vgJa0O50YNP7TVo_nVBp_wNUONVSx76E8zG5fWA"
  Responses: ["ekoDB is a high-performance database that boasts intelligent caching, real-time capabilities, and AI integration. It is designed to provide flexibility and efficiency for various data management tasks.\n\nHere are some key features of ekoDB:\n\n1. **AI Chat Integration**: The chat feature allows you to query your database using natural language and receive AI-powered responses with relevant context.\n\n2. **Search Capabilities**: ekoDB supports full-text search, vector search, and hybrid search, enabling automatic context retrieval to enhance the search experience.\n\n3. **Performance Optimization**: With intelligent caching and real-time capabilities, ekoDB ensures high performance for data retrieval and management tasks.\n\nOverall, ekoDB combines advanced search functionalities and AI integration to streamline interactions with your data."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatMessageStream.kt ===
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
Created session: m0uT0hOoU4uXwxOMMtGQo7l31fb1CmBgWraQj7yMnzIIr_8ZnYubr8Xqz5P0s2biBI6T6lb7sw2sgFS4zxuc5A

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a high-performance, cloud-native database that is designed to handle large volumes of data and enable real-time processing. It is often optimized for certain use cases, such as time-series data, event handling, and analytical workloads, making it suitable for industries that require fast data access and processing capabilities.

EkoDB typically offers features like:

- Scalability: The ability to scale out horizontally to manage increasing workloads.
- Low latency: Quick data retrieval and updates, which is essential for real-time applications.
- High availability: Features that ensure the database remains operational and accessible even in the event of hardware failures or other issues.
- Flexibility: Supporting various data models, including structured, unstructured, and semi-structured data.
- Integration: Easy integration with other cloud services and platforms, allowing for seamless data flow and processing.

Use cases for EkoDB might include Internet of Things (IoT) applications, financial services analytics, social media data processing, and any other domain where rapid data processing is critical.

It's worth noting that specific implementations and features of EkoDB may vary based on the version or deployment context, so checking the official documentation or resources may provide the most current and detailed information.

--- Stream complete ---
Message ID: 1Yf9zk5IcDkfOtcTVMbQRYbNvmkTcQqy0vhSD53tMcdzhqp_wNJu8vxLkKX7lUqfK3uBQTnbhRX7YeD4bkMD0g
Execution time: 2114ms
Context window: 128000 tokens

✓ Chat message stream example completed

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatModels.kt ===
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

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatSessions.kt ===
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
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 2xG7WFAwTzplk-MA71vhiL_UHzV7s4AhMi9LGmSUd4o_bfJzB2U3bJ1J9hgZf4BkXI8kjE75TNztXe9Z_xJQEQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you need further information or have additional questions, feel free to ask!"]

✓ Message 2 sent
  Responses: ["The price of the product ekoDB is **$99**."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"2xG7WFAwTzplk-MA71vhiL_UHzV7s4AhMi9LGmSUd4o_bfJzB2U3bJ1J9hgZf4BkXI8kjE75TNztXe9Z_xJQEQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"FAv5CykZ8y87Ikb6aF45xbiUCtT6GIl7LtTr7OfQGd46NLcWHsocyj3Cb-RIP5pDavMVLiatWjKOHzPSVq0gfw","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-09-16T20:49:29.701496+00:00"},"id":"ZMj6nPKzoJdpwhIagku7oNoja04RxgZrWbMq0JIn8c8LIzSoncYfofjSOpVXk8qErK4GH_XZF_TC7NSZ-9uvig","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":66,"prompt_tokens":6740,"total_tokens":6806}},"updated_at":{"type":"DateTime","value":"2026-09-16T20:49:29.701496+00:00"}},{"chat_id":{"type":"String","value":"2xG7WFAwTzplk-MA71vhiL_UHzV7s4AhMi9LGmSUd4o_bfJzB2U3bJ1J9hgZf4BkXI8kjE75TNztXe9Z_xJQEQ"},"content":{"type":"String","value":"The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you need further information or have additional questions, feel free to ask!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"FAv5CykZ8y87Ikb6aF45xbiUCtT6GIl7LtTr7OfQGd46NLcWHsocyj3Cb-RIP5pDavMVLiatWjKOHzPSVq0gfw","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-09-16T20:49:29.718063+00:00"},"id":"OZ9gdCvlrUE5PZzYZicZa75DdDpLSL5uKB59hSQyBlDA7IQvPXYs8xfCF9Lnk8ydFDvnorBnZpCabxnmTzDrPQ","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":66,"prompt_tokens":6740,"total_tokens":6806}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example"},"id":"call_USSL71jK9ONowirWrK0o4qOE","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"FAv5CykZ8y87Ikb6aF45xbiUCtT6GIl7LtTr7OfQGd46NLcWHsocyj3Cb-RIP5pDavMVLiatWjKOHzPSVq0gfw","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_USSL71jK9ONowirWrK0o4qOE","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-09-16T20:49:29.718063+00:00"}},{"chat_id":{"type":"String","value":"2xG7WFAwTzplk-MA71vhiL_UHzV7s4AhMi9LGmSUd4o_bfJzB2U3bJ1J9hgZf4BkXI8kjE75TNztXe9Z_xJQEQ"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"FAv5CykZ8y87Ikb6aF45xbiUCtT6GIl7LtTr7OfQGd46NLcWHsocyj3Cb-RIP5pDavMVLiatWjKOHzPSVq0gfw","price":99,"product":"ekoDB"},"score":2.095}]},"created_at":{"type":"DateTime","value":"2026-09-16T20:49:30.719634+00:00"},"id":"0o2xmdkeL55iJMcBDugBJLMwPlYQRvHSykChcwLqcwvpI6LK0pmC7pVpLRp7WGkH3n-Z37ithROWuXXEE-Rpag","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":10,"prompt_tokens":3387,"total_tokens":3397}},"updated_at":{"type":"DateTime","value":"2026-09-16T20:49:30.719634+00:00"}},{"chat_id":{"type":"String","value":"2xG7WFAwTzplk-MA71vhiL_UHzV7s4AhMi9LGmSUd4o_bfJzB2U3bJ1J9hgZf4BkXI8kjE75TNztXe9Z_xJQEQ"},"content":{"type":"String","value":"The price of the product ekoDB is **$99**."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"FAv5CykZ8y87Ikb6aF45xbiUCtT6GIl7LtTr7OfQGd46NLcWHsocyj3Cb-RIP5pDavMVLiatWjKOHzPSVq0gfw","price":99,"product":"ekoDB"},"score":2.095}]},"created_at":{"type":"DateTime","value":"2026-09-16T20:49:30.736321+00:00"},"id":"PkyMYYSRLW2pQhiUuBd3GxXY6PqXlJ8sIRKG71f77txh-At2gdyXonXQu3WwL6cuB3a3FZMpfCXHVI_bUMQTRQ","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":10,"prompt_tokens":3387,"total_tokens":3397}},"updated_at":{"type":"DateTime","value":"2026-09-16T20:49:30.736321+00:00"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 1

=== Branching Session ===
✓ Created branched session: k4s0fiZzClFpjL3ylWokTAb0wNIMx3plpl3eIWdfSxicJSPp6jPTSAZftBoEg9zk5TVXOb7JRlSsIl6C69mcFg

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCollectionManagement.kt ===
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
✓ Found 26 collections
  - schema_users_client_ts
  - schema_documents_client_ts
  - chat_goals__ek0_testing
  - schema_products_client_js
  - schedules__ek0_testing

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-09-16T20:49:35.869225Z","last_modified":"2026-09-16T20:49:35.869225Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCollectionUtils.kt ===
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
Collection metadata: {"analytics":["collection_utils_test_kt",{"manifest_backed":false,"record_count":5,"resident_record_ids":5,"total_size":675}],"collection":{"bypass_ripple":false,"created_at":"2026-09-16T20:49:41.038236Z","fields":{"index":{"default":null,"enums":[],"field_type":"Integer","max":null,"min":null,"regex":null,"required":false,"unique":false},"name":{"default":null,"enums":[],"field_type":"String","max":null,"min":null,"regex":null,"required":false,"unique":false}},"last_modified":"2026-09-16T20:49:41.038507Z","primary_key_alias":"id","version":1}}

=== List Collections ===
All collections (27):
  - schema_users_client_ts
  - schema_documents_client_ts
  - chat_goals__ek0_testing
  - schema_products_client_js
  - schedules__ek0_testing
  - schema_employees_client_js
  - schema_products_client_go
  - agent_function_versions__ek0_testing
  - schema_products_client_ts
  - chat_raw_completions__ek0_testing
  - chat_turns__ek0_testing
  - schema_employees_client_go
  - audit__ek0_testing
  - schema_users_client_go
  - schema_documents_client_js
  - chat_goal_templates__ek0_testing
  - schema_users_client_js
  - collection_utils_test_kt
  - schema_employees_client_ts
  - chat_tasks__ek0_testing
  - test_accounts
  - schema_documents_client_go
  - test_collection
  - chat_messages__ek0_testing
  - functions__ek0_testing
  - chat_configurations__ek0_testing
  - chat_agent_configs__ek0_testing

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz_397841971344791' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_kt'

=== Collection Utilities Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientConcurrencyStages.kt ===
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

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientConvenienceMethods.kt ===
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
✓ Created record: EkoRecord(fields={id=StringValue(value=m-C3C_gK43A7v_Ki1HjeEBVD3GqO22SHpNxqoqEVhzxjbmQkb___EzEJNNxOFuXM0GTgFd6Hax1HK6g-7GnMBQ)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={id=StringValue(value=m-C3C_gK43A7v_Ki1HjeEBVD3GqO22SHpNxqoqEVhzxjbmQkb___EzEJNNxOFuXM0GTgFd6Hax1HK6g-7GnMBQ), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=O8WK04yGOEv_okgtGfzQtqkoWLV2WVuFrrURVZSOSjkWLjmfNr0WhGAuMGShVDG6eBGbSqAPRPLEfndRKK9nXg)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), id=StringValue(value=m-C3C_gK43A7v_Ki1HjeEBVD3GqO22SHpNxqoqEVhzxjbmQkb___EzEJNNxOFuXM0GTgFd6Hax1HK6g-7GnMBQ), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)})})
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

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCryptoStages.kt ===
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
✓ Client created
✓ crypto_demo_hmac_kt saved
✓ crypto_demo_aes_kt saved
✓ crypto_demo_uuid_kt saved
✓ crypto_demo_totp_kt saved
✓ crypto_demo_encoding_kt saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_kt { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_kt { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_kt
  POST /api/functions/crypto_demo_totp_kt
  POST /api/functions/crypto_demo_encoding_kt { "title": "Héllo World" }

✓ Cleaned up demo functions

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientDistinctValues.kt ===
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

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientDocumentTtl.kt ===
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
✓ Inserted document with 10s TTL
  Document ID: 9wQj4m7tjPPN6GHcIKihjKgITeT1LinOfzbz-A6hri0xpyeaxTeqE4HOFkePcAPlOaABmjOc1DTCUQXx1JkQkA

=== Verify Document Exists ===
✓ Document found: id, created_at, session_id, user_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: h31guTPyOG-donTGpg78QUkMoICifha1W07UO-uUCUDx8QnY-yPeRW3ab7uZWL7edE2bVWcVjY9WKPiIWCJn_Q

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: cache_key, value, id, ttl

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientEdgeCache.kt ===
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
✓ Cache entry created

Creating edge cache lookup function...
✓ Edge cache function created: U8Vy-iE5gZqsDfVWWIyGORjSeP_a95HqNAgI5dB6_O-UGGBlTW5kDgppSqwxPNc0ZFhkxkbBdYufWCFxAJ4diw

Call 1: Cache lookup
Found 1 cached entries
Response time: 17ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 4ms

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

🧹 Cleaning up...
✓ Cleanup complete

✓ Example complete!

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionComposition.kt ===
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

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 92ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 5ms
   📊 Records: 1
   🚀 Cache speedup: 18.4x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow

🧹 Cleaning up...
✅ Cleanup complete

✅ All composition examples completed!

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionContract.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
client_function_contract: ok

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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

✅ Function saved: GGkJCTTnZTfTEf5OIU2d-q2DLskt0ZFo9y06f9pZqdwejlfZixK7WCRWXttOBiRJ7x0DdNvjTe0tUIboR_ZA2w
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":60.0},"status":{"type":"String","value":"active"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"value":50.0,"type":"Float"},"status":{"type":"String","value":"inactive"}}
⏱️  Execution time: 0ms

📝 Example 4: function Management

📋 Total functions: 12
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

✅ Count function saved
📊 Total user count: {"type":"Integer","value":10}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted test functions
✅ Deleted collection

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsAdvanced.kt ===
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
   {"category":{"type":"String","value":"Furniture"},"count":{"value":3,"type":"Integer"},"avg_price":{"value":365.6666666666667,"type":"Float"}}
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":5},"avg_price":{"value":367.0,"type":"Float"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsAi.kt ===
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
🚀 ekoDB Kotlin AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel in searching high-dimensional data quickly, using techniques like approximate nearest neighbors (ANN).

2. **Handling Unstructured Data**: They effectively manage and retrieve unstructured data, such as text, images, and audio, by converting them into vector representations.

3. **Scalability**: Many vector databases are designed to scale horizontally, accommodating large datasets without compromising performance.

4. **Improved Machine Learning Integration**: They facilitate seamless integration with machine learning workflows, supporting tasks like recommendation systems and natural language processing.

5. **Real-time Processing**: Vector databases often enable real-time querying, which is useful for applications requiring immediate feedback.

6. **Advanced Search Capabilities**: They allow for semantic search, enabling users to find relevant results based on meaning rather than exact matches.

7. **Multi-modal Data Support**: Vector databases can store and query multiple data types together, enhancing data analysis and usage.

8. **Robustness to Data Variability**: They typically handle noisy and varied data better than traditional relational databases.

These advantages make vector databases particularly valuable for applications in AI, machine learning, and data analytics.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed function saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI function examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsComplete.kt ===
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
🚀 ekoDB Kotlin Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 6mEgGTVUKJRHkCBda4skNBWTNYotT8DDYzYRvUEXv3lhdbJU7p8gRBX5si_dOqDgYUvvFjmM7mtRLub-7w9uCA
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

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsCrud.kt ===
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
   {"status":{"value":"active","type":"String"},"count":{"type":"Integer","value":7}}
   {"status":{"value":"inactive","type":"String"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsKvWrapped.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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

✅ Inserted order: StringValue(value=1-ZCNz_rrDlzDSYyScNg18rRPQeNm70Wwn3Xrrq2YYW0_fGWcuCc7tgcg2ce1kTmQdp7osHUzYAjLVACQP0GDQ)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: i-W7GY6Ke9fZc7CShrj1LKwOBqJlAPl4Cmi7tHyXvkGKTmi8o8H0E6AP5Z5I-wdv8R1MVTGiY5OVqxroDYO42w
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: -7IR3rmO-8Cha3IaxfPrano5dCWkkXKsXGDN1gFtJPn3H60qZJJV-genkk9ayGOSp_07zPlfb_GLoedSiO726w
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: HsK1T5CUdoqLNS8AAVhe_WbvilcFtxZnoQ0hrgvC1VaoE_226dzknnRcWvfuIcYrWbMry_0Bck8i8MFl76mKfQ
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
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsSearch.kt ===
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
🚀 ekoDB Kotlin Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. {"type":"String","value":"Getting Started with ekoDB"} ({"value":"Database","type":"String"})
   2. {"type":"String","value":"Vector Databases Explained"} ({"type":"String","value":"Database"})
   3. {"type":"String","value":"Introduction to Machine Learning"} ({"value":"AI","type":"String"})
   4. {"type":"String","value":"Database Design Principles"} ({"value":"Database","type":"String"})
   5. {"type":"String","value":"Natural Language Processing"} ({"type":"String","value":"AI"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"type":"Integer","value":2},"category":{"type":"String","value":"AI"}}
   {"category":{"type":"String","value":"Database"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientGoalTemplates.kt ===
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
Created template: Data Migration (id: BEvw_7LEzRWRb_JLaXl1CbPE53mIvyHTJBa4_KdSSvupvaVJ1as3DxHuK7wBl_jVWwctsJggD46mYgxCi9omUg)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"BEvw_7LEzRWRb_JLaXl1CbPE53mIvyHTJBa4_KdSSvupvaVJ1as3DxHuK7wBl_jVWwctsJggD46mYgxCi9omUg","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientGoalsTasksAgents.kt ===
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
Created goal: nfq-POp9rIQXZGWnvAU_9tCauIbEgSyoswwFEnGMd4aWNcv4rDQd-QdjWzqDKFkNIaRZyINB2kIxFAKiviJp1g

--- Listing goals ---
Goals: {"count":1,"goals":[{"created_at":"2026-09-16T20:51:14.977316+00:00","description":"Ship the next major release","id":"nfq-POp9rIQXZGWnvAU_9tCauIbEgSyoswwFEnGMd4aWNcv4rDQd-QdjWzqDKFkNIaRZyINB2kIxFAKiviJp1g","status":"pending","steps":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0","updated_at":"2026-09-16T20:51:14.977316+00:00"}]}

--- Getting goal ---
Fetched: {"type":"String","value":"Deploy v2.0"}

--- Updating goal ---
Updated description: {"type":"String","value":"Ship v2.0 with full test coverage"}

--- Searching goals ---
Search results: {"count":1,"items":[{"_score":12.870000000000001,"created_at":{"type":"DateTime","value":"2026-09-16T20:51:14.977316+00:00"},"description":{"type":"String","value":"Ship v2.0 with full test coverage"},"id":"nfq-POp9rIQXZGWnvAU_9tCauIbEgSyoswwFEnGMd4aWNcv4rDQd-QdjWzqDKFkNIaRZyINB2kIxFAKiviJp1g","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0"},"updated_at":{"type":"DateTime","value":"2026-09-16T20:51:15.001335+00:00"}}]}

--- Goal step lifecycle ---
Step 0 started: "nfq-POp9rIQXZGWnvAU_9tCauIbEgSyoswwFEnGMd4aWNcv4rDQd-QdjWzqDKFkNIaRZyINB2kIxFAKiviJp1g"
Step 0 completed: "nfq-POp9rIQXZGWnvAU_9tCauIbEgSyoswwFEnGMd4aWNcv4rDQd-QdjWzqDKFkNIaRZyINB2kIxFAKiviJp1g"
Step 1 started: "nfq-POp9rIQXZGWnvAU_9tCauIbEgSyoswwFEnGMd4aWNcv4rDQd-QdjWzqDKFkNIaRZyINB2kIxFAKiviJp1g"
Step 1 failed: "nfq-POp9rIQXZGWnvAU_9tCauIbEgSyoswwFEnGMd4aWNcv4rDQd-QdjWzqDKFkNIaRZyINB2kIxFAKiviJp1g"

--- Completing goal ---
Goal status after complete: {"type":"String","value":"pending_review"}

--- Approving goal ---
Goal status after approve: {"type":"String","value":"in_progress"}

--- Creating goal for rejection ---
Goal status after reject: {"type":"String","value":"failed"}

--- Creating task ---
Created task: Bvt7m6u4KSaNN9eoVTDTXyLMc_DwO9s88U021mNnZ1Vdh4gJJTEGTa_zXdk-7bD7Lu30odxDv9RQ6dGpu7EcPg

--- Listing tasks ---
Tasks: {"count":1,"items":[{"cron":{"type":"String","value":"0 2 * * *"},"description":{"type":"String","value":"Full database backup every night at 2 AM"},"id":"Bvt7m6u4KSaNN9eoVTDTXyLMc_DwO9s88U021mNnZ1Vdh4gJJTEGTa_zXdk-7bD7Lu30odxDv9RQ6dGpu7EcPg","name":{"type":"String","value":"Nightly Backup"}}]}

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
Created agent: mlQpTxEh00XB15ERWh91uUaHOAxr8ExaPCXP3lthSkP7H9LxxPsC3K_lY7d1mand5Q3BPB0LLHj8hFzadLH8sQ — null

--- Listing agents ---
Agents: {"count":1,"items":[{"deployment_id":{"type":"String","value":"deploy_kt_example"},"id":"mlQpTxEh00XB15ERWh91uUaHOAxr8ExaPCXP3lthSkP7H9LxxPsC3K_lY7d1mand5Q3BPB0LLHj8hFzadLH8sQ","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"DataBot"},"system_prompt":{"type":"String","value":"You are a data analysis assistant."}}]}

--- Getting agent by ID ---
Agent: {"type":"String","value":"DataBot"}

--- Getting agent by name ---
By name: {"type":"String","value":"DataBot"}

--- Updating agent ---
Updated agent system_prompt

--- Agents by deployment ---
Agents for deployment: {"count":0,"items":[]}
WARNING: agents-by-deployment omitted created agent mlQpTxEh00XB15ERWh91uUaHOAxr8ExaPCXP3lthSkP7H9LxxPsC3K_lY7d1mand5Q3BPB0LLHj8hFzadLH8sQ; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted

--- Cleanup: deleting goals ---
Goals deleted

=== Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientJoins.kt ===
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
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientJwtAuthFlow.kt ===
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
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvLinks.kt ===
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
Links for user:alice: [{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-09-16T20:51:30.534819Z","last_accessed":"2026-09-16T20:51:30.538463Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-09-16T20:51:30.528086Z","last_accessed":"2026-09-16T20:51:30.538463Z","metadata":{}},{"collection":"orders","document_id":"order_002","field_path":null,"created_at":"2026-09-16T20:51:30.531738Z","last_accessed":"2026-09-16T20:51:30.538463Z","metadata":{}}]

--- Unlinking document ---
Unlinked order_002: null

--- Verifying remaining links ---
Remaining links: [{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-09-16T20:51:30.534819Z","last_accessed":"2026-09-16T20:51:30.546020Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-09-16T20:51:30.528086Z","last_accessed":"2026-09-16T20:51:30.546020Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining documents
Deleted key: user:alice

=== Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvOperations.kt ===
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
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"type":"Object","value":{"email":"alice@example.com","name":"Alice","role":"admin"}}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"value":{"created_at":1789591895586,"user_id":"123"},"type":"Object"}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_kt_1789591895472:config:db: success
  kv_ops_kt_1789591895472:config:cache: success
  kv_ops_kt_1789591895472:config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_kt_1789591895472:config:db: {"port":5432,"host":"localhost"}
  kv_ops_kt_1789591895472:config:cache: {"enabled":true,"ttl":3600}
  kv_ops_kt_1789591895472:config:api: {"timeout":30,"retries":3}

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

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_ops_kt_1789591895472:config:db: deleted
  kv_ops_kt_1789591895472:config:cache: deleted
  kv_ops_kt_1789591895472:config:api: deleted

=== Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvPrecision.kt ===
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

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientPathRoutedFunction.kt ===
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
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientProjection.kt ===
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
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientQueryBuilder.kt ===
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

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientRawCompletionStream.kt ===
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
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Mercury",
    "diameter_km": 4879
  },
  {
    "name": "Venus",
    "diameter_km": 12104
  },
  {
    "name": "Earth",
    "diameter_km": 12742
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you’re having a wonderful day.

=== Done ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSchedules.kt ===
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
=== ekoDB Kotlin Client - Schedules Example ===

--- Creating schedule ---
Created schedule: ba818451-0666-4729-9f00-86116d53dccb — "Hourly Health Check"

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-09-16T20:52:08.545693Z","cron_expression":"0 0 * * * *","description":"Ping all services every hour","enabled":true,"function_label":"schedule_noop_kotlin_75993_1789591928422","id":"ba818451-0666-4729-9f00-86116d53dccb","last_execution":null,"name":"Hourly Health Check","next_execution":"2026-09-16T21:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-09-16T20:52:08.545693Z"}]}

--- Getting schedule ---
Fetched: "Hourly Health Check" (cron: "0 0 * * * *")

--- Updating schedule ---
Updated cron: "0 */30 * * * *"

--- Triggering schedule ---
Trigger response: {"schedule_id":"ba818451-0666-4729-9f00-86116d53dccb","status":"triggered"}

--- Pausing schedule ---
Enabled after pause: false

--- Resuming schedule ---
Enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSchemaManagement.kt ===
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
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=tQTXq3T5vFak3_KuW1PZVRmn_1NewygFXmpDkjpk_tAb7c50dnOMHjXHt6f8xjmcqY13LSScgwTzAwMDDlNiKA)
✓ Inserted user 2: StringValue(value=ZazaxdIJjwcspeGnGp3ff-BRiYzhWDRvNRimnSxMtUjP3W7un1vFovNShFf00pvl2QLpv1A_IQrKYixrdX_Jfw)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 27
  Sample: [schema_users_client_ts, schema_documents_client_ts, chat_goals__ek0_testing, schema_products_client_js, schedules__ek0_testing]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSearch.kt ===
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
Text results: 2; execution time: 1 ms
{"type":"String","value":"Python Programming"}: score=13.200000000000001, matched=[category, title]
{"value":"Rust Programming","type":"String"}: score=13.200000000000001, matched=[category, title]
Vector results: 3
Filtered vector results: 2
Custom-weight hybrid results: 3
Raw search: {"results":[{"record":{"id":"GThoHpnKn-zT8dPaV8Q-Gc4GM7ZJpHL5ZiWfZwDVRoctnh_3OmuoxCq-Fy7znziRf-TBc4Aq2FhvtnorJpJsqg","title":{"value":"Python Programming","type":"String"},"category":{"type":"String","value":"programming"},"embedding":{"value":[0.8,0.2,0.1],"type":"Vector"}},"score":1.0,"matched_fields":[]},{"record":{"id":"gyRqGwU9GXb2cU70TFRAZqXRIQPehYdT5PC6-6DKD-u9NHO5Rntp9jqZEfuVZj0iaVK3V4PHqEO1cXQGj9DgOg","title":{"type":"String","value":"Rust Programming"},"category":{"value":"programming","type":"String"},"embedding":{"type":"Vector","value":[0.9,0.1,0.2]}},"score":1.0,"matched_fields":[]}],"total":2,"execution_time_ms":0}

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSimpleCrud.kt ===
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
=== ekoDB Kotlin Client - Simple CRUD Example ===

=== Create ===
✓ Inserted user: EkoRecord(fields={id=StringValue(value=3i1hGuBPab5pMDeMzPEKG_5UnWQc-YoBSKnOTmW61eHeFpa4MtVECXLXOOGdzLcWmJ0oI8cpe6fowGFqTtF-8w)})
  User ID: 3i1hGuBPab5pMDeMzPEKG_5UnWQc-YoBSKnOTmW61eHeFpa4MtVECXLXOOGdzLcWmJ0oI8cpe6fowGFqTtF-8w

=== Read ===
✓ Found user by ID: EkoRecord(fields={tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), id=StringValue(value=3i1hGuBPab5pMDeMzPEKG_5UnWQc-YoBSKnOTmW61eHeFpa4MtVECXLXOOGdzLcWmJ0oI8cpe6fowGFqTtF-8w), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Wed Sep 16 16:52:23 EDT 2026)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Wed Sep 16 16:52:23 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: tags, active, data, id, created_at, age, categories, user_id, name, email, metadata, embedding, price

=== Update ===
✓ Updated user: EkoRecord(fields={categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), created_at=ObjectValue(value={value=StringValue(value=Wed Sep 16 16:52:23 EDT 2026), type=StringValue(value=String)}), id=StringValue(value=3i1hGuBPab5pMDeMzPEKG_5UnWQc-YoBSKnOTmW61eHeFpa4MtVECXLXOOGdzLcWmJ0oI8cpe6fowGFqTtF-8w)})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={id=StringValue(value=3i1hGuBPab5pMDeMzPEKG_5UnWQc-YoBSKnOTmW61eHeFpa4MtVECXLXOOGdzLcWmJ0oI8cpe6fowGFqTtF-8w), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Wed Sep 16 16:52:23 EDT 2026)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Delete ===
✓ Deleted user with ID: 3i1hGuBPab5pMDeMzPEKG_5UnWQc-YoBSKnOTmW61eHeFpa4MtVECXLXOOGdzLcWmJ0oI8cpe6fowGFqTtF-8w

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSimpleWebsocket.kt ===
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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"data":[{"id":"K4xD8QA5bTFBStg1tfvFtsh1qoeJXPYvy-qjJ80cBJTvi8Got1msdzQF0DRPxbHcKmfT3yY7rU_AUMh2GBAxiw","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSwrNative.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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

🧹 Cleaning up...
✓ Deleted 0 test functions and owned SWR collections

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR function: github_user_native_kt (6DYYV3-1SZgYeLRsMlTKSfajSf9rCjxe_uFso2Xw254_SwrnD8q4ysUBxjTIp5-FY-WsO6xkjOSVyPFKYvSxEA)

First call (cache miss - will fetch from GitHub API):
  Response time: 196ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 6ms
  Speedup: 32.7x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit_kt (0rQv_CYEXhewXdzHK8rgUJ_IGfCn8ggzuZTMIO4ozSIzg5DmlfMU9_5e_DGs3BCkjXR322r_ZI_fUN4GMMaqpw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_kt' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_kt (hqwmnOub7QD2PT1My4tZY0dfi7h62_2tqusN3iNo4DlozkTB9ac66oy4nJ3xKihlUVWmBXDICtu8yJdqNgDJEA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_kt' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache_kt (Qd7edcAZlBEiyQwIafAsxfo5plL55bClasOW1DeVNdE6hS3FBWZvu5sh24vma8E7ccEhJMKF1qH9jhh-yCiMvQ)
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
✓ Deleted 4 test functions and owned SWR collections

✅ All examples completed!

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSwrPattern.kt ===
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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Setting up cache collection...
✓ Cache entry created

Step 2: Create SWR cache lookup function
✓ Created SWR function: swr_cache_lookup_kt_1789591959737 (JNRtwV0wmbtnkPzpu5GMI7ezI1oz3Bt8EmsG_JD8UYY-PtPxmCMaZcZutlmrwvitMoeWU8WRmuyHRtM_mbZnlQ)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 4ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete


BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientTransactions.kt ===
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: NacIptDiamV6mP3ePdHhI_HRiLuowEUhCSSXi7GM5vDCfkTB3-xadIks4pRTnS-i7ESiQ8ho0vUAnDa9IpLRBw
Created Bob: $500 - ID: UG60lZ5YmgtbZB52k4GdeIi3tUyBRB7iyqonmbMcS-RDryqbqitKGDFK3OjRbqS8bHSfmByZf-3l6-UygUSoVw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): f38f2ab8-9e20-453f-90cf-85ea66d93a51

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 489810c7-3a9b-4a25-938d-f1be8d86d39e
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientUserFunctions.kt ===
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
=== ekoDB Kotlin Client - User Functions Example ===

=== Create User Function ===
Created user function with ID: aPIO-3-4YQz_uMq-KBlr60HVKmbRQwzABpx4rcE5fidXQP2f3LONbPemuVnFuaLP65b1sdoL3-1AUxpP2uOXNw

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 11 user functions:
  - "get_active_users_kt": "Get Active Users (Kotlin)"
  - "get_active_users_client_js": "Get Active Users (Updated)"
  - "fetch_product_reviews_ts_68859_1789591599098": "Fetch Product with Reviews (Multi-API)"
  - "get_active_users_client_ts_updated": "Get Active Users (Updated)"
  - "fetch_product_reviews_ts_71483_1789591713776": "Fetch Product with Reviews (Multi-API)"

=== List User Functions by Tag ===
Found 1 user functions with 'kotlin' tag:
  - "get_active_users_kt"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketChatStream.kt ===
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
=== WebSocket Chat Streaming Example (Kotlin) ===

Created chat session: ZRr2a5mq9zuIkB1D1Ssx4Hv7FGZT4StjQOu3ZNCOhkNBbOPaiiEnV4VQHAAWkMxKby49kU4m3MD_ZvRPaf-JyA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: n2Z5A6lKcZ7hWDKRD_SfKKBvPn37NovCHngwJm5umyNuelg0ZEdQqqRysMRdQqv3qkDJeWFUudOfEnVp7mkAaQ
Execution time: 815ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketSubscribe.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== WebSocket Subscription Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_kt' ===
✓ Subscribed (subscription_id: sub_a2060ddb7c47450bb9ed003c079a95bb)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "8xEvkMx8hOJ0uOD_eJFBHNCU_1bZTynvH2jOVlIrYG9SuC0T_V3D9NNQcejsh-oUrlGy7SBhgQ0Wuwmdl4xtXQ"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["8xEvkMx8hOJ0uOD_eJFBHNCU_1bZTynvH2jOVlIrYG9SuC0T_V3D9NNQcejsh-oUrlGy7SBhgQ0Wuwmdl4xtXQ"]
     Timestamp:  "2026-09-16T20:53:01.216961+00:00"

Inserting record 2...
✓ Inserted: "9DsJtLGLeS_oLFvI2jP3I1XUG9bSqrexAAmcPyz8VrD-5HiTbJWvZK_MLdn4pWXGo94E50kez5bixR0OO8FA2w"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["9DsJtLGLeS_oLFvI2jP3I1XUG9bSqrexAAmcPyz8VrD-5HiTbJWvZK_MLdn4pWXGo94E50kez5bixR0OO8FA2w"]

=== Unsubscribing ===
✓ Unsubscribed: {"payload":{"data":{"collection":"ws_subscribe_example_kt","found":true,"unsubscribed":true}},"type":"Success"}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_kt'

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketTtl.kt ===
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
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=_L13y--heJXOKS517FrDxmFWo37nakYt2zSOmDurEmL1W_hIZhGfIPJuToaykGbvLwlfIMIsnofdcqILKtM8Mw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"created_at":{"type":"Integer","value":1789591986134},"name":{"type":"String","value":"WebSocket TTL Test"},"value":{"type":"Integer","value":42},"id":"_L13y--heJXOKS517FrDxmFWo37nakYt2zSOmDurEmL1W_hIZhGfIPJuToaykGbvLwlfIMIsnofdcqILKtM8Mw","ttl":"2026-09-16T21:53:06.236170Z"}]}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: BypassRippleExample.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== Bypass Ripple Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
1. Basic insert (ripple enabled):
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=NPpkz99HsAZM1b_62H09O5FBSwEPAfkT6bF2eJ4qs6s11EhPOQCUDsrruzusKECbjnrDEqR_LAuAvqzF22GpFw)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=RYfn20pXVGptyiCCxyx_820ehQs-GT0LG5aOYLZapel7rmk210I5pFvW0M-3X9RlCrHv5Jku1rN7NuKzQt7v3Q)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={id=StringValue(value=NPpkz99HsAZM1b_62H09O5FBSwEPAfkT6bF2eJ4qs6s11EhPOQCUDsrruzusKECbjnrDEqR_LAuAvqzF22GpFw), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=NPpkz99HsAZM1b_62H09O5FBSwEPAfkT6bF2eJ4qs6s11EhPOQCUDsrruzusKECbjnrDEqR_LAuAvqzF22GpFw), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)}), price=ObjectValue(value={value=IntegerValue(value=500), type=StringValue(value=Integer)})})

✅ All bypass_ripple operations completed successfully!

BUILD SUCCESSFUL in 4s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
