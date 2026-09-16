make test-examples-typescript
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.26.4 prepare
> npm run build


> @ekodb/ekodb-client@0.26.4 build
> tsc


up to date, audited 46 packages in 576ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.26.4 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 13 packages in 390ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
bash: warning: setlocale: LC_ALL: cannot change locale (C.UTF-8): No such file or directory
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: YTKswpeH0xQcnZMsWXaBICQf--0Up8AXP3tvjt6Fw0VJCYF7V5ZFwCbhWITpqVFKjgtFPKHeHvi5iQ_ch-sB2w

--- updateWithAction: increment ---
Score after increment: 150

--- updateWithAction: push ---
Tags after push: ["beginner","pro"]

--- updateWithAction: clear ---
temp_data after clear: null

--- updateWithActionSequence: multiple atomic actions ---
After sequence - score: 160, lives: 2, tags: ["beginner","pro","veteran"]

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: gF6YiDd7ia8MzrNsNkVsHqxzxqJekYhUDyBh0VPIeKUY3R4gEHcV4nBxKUQM965t6dThIQa0pzuW8YiTKl4Vpg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you have any further questions or need more information, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: T_NfF1iplssUw35I_WE9NWEexhij3Ibw0AlQsWaJ8ZaFnyxBjbRRxcWs2zG9IxvrsdJbbYERP34L1343LkyIYw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: T_NfF1iplssUw35I_WE9NWEexhij3Ibw0AlQsWaJ8ZaFnyxBjbRRxcWs2zG9IxvrsdJbbYERP34L1343LkyIYw
✓ Deleted chat session: gF6YiDd7ia8MzrNsNkVsHqxzxqJekYhUDyBh0VPIeKUY3R4gEHcV4nBxKUQM965t6dThIQa0pzuW8YiTKl4Vpg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: L5AA1VCt9I1tJU3s5nWJLq3Y_whSqweKWv3rbvo_1vWOWt7CidfyYXWnhEJcHBOFekoze2r2itAA48KNXLPoCA

=== Sending Chat Message ===
Message ID: PhdxgKf1JE0PaCfqrJl_QSsSX1ksZwQ8JaUkDWD7i7qsXWoz9WDMI5SilwXrwVj71gOlIWtDiMXqFt-nT8U6IQ

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'CtaTJoqvQ3TyVfcIGDiJZ_E4teTAcKJnmmjcPm8rttxqdpNrMuQeGN3R0slYgOGBkwAEF18qFjvquRn1EdfuVg',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'cMcOp6cansv6llaoPH4gm2mVxk3FJmxEXN1MU0IfX5pcw4qg_MT_WN3oZwXJHB8Qweok0ShxywiNj7E_sqUfBg',
    price: 499,
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'gRANOxcM_j0n1kErwj2-y9j8wpwyPqbYe5ZrdiJPDyfHuNfgQVurtjGVPI-HyrIe91t-s52DYTedmFx4f2hV5A',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 6509ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: YsEi5x23cRPYqrrDlR6osum7XCXbYsR2W7Yo6LfbdoAgLePGGzuah0jd_qHNGuLhoK0PTWDcw-ipAHq7fm61ig

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** (Environmental Knowledge Organization DataBase) is a term that may refer to different resources or databases that catalog information related to ecosystems, environmental knowledge, or eco-organisms, depending on the context. However, as of June 2024, the most widely referenced **ekoDB** is an online database and resource platform specializing in the study of non-coding RNAs (ncRNAs) associated with bacterial and archaeal hosts, especially those involved in defense mechanisms such as CRISPR-Cas systems.

### ekoDB in the Context of Prokaryotic RNA Defense Systems
- **ekoDB** stands for **"endogenous prokaryotic RNA defense sYstem DataBase."**
- It was designed to help researchers explore, analyze, and compare **prokaryotic Argonaute proteins** and other defense-related non-coding RNAs.
- The database provides curated information on sequences, structures, functions, and evolutionary relationships among various non-coding RNAs involved in prokaryotic immune systems.
- Resources include:
  - Genome annotations
  - Experimental data
  - Bioinformatic analysis tools
  - Cross-references to related databases (such as CRISPRdb, anti-CRISPRdb, and others)

### Possible Broader Uses
- In some contexts, "ekoDB" might generally refer to databases cataloging ecological or environmental data, but this is less common and not standardized. If your context is ecology, please clarify for a more tailored answer.

### Key References
- **ekoDB:** [ekoDB Database—RNA Defense System in Prokaryotes](https://ekodb.genomics.cn/) (frequently cited in bioinformatics and microbiome research literature)
- Example publication: **Zhang J, et al. (2021). ekoDB: A comprehensive database of prokaryotic RNA defense systems. Nucleic Acids Research.**

---

**In summary:**
**ekoDB** is primarily known as a bioinformatics database for studying RNA-based defense systems in prokaryotes, especially those involving Argonaute and CRISPR-Cas systems, though the term could sometimes refer to other environmental or knowledge-organization databases depending on the context. If you meant a different field or a specific application, please specify for a more precise explanation!

--- Stream complete ---
Message ID: WEMudlpJWmx3DwX2k6x8O68qQRhYz2WI3leEJDyT3VPH4D8TH_9SAVbxRtGVgLlG51EQjFEZikEoVH1DAfGUNQ
Execution time: 7248ms
Context window: 1000000 tokens

✓ Chat message stream example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

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

=== Get Specific Provider Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper, gpt-5.6-sol, gpt-5.6-terra, gpt-5.6-luna, gpt-realtime-2.1, gpt-realtime-2.1-mini, gpt-transcribe, gpt-live-transcribe, gpt-6-astra, gpt-image-2.5-flare, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.5-sunburst-2026-09-08, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: EnuGFcCEp5WX4Zp0-1-NWNuz9wbFdMH1BqUI8LfoBMHcPrX4aOwfU7xQ0UYW22OxxmfD7BFaXWJhp-suER_maA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the database context, the available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or specifics about this product, feel free to ask!

✓ Message 2 sent
  Response: The price of the product **ekoDB** is **$99**. If you have any other questions or need further details, let me know!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1XFz__JkGzeOfEFIHxh6CUmX4ZkOntrR63XnaHNRsfShUVRAEJQ2uAHzygZpM5nHkeYI2j0jZlLJnDaRopwuhQ
  Parent: EnuGFcCEp5WX4Zp0-1-NWNuz9wbFdMH1BqUI8LfoBMHcPrX4aOwfU7xQ0UYW22OxxmfD7BFaXWJhp-suER_maA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 1XFz__JkGzeOfEFIHxh6CUmX4ZkOntrR63XnaHNRsfShUVRAEJQ2uAHzygZpM5nHkeYI2j0jZlLJnDaRopwuhQ (Untitled)
  Session 2: EnuGFcCEp5WX4Zp0-1-NWNuz9wbFdMH1BqUI8LfoBMHcPrX4aOwfU7xQ0UYW22OxxmfD7BFaXWJhp-suER_maA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 1XFz__JkGzeOfEFIHxh6CUmX4ZkOntrR63XnaHNRsfShUVRAEJQ2uAHzygZpM5nHkeYI2j0jZlLJnDaRopwuhQ

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: sNpbVQkDtJe4rFxEZ-CUPyPUkyty4Z3-2mnaq2kOHozzjwbeL2zpD-_ytKc9VZyNm5VciZh6y2joS0rYYjBNfQ

=== List Collections ===
Total collections: 21
Sample collections: chat_agent_configs__ek0_testing,agent_function_versions__ek0_testing,schedules__ek0_testing,schema_employees_client_go,schema_products_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_ts' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_ts' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_ts': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_ts'

✓ Collection Utilities example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay_ts_43664_1789567012579 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_43664_1789567012579 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_43664_1789567012579 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_43664_1789567012579 { "resource": "queue:drain" }

✓ Cleaned up demo functions
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: '4pUsgk8BBBUWa6m_STPjcuuMEbZpeBkgEC5rDRoFdPwR0D8N08n4Sn6LSf6uHdpPDsQV6b5SssV6gj-Clv-ViA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: '4pUsgk8BBBUWa6m_STPjcuuMEbZpeBkgEC5rDRoFdPwR0D8N08n4Sn6LSf6uHdpPDsQV6b5SssV6gj-Clv-ViA',
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Alice Johnson' }
}
✓ Second upsert (insert): {
  id: 'uLdv1LQyQmeg-P5ACj44wbO7IDpFP9I5Sk12EBA0dMI7r0mATdiS-ZOWsymA6DyXTv62qDVlf5bfacckf94_VA'
}

=== Find One Operation ===
✓ Found user by email: {
  name: { type: 'String', value: 'Alice Johnson' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  id: '4pUsgk8BBBUWa6m_STPjcuuMEbZpeBkgEC5rDRoFdPwR0D8N08n4Sn6LSf6uHdpPDsQV6b5SssV6gj-Clv-ViA',
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 }
}
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created
✓ crypto_demo_hmac_ts saved
✓ crypto_demo_aes_ts saved
✓ crypto_demo_uuid_ts saved
✓ crypto_demo_totp_ts saved
✓ crypto_demo_encoding_ts saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_ts { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_ts { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_ts
  POST /api/functions/crypto_demo_totp_ts
  POST /api/functions/crypto_demo_encoding_ts { "title": "Héllo World" }

✓ Cleaned up demo functions
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - books
  - clothing
  - electronics

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - active
  - archived
  - discontinued

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - active
  - discontinued

Cleanup done.
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Pyd9xz2Eu9iHHD9aoIWr1iqFzuCNheix1LA5lA28bxEMFUamJeJUxOGH0tTzjlsr7GuaH86cGBxhNktOea4BKQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: RqIJtIhnBGLutZw60JXgtjBGYR5QwqBgtNn6LkuhywQMLjW2LWsvbe_u6WqDB9k4kwLJblDXYJREkNcYLUxfdQ

=== Query Documents ===
✓ Found 2 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: XocyYnJ25whFdU1a94R4pZnVvlSbw5EKInYHaWUCWDL1ndawB_jjgG8Dwbzw79ZnM3Gd_Y6hNqwenX6XrrfDIQ

Call 1: Cache miss (fetches from API)
Response time: 116ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "id": 1,
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "website": "hildegard.org"
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 9ms (12.9x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "id": 1,
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "website": "hildegard.org"
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
   ⏱️  Duration: 106ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "address": {
        "geo": {
          "lng": "81.1496",
          "lat": "-37.3159"
        },
        "suite": "Apt. 556"...

Second call (cache hit - from cache):
   ⏱️  Duration: 6ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "address": {
        "geo": {
          "lng": "81.1496",
          "lat": "-37.3159"
        },
     ...
   🚀 Cache speedup: 17.7x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
client_function_contract: ok
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: hepoXMT9m0PENfDrq6D2YaurJivbC8dMhClqekabo9-pZDTK1EMkQBCzICLW_jxX1QgbVXyqxIZDJct1wANdlg
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: 1sRyK92uMJgIg5nfHZCKEIKn5-af_5F1dkn8skWvUtZctqqONrXZomYNTfJf3jz6yV8XywMs0E9kTFULSWEfcQ
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: fRc0u3FVNOk_gDADTL_ZTF4UWwPGVNHo3oUEjNUxqJ-SgxQHhYm4NyVrcg8JAZH7dsO7eZYUE0GKPdIRIrxOjg
📊 Statistics: 2 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":50},"status":{"value":"inactive","type":"String"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":60,"type":"Float"},"status":{"value":"active","type":"String"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 11
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB TypeScript Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Furniture"},"avg_price":{"type":"Float","value":365.6666666666667}}
   {"avg_price":{"type":"Float","value":367},"category":{"type":"String","value":"Electronics"},"count":{"value":5,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB TypeScript AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel in high-dimensional data retrieval, enabling fast similarity searches based on vector representations, like embeddings.

2. **Scalability**: Designed to handle large-scale datasets, they efficiently manage and index vast amounts of vector data.

3. **Real-time Querying**: Support for real-time access allows for quick responses to queries, which is crucial for applications like recommendation systems.

4. **Enhanced Machine Learning Integration**: They easily integrate with ML models to store and retrieve embeddings, facilitating features like semantic search.

5. **Support for Complex Data Types**: Capable of handling various types of unstructured data, including images, text, and audio.

6. **Metadata Storage**: Many vector databases allow the storage of associated metadata alongside vectors, providing context to the data retrieved.

7. **Versatile Applications**: Useful in various fields, including natural language processing, image recognition, and anomaly detection.

8. **Flexibility**: Can accommodate various distance metrics (e.g., cosine similarity, Euclidean distance), allowing for tailored search functionality.

These advantages make vector databases a powerful tool for modern data-intensive applications.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: LS8AwvDPDoarY-1wNXS1QFZik_sSrIkiztTWNZDzHWrfh614pf8u5QYZzyJkd_6andMxMT7PZGeYapr6qI-OUA
📊 Found 2 product groups
   {"count":{"value":2,"type":"Integer"},"avg_price":{"type":"Float","value":474},"category":{"value":"Furniture","type":"String"}}
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":575.6666666666666}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Electronics"},"count":{"value":3,"type":"Integer"}}
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: UserFunction with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB TypeScript CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"role":{"value":"user","type":"String"},"avg_score":{"type":"Float","value":70},"count":{"type":"Integer","value":7}}
   {"role":{"type":"String","value":"admin"},"avg_score":{"type":"Float","value":20},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: exVQGRG-8-zXXe6a52fvSaHbhaO7VAZenRXq5ACimpJu41UIna8aZmVJaKpepd4SxeN5oFIfkkCZ068WxQHnFQ
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: EGAING45NQKEEwYEo5BIXkQnOXTwSSeQfCgXUZC1iGZBLc7u8j_TK02QKw3tOQte1vLQWxjdDwWN2nqM9IfYPw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: lRn6fi1YNrGPPuCHPSqOZq64QRIrkddPbb3o-SPgxltesmtAcom9QbO0JV0ls6hZBu8n_NEWusKQoNsCla3wvA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: QeS0eqNfn39ejnbElSf7w3yWhBfoGmSqFamvfieKk3Dz4Yy3N2AceF9ytSKdTCUKPbRWAdvO8J38KSx4JGc-KA
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"type":"Integer","value":3}}
   {"category":{"type":"String","value":"AI"},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: ytkpWUsrmbJzWVFcnphz1UySkvVRWC0LN1mdL_L9cyz_uSEu11CEQ4LY7QWlFCLHRK7g-CpdGV1FND476BbEew)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'ytkpWUsrmbJzWVFcnphz1UySkvVRWC0LN1mdL_L9cyz_uSEu11CEQ4LY7QWlFCLHRK7g-CpdGV1FND476BbEew',
      steps: [Object],
      title: [Object]
    }
  ]
}

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Goals, Tasks & Agents Example (TypeScript) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: HN6d44HfdlxMEb7ZGLowl8DWb0ZQj6tPasBZfODcazMi790fFRi6gp3HwpUQhPTJ-KC_zy22a3yEIsuM2tsFbA)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-09-16T13:57:03.725190+00:00",
      "description": "Ship version 2.0 to production",
      "id": "HN6d44HfdlxMEb7ZGLowl8DWb0ZQj6tPasBZfODcazMi790fFRi6gp3HwpUQhPTJ-KC_zy22a3yEIsuM2tsFbA",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-09-16T13:57:03.725190+00:00"
    }
  ]
}

--- Getting goal ---
Fetched: Deploy v2.0

--- Updating goal ---
Updated description: Ship version 2.0 with hot-fix patches

--- Searching goals ---
Search results: {
  "count": 1,
  "items": [
    {
      "_score": 12.870000000000001,
      "created_at": {
        "type": "DateTime",
        "value": "2026-09-16T13:57:03.725190+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "HN6d44HfdlxMEb7ZGLowl8DWb0ZQj6tPasBZfODcazMi790fFRi6gp3HwpUQhPTJ-KC_zy22a3yEIsuM2tsFbA",
      "status": {
        "type": "String",
        "value": "pending"
      },
      "steps": {
        "type": "String",
        "value": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"
      },
      "title": {
        "type": "String",
        "value": "Deploy v2.0"
      },
      "updated_at": {
        "type": "DateTime",
        "value": "2026-09-16T13:57:03.746062+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal HN6d44HfdlxMEb7ZGLowl8DWb0ZQj6tPasBZfODcazMi790fFRi6gp3HwpUQhPTJ-KC_zy22a3yEIsuM2tsFbA
--- Goal step: complete step 0 ---
Step 0 completed on goal HN6d44HfdlxMEb7ZGLowl8DWb0ZQj6tPasBZfODcazMi790fFRi6gp3HwpUQhPTJ-KC_zy22a3yEIsuM2tsFbA
--- Goal step: fail step 1 ---
Step 1 failed on goal HN6d44HfdlxMEb7ZGLowl8DWb0ZQj6tPasBZfODcazMi790fFRi6gp3HwpUQhPTJ-KC_zy22a3yEIsuM2tsFbA

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: JCkZP3yeHoq8JtDgfwGNUuryq-5Dzz85SpwkciXEpJotYx5-VikImfIlH59_1N_OKiD_RPZiMNk_QgAiD6toqQ)

--- Listing tasks ---
Tasks: {
  "count": 1,
  "items": [
    {
      "action": {
        "type": "String",
        "value": "health_check"
      },
      "config": {
        "type": "Object",
        "value": {
          "endpoint": "/health"
        }
      },
      "cron": {
        "type": "String",
        "value": "0 * * * *"
      },
      "id": "JCkZP3yeHoq8JtDgfwGNUuryq-5Dzz85SpwkciXEpJotYx5-VikImfIlH59_1N_OKiD_RPZiMNk_QgAiD6toqQ",
      "name": {
        "type": "String",
        "value": "Hourly Health Check"
      }
    }
  ]
}

--- Getting task ---
Fetched: Hourly Health Check

--- Starting task ---
Task status: running

--- Succeeding task ---
Task status after succeed: active

--- Pausing task ---
Task status after pause: paused

--- Resuming task ---
Task status after resume: active

--- Failing task ---
Task status after fail: active

--- Getting due tasks ---
Due tasks: {
  "count": 0,
  "items": []
}

--- Deleting task ---
Task deleted successfully


--- Creating agent ---
Created agent: SupportBot (id: 9n0Tg_h7UolilyOfJSMYsi13sf89X3l9NuZu7YX6E9mHUQEFBIfQOKXznUeCpNG3gGST8W3dmze5NZx69fWARg)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "9n0Tg_h7UolilyOfJSMYsi13sf89X3l9NuZu7YX6E9mHUQEFBIfQOKXznUeCpNG3gGST8W3dmze5NZx69fWARg",
      "llm_model": {
        "type": "String",
        "value": "gpt-4"
      },
      "name": {
        "type": "String",
        "value": "SupportBot"
      },
      "system_prompt": {
        "type": "String",
        "value": "You are a helpful customer support agent."
      }
    }
  ]
}

--- Getting agent by ID ---
Fetched: SupportBot

--- Getting agent by name ---
By name: SupportBot (id: 9n0Tg_h7UolilyOfJSMYsi13sf89X3l9NuZu7YX6E9mHUQEFBIfQOKXznUeCpNG3gGST8W3dmze5NZx69fWARg)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent 9n0Tg_h7UolilyOfJSMYsi13sf89X3l9NuZu7YX6E9mHUQEFBIfQOKXznUeCpNG3gGST8W3dmze5NZx69fWARg; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted successfully

--- Cleanup: deleting goals ---
Goals deleted successfully

=== All goals, tasks & agents operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data:
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

4. Join orders with user data:
Found 2 completed orders:
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B


✅ Join operations examples completed!
=== Cleanup ===
✅ Deleted test collections
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/jwt_register_ts_44229_1789567025055 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_44229_1789567025055 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_44229_1789567025055 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: yd6IIWRlX2kl3zX5pPTLxLvb3cYnQarkqZre4qsJ0_H8qXKACr76AnSvSwNlsroQ5bxui6tCg0aMENKH4U-n9A
Inserted document 2: 57ciVJwpuOb_vhrWYNBdxcWkdUQIyce2Qmwm-z-rR-SjH8pfalkpgIDmMh3VYKzihGGoqFkkOtStDUX01LwhBA

--- Linking documents to KV key ---
Linked doc yd6IIWRlX2kl3zX5pPTLxLvb3cYnQarkqZre4qsJ0_H8qXKACr76AnSvSwNlsroQ5bxui6tCg0aMENKH4U-n9A: null
Linked doc 57ciVJwpuOb_vhrWYNBdxcWkdUQIyce2Qmwm-z-rR-SjH8pfalkpgIDmMh3VYKzihGGoqFkkOtStDUX01LwhBA: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_44269_1789567025406",
    "document_id": "57ciVJwpuOb_vhrWYNBdxcWkdUQIyce2Qmwm-z-rR-SjH8pfalkpgIDmMh3VYKzihGGoqFkkOtStDUX01LwhBA",
    "field_path": null,
    "created_at": "2026-09-16T13:57:05.497356Z",
    "last_accessed": "2026-09-16T13:57:05.499629Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_44269_1789567025406",
    "document_id": "yd6IIWRlX2kl3zX5pPTLxLvb3cYnQarkqZre4qsJ0_H8qXKACr76AnSvSwNlsroQ5bxui6tCg0aMENKH4U-n9A",
    "field_path": null,
    "created_at": "2026-09-16T13:57:05.494021Z",
    "last_accessed": "2026-09-16T13:57:05.499629Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc 57ciVJwpuOb_vhrWYNBdxcWkdUQIyce2Qmwm-z-rR-SjH8pfalkpgIDmMh3VYKzihGGoqFkkOtStDUX01LwhBA: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_44269_1789567025406",
    "document_id": "yd6IIWRlX2kl3zX5pPTLxLvb3cYnQarkqZre4qsJ0_H8qXKACr76AnSvSwNlsroQ5bxui6tCg0aMENKH4U-n9A",
    "field_path": null,
    "created_at": "2026-09-16T13:57:05.494021Z",
    "last_accessed": "2026-09-16T13:57:05.502709Z",
    "metadata": {}
  }
]

=== All KV linking operations completed ===

--- Cleanup ---
Cleanup complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { type: 'Object', value: { username: 'john_doe', userId: 123 } }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_44298_1789567025865:cache:product:1: success
  kv_ops_ts_44298_1789567025865:cache:product:2: success
  kv_ops_ts_44298_1789567025865:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_44298_1789567025865:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_44298_1789567025865:cache:product:2: { price: 39.99, name: 'Product 2' }
  kv_ops_ts_44298_1789567025865:cache:product:3: { name: 'Product 3', price: 49.99 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_ops_ts_44298_1789567025865:cache:product:1: deleted
  kv_ops_ts_44298_1789567025865:cache:product:2: deleted
  kv_ops_ts_44298_1789567025865:cache:product:3: deleted

✓ All KV operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== KV Precision: Float vs Decimal ===

=== Test 1: Using JavaScript Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using Field.decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===

=== Summary ===
✅ Use Field.decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ Field.decimal() stores values as strings internally,
   preserving exact precision across all operations.

=== Cleanup ===
✓ Cleaned up test keys
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created
✓ ts_route_admin → GET /api/route/users/admin
✓ ts_route_user_by_id → GET /api/route/users/:id
✓ ts_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ ts_route_org_create_member → POST /api/route/orgs/:org/members

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Query Builder Examples ===

Setting up test data...
✅ Test data created

1. Simple equality query:
Found 2 active users

2. Range query with sorting:
Found 3 users aged 18-65

3. String operations:
Found 2 users with @example.com emails

4. IN operator:
Found 2 privileged users

5. Complex query with multiple conditions:
Found 1 active US users over 21

6. Pagination:
Page 1: 2 users

7. NOT IN operator:
Found 3 valid users

8. Using bypass flags:
Found 2 users (bypassed cache)

=== Cleanup ===
✅ Deleted test collection

✅ Query Builder examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: b4c3c035-310d-4b3e-9a23-0deec07b44b1, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-09-16T13:57:10.081119Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_44451_1789567030044",
      "id": "b4c3c035-310d-4b3e-9a23-0deec07b44b1",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-09-17T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-09-16T13:57:10.081119Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "b4c3c035-310d-4b3e-9a23-0deec07b44b1",
  "status": "triggered"
}

--- Pausing schedule ---
Schedule enabled after pause: false

--- Resuming schedule ---
Schedule enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Schema Management Examples ===

1. Creating user schema with basic fields:
✅ User schema created

2. Creating product schema with text index:
✅ Product schema with indexes created

3. Creating document schema with vector index:
✅ Document schema with vector index created

4. Retrieving collection schema:
Schema fields: [ 'age', 'email', 'name', 'status' ]
Schema version: 1

5. Retrieving collection metadata:
Collection has 4 fields

6. Creating employee schema with all constraint types:
✅ Employee schema with all constraints created

✅ Schema management examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.773, Matched:
  2. Score: 0.743, Matched:
  3. Score: 0.730, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.509, Matched: title, content
  2. Score: 0.897, Matched: content, title
  3. Score: 0.292, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)


✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '6uPvvKVzFjh_rV8M4z8nSfJujtBcV0jkKBuku-mrViwy3E_ix8siVZ_ySTLX9kg6rLJCGkMwgpta-ZcBalDSig'
}

=== Find by ID ===
Found: {
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  name: { type: 'String', value: 'Test Record' },
  price: { value: 99.99, type: 'Float' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 42 },
  created_at: { value: '2026-09-16T13:57:11.521+00:00', type: 'DateTime' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  id: '6uPvvKVzFjh_rV8M4z8nSfJujtBcV0jkKBuku-mrViwy3E_ix8siVZ_ySTLX9kg6rLJCGkMwgpta-ZcBalDSig'
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-09-16T13:57:11.521Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'Test Record',
  price: 99.99,
  metadata: { key: 'value', nested: { deep: true } },
  active: true,
  value: 42,
  created_at: '2026-09-16T13:57:11.521+00:00',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ=',
  id: '6uPvvKVzFjh_rV8M4z8nSfJujtBcV0jkKBuku-mrViwy3E_ix8siVZ_ySTLX9kg6rLJCGkMwgpta-ZcBalDSig'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  id: '6uPvvKVzFjh_rV8M4z8nSfJujtBcV0jkKBuku-mrViwy3E_ix8siVZ_ySTLX9kg6rLJCGkMwgpta-ZcBalDSig',
  active: { value: true, type: 'Boolean' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  name: { value: 'Updated Record', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  price: { type: 'Float', value: 99.99 },
  created_at: { type: 'DateTime', value: '2026-09-16T13:57:11.521+00:00' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 1e3c0qoAn3EcbJ0T-vP_sk1akMc5ZCksB5OTpdyBFlKzdRedIyl_ZfZMpS6l2gszuvSzJfZ5m_YqsDuaKcH8kQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB TypeScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test scripts and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_ts (rF7Co0dMr7y5ZPXvkAWjgd9-5g2ZnbjczF3rXpzckpfojICxG-F6KjX79yVrPZkfisf2j4qzDbghL3aS4dRCVQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 174ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 34.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (7vHIO0IpfPdBYxdheHiYgfo6Dchr6KA2F2-oMFVV12MgkXymvbLU1F_DzYg5Ziit8oukluO-nJtqEQGfjBYhjQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (YkpH-3izGy9ESfBr5tcZik5FmMZGcm2S7JyVbEiD6RGWQF2pO1FajLgBtJams63VHl89GspGYomlsDg8MIJ_Vw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (ZjZwehFwX-NcY4B5PNNs-1djiXI77f_Hv7hgpgTtZ4jWwno58qv-lBKfGCZxAbMUBYAhUL5UF3YDEASB3RCHEQ)
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
✓ Deleted 4 test scripts and owned SWR resources

✅ All examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_44654_1789567033797 (C4aA0eLf1hBeFUm0J_ERRpbszRx-G9G_NCan1KV-iQ1aPor2-52ZQECBM4pVvgwKCZbt1PUg8Y4QmiWtmOrgKA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "username": "Bret",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "id": 1,
          "address": {
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "city": "Gwenborough",
            "street": "Kulas Light"
          }
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 8ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "username": "Bret",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "id": 1,
          "address": {
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "city": "Gwenborough",
            "street": "Kulas Light"
          }
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_44654_1789567033797 (9GM0HCuwR1ivD2uQ9eMyydEJbYUgO_522x5CaIHNLMOMxkGbfFSfOhOXCH9rTRHv2dx5eThSftw4bLdM4EXi-w)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "category": "beauty",
          "stock": 99,
          "reviews": [
            {
              "rating": 3,
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            },
            {
              "reviewerName": "Lucas Gordon",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "rating": 4
            },
            {
              "reviewerName": "Eleanor Collins",
              "comment": "Highly impressed!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5
            }
          ],
          "price": 9.99,
          "sku": "BEA-ESS-ESS-001",
          "dimensions": {
            "depth": 22.99,
            "height": 13.08,
            "width": 15.14
          },
          "shippingInformation": "Ships in 3-5 business days",
          "id": 1,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "warrantyInformation": "1 week warranty",
          "minimumOrderQuantity": 48,
          "brand": "Essence",
          "availabilityStatus": "In Stock",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "discountPercentage": 10.48,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "rating": 2.56,
          "meta": {
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "createdAt": "2025-10-09T14:47:01.588Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "returnPolicy": "No return policy",
          "tags": [
            "beauty",
            "mascara"
          ],
          "title": "Essence Mascara Lash Princess",
          "weight": 4
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Multi-API data fetched, merged, and cached atomically

=== Why This Is Powerful ===

✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===

1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: l0t13w8Bc4PT2zMBHo1AFUumOu1sFUwAomyDicvNbxA9FkYFKKEPIfs2US_zth50yad4bHn3c4nXOHLizu59Tg
Created Bob: $500 - ID: n6a6xjh10AiKr1vibZGSzcaMUG0et7s9t1Vokkylye8zV9JTtpmlw63oYVSnWaiSGFKSydowehMeNvSkNEmQ1A

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 703dded5-cf26-4f0c-bfee-a1a636e21028

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
New transaction: becf42c6-cde9-4439-bfc3-f07b19ca41d9
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Create User Function ===
Created user function with ID: t5Cs3dhQH9reoGHcdmUSctND3NldctK0NB7ztdKpDkQPiF9CX3FbIKAGQrpyNvP3kL9Z-7oNz6cCf6wgY74Qrg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 11 user functions:
  - conc_demo_rl_fail_ts_30955_1789566761407: Rate-limit (fail mode)
  - conc_demo_rl_skip_ts_43664_1789567012579: Rate-limit (skip mode)
  - conc_demo_rl_skip_ts_30955_1789566761407: Rate-limit (skip mode)
  - conc_demo_rl_fail_ts_19088_1789566533301: Rate-limit (fail mode)
  - conc_demo_rl_fail_ts_43664_1789567012579: Rate-limit (fail mode)
  - get_active_users_ts: Get Active Users
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - fetch_product_reviews_ts_32495_1789566782714: Fetch Product with Reviews (Multi-API)
  - fetch_product_reviews_ts_44654_1789567033797: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_skip_ts_19088_1789566533301: Rate-limit (skip mode)
  - avg_score_by_role_ts_19467_1789566544643: Average Score by Role

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: 55eVXkoOfGX8HW1fCOZbqEC4HP7MHmAuTTT2zNEcwkn8sg1uIjD-QLc7FROp8RrcWAy7Fj4sRuVx83HUw4OFcw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: oeTGyOd-IEst_e64cXenHWlW2apIKAcb5ntf1VQIQghxvDpfwI7WdTDWqypW-oageUm7Uhm6QqvdPLISN0Bd2g
Execution time: 513ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_8014a7a55c2446a796e88a5008925e01)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: 4xm8tj3LhYAYPvY4Ad5d8AT95dz3NFkPbq97CFPUgtECjAmBcJ30bqqbKSG4k1gZXONXo3FHznmMdjaXOp6OFQ
  📡 Notification received for 4xm8tj3LhYAYPvY4Ad5d8AT95dz3NFkPbq97CFPUgtECjAmBcJ30bqqbKSG4k1gZXONXo3FHznmMdjaXOp6OFQ

Inserting another record...
✓ Inserted record: U6uw4WnOPK2IV9a57c-Vd5nwhFJ9HHGIoqFALwJF6mSMakUFQVgEicxvbUss-eD8XB5T6TNHiMRaXzo8E-2pyw
  📡 Notification received for U6uw4WnOPK2IV9a57c-Vd5nwhFJ9HHGIoqFALwJF6mSMakUFQVgEicxvbUss-eD8XB5T6TNHiMRaXzo8E-2pyw

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: X4Mv0G4FlDcPOX7EizLypZMulLe37eUVo6jeD4ARSpPXT41kboNNNDBb5fQZxr_-GkndC-_jMsnpmLHU8Vzvpg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {"id":"lpwUJJ0Zdyfln0VIf2q-L1LXJe3BF6bwNkiIHNo7l9KEYtghfkkJM94hJaNWH_hY-FrfvqtO6HK275he3rj6NA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"FAnu5BvYFg3qt_c26mQA_M2TbjFxQUjcZ19od3_53zdtW581SKr8d6zRoIifgjwYHyDE3GbdICxJU9AuaqX4Bg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"lpwUJJ0Zdyfln0VIf2q-L1LXJe3BF6bwNkiIHNo7l9KEYtghfkkJM94hJaNWH_hY-FrfvqtO6HK275he3rj6NA","price":{"value":150,"type":"Integer"},"name":{"type":"String","value":"Product 1"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"oVupjWL0yw8kwtMHQljrEPBdYEtnWlghP_dVa1v_qpveo4wS-tLVIUIeNIjRrJ4bOdeaBE2HGo-Qqmx_kKEVBg"}

✅ All bypass_ripple operations completed successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id","name","email"]
  First user: Alice Johnson <alice@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["created_at","email","user_role","bio","avatar_url","age","id","name","status"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - Dave Brown (age 45)
    - Alice Johnson (age 30)
    - Bob Smith (age 25)

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - Carol White: Manager

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["password","secret_token","age","email","created_at","id","api_key","status","user_role","bio","avatar_url","name"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","id","email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
