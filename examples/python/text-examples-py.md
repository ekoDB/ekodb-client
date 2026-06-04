make test-examples-python
📦 [36mEnsuring Python example dependencies in .venv...[0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
Traceback (most recent call last):
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/simple_crud.py", line 10, in <module>
    import requests
ModuleNotFoundError: No module named 'requests'
Traceback (most recent call last):
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/simple_websocket.py", line 12, in <module>
    import requests
ModuleNotFoundError: No module named 'requests'
Traceback (most recent call last):
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/http_functions.py", line 12, in <module>
    import aiohttp
ModuleNotFoundError: No module named 'aiohttp'
Traceback (most recent call last):
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/batch_operations.py", line 10, in <module>
    import requests
ModuleNotFoundError: No module named 'requests'
Traceback (most recent call last):
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/kv_operations.py", line 10, in <module>
    import requests
ModuleNotFoundError: No module named 'requests'
Traceback (most recent call last):
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/collection_management.py", line 10, in <module>
    import requests
ModuleNotFoundError: No module named 'requests'
Traceback (most recent call last):
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/transactions.py", line 7, in <module>
    import aiohttp
ModuleNotFoundError: No module named 'aiohttp'
Traceback (most recent call last):
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/crud_functions.py", line 17, in <module>
    import aiohttp
ModuleNotFoundError: No module named 'aiohttp'
Traceback (most recent call last):
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/document_ttl.py", line 19, in <module>
    from dotenv import load_dotenv
ModuleNotFoundError: No module named 'dotenv'
Traceback (most recent call last):
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/websocket_ttl.py", line 20, in <module>
    from dotenv import load_dotenv
ModuleNotFoundError: No module named 'dotenv'
[34m
╔════════════════════════════════════════╗[0m
[34m║   ekoDB Python Examples Test Suite    ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running simple_crud.py ===[0m
[31m✗ simple_crud.py failed with code 1[0m
[33mContinuing with next example...[0m
[34m
=== Running simple_websocket.py ===[0m
[31m✗ simple_websocket.py failed with code 1[0m
[33mContinuing with next example...[0m
[34m
=== Running http_functions.py ===[0m
[31m✗ http_functions.py failed with code 1[0m
[33mContinuing with next example...[0m
[34m
=== Running batch_operations.py ===[0m
[31m✗ batch_operations.py failed with code 1[0m
[33mContinuing with next example...[0m
[34m
=== Running kv_operations.py ===[0m
[31m✗ kv_operations.py failed with code 1[0m
[33mContinuing with next example...[0m
[34m
=== Running collection_management.py ===[0m
[31m✗ collection_management.py failed with code 1[0m
[33mContinuing with next example...[0m
[34m
=== Running transactions.py ===[0m
[31m✗ transactions.py failed with code 1[0m
[33mContinuing with next example...[0m
[34m
=== Running crud_functions.py ===[0m
[31m✗ crud_functions.py failed with code 1[0m
[33mContinuing with next example...[0m
[34m
=== Running document_ttl.py ===[0m
[31m✗ document_ttl.py failed with code 1[0m
[33mContinuing with next example...[0m
[34m
=== Running websocket_ttl.py ===[0m
[31m✗ websocket_ttl.py failed with code 1[0m
[33mContinuing with next example...[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 0[0m
[31mFailed: 10[0m
make[2]: *** [test-examples-python-direct] Error 1
