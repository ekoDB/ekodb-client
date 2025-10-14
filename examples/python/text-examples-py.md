make test-examples-py 🧪 Running Python examples
(direct HTTP/WebSocket)...

╔════════════════════════════════════════╗ ║ ekoDB Python Examples Test Suite ║
╚════════════════════════════════════════╝

=== Checking Server Connection === ✓ Server is ready

=== Getting Authentication Token === ✓ Authentication successful

=== Running 7 Examples ===

=== Running simple_crud.py === ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'Cr-e0onmH033rqxyZlJYYloOFay_LB9Gp-bkEcoAyPyG3MgSRc5B0LFLA0oD_8JJW-llSyhYa3-ZQ7Jtnj1EeA'}

=== Find by ID === Found: {'active': {'type': 'Boolean', 'value': True}, 'name':
{'value': 'Test Record', 'type': 'String'}, 'value': {'type': 'Integer',
'value': 42}, 'id':
'Cr-e0onmH033rqxyZlJYYloOFay_LB9Gp-bkEcoAyPyG3MgSRc5B0LFLA0oD_8JJW-llSyhYa3-ZQ7Jtnj1EeA'}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'name': 'Updated Record', 'value': 100, 'id':
'Cr-e0onmH033rqxyZlJYYloOFay_LB9Gp-bkEcoAyPyG3MgSRc5B0LFLA0oD_8JJW-llSyhYa3-ZQ7Jtnj1EeA',
'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ simple_crud.py completed
successfully

=== Running simple_websocket.py === ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
ADdD8WPgWbD67dR4j_DTtBDBWuj3IoZGllz-p5HlUesLGwghLshHNQme-OIhNWaMxFvgw7y9fiInHnQeT1DIlw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ADdD8WPgWbD67dR4j_DTtBDBWuj3IoZGllz-p5HlUesLGwghLshHNQme-OIhNWaMxFvgw7y9fiInHnQeT1DIlw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 1
record(s) via WebSocket

✓ WebSocket example completed successfully ✓ simple_websocket.py completed
successfully

=== Running batch_operations.py === ✓ Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="None"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully ✓ batch_operations.py completed
successfully

=== Running kv_operations.py === ✓ Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ kv_operations.py completed
successfully

=== Running collection_management.py === ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
KVhgLZ2xilFiysItoo2a51KYmBCexbryEfPhDBJiq8v5gNdqlTxAbeZ3j0acaKePnew9Bh81-2cUIc5IrFV5-A

=== List Collections === Total collections: 4 Sample collections:
['test_collection', 'demo_collection', 'websocket_test', 'batch_users']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓
collection_management.py completed successfully

=== Running document_ttl.py === ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
IsmUbE3uj71LjIn79Kn6uCCbsbW1iy-aTkhXmfBVLjtOhTVzeiOHwD58wssguL6w7L62PzZD0d6ltLA97FyooA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 1 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ document_ttl.py completed
successfully

=== Running websocket_ttl.py === ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
CqJ2nu8Ep6sGKu8yb1BX4fMdxL6n66v1BQFNCKw2k8c4u6-sehCKOUdhWbHgzMttjP5X3kGWKz4Vcp2Wxv3C5A

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✓ websocket_ttl.py completed
successfully

╔════════════════════════════════════════╗ ║ Test Summary ║
╚════════════════════════════════════════╝ Total: 7 Passed: 7 Failed: 0 ✅
Python direct examples complete! 🐍 Building and installing Python client
package... 🍹 Building a mixed python/rust project 🔗 Found pyo3 bindings with
abi3 support 🐍 Not using a specific python interpreter 💻 Using
`MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default Finished
`release` profile [optimized] target(s) in 0.08s 📦 Built wheel for abi3 Python
≥ 3.8 to
Processing
./ekodb-client-py/target/wheels/ekodb_client-0.1.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client Attempting uninstall: ekodb-client
Found existing installation: ekodb_client 0.1.0 Uninstalling ekodb_client-0.1.0:
Successfully uninstalled ekodb_client-0.1.0 Successfully installed
ekodb-client-0.1.0 ✅ Python client package built and installed! 🧪 Running
Python client library examples... ✓ Client created (token exchange happens
automatically)

=== Insert Document === Inserted: {'id':
'-y5-NKbcoHwiXcIDnptaRwxRYw98qQivxW5WzyPS4Z8qJGI1jaVTdJJ3SECfhyvIoIRzkl6Jm8_E2cgn6IGarQ'}

=== Find by ID === Found: {'name': {'value': 'Test Record', 'type': 'String'},
'id':
'-y5-NKbcoHwiXcIDnptaRwxRYw98qQivxW5WzyPS4Z8qJGI1jaVTdJJ3SECfhyvIoIRzkl6Jm8_E2cgn6IGarQ',
'active': {'value': 1, 'type': 'Integer'}, 'value': {'type': 'Integer', 'value':
42}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'active': {'value': 1, 'type': 'Integer'},
'id':
'-y5-NKbcoHwiXcIDnptaRwxRYw98qQivxW5WzyPS4Z8qJGI1jaVTdJJ3SECfhyvIoIRzkl6Jm8_E2cgn6IGarQ',
'value': 100, 'name': 'Updated Record'}

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully ✓ Client created

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Batch Update === ✓ Batch updated 3 records

=== Batch Delete === ✓ Batch deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: {'value':
'{"userId":123,"username":"john_doe"}'}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'value': '{"name":"Product
1","price":29.99}'} cache:product:2: {'value': '{"name":"Product
2","price":39.989999999999995}'} cache:product:3: {'value': '{"name":"Product
3","price":49.989999999999995}'}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"OUe-y1NwlkMs6y8DWBjXU4hOu3s0kyJoAyhk9rX2snK4fdqWC7Dw77PEQWh1EybQPMPnerca19WLPj3AKvivBg"

=== List Collections === Total collections: 6 Sample collections:
['test_collection', 'ws_ttl_test', 'websocket_test', 'batch_users', 'ttl_cache']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
769cjWEa59RLWrri3WJnXCh8Sq1nCxcvJAXYMg7QbeDHwn9mxmq0u6bpUM29ZLdGI26VPb94DW2CYUQpE-XHOQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
K8VqtunP_W5pJE1Nfx_i2FHa4uDdOV8XEgspJf5TIKTasxrEtwgKRJ68MbwUNP6WyL3gu7uWivJ176TcBlwQLg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
P7bSK9sugXa9T9cjr_FVHuLTJJyQiwzWZqIrs3oU1IB3c2QN5Hy0GtUoTwGtPkaxn_mxFhbVgKLwAMCJD-pYrg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
XeRrTNGcA6iYhmoZBiHLmjctSgi6XBuUJYgcdRDs5nfPprpSe5k0rm9rMXloEzOMMTbhR6VTqNNt3e0Ky1WSVw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
DS5Y7gIpiFmI7iDJinJWIuhdzqLzzCdkQrL2xS2MgsLwEgHRE_jI0wCiO9UlBlX44dhabzyVopjNrVb-l-Gnrw

=== Sending Chat Message === Message ID:
3NC3hx6xouKAORkiMA_OXWVDuly2H5MeYH-tijCq56Qh0s0_X1bcsrPlDxiQROGFijpCxLOG1Ot0nBMwlQBUoA

=== AI Response === There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It
   is priced at $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. It is
   priced at $499.

3. ekoDB Pro: This is an enterprise edition product with advanced features. It
   is priced at $299.

Execution Time: 2667ms

=== Token Usage === Prompt tokens: 376 Completion tokens: 78 Total tokens: 454

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
4NLBtgQXzDiyh1IFu6Cym_FCjKJ18AKx62_yRqL5vZ0e5ehiC_tXv-llT2zY5z-Yk1k7vEagtRKU9NiM9UhhBg

=== Sending Messages === ✓ Message 1 sent Response: Based on the provided
context, the available product is a high-performance database product named
ekoDB. It is priced at 99.

✓ Message 2 sent Response: As an assistant, I'm sorry but I can't provide the
price as there's no specific product mentioned in your context. Could you please
provide more details about the product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
VE3mRGZR2yYvvRvPwonqaAae-MZ785MoCR0OFg8V0cw3zJGbUvSyJQTNvX7eiYxeeHHSQvatQ6jgEuFS4T4YdA
Parent:
4NLBtgQXzDiyh1IFu6Cym_FCjKJ18AKx62_yRqL5vZ0e5ehiC_tXv-llT2zY5z-Yk1k7vEagtRKU9NiM9UhhBg

=== Listing Sessions === ✓ Found 3 sessions Session 1:
VE3mRGZR2yYvvRvPwonqaAae-MZ785MoCR0OFg8V0cw3zJGbUvSyJQTNvX7eiYxeeHHSQvatQ6jgEuFS4T4YdA
(Untitled) Session 2:
4NLBtgQXzDiyh1IFu6Cym_FCjKJ18AKx62_yRqL5vZ0e5ehiC_tXv-llT2zY5z-Yk1k7vEagtRKU9NiM9UhhBg
(Untitled) Session 3:
DS5Y7gIpiFmI7iDJinJWIuhdzqLzzCdkQrL2xS2MgsLwEgHRE_jI0wCiO9UlBlX44dhabzyVopjNrVb-l-Gnrw
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
VE3mRGZR2yYvvRvPwonqaAae-MZ785MoCR0OFg8V0cw3zJGbUvSyJQTNvX7eiYxeeHHSQvatQ6jgEuFS4T4YdA

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
-ZwuXxsPasY4ofGSIZZ5eepHD5fAD6QhdSOELfRMZczgmFAlGu7A2d9GPqip03-KihnKCGUuba1DoPPGMGlHWw

=== Sending Initial Message === ✓ Message sent Response: Based on the
information provided, the available product is ekoDB. It is a high-performance
database product priced at $99.

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['id', 'chat_id',
'updated_at', 'content', 'created_at', 'token_usage', 'role']) Debug: First
message role: {'type': 'String', 'value': 'assistant'} === Feature 1: Regenerate
AI Response === ✓ Message regenerated New response: I'm sorry, but the context
provided does not include information about the price of ekoDB. You may need to
check the official website or contact the sellers for the most accurate and
current price.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
1AKcdSS5xWb-903LT9m9IavYmrL0ERTT6KW2IUjK-GMOWvXYFA7D2bQ_IfMkpLM9x7ooyP9O3GK9KpeETXmKpQ
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! ✅ Python client
examples complete! ✅ All Python integration tests complete!
