make test-examples-typescript
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.27.0 prepare
> npm run build


> @ekodb/ekodb-client@0.27.0 build
> tsc


up to date, audited 44 packages in 993ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.27.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 13 packages in 389ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: NFl83suLfaKGbK3RKTg5jHaNPG74hOfUwE9EpB20memb2olIqZUUQsHuduty-F-0QtdQSmiZ5ynlX9DV9jSzIw

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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 7_gKcQ4EJMvE39N9w3E2YOlzNFE8d6DiSpMADpxzdVEkFgkMSfNtRK_IZK9udiaSl_MukhAzdoxtc_UWtZfsZg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or have other questions, feel free to ask!

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
✓ Created second session: AaVxj4u5JE1ZUObToWxSCupny9qQ9avEZzl9vnVLQJ2RzU9XnyIQ5QZTXDumaBNtOl4aKDpZ4Gn-sRXGZhkDQw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: AaVxj4u5JE1ZUObToWxSCupny9qQ9avEZzl9vnVLQJ2RzU9XnyIQ5QZTXDumaBNtOl4aKDpZ4Gn-sRXGZhkDQw
✓ Deleted chat session: 7_gKcQ4EJMvE39N9w3E2YOlzNFE8d6DiSpMADpxzdVEkFgkMSfNtRK_IZK9udiaSl_MukhAzdoxtc_UWtZfsZg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: UzdiDwJTzFPobT6dyFoqzjk3JrglBx9MsmRMUiT0MMCCaeYBfBwLux7ecuLlobgNRq6z192_6lsC8xyj_XxHqQ

=== Sending Chat Message ===
Message ID: Xi__genEL2tIqfiA_27APLwJSJyellhm1Nor8zzx0Z9LaDg_I80XaS0Sf5jG7hYQnbA2VeNM7P6gIf39HB3lzw

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    id: '9JqNyu2ybyTcxmi2hRdrsPAeMXJTsaUti5yr2nP06QCbqrVEqrskuGbcGDrJ3x13PCqswIttrOOAo-UP4g4NaQ',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'WEJug2LXsn-ayd35HE8mMsjPqFIVHZIMa4RWxOlSQNn_V3_E518LqwbGPoLBowEoy52poBG0H-bkpRHLWttCiQ',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499,
    id: 'oxy95jXdM-QSomKza0yoxYCfL7UGjMkyfuUA_DbtjwEjrVUw-3e9MitqditC9xSZnFY4r16EwoSfQHfE_0g2kw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 7067ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: j1BBc1B-D79SHw60VcOtpHGqh-vz9ydSdgWF_vTwHHs4xxFx9eqg4548cNgW9Civ9Q9OAgEEyyJpwEgljDMCHQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a curation-based database that focuses on the **ecological and evolutionary information of prokaryotic taxa** (mainly bacteria and archaea). The name "ekoDB" comes from "ecology" and "database."

### Key Features of ekoDB:
- **Curation-Based:** Information in ekoDB is hand-curated from scientific literature, ensuring high quality and reliability, rather than relying solely on automated computational predictions.
- **Taxonomy:** Organizes data according to prokaryotic taxonomy.
- **Ecological/Evolutionary Data:** Contains detailed information on lifestyles (e.g., symbiosis, pathogenesis, extremophily), habitats (e.g., marine, soil, host-associated), and evolutionary traits of prokaryotes.
- **User-Friendly Search:** Allows users to search, browse, and analyze ecological and evolutionary attributes of microbes at different taxonomic levels (species, genus, etc.).
- **Integration:** Complements genomic databases by providing context on ecological functions and evolutionary history.

### Applications:
- Understanding the adaptation and diversity of prokaryotes.
- Comparing the ecology of microbial taxa.
- Assisting in metagenomic studies and microbial ecology research.

### Reference:
For example, see the original publication:
- Yamashita, S., et al. (2023) "**ekoDB: a curation-based database of ecological and evolutionary information of prokaryotic taxa**." *Nucleic Acids Research*, 51(D1), D808–D815. [doi:10.1093/nar/gkac979](https://academic.oup.com/nar/article/51/D1/D808/6849227)

**In summary:**
ekoDB is a specialized, literature-curated database providing ecological and evolutionary information about prokaryotic organisms, designed to help researchers explore how these microbes live, adapt, and interact with their environments.

--- Stream complete ---
Message ID: iX0D_TTqMwn7hsTxdlTFBpiEXoC769kSXBVuac_fV-OtmKbbSF05151CJX_w9f8HkcJgW5QJcwyWmNqfaBWzsg
Execution time: 3383ms
Context window: 1000000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - o3-2025-04-16
    - gpt-5.1-chat-latest
    - gpt-5.3-chat-latest
    - gpt-5.2-2025-12-11
    - sora-2
    - chatgpt-image-latest
    - gpt-4.1-mini
    - gpt-3.5-turbo
    - gpt-4o-mini
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-6-luna
    - gpt-5.4-mini-2026-03-17
    - gpt-4-turbo
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-5.4-pro
    - gpt-realtime-mini-2025-12-15
    - gpt-3.5-turbo-16k
    - babbage-002
    - text-embedding-ada-002
    - gpt-4o-mini-tts
    - gpt-4o-2024-08-06
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - omni-moderation-latest
    - gpt-realtime
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-4o
    - o1-2024-12-17
    - gpt-5.2-chat-latest
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - davinci-002
    - gpt-image-2-2026-04-21
    - gpt-4o-mini-transcribe
    - o3-mini
    - gpt-audio
    - gpt-5.4-nano
    - o3
    - gpt-5.1-codex-max
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-mini-search-preview-2025-03-11
    - text-embedding-3-large
    - gpt-4o-mini-search-preview
    - gpt-4o-2024-05-13
    - gpt-6-sol
    - gpt-5.3-codex
    - gpt-3.5-turbo-instruct
    - sora-2-pro
    - gpt-5
    - gpt-audio-mini-2025-12-15
    - gpt-transcribe
    - o4-mini-deep-research-2025-06-26
    - o1-pro-2025-03-19
    - gpt-4.1-nano-2025-04-14
    - gpt-5-search-api-2025-10-14
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - tts-1
    - gpt-5-mini
    - omni-moderation-2024-09-26
    - gpt-5-mini-2025-08-07
    - gpt-realtime-2025-08-28
    - gpt-5.4-2026-03-05
    - gpt-live-transcribe
    - gpt-3.5-turbo-0125
    - gpt-5-nano
    - gpt-5.6-luna
    - gpt-4
    - gpt-image-2.5-sunburst-2026-09-08
    - whisper-1
    - gpt-5.4-mini
    - gpt-5-pro
    - o1-pro
    - gpt-realtime-whisper
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-live-1
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-2025-08-07
    - tts-1-1106
    - gpt-5.5-pro
    - gpt-5-pro-2025-10-06
    - chat-latest
    - gpt-5.2-pro-2025-12-11
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-audio-2025-08-28
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-translate
    - gpt-5.6-sol
    - gpt-audio-mini-2025-10-06
    - gpt-5.5-pro-2026-04-23
    - tts-1-hd-1106
    - gpt-realtime-mini
    - gpt-5.4
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-image-1.5
    - gpt-5.1-2025-11-13
    - o4-mini-deep-research
    - gpt-4o-search-preview-2025-03-11
    - o3-mini-2025-01-31
    - gpt-4-0613
    - gpt-4o-search-preview
    - gpt-5.6-terra
    - text-embedding-3-small
    - gpt-audio-1.5
    - gpt-3.5-turbo-instruct-0914
    - gpt-6-astra
    - gpt-5.2-codex
    - gpt-3.5-turbo-1106
    - gpt-5-nano-2025-08-07
    - gpt-audio-mini
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-2025-04-14
    - gpt-4o-mini-2024-07-18
    - o4-mini
    - o1
    - gpt-5.4-nano-2026-03-17
    - gpt-5.1-codex-mini
    - gpt-4o-transcribe
    - gpt-image-2.5-flare
    - gpt-6.1-sol
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
Provider status:
  anthropic: ok 14 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: o3-2025-04-16, gpt-5.1-chat-latest, gpt-5.3-chat-latest, gpt-5.2-2025-12-11, sora-2, chatgpt-image-latest, gpt-4.1-mini, gpt-3.5-turbo, gpt-4o-mini, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6-luna, gpt-5.4-mini-2026-03-17, gpt-4-turbo, gpt-4.1-nano, gpt-image-2, gpt-5.4-pro, gpt-realtime-mini-2025-12-15, gpt-3.5-turbo-16k, babbage-002, text-embedding-ada-002, gpt-4o-mini-tts, gpt-4o-2024-08-06, o4-mini-2025-04-16, gpt-4o-2024-11-20, omni-moderation-latest, gpt-realtime, tts-1-hd, gpt-realtime-1.5, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.1, gpt-5.1-codex, gpt-4o, o1-2024-12-17, gpt-5.2-chat-latest, gpt-4o-mini-tts-2025-12-15, gpt-image-1, davinci-002, gpt-image-2-2026-04-21, gpt-4o-mini-transcribe, o3-mini, gpt-audio, gpt-5.4-nano, o3, gpt-5.1-codex-max, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-mini-search-preview-2025-03-11, text-embedding-3-large, gpt-4o-mini-search-preview, gpt-4o-2024-05-13, gpt-6-sol, gpt-5.3-codex, gpt-3.5-turbo-instruct, sora-2-pro, gpt-5, gpt-audio-mini-2025-12-15, gpt-transcribe, o4-mini-deep-research-2025-06-26, o1-pro-2025-03-19, gpt-4.1-nano-2025-04-14, gpt-5-search-api-2025-10-14, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, tts-1, gpt-5-mini, omni-moderation-2024-09-26, gpt-5-mini-2025-08-07, gpt-realtime-2025-08-28, gpt-5.4-2026-03-05, gpt-live-transcribe, gpt-3.5-turbo-0125, gpt-5-nano, gpt-5.6-luna, gpt-4, gpt-image-2.5-sunburst-2026-09-08, whisper-1, gpt-5.4-mini, gpt-5-pro, o1-pro, gpt-realtime-whisper, gpt-5.2-pro, gpt-5-chat-latest, gpt-live-1, gpt-4o-mini-transcribe-2025-12-15, gpt-5-2025-08-07, tts-1-1106, gpt-5.5-pro, gpt-5-pro-2025-10-06, chat-latest, gpt-5.2-pro-2025-12-11, gpt-4o-mini-transcribe-2025-03-20, gpt-audio-2025-08-28, gpt-5.4-pro-2026-03-05, gpt-realtime-translate, gpt-5.6-sol, gpt-audio-mini-2025-10-06, gpt-5.5-pro-2026-04-23, tts-1-hd-1106, gpt-realtime-mini, gpt-5.4, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-image-1-mini, gpt-image-1.5, gpt-5.1-2025-11-13, o4-mini-deep-research, gpt-4o-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-4-0613, gpt-4o-search-preview, gpt-5.6-terra, text-embedding-3-small, gpt-audio-1.5, gpt-3.5-turbo-instruct-0914, gpt-6-astra, gpt-5.2-codex, gpt-3.5-turbo-1106, gpt-5-nano-2025-08-07, gpt-audio-mini, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-4.1-mini-2025-04-14, gpt-4.1-2025-04-14, gpt-4o-mini-2024-07-18, o4-mini, o1, gpt-5.4-nano-2026-03-17, gpt-5.1-codex-mini, gpt-4o-transcribe, gpt-image-2.5-flare, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: JAW-F2jLawSx5tSDvxVir2hfDVXx9QvELK4MbNumt8rp2O6OGDggXkCgPqzSQpozBZKm1YJkQrQXsuBEFDwd7A

=== Sending Messages ===
✓ Message 1 sent
  Response: It seems that there are no records available in the specified collection. If you're looking for products, particularly the ekoDB product, I can share the details I have:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need further information or specific queries related to available products, feel free to ask!

✓ Message 2 sent
  Response: It seems that I do not have access to the 'products' collection directly. However, based on the context you provided, the price of the product "ekoDB" is **$99**. If you need more information or details on a specific aspect, let me know!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 3KJqn5p1rf2_gr5ttkzd3MKnJgdSNXn3RA5SUTQer-uuHpvNFJe_CIuU5Gx6lPFiSyqujJRJD7AZhsOTurvvLw
  Parent: JAW-F2jLawSx5tSDvxVir2hfDVXx9QvELK4MbNumt8rp2O6OGDggXkCgPqzSQpozBZKm1YJkQrQXsuBEFDwd7A

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 3KJqn5p1rf2_gr5ttkzd3MKnJgdSNXn3RA5SUTQer-uuHpvNFJe_CIuU5Gx6lPFiSyqujJRJD7AZhsOTurvvLw (Untitled)
  Session 2: JAW-F2jLawSx5tSDvxVir2hfDVXx9QvELK4MbNumt8rp2O6OGDggXkCgPqzSQpozBZKm1YJkQrQXsuBEFDwd7A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 3KJqn5p1rf2_gr5ttkzd3MKnJgdSNXn3RA5SUTQer-uuHpvNFJe_CIuU5Gx6lPFiSyqujJRJD7AZhsOTurvvLw

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: RHCX__fDZ6av-aVrKtln80ouevSq7ogm5a_jbcbuipnA3ofjzjvwd-uWZGz2-1ne2qVYjFokpB57nXmpTGkxww

=== List Collections ===
Total collections: 17
Sample collections: chat_agent_configs__ek0_testing,chat_goal_templates__ek0_testing,schema_documents_client_go,audit__ek0_testing,chat_tasks__ek0_testing

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
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
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay_ts_75441_1791438514383 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_75441_1791438514383 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_75441_1791438514383 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_75441_1791438514383 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'h-2Gr1hZtFXlXVAKKWURSd4Bg72FLwSj5nuGE3G_NFLYB1Y_cVlElNRuz1fgsXn6w_EEgufJkPnAvOrb49D0lA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'h-2Gr1hZtFXlXVAKKWURSd4Bg72FLwSj5nuGE3G_NFLYB1Y_cVlElNRuz1fgsXn6w_EEgufJkPnAvOrb49D0lA',
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  active: { value: true, type: 'Boolean' },
  name: { value: 'Alice Johnson', type: 'String' }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  age: { value: 29, type: 'Integer' },
  name: { type: 'String', value: 'Alice Johnson' },
  active: { type: 'Boolean', value: true },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  id: 'h-2Gr1hZtFXlXVAKKWURSd4Bg72FLwSj5nuGE3G_NFLYB1Y_cVlElNRuz1fgsXn6w_EEgufJkPnAvOrb49D0lA'
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
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: BFjgXMfP8NeAlQAxeXSMb4V7Hwx2kWzRcYaEH3f7LckoPzqaR46vjwsdCck6FFTcyJIw8_jinNqP4ecx9CZw7Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 4w-wiliM1NzPuTApGgrbERoyv__L1Nhevx-l1_G9njVud5ua1NkrbiYEThU2mDDYZo8Qzp2phAyUwl8onVgIng

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
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: 7l4POmPBPmoi1fq8S8bMmliIbul7uj773aTJwS7Eh6JFoJ0XRANPlsxyfb9ISkNgXTjDMx1r2ElEySQ-aRihpw

Call 1: Cache miss (fetches from API)
Response time: 62ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "street": "Kulas Light"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "id": 1
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
Response time: 3ms (20.7x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "street": "Kulas Light"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "id": 1
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
   ⏱️  Duration: 51ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "id": 1,
      "username": "Bret",
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "id": 1,
      "username": "Bret",
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "website": "hild...
   🚀 Cache speedup: 17.0x faster!

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
client_function_contract: ok
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: xf0UaBXfXf6V2mViBVRorXRoaJmsN7RvkLSbqt8sThT8VwQZt9KZFCWNuCkBocJ8sxLYzq8NMlFjdzYpcXYhTQ
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: GJ0RT5LIs6t2bPS-Da0uRa_UpeD1us3OH-RPuKzXiglgEGJC-9bRu7Zel73N0XrwFRmZdhVzc-mve_kGB_0e4A
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: 6fnvUpg-D7VATB4veNqq99IVAU4PYP6QFETPaLxou2rZNaFxtN55Zy6O2GrGUnKkL_vwEYpbA5Im44w1IqQGqA
📊 Statistics: 2 groups
   {"status":{"type":"String","value":"active"},"avg_score":{"value":60,"type":"Float"},"count":{"type":"Integer","value":5}}
   {"avg_score":{"type":"Float","value":50},"status":{"type":"String","value":"inactive"},"count":{"value":5,"type":"Integer"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 5
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
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
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":5},"avg_price":{"value":367,"type":"Float"}}
   {"category":{"type":"String","value":"Furniture"},"avg_price":{"type":"Float","value":365.6666666666667},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB TypeScript AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They excel at finding similar items based on vector embeddings, ideal for applications in recommendation systems and image retrieval.

2. **High Dimensionality Support**: Designed to manage high-dimensional data effectively, which is common in machine learning and AI applications.

3. **Scalability**: Many vector databases are built to scale horizontally, handling large datasets with ease.

4. **Real-time Processing**: They often support fast, real-time queries, enabling immediate responses for applications like chatbots and search engines.

5. **Integration with Machine Learning**: Seamlessly integrates with ML workflows, making it easier to store and query embeddings generated by models.

6. **Flexibility**: Supports various data types, including text, images, and videos, which can be represented as vectors.

7. **Advanced Indexing Techniques**: Uses methods like Approximate Nearest Neighbor (ANN) to speed up searches without compromising much on accuracy.

8. **Enhanced Analytics**: Facilitates powerful analytics on unstructured data, providing deeper insights from the stored vectors.

These features make vector databases increasingly popular in fields like AI, natural language processing, and computer vision.
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
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 1akDP9MpqfGjK88EmHBCagued158TXjAvtIXJhxVoKzHT3Z0dgS-SgaCaK7L7ShR2sv5oPFLXxOnVcVxSrDCEw
📊 Found 2 product groups
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3},"avg_price":{"value":575.6666666666666,"type":"Float"}}
   {"avg_price":{"value":474,"type":"Float"},"count":{"value":2,"type":"Integer"},"category":{"value":"Furniture","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"}}
   {"category":{"value":"Furniture","type":"String"},"count":{"value":2,"type":"Integer"}}
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
   {"role":{"type":"String","value":"admin"},"count":{"type":"Integer","value":3},"avg_score":{"value":20,"type":"Float"}}
   {"role":{"value":"user","type":"String"},"count":{"value":7,"type":"Integer"},"avg_score":{"type":"Float","value":70}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: WPSDQoWq1gYHn0gTfBIKXGIW_-AM4ciOaXpEKapMW3MWZRELllv7oHs0UzbcrVpemQDMnTDGVUOSJcRzYEKY5A
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: Qxgq39XFnRMJ3sx_08V_A627xfOh9ZWIn4VZgBUuy7s4l8hegpGlsye1wf9k_rvuYpUTLZ-1zYN3QPwxDzpZDQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: O9mz0SaXPGFhawaQKgHAipSjI7GqlEw5hgXY2C7zuKub6xtT09-nvJjC4vBaeJFIH1THjKuiKdC1jkYo6Vy4dg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: 6NAOaFrKTz205hit3_cslSIQg1VRClQETUFzfRUXgiLVMTQdYSjYbhnhQcleLTw9KrwAFCbjRWH1WYe4vNO2XQ
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
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"type":"Integer","value":3}}
   {"count":{"value":2,"type":"Integer"},"category":{"value":"AI","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: qVG4M9AhrqKFYRm-Lzk-sxs2NjKoGbcsHa4dB7sxmAxYZD1EUAeEl0npEIyFAqxq4v9X4DqBOMWLTT8VYGNVyg)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'qVG4M9AhrqKFYRm-Lzk-sxs2NjKoGbcsHa4dB7sxmAxYZD1EUAeEl0npEIyFAqxq4v9X4DqBOMWLTT8VYGNVyg',
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
=== ekoDB Goals, Tasks & Agents Example (TypeScript) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: 6NZwEUPBVu3tOi5ndbqheBQl-aoIu63a5CXELnMscPGLtamV2mBYiOtPifIM7PGaX6jJhPXud08VoA4HXMLB6g)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-10-08T05:48:47.801030+00:00",
      "description": "Ship version 2.0 to production",
      "id": "6NZwEUPBVu3tOi5ndbqheBQl-aoIu63a5CXELnMscPGLtamV2mBYiOtPifIM7PGaX6jJhPXud08VoA4HXMLB6g",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-10-08T05:48:47.801030+00:00"
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
        "value": "2026-10-08T05:48:47.801030+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "6NZwEUPBVu3tOi5ndbqheBQl-aoIu63a5CXELnMscPGLtamV2mBYiOtPifIM7PGaX6jJhPXud08VoA4HXMLB6g",
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
        "value": "2026-10-08T05:48:47.813287+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal 6NZwEUPBVu3tOi5ndbqheBQl-aoIu63a5CXELnMscPGLtamV2mBYiOtPifIM7PGaX6jJhPXud08VoA4HXMLB6g
--- Goal step: complete step 0 ---
Step 0 completed on goal 6NZwEUPBVu3tOi5ndbqheBQl-aoIu63a5CXELnMscPGLtamV2mBYiOtPifIM7PGaX6jJhPXud08VoA4HXMLB6g
--- Goal step: fail step 1 ---
Step 1 failed on goal 6NZwEUPBVu3tOi5ndbqheBQl-aoIu63a5CXELnMscPGLtamV2mBYiOtPifIM7PGaX6jJhPXud08VoA4HXMLB6g

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: KoU9ltKZ4geVKvjwrIslaBWPz4HwInnkc2Xkx6yzbZx4a15L96OMA85Ge6Bi8QskMem9qGMvBhYCzQsziKjkBg)

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
      "id": "KoU9ltKZ4geVKvjwrIslaBWPz4HwInnkc2Xkx6yzbZx4a15L96OMA85Ge6Bi8QskMem9qGMvBhYCzQsziKjkBg",
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
Created agent: SupportBot (id: 3PBXiosS0p0QzHYhu-u3-RfIKsZxKJH5r2P3xqN3QOQvJ7qTYrMy0LEO9e7vcxnw5XCJmCeCjFQjCg3qCGkCzg)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "3PBXiosS0p0QzHYhu-u3-RfIKsZxKJH5r2P3xqN3QOQvJ7qTYrMy0LEO9e7vcxnw5XCJmCeCjFQjCg3qCGkCzg",
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
By name: SupportBot (id: 3PBXiosS0p0QzHYhu-u3-RfIKsZxKJH5r2P3xqN3QOQvJ7qTYrMy0LEO9e7vcxnw5XCJmCeCjFQjCg3qCGkCzg)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent 3PBXiosS0p0QzHYhu-u3-RfIKsZxKJH5r2P3xqN3QOQvJ7qTYrMy0LEO9e7vcxnw5XCJmCeCjFQjCg3qCGkCzg; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted successfully

--- Cleanup: deleting goals ---
Goals deleted successfully

=== All goals, tasks & agents operations completed ===
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data:
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

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
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/jwt_register_ts_76029_1791438528902 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_76029_1791438528902 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_76029_1791438528902 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: MdsmsdmjJOeLx6YB74ku0jlUBr_FsKr_zH0v2St5gamiyjigMufqO0sFUqAsaEoDwMDBbjWftXWbZQnF250_FA
Inserted document 2: JqKMHbSSj_XYjmjT6ZClnYDXabGFKbUhLJNskIbpMU-WKayN7AIZOJPoeM3D2cmHchb6kbIn56RJBOGZlxqIHw

--- Linking documents to KV key ---
Linked doc MdsmsdmjJOeLx6YB74ku0jlUBr_FsKr_zH0v2St5gamiyjigMufqO0sFUqAsaEoDwMDBbjWftXWbZQnF250_FA: null
Linked doc JqKMHbSSj_XYjmjT6ZClnYDXabGFKbUhLJNskIbpMU-WKayN7AIZOJPoeM3D2cmHchb6kbIn56RJBOGZlxqIHw: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_76060_1791438529259",
    "document_id": "MdsmsdmjJOeLx6YB74ku0jlUBr_FsKr_zH0v2St5gamiyjigMufqO0sFUqAsaEoDwMDBbjWftXWbZQnF250_FA",
    "field_path": null,
    "created_at": "2026-10-08T05:48:49.337006Z",
    "last_accessed": "2026-10-08T05:48:49.340013Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_76060_1791438529259",
    "document_id": "JqKMHbSSj_XYjmjT6ZClnYDXabGFKbUhLJNskIbpMU-WKayN7AIZOJPoeM3D2cmHchb6kbIn56RJBOGZlxqIHw",
    "field_path": null,
    "created_at": "2026-10-08T05:48:49.338656Z",
    "last_accessed": "2026-10-08T05:48:49.340013Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc JqKMHbSSj_XYjmjT6ZClnYDXabGFKbUhLJNskIbpMU-WKayN7AIZOJPoeM3D2cmHchb6kbIn56RJBOGZlxqIHw: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_76060_1791438529259",
    "document_id": "MdsmsdmjJOeLx6YB74ku0jlUBr_FsKr_zH0v2St5gamiyjigMufqO0sFUqAsaEoDwMDBbjWftXWbZQnF250_FA",
    "field_path": null,
    "created_at": "2026-10-08T05:48:49.337006Z",
    "last_accessed": "2026-10-08T05:48:49.342408Z",
    "metadata": {}
  }
]

=== All KV linking operations completed ===

--- Cleanup ---
Cleanup complete
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_76091_1791438529710:cache:product:1: success
  kv_ops_ts_76091_1791438529710:cache:product:2: success
  kv_ops_ts_76091_1791438529710:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_76091_1791438529710:cache:product:1: { price: 29.99, name: 'Product 1' }
  kv_ops_ts_76091_1791438529710:cache:product:2: { price: 39.99, name: 'Product 2' }
  kv_ops_ts_76091_1791438529710:cache:product:3: { price: 49.99, name: 'Product 3' }

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
  kv_ops_ts_76091_1791438529710:cache:product:1: deleted
  kv_ops_ts_76091_1791438529710:cache:product:2: deleted
  kv_ops_ts_76091_1791438529710:cache:product:3: deleted

✓ All KV operations completed successfully
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
    "name": "Mars",
    "diameter_km": 6779
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: c90d9636-f802-4c93-98de-0564e63f620d, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-10-08T05:48:53.617089Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_76251_1791438533586",
      "id": "c90d9636-f802-4c93-98de-0564e63f620d",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-10-09T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-10-08T05:48:53.617089Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "c90d9636-f802-4c93-98de-0564e63f620d",
  "status": "triggered"
}

--- Pausing schedule ---
Schedule enabled after pause: false

--- Resuming schedule ---
Schedule enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
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
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.774, Matched:
  2. Score: 0.766, Matched:
  3. Score: 0.753, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.501, Matched: content, title
  2. Score: 0.910, Matched: content, title
  3. Score: 0.306, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)


✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'Wc8R7r5Yor_LWwKY_mDqaG-pKxGcJ3MzACoDvYcmGUt93Ca65YSOTzjXqfG0Os_fmILVQilfLyyStX13V4P-iA'
}

=== Find by ID ===
Found: {
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { value: 'Test Record', type: 'String' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  value: { value: 42, type: 'Integer' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  created_at: { value: '2026-10-08T05:48:55.023+00:00', type: 'DateTime' },
  id: 'Wc8R7r5Yor_LWwKY_mDqaG-pKxGcJ3MzACoDvYcmGUt93Ca65YSOTzjXqfG0Os_fmILVQilfLyyStX13V4P-iA',
  active: { value: true, type: 'Boolean' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  price: { value: 99.99, type: 'Float' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-08T05:48:55.023Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  data: 'aGVsbG8gd29ybGQ=',
  categories: [ 'electronics', 'computers' ],
  name: 'Test Record',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  value: 42,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  created_at: '2026-10-08T05:48:55.023+00:00',
  id: 'Wc8R7r5Yor_LWwKY_mDqaG-pKxGcJ3MzACoDvYcmGUt93Ca65YSOTzjXqfG0Os_fmILVQilfLyyStX13V4P-iA',
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  price: 99.99,
  metadata: { nested: { deep: true }, key: 'value' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  value: { type: 'Integer', value: 100 },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  active: { type: 'Boolean', value: true },
  created_at: { type: 'DateTime', value: '2026-10-08T05:48:55.023+00:00' },
  name: { value: 'Updated Record', type: 'String' },
  id: 'Wc8R7r5Yor_LWwKY_mDqaG-pKxGcJ3MzACoDvYcmGUt93Ca65YSOTzjXqfG0Os_fmILVQilfLyyStX13V4P-iA',
  price: { value: 99.99, type: 'Float' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: wsIioEfq10moQKqUEBeWx7Eck7k-D2VvwwWcZUTwaMWC5euLFM6ICEvA779LdCqiLmajUUz77O-RAp8wHhB5HA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
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
✓ Created native SWR script: github_user_native_ts (UVpLhFfG2EyIqAE4O5bkWXgqyQHI_oLlo2waUvvXO-MyNOZSAqZAfmrEiMKOirpn-Gk5Qvw7dEcehygQvyPNTg)

First call (cache miss - will fetch from GitHub API):
  Response time: 127ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 42.3x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (XIoUkrUGfGI83f1b_zmf_1YCYr9lnhZlFd9-Tvb6vgtoYaYzeUlnoeteRgRsaNCQlu5eNecFWqacIpgk2D1SpA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (43uNMAYI3V5tBFpXUMuevGj3hzijpcYTT883DKjhQ9NIQoXb48QKyxtoEh9ysUCs_dV6AaFknRDlxLgIQUmQtQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (9PxuhIbWw5-na5bvAwrUA7QUCZwKTMsU44i4NQR0r70rr8S8hTtKIut3iUzJgXP_CmFP-KZO2X6ndnAOx6isbA)
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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_76477_1791438537065 (CxJKN8F0SB0RPl7T0nCFIUGTeZyRtltVrGv_m04NymSSEEi3YN9N3uogMmwlBZLj0c_ryynp0lXI6qdaYRhL4w)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "id": 1,
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "city": "Gwenborough"
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "username": "Bret",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz"
        },
        "type": "Object"
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
Response time: 2ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "city": "Gwenborough"
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "username": "Bret",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz"
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
✓ Created enrichment script: fetch_product_reviews_ts_76477_1791438537065 (rk5GDFT28L2p5BE2Yos-QB9nN67DK3Ln_kb9FXJcyjCa2bxHjBLjrd57LnWMqf2ifjcMZOs6SSqzfohfpJAj6g)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "category": "beauty",
          "tags": [
            "beauty",
            "mascara"
          ],
          "title": "Essence Mascara Lash Princess",
          "meta": {
            "createdAt": "2025-10-09T14:47:01.588Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "barcode": "5784719087687"
          },
          "weight": 4,
          "dimensions": {
            "depth": 22.99,
            "height": 13.08,
            "width": 15.14
          },
          "reviews": [
            {
              "reviewerName": "Eleanor Collins",
              "comment": "Would not recommend!",
              "rating": 3,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Lucas Gordon",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "rating": 4
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Highly impressed!",
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5
            }
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "returnPolicy": "No return policy",
          "availabilityStatus": "In Stock",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "discountPercentage": 10.48,
          "price": 9.99,
          "stock": 99,
          "minimumOrderQuantity": 48,
          "shippingInformation": "Ships in 3-5 business days",
          "id": 1,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "warrantyInformation": "1 week warranty",
          "sku": "BEA-ESS-ESS-001",
          "brand": "Essence",
          "rating": 2.56
        },
        "type": "Object"
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

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: CCMgisAy_j9j3iZedIyV109w8xRnfakny5UjHoRPisbVu_T3uHRfdFZei3ZmOldsDrWIn0H5Qv4iE7UqZh7JoA
Created Bob: $500 - ID: lXqzZgkkj7ugIU_dENFgxZU_4xJnFiWyfcq9ZC2LJMcNhQGXXRPSVNRIYmdnbyhFZGONy8vgp4l443ltQo_EdQ

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 7ea5ab15-2501-461f-aa33-582a85ab6df9

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
New transaction: 565ddf6d-54a8-497c-9339-b300e86e7604
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: Hqwvj0sN8FnFYbL5nUJwuMeHsgbKXaEXUwIYVZadzRzzqQK4R31dl2uAVHEFW4nAl1HgpopV3FWLbpA8EpycXg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 5 user functions:
  - get_active_users_ts: Get Active Users
  - conc_demo_rl_skip_ts_75441_1791438514383: Rate-limit (skip mode)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - conc_demo_rl_fail_ts_75441_1791438514383: Rate-limit (fail mode)
  - fetch_product_reviews_ts_76477_1791438537065: Fetch Product with Reviews (Multi-API)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: cTvWGdHW7zTTDlgRYhtKFP2RCzeL5jDX-Px4lQkGH3gZap2i25CHXq8CJP70T_yiN3qfYCghJfLIOo6vCF1Lhw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: NzBL1_N7X4UYLXG6pZR5zULrExy5SToleGKncYH5JuZ-oDGKFZXRx6j7igG-8k2JbnibjSYb2wX7101RHsRrrg
Execution time: 728ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_f748bc11001647679bb922131a6b138d)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: njACYSVJNMjJ1GU5n38G5pCi0h9tQ_QiI0UcbPjnVnK001OEjf1TqKSav__8BNz5Rr_yDcNny_S0_JER6OIyOw
  📡 Notification received for njACYSVJNMjJ1GU5n38G5pCi0h9tQ_QiI0UcbPjnVnK001OEjf1TqKSav__8BNz5Rr_yDcNny_S0_JER6OIyOw

Inserting another record...
✓ Inserted record: _jO_6kr_IEVsifZChPeDb0OZMPzzwmjUeoAsmTClIH6mTwxyp-9KODK1zzW1GxUpToQhXpp9N-Re4wQvpLridg
  📡 Notification received for _jO_6kr_IEVsifZChPeDb0OZMPzzwmjUeoAsmTClIH6mTwxyp-9KODK1zzW1GxUpToQhXpp9N-Re4wQvpLridg

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: WH-Biw-b1HegR-WCGclp6MxhhoXjhS__IMRahZXw9DzG0GJYWrMzG_YVfcR9uR9djPlv6PrT78e8bhGru4tCqA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {"id":"seKtKBoGv8GUebyqVrf3dtVdEyWsXpUuRNcxRXAQQKTlQsxDRPKldeXNJ6iG6d-eAmrL4WeL5YJVS95vJWy7RA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"X0kEpALmDUiqckMaZe8eOwa_nzG32qNN23U8VA6hRzDXcHuW512FkGTfKHhruurtX2s3cvQZcarBUUvBlXVXPg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150},"id":"seKtKBoGv8GUebyqVrf3dtVdEyWsXpUuRNcxRXAQQKTlQsxDRPKldeXNJ6iG6d-eAmrL4WeL5YJVS95vJWy7RA"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"custom-id"}

✅ All bypass_ripple operations completed successfully!
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
  Fields returned: ["created_at","bio","name","avatar_url","status","id","user_role","age","email"]

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
    - Fields: ["created_at","status","email","api_key","name","avatar_url","secret_token","id","password","user_role","bio","age"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","email","id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
