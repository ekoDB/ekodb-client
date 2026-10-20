make test-examples-kotlin
🟣 Building Kotlin client library...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 6s
11 actionable tasks: 11 up-to-date
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
✅ Kotlin client built!
=== Running Kotlin example: ClientAdmin.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Admin permissions read successfully

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientAdvancedCrud.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Inserted: Zq0YJumT3Pa4oimAE7BGARBoc8VqJ-NYNZHD7OKa_vnp6uK7h9AkN3SN1GQAZNWZa5AuaD0drwl3Q3PNogSSiw

--- updateWithAction: increment views ---
After increment: EkoRecord(fields={name=ObjectValue(value={value=StringValue(value=Counter Record), type=StringValue(value=String)}), id=StringValue(value=Zq0YJumT3Pa4oimAE7BGARBoc8VqJ-NYNZHD7OKa_vnp6uK7h9AkN3SN1GQAZNWZa5AuaD0drwl3Q3PNogSSiw), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example)]), type=StringValue(value=Array)}), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)})})

--- updateWithAction: push to tags ---
After push: EkoRecord(fields={id=StringValue(value=Zq0YJumT3Pa4oimAE7BGARBoc8VqJ-NYNZHD7OKa_vnp6uK7h9AkN3SN1GQAZNWZa5AuaD0drwl3Q3PNogSSiw), name=ObjectValue(value={value=StringValue(value=Counter Record), type=StringValue(value=String)}), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced)])}), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)})})

--- updateWithActionSequence: multiple operations ---
After sequence: EkoRecord(fields={tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced), StringValue(value=sequenced)])}), score=ObjectValue(value={value=FloatValue(value=75.0), type=StringValue(value=Float)}), name=ObjectValue(value={value=StringValue(value=Counter Record), type=StringValue(value=String)}), id=StringValue(value=Zq0YJumT3Pa4oimAE7BGARBoc8VqJ-NYNZHD7OKa_vnp6uK7h9AkN3SN1GQAZNWZa5AuaD0drwl3Q3PNogSSiw), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=115)})})

--- Cleanup ---
Deleted collection: kotlin_advanced_crud_example

=== Example Complete ===

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientBatchOperations.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
  IDs: _EN2Vbc3tbahQVIQomJ0Yebi4j-ZLTYucR2sM5D2pDuPi9DfzZ9ncCqh5gXCTCc6chbhvkNjqoC3XwUIzxVSSg, 82R4uKzPFfUPXI28nOqBJOWTluEsyJBN4ddeQtOL_pTHlnMKPdQgcRHMrh9-Z2EW2WJjhMl-NRKqEeSZ1Ge0jw, AaZUaVh2sXLJycodG20jVqs7tyfNbhVXnmQmQAI0dqQl7EuZMSwb9wbzItQA-XyoK4sSrRJJA-a8CQ6w4EVgjA...

=== Batch Update ===
✓ Updated 3 records; 0 failed

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatAdvanced.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created session: Hc_JFEIUFWXKJnefaeGSOYrutQzDPcRJkWr98rfCx3Nyc1F8gHyZdrhqvHfpcAB0xV4PRKJsI06hTAD46RmkKA

=== Sending Initial Message ===
✓ Message sent
  Responses: ["It seems that there are currently no available products in the database. Would you like to add any products or check for something else?"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The price of ekoDB is $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: ntGUyB3d9JLMpimjXDKa-XPuqNEkoP0T_ufVrqkngG8uZmIxfcn4_mjweX8Bk_n3UEyqFHfQxc2gMUNzx5RCGA

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat session: ntGUyB3d9JLMpimjXDKa-XPuqNEkoP0T_ufVrqkngG8uZmIxfcn4_mjweX8Bk_n3UEyqFHfQxc2gMUNzx5RCGA
✓ Deleted chat session: Hc_JFEIUFWXKJnefaeGSOYrutQzDPcRJkWr98rfCx3Nyc1F8gHyZdrhqvHfpcAB0xV4PRKJsI06hTAD46RmkKA
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 17s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatBasic.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created session: BBiND7jyKwbScAnnayo8dsZ0xGe3H0GW80W44XI0QZw9Rz93DxN1CxdmPbgWE0bvtIiV38NwTADgt_GmOVB_0A

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "Qp0u69YHzmXtF867nWO__LBJxNazI7Q5Si-KeQu0ewTxgZnAhUZa_AVFcMJHs5a98vv0SoHSIXRRauVw6J5jzw"
  Responses: ["ekoDB is a high-performance database that integrates intelligent caching and real-time capabilities along with AI features. It allows users to utilize advanced search options including full-text search, vector search, and hybrid search with automatic context retrieval. \n\n### Key Features of ekoDB:\n1. **AI Chat Integration:** ekoDB supports a chat feature that enables users to query the database using natural language, providing AI-powered responses that include relevant context.\n2. **Search Capabilities:** It supports various search methods:\n   - **Full-text Search:** Allows for keyword-based querying.\n   - **Vector Search:** Utilizes vector embeddings for semantic searches.\n   - **Hybrid Search:** Combines both keyword and semantic searching for more effective results. \n\nIn summary, ekoDB is designed for high-performance data handling with intelligent search capabilities and seamless AI integration."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 12s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatMessageStream.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created session: TOvlnJ_JnnXG8n81MIqrogGIRrvyuImgQcf9qeCKpTHwbvtzrUMXKJ_RtQ5KE8J5FfSo-enSqKzZX9Zf10ET_g

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a database management system designed for high-performance and scalability, often tailored for applications that require efficient data handling and fast access times. Its specific features can vary, but it generally emphasizes efficient data storage, retrieval, and management capabilities that cater to modern application needs.

If you're looking for specific functions, architecture details, or use cases of EkoDB, please provide more context, and I'll do my best to give you the information you need!

--- Stream complete ---
Message ID: 6St0GoXKQUWyoqTmWeW1iU6gSHh3CorC9blYDzULKjZNzGardF88it0JGILTGnT61iEdzJDorTeN5cIDSvmFwg
Execution time: 1456ms
Context window: 128000 tokens

✓ Chat message stream example completed

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatModels.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - gpt-4
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-3.5-turbo-0125
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - o1-pro
    - gpt-5.1-codex
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - omni-moderation-latest
    - gpt-5-2025-08-07
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-5.4-mini-2026-03-17
    - babbage-002
    - gpt-4o-2024-05-13
    - text-embedding-3-large
    - gpt-4o-mini-transcribe-2025-12-15
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-audio-mini-2025-10-06
    - gpt-5.2-pro-2025-12-11
    - gpt-4-turbo
    - gpt-4o-2024-08-06
    - o3
    - gpt-4o-mini-search-preview-2025-03-11
    - o3-mini-2025-01-31
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-3.5-turbo-16k
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-4-0613
    - gpt-5.1-2025-11-13
    - gpt-4o-search-preview
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-5.1-codex-max
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-3.5-turbo-instruct-0914
    - o1-2024-12-17
    - sora-2
    - tts-1-hd-1106
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-5
    - gpt-4o-mini-transcribe
    - gpt-image-2-2026-04-21
    - gpt-5.2-chat-latest
    - sora-2-pro
    - gpt-3.5-turbo-instruct
    - gpt-transcribe
    - gpt-4.1-nano-2025-04-14
    - o4-mini-deep-research
    - gpt-5.4
    - gpt-4o-search-preview-2025-03-11
    - text-embedding-3-small
    - gpt-5.1-chat-latest
    - gpt-4o-mini-2024-07-18
    - gpt-4.1-2025-04-14
    - gpt-5.3-chat-latest
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - gpt-3.5-turbo
    - gpt-5.2-codex
    - gpt-4o
    - davinci-002
    - gpt-5.4-nano
    - o3-mini
    - o4-mini-deep-research-2025-06-26
    - gpt-audio-mini-2025-12-15
    - o1-pro-2025-03-19
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - o4-mini
    - gpt-5.1-codex-mini
    - gpt-3.5-turbo-1106
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - o1
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-6-luna
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-image-2.5-flare
    - gpt-4o-mini-search-preview
    - gpt-6-sol
    - gpt-6-astra
    - gpt-6.1-sol
    - gpt-live-1
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
  anthropic:
    - claude-haiku-5-5
    - claude-sonnet-5-5
    - claude-opus-5-5
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
  gemini:
    - gemini-2.5-flash
    - gemini-2.5-pro
    - gemini-2.5-flash-preview-tts
    - gemini-2.5-pro-preview-tts
    - gemma-4-26b-a4b-it
    - gemma-4-31b-it
    - gemini-flash-latest
    - gemini-flash-lite-latest
    - gemini-pro-latest
    - gemini-2.5-flash-lite
    - gemini-2.5-flash-image
    - gemini-3-flash-preview
    - gemini-3.1-pro-preview
    - gemini-3.1-pro-preview-customtools
    - gemini-3.1-flash-lite-preview
    - gemini-3.1-flash-lite
    - gemini-3-pro-image-preview
    - gemini-3-pro-image
    - nano-banana-pro-preview
    - gemini-3.1-flash-image-preview
    - gemini-3.1-flash-image
    - gemini-3.1-flash-lite-image
    - gemini-nano-banana-2.1
    - gemini-3.5-flash
    - gemini-3.5-flash-lite
    - gemini-omni-flash-preview
    - gemini-omni-1.1-flash
    - gemini-3.5-transcribe
    - gemini-3.6-flash
    - gemini-3.7-flash
    - gemini-3.8-flash
    - gemini-4-argon
    - lyria-3-clip-preview
    - lyria-3-pro-preview
    - lyria-3.5
    - gemini-3.1-flash-tts-preview
    - gemini-3.8-flash-tts
    - gemini-3.8-flash-lite-tts
    - gemini-robotics-er-2-preview
    - gemini-2.5-computer-use-preview-10-2025
    - antigravity-preview-05-2026
    - antigravity-preview-09-2026
    - antigravity-preview-latest
    - deep-research-max-preview-04-2026
    - deep-research-preview-04-2026
    - deep-research-pro-preview-12-2025
    - gemini-embedding-001
    - gemini-embedding-2-preview
    - gemini-embedding-2
    - aqa
    - veo-3.1-generate-preview
    - veo-3.1-fast-generate-preview
    - veo-3.1-lite-generate-preview
    - gemini-3.5-transcribe-live
    - gemini-2.5-flash-native-audio-latest
    - gemini-2.5-flash-native-audio-preview-09-2025
    - gemini-2.5-flash-native-audio-preview-12-2025
    - gemini-3.1-flash-live-preview
    - gemini-3.8-live
    - gemini-3.8-live-extended-thinking
    - gemini-robotics-er-2-streaming-preview
    - gemini-3.5-live-translate-preview
    - lyria-realtime-exp
Provider status:
  anthropic: ok 14 models
  gemini: ok 63 models
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, gpt-5-2025-08-07, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, gpt-4o-mini-transcribe-2025-12-15, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4.1-mini, gpt-4o-mini, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, o1-2024-12-17, sora-2, tts-1-hd-1106, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5.1-chat-latest, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-5.2-codex, gpt-4o, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, gpt-image-2.5-sunburst-2026-09-08, gpt-6-luna, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6-astra, gpt-6.1-sol, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

=== Chat Models Example Complete ===

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatSessions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created session: 87t_ueZy7PrZuoudOIuC38PHl9K1aseSWJJnt9d24ammXaxSiWlj33P0gutVUu8dvBbwaFsjgFLL-wsfmEwLIQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["It appears that there are no products available based on the current query filters. Would you like to search using different criteria or explore more details about any specific product?"]

✓ Message 2 sent
  Responses: ["The price of the product \"ekoDB\" is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"87t_ueZy7PrZuoudOIuC38PHl9K1aseSWJJnt9d24ammXaxSiWlj33P0gutVUu8dvBbwaFsjgFLL-wsfmEwLIQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"qxXYVNgYguRcfrNmLoPb-g5E5OoFDV0i53QwHvsxZn8Yemp9P07sY43iW-x9bes85zbsuixHqIh_msYatZSLew","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-10-20T20:09:08.226492+00:00"},"id":"fs0cW_tzj1rJpam2WQgevCA3y1mz_qvNNgkKbQo5E-eTWAXzzq0pt-t1PF1VZD_eTme8Sf8noTIkh3nAfiztlA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":83,"prompt_tokens":6671,"total_tokens":6754}},"updated_at":{"type":"DateTime","value":"2026-10-20T20:09:08.226492+00:00"}},{"chat_id":{"type":"String","value":"87t_ueZy7PrZuoudOIuC38PHl9K1aseSWJJnt9d24ammXaxSiWlj33P0gutVUu8dvBbwaFsjgFLL-wsfmEwLIQ"},"content":{"type":"String","value":"It appears that there are no products available based on the current query filters. Would you like to search using different criteria or explore more details about any specific product?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"qxXYVNgYguRcfrNmLoPb-g5E5OoFDV0i53QwHvsxZn8Yemp9P07sY43iW-x9bes85zbsuixHqIh_msYatZSLew","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-10-20T20:09:08.237163+00:00"},"id":"B8PDDHGhtl-n9b888Az5IHVUtXIcfKWDfV3FRj048KbRqV2YD3xJhpdPbrEHULvNTrFCNaP-x2Dik8CWnJ37Dg","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":83,"prompt_tokens":6671,"total_tokens":6754}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","filter":{"content":{"field":"__rag_matched","operator":"Eq","value":"description"},"type":"Condition"}},"id":"call_JwO4u7yQZg3fYqgFFGAyeqqI","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":0,"records":[]},"success":true,"tool_call_id":"call_JwO4u7yQZg3fYqgFFGAyeqqI","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-10-20T20:09:08.237163+00:00"}},{"chat_id":{"type":"String","value":"87t_ueZy7PrZuoudOIuC38PHl9K1aseSWJJnt9d24ammXaxSiWlj33P0gutVUu8dvBbwaFsjgFLL-wsfmEwLIQ"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"qxXYVNgYguRcfrNmLoPb-g5E5OoFDV0i53QwHvsxZn8Yemp9P07sY43iW-x9bes85zbsuixHqIh_msYatZSLew","price":99,"product":"ekoDB"},"score":0.7157894736842105}]},"created_at":{"type":"DateTime","value":"2026-10-20T20:09:09.467195+00:00"},"id":"MpVEfKLvtqLUC_AIuxLscFX71089P1r8egSKLzacuNvqxl5KVOwlkyI5qgN6-tqPxVc4lDmlsla7SM4yRy7Bqw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":10,"prompt_tokens":3378,"total_tokens":3388}},"updated_at":{"type":"DateTime","value":"2026-10-20T20:09:09.467195+00:00"}},{"chat_id":{"type":"String","value":"87t_ueZy7PrZuoudOIuC38PHl9K1aseSWJJnt9d24ammXaxSiWlj33P0gutVUu8dvBbwaFsjgFLL-wsfmEwLIQ"},"content":{"type":"String","value":"The price of the product \"ekoDB\" is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"qxXYVNgYguRcfrNmLoPb-g5E5OoFDV0i53QwHvsxZn8Yemp9P07sY43iW-x9bes85zbsuixHqIh_msYatZSLew","price":99,"product":"ekoDB"},"score":0.7157894736842105}]},"created_at":{"type":"DateTime","value":"2026-10-20T20:09:09.478855+00:00"},"id":"R6EQPlXqK_YetSUYEHhcG7J_Rwk6oKyjvwUaSlFyqKRitar-TBjhIrHu1pYPn4Mr31bWpV_6PxzZafDcw0eDAA","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":10,"prompt_tokens":3378,"total_tokens":3388}},"updated_at":{"type":"DateTime","value":"2026-10-20T20:09:09.478855+00:00"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 1

=== Branching Session ===
✓ Created branched session: 1EmW4LP3NLnIyocaYdN-mf2-GQczbl34Utd7RwNO48AltQHVNOdK3YpncpcaTHlvOaWXBCasU1CqbGKBR3wsiA

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 15s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCollectionManagement.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
  - schema_employees_client_ts
  - chat_goals__ek0_testing
  - schema_documents_client_js
  - chat_agent_configs__ek0_testing
  - chat_tasks__ek0_testing

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-10-20T20:09:19.310042Z","last_modified":"2026-10-20T20:09:19.310042Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCollectionUtils.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Collection metadata: {"analytics":["collection_utils_test_kt",{"manifest_backed":false,"record_count":5,"resident_record_ids":5,"total_size":675}],"collection":{"bypass_ripple":false,"created_at":"2026-10-20T20:09:28.850700Z","fields":{"index":{"default":null,"enums":[],"field_type":"Integer","max":null,"min":null,"regex":null,"required":false,"unique":false},"name":{"default":null,"enums":[],"field_type":"String","max":null,"min":null,"regex":null,"required":false,"unique":false}},"last_modified":"2026-10-20T20:09:28.851048Z","primary_key_alias":"id","version":1}}

=== List Collections ===
All collections (27):
  - schema_employees_client_ts
  - chat_goals__ek0_testing
  - schema_documents_client_js
  - collection_utils_test_kt
  - chat_agent_configs__ek0_testing
  - chat_tasks__ek0_testing
  - test_accounts
  - schema_employees_client_go
  - schema_documents_client_ts
  - schema_users_client_ts
  - schema_users_client_go
  - chat_goal_templates__ek0_testing
  - schema_employees_client_js
  - agent_function_versions__ek0_testing
  - chat_configurations__ek0_testing
  - schema_documents_client_go
  - chat_raw_completions__ek0_testing
  - functions__ek0_testing
  - schedules__ek0_testing
  - chat_turns__ek0_testing
  - test_collection
  - schema_products_client_js
  - schema_users_client_js
  - audit__ek0_testing
  - schema_products_client_go
  - chat_messages__ek0_testing
  - schema_products_client_ts

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz_2148977175491041' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_kt'

=== Collection Utilities Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientConcurrencyStages.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientConvenienceMethods.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created record: EkoRecord(fields={id=StringValue(value=fkcSkg4A9X94uICqTDuRE27an0dAL8ucYc9eZ3qc3xRi6N-dG19XsNeoGKwLSIaq8GvYSwiC5NEdT41IE-G7yw)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), id=StringValue(value=fkcSkg4A9X94uICqTDuRE27an0dAL8ucYc9eZ3qc3xRi6N-dG19XsNeoGKwLSIaq8GvYSwiC5NEdT41IE-G7yw), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=new-user-id)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={id=StringValue(value=fkcSkg4A9X94uICqTDuRE27an0dAL8ucYc9eZ3qc3xRi6N-dG19XsNeoGKwLSIaq8GvYSwiC5NEdT41IE-G7yw), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCryptoStages.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientDistinctValues.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientDocumentTtl.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
  Document ID: CFJwczzpmZhU32THePV089GREewlVUSfqpcSeAjGbc5_9pm2-p7gaU7lHV0PgWD_PBjiVwkfsn1j1_fw-5R-_g

=== Verify Document Exists ===
✓ Document found: id, session_id, user_id, created_at, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: cSW24Y3gqM9T2FswOa0yRfvMwGovA5ej540sGEZo6WZp1apFmIwPVs9iqlveKHArwsfnZ-tCDBeFiL0bmh2xSw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: value, id, cache_key, ttl

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientEdgeCache.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Edge cache function created: IMEX4GN1GttGzwXvrFNyOqRpIZLEV_h1M49zBgZ1UWiaIuTyaRen6ylYbyUM_0wpr_h10YEz9_U7jAS1xO45wg

Call 1: Cache lookup
Found 1 cached entries
Response time: 18ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 9ms

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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionComposition.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 77ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   🚀 Cache speedup: 25.7x faster!

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

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionContract.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
client_function_contract: ok

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

✅ Function saved: tIcu0oX4mANyLZKIC_rh181RPuecTbQAImM_ITglIA6XTiYiYoQIKwnp9xRY2rpZCKYI2W7oU0waZWiDKHJaxw
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":50.0},"status":{"value":"inactive","type":"String"}}
   {"avg_score":{"value":60.0,"type":"Float"},"status":{"type":"String","value":"active"},"count":{"value":5,"type":"Integer"}}
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
📊 Total user count: {"value":10,"type":"Integer"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted test functions
✅ Deleted collection

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsAdvanced.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
   {"category":{"value":"Electronics","type":"String"},"avg_price":{"value":367.0,"type":"Float"},"count":{"type":"Integer","value":5}}
   {"avg_price":{"value":365.6666666666667,"type":"Float"},"category":{"type":"String","value":"Furniture"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsAi.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

1. **Efficient Similarity Search**: They enable fast and accurate similarity searches by storing embeddings and supporting nearest neighbor algorithms.

2. **High-dimensional Data Handling**: Ideal for managing high-dimensional data, such as images, audio, and text embeddings.

3. **Scalability**: Designed to handle large datasets, allowing for efficient scaling as data grows.

4. **Real-time Processing**: Support for real-time querying, making them suitable for applications like recommendation systems.

5. **Integration with ML Models**: Seamlessly integrate with machine learning workflows, enhancing model deployment and inference.

6. **Flexibility**: Can accommodate various data types and formats, making them versatile for diverse applications.

7. **Improved Search Relevance**: Provide more contextually relevant search results compared to traditional databases.

8. **Support for Advanced Analytics**: Facilitate complex queries and analytics on unstructured data.

Overall, vector databases significantly enhance the capabilities of applications dealing with large-scale, complex data.
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

BUILD SUCCESSFUL in 12s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsComplete.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

✅ Function saved: DrzOFBss4Y30wMaya_DAY2A7kLBOXBjOtvyhItQpzxUCyW1BWaXKNAFHDje_S85LNswvasPoucijb5U_TJ6dig
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsCrud.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
   {"status":{"value":"inactive","type":"String"},"count":{"type":"Integer","value":3}}
   {"status":{"value":"active","type":"String"},"count":{"value":7,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsKvWrapped.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

✅ Inserted order: StringValue(value=EGvo2MonsX0PQWoplpTcemugIGdQinlf9PtzYTJVtrWRkRTT_GfaUbZxg32gN3aFx1CbrRdGN0aBTmBv9PcKog)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: DEjao63okRc_EH7rOWKetcLba8PW1qevwWTrsXMeZy109eVXPc7xugJXWpmfw-Fbf2D-RLTLQIz4M0ohlA4AxQ
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: Q4nYum2llBWSGYccVfFpt5ucdUTaW0G9ep3a0vG8k0wjxXF3pzITWyq0BOORBYJPJGjYQ567kdj16gfNIdANqg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: dUt184HNB1JxEPcPFWlVwx0LN6JRM1zTs19qWZ8HgDlyXVHjck21TLKerX4kXXdVM37B1LHkaXeDmO5QT43i7A
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsSearch.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
   1. {"type":"String","value":"Database Design Principles"} ({"value":"Database","type":"String"})
   2. {"value":"Getting Started with ekoDB","type":"String"} ({"value":"Database","type":"String"})
   3. {"type":"String","value":"Introduction to Machine Learning"} ({"value":"AI","type":"String"})
   4. {"value":"Vector Databases Explained","type":"String"} ({"type":"String","value":"Database"})
   5. {"value":"Natural Language Processing","type":"String"} ({"value":"AI","type":"String"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"type":"Integer","value":3}}
   {"category":{"value":"AI","type":"String"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientGoalTemplates.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created template: Data Migration (id: VeKkrc5bnd3TATPRPlYTXZqAd0dSsGDO4Qf8-1eAXmH_jhAIn5XwphJAfyoi-qA2PiBpPeZcd5Yk8y8xmj3EIA)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"VeKkrc5bnd3TATPRPlYTXZqAd0dSsGDO4Qf8-1eAXmH_jhAIn5XwphJAfyoi-qA2PiBpPeZcd5Yk8y8xmj3EIA","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientGoalsTasksAgents.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created goal: _6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA

--- Listing goals ---
Goals: {"count":1,"goals":[{"created_at":"2026-10-20T20:12:23.972009+00:00","description":"Ship the next major release","id":"_6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA","status":"pending","steps":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0","updated_at":"2026-10-20T20:12:23.972009+00:00"}]}

--- Getting goal ---
Fetched: {"type":"String","value":"Deploy v2.0"}

--- Updating goal ---
Updated description: {"type":"String","value":"Ship v2.0 with full test coverage"}

--- Searching goals ---
Search results: {"count":1,"items":[{"_score":12.870000000000001,"created_at":{"type":"DateTime","value":"2026-10-20T20:12:23.972009+00:00"},"description":{"type":"String","value":"Ship v2.0 with full test coverage"},"id":"_6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0"},"updated_at":{"type":"DateTime","value":"2026-10-20T20:12:23.995448+00:00"}}]}

--- Goal step lifecycle ---
Step 0 started: "_6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA"
Step 0 completed: "_6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA"
Step 1 started: "_6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA"
Step 1 failed: "_6QjemzdQ8-olPU889KtRfQ7Rn8drSCzCxdo2uRYY5B5yHK-2mT1VroxDwGiMNqpU98-HTcf0WPeKSSg8ejFWA"

--- Completing goal ---
Goal status after complete: {"type":"String","value":"pending_review"}

--- Approving goal ---
Goal status after approve: {"type":"String","value":"in_progress"}

--- Creating goal for rejection ---
Goal status after reject: {"type":"String","value":"failed"}

--- Creating task ---
Created task: tbtxHlmoE9ZOBpchWuS7B9sMdQsjiAd-ZwYypk9gh_giA_alIJYkqv1Qx3rLamwAAky_tgG7ZVXOTOK2O1NdPg

--- Listing tasks ---
Tasks: {"count":1,"items":[{"cron":{"type":"String","value":"0 2 * * *"},"description":{"type":"String","value":"Full database backup every night at 2 AM"},"id":"tbtxHlmoE9ZOBpchWuS7B9sMdQsjiAd-ZwYypk9gh_giA_alIJYkqv1Qx3rLamwAAky_tgG7ZVXOTOK2O1NdPg","name":{"type":"String","value":"Nightly Backup"}}]}

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
Created agent: 8Mi0RgBe-dhuoQYzoiyKnDP8MxM4K8fS4nbQX2o9bAg6zBvuY2vAwt_fkX7upSjDGlsydYxMoqzQtFB06ptHLw — null

--- Listing agents ---
Agents: {"count":1,"items":[{"deployment_id":{"type":"String","value":"deploy_kt_example"},"id":"8Mi0RgBe-dhuoQYzoiyKnDP8MxM4K8fS4nbQX2o9bAg6zBvuY2vAwt_fkX7upSjDGlsydYxMoqzQtFB06ptHLw","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"DataBot"},"system_prompt":{"type":"String","value":"You are a data analysis assistant."}}]}

--- Getting agent by ID ---
Agent: {"type":"String","value":"DataBot"}

--- Getting agent by name ---
By name: {"type":"String","value":"DataBot"}

--- Updating agent ---
Updated agent system_prompt

--- Agents by deployment ---
Agents for deployment: {"count":0,"items":[]}
WARNING: agents-by-deployment omitted created agent 8Mi0RgBe-dhuoQYzoiyKnDP8MxM4K8fS4nbQX2o9bAg6zBvuY2vAwt_fkX7upSjDGlsydYxMoqzQtFB06ptHLw; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted

--- Cleanup: deleting goals ---
Goals deleted

=== Example Complete ===

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientJoins.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientJwtAuthFlow.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvLinks.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Links for user:alice: [{"collection":"orders","document_id":"order_002","field_path":null,"created_at":"2026-10-20T20:12:54.649053Z","last_accessed":"2026-10-20T20:12:54.657256Z","metadata":{}},{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-10-20T20:12:54.653041Z","last_accessed":"2026-10-20T20:12:54.657256Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-10-20T20:12:54.641679Z","last_accessed":"2026-10-20T20:12:54.657256Z","metadata":{}}]

--- Unlinking document ---
Unlinked order_002: null

--- Verifying remaining links ---
Remaining links: [{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-10-20T20:12:54.641679Z","last_accessed":"2026-10-20T20:12:54.664437Z","metadata":{}},{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-10-20T20:12:54.653041Z","last_accessed":"2026-10-20T20:12:54.664437Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining documents
Deleted key: user:alice

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvOperations.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Retrieved value: {"value":{"email":"alice@example.com","name":"Alice","role":"admin"},"type":"Object"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"value":{"created_at":1792527184736,"user_id":"123"},"type":"Object"}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_kt_1792527184596:config:db: success
  kv_ops_kt_1792527184596:config:cache: success
  kv_ops_kt_1792527184596:config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_kt_1792527184596:config:db: {"host":"localhost","port":5432}
  kv_ops_kt_1792527184596:config:cache: {"ttl":3600,"enabled":true}
  kv_ops_kt_1792527184596:config:api: {"timeout":30,"retries":3}

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
  kv_ops_kt_1792527184596:config:db: deleted
  kv_ops_kt_1792527184596:config:cache: deleted
  kv_ops_kt_1792527184596:config:api: deleted

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvPrecision.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientPathRoutedFunction.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientProjection.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientQueryBuilder.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=92)})
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

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientRawCompletionStream.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===

BUILD SUCCESSFUL in 11s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSchedules.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created schedule: 707ad563-b078-488f-9f49-cdd3f2910203 — "Hourly Health Check"

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-10-20T20:14:06.348376Z","cron_expression":"0 0 * * * *","description":"Ping all services every hour","enabled":true,"function_label":"schedule_noop_kotlin_58353_1792527246207","id":"707ad563-b078-488f-9f49-cdd3f2910203","last_execution":null,"name":"Hourly Health Check","next_execution":"2026-10-20T21:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-20T20:14:06.348376Z"}]}

--- Getting schedule ---
Fetched: "Hourly Health Check" (cron: "0 0 * * * *")

--- Updating schedule ---
Updated cron: "0 */30 * * * *"

--- Triggering schedule ---
Trigger response: {"schedule_id":"707ad563-b078-488f-9f49-cdd3f2910203","status":"triggered"}

--- Pausing schedule ---
Enabled after pause: false

--- Resuming schedule ---
Enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSchemaManagement.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Inserted user 1: StringValue(value=NLaancsbWEZnzOkHsmYFhuC7MLpAfEmy684GLDJ3ZbUWb07CEH-HPZfZvcGseIUAncF8E5otTOe8IQm2DfKUxA)
✓ Inserted user 2: StringValue(value=KPnT3RK0VTvY26pTJAZrEAYO28UbiKOOVsS9_nMSedAjQnnClwuvwuMCXqdsPfJXI9UZBbZMfAKKXVSnvH3kPg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 27
  Sample: [schema_employees_client_ts, chat_goals__ek0_testing, schema_documents_client_js, chat_agent_configs__ek0_testing, chat_tasks__ek0_testing]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSearch.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Text results: 2; execution time: 0 ms
{"type":"String","value":"Rust Programming Question"}: score=0.9162907318741551, matched=[title]
{"type":"String","value":"Rust Programming Answer"}: score=0.9162907318741551, matched=[title]
Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
Filtered vector results: 2
Custom-weight hybrid results: 3
Raw search: {"results":[{"record":{"id":"DXi57bru5wO5LW9WYewQiun9-ccuEOLlv-dWdVSIzWMDHGXhMy9NVMBchL86a_xp650-8lpin4-yakJbaCwO2w","title":{"value":"Rust Programming Question","type":"String"},"category":{"value":"programming","type":"String"},"query_embedding":{"type":"Vector","value":[1.0,0.0,0.0]},"document_embedding":{"type":"Vector","value":[0.0,1.0,0.0]}},"score":1.0,"matched_fields":[]},{"record":{"title":{"value":"Rust Programming Answer","type":"String"},"query_embedding":{"value":[0.0,1.0,0.0],"type":"Vector"},"document_embedding":{"value":[1.0,0.0,0.0],"type":"Vector"},"id":"vozd1orodN8qez8jhDtrtwyz-iSRN43qP_frWyReptWFYiGDgpbIcKPRdukrjsffpytY7wfgijScs4A2qz3-Lw","category":{"value":"programming","type":"String"}},"score":1.0,"matched_fields":[]}],"total":2,"execution_time_ms":0}

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSimpleCrud.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=OhXkKk7qHiGJmeJ_fcZqB-5z6beyBHrS3ov1UNhLshEQJYr4pKkxcGdR_MU71iJswV-JaJCYDhXj-mZtPFWz5Q)})
  User ID: OhXkKk7qHiGJmeJ_fcZqB-5z6beyBHrS3ov1UNhLshEQJYr4pKkxcGdR_MU71iJswV-JaJCYDhXj-mZtPFWz5Q

=== Read ===
✓ Found user by ID: EkoRecord(fields={categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), id=StringValue(value=OhXkKk7qHiGJmeJ_fcZqB-5z6beyBHrS3ov1UNhLshEQJYr4pKkxcGdR_MU71iJswV-JaJCYDhXj-mZtPFWz5Q), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), created_at=ObjectValue(value={value=StringValue(value=Tue Oct 20 16:14:35 EDT 2026), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Tue Oct 20 16:14:35 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: categories, metadata, data, price, age, embedding, active, id, email, user_id, tags, created_at, name

=== Update ===
✓ Updated user: EkoRecord(fields={user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), id=StringValue(value=OhXkKk7qHiGJmeJ_fcZqB-5z6beyBHrS3ov1UNhLshEQJYr4pKkxcGdR_MU71iJswV-JaJCYDhXj-mZtPFWz5Q), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Tue Oct 20 16:14:35 EDT 2026)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), created_at=ObjectValue(value={value=StringValue(value=Tue Oct 20 16:14:35 EDT 2026), type=StringValue(value=String)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), id=StringValue(value=OhXkKk7qHiGJmeJ_fcZqB-5z6beyBHrS3ov1UNhLshEQJYr4pKkxcGdR_MU71iJswV-JaJCYDhXj-mZtPFWz5Q), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)})})

=== Delete ===
✓ Deleted user with ID: OhXkKk7qHiGJmeJ_fcZqB-5z6beyBHrS3ov1UNhLshEQJYr4pKkxcGdR_MU71iJswV-JaJCYDhXj-mZtPFWz5Q

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSimpleWebsocket.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
  {"data":[{"name":{"type":"String","value":"Test User"},"status":{"value":"active","type":"String"},"id":"P15UPIGwDxVxB-SWIGguUTQ_MKIGeR8VZ9YwvdbY5lLiZZH54414-ZRx_e0Wdxy8DA0NmU9_lKp2Rz7JeM_d5g"}]}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSwrNative.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created native SWR function: github_user_native_kt (7-1AEKj7dFQF-yjI_lms8jiXGGwBQhBrg-0ywh6CM8CeQa4fV8CRDuhYd3LZLyfFdbhRZcRj3uYq0u5EjDYQSA)

First call (cache miss - will fetch from GitHub API):
  Response time: 180ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 60.0x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit_kt (mROakmssOUK5dvPwtmCcggJiCXHKhxcV3ntR_Qpnzt1z9KHdQKdo5swd614j7zyaDFyUwp6DU_MpolqE9lXaTw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_kt' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_kt (sRgcuMTeO08jKtviijSCUZClGCeJU4JpHIKb0L6fsZfbm-S-dyjyjFQLz1jgEHPN0KKceBJpb4N2q-xm7hKbtw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_kt' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache_kt (BPjKyZIUKOleba3ulrPDBwDpiBwZMgsVWp9dbnWJn7gObbYMasDTv0sHiQ9JZpXDaZDt5eCBHJ5zYHbaac4hHw)
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

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSwrPattern.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Created SWR function: swr_cache_lookup_kt_1792527306645 (rJLj-gvI7LVxhAoCLwCxMYizk2z9byxlUFG0iQWK1NpPZFetYHQXZIb30jHM2m2YWNdFn60hQso50DyzswO31g)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 9ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete


BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientTransactions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created Alice: $1000 - ID: nAiKAKym_IqCu_eW9zHxumGmr5i38VOR_RDnMrZ9t72pj6peimD-xULE1xx_XU7CxA969ueB0ePWHiVjZqKvAg
Created Bob: $500 - ID: KaX2pzqRgsyLLdfePw6Y-XL9hl0N7jX5I4oh3Z6TNvEwe_C8JvG1pmuaoaYQj-9w1gEt79drG8p2IK6de-2d6Q

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 1944d4e3-5714-4206-93f0-e0273fe271b8

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
New transaction: 63b0e99b-9baf-4dc0-9ce9-0ac932cf75e7
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientUserFunctions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created user function with ID: 4IoHqPIHej_4X0SSIcbWplKQzJ5h_kRlTPdQX0yMCq3cNtMjb_ngCD5K2M5S6zWLxm7ym93OrSs35sPvttNv_A

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 11 user functions:
  - "conc_demo_rl_skip_ts_12240_1792526703280": "Rate-limit (skip mode)"
  - "get_active_users_kt": "Get Active Users (Kotlin)"
  - "conc_demo_rl_fail_ts_12240_1792526703280": "Rate-limit (fail mode)"
  - "get_active_users_client_js": "Get Active Users (Updated)"
  - "conc_demo_rl_fail_ts_70491_1792526834418": "Rate-limit (fail mode)"

=== List User Functions by Tag ===
Found 1 user functions with 'kotlin' tag:
  - "get_active_users_kt"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketChatStream.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

Created chat session: oTycZH3_E2tFTvgfo8GIoYcQ_ayRSgjaN3IvD5PKsRfy1tnESrxPwvHo1x43tRBZU_bUveXB3yG5TW7Mfw5vIQ

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: 462LGn6Gtmh2I8SvQKpAiG3rRw4s8FYHP3ter52LrYQ0Zh6sTJJldIpVSFu8DSIMNrRSqjB_ALbsMBjA6nK8Jw
Execution time: 739ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketSubscribe.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Subscribed (subscription_id: sub_ccf9e5b2e4314242ad52ca88e1a23ad3)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "BR4PlFlEtsffIsHFlV26NyZpLWnulBsG8fOC8WSxGNFvOXsk_eA3wUE0dLaYm6omS7tX7dFYrBpse3WHYhhxyw"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["BR4PlFlEtsffIsHFlV26NyZpLWnulBsG8fOC8WSxGNFvOXsk_eA3wUE0dLaYm6omS7tX7dFYrBpse3WHYhhxyw"]
     Timestamp:  "2026-10-20T20:15:47.135964+00:00"

Inserting record 2...
✓ Inserted: "Lc9sMJ3u48KOfWDPChTh_FWQAdM4CHMWoaWwQmiW4mwtaYwQIolyNgGn94_3zfPsVDdxh6ysWNUbXLwt19SkSA"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["Lc9sMJ3u48KOfWDPChTh_FWQAdM4CHMWoaWwQmiW4mwtaYwQIolyNgGn94_3zfPsVDdxh6ysWNUbXLwt19SkSA"]

=== Unsubscribing ===
✓ Unsubscribed: {"payload":{"data":{"collection":"ws_subscribe_example_kt","found":true,"unsubscribed":true}},"type":"Success"}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_kt'

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketTtl.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
✓ Inserted document with TTL: StringValue(value=dKdoqyMs9QGNS8aHoZIyMiYsuvcMiw0HGLIH4tERbDldrxZA4uAJbZiGOVjbLSNhnVWXizYPuYK8rsyIk_deSw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"created_at":{"type":"Integer","value":1792527356273},"value":{"type":"Integer","value":42},"id":"dKdoqyMs9QGNS8aHoZIyMiYsuvcMiw0HGLIH4tERbDldrxZA4uAJbZiGOVjbLSNhnVWXizYPuYK8rsyIk_deSw","name":{"value":"WebSocket TTL Test","type":"String"},"ttl":"2026-10-20T21:15:56.398292Z"}]}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: BypassRippleExample.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=gpzBwaCXqUtTuB0Hsq9AK4pheREcrOe0F-s9o3XZmqbsVl3M5ewDqwcK-HyMbQipfwwA2qVEI40l9sjq2ncOnA)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=DiGo0vfYAZDlkfjLgTmV16r1TfoCz9dER0Y5w0QJy68tj3v3pH_XAhYWxvZetUs0b2lUHrEjIEMVT6q4yUie-w)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)}), name=ObjectValue(value={value=StringValue(value=Product 1), type=StringValue(value=String)}), id=StringValue(value=gpzBwaCXqUtTuB0Hsq9AK4pheREcrOe0F-s9o3XZmqbsVl3M5ewDqwcK-HyMbQipfwwA2qVEI40l9sjq2ncOnA)})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), id=StringValue(value=gpzBwaCXqUtTuB0Hsq9AK4pheREcrOe0F-s9o3XZmqbsVl3M5ewDqwcK-HyMbQipfwwA2qVEI40l9sjq2ncOnA), name=ObjectValue(value={value=StringValue(value=Upsert Product), type=StringValue(value=String)})})

✅ All bypass_ripple operations completed successfully!

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
