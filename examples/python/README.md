# ekoDB Python Examples

This directory contains Python examples for ekoDB.

## Installation

### Using Published Package (Default)

```bash
pip install -r requirements.txt
```

This installs `ekodb-client` from PyPI.

### Using Local Development Version

```bash
pip install -r requirements-dev.txt
```

This installs the local editable version from `../../ekodb-client-py`.

### Switching Between Versions

**To use published version:**

```bash
pip uninstall ekodb-client
pip install ekodb-client
```

**To use local version:**

```bash
pip uninstall ekodb-client
pip install -e ../../ekodb-client-py
```

**Or edit `requirements.txt`:**

```python
# Use published:
ekodb-client>=0.1.0

# Use local (comment out above, uncomment below):
# ekodb-client @ file://../../ekodb-client-py
```

## Prerequisites

- Python 3.10 or higher
- pip (Python package manager)

The requirements files require aiohttp 3.14.4, multidict 6.9.1, and
python-dotenv 1.2.2 or later. The shared `../requirements.txt` also requires
requests 2.33.0, certifi 2026.10.19, and urllib3 2.8.0 or later for the direct
HTTP examples. Refresh an existing environment with:

```bash
python -m pip install --upgrade "pip>=26.2" "setuptools>=83.0.0" "black>=26.10.1"
python -m pip install --upgrade -r ../requirements.txt
```

Use a supported, patched Python interpreter when creating the environment;
upgrading pip packages does not update its interpreter.

## Installation

```bash
pip install -r requirements.txt
```

## Examples

### TTL Features

- [Document TTL](document_ttl.py) - TTL expiration verification test
- [WebSocket TTL](websocket_ttl.py) - Real-time TTL events via WebSocket

### Basic Operations

- [Simple CRUD](basic-crud/simple_crud.py) - Basic create, read, update, delete
  operations
- [Batch Operations](performance/batch.py) - Efficient batch operations
- [Real-time Updates](websocket/realtime.py) - WebSocket-based real-time updates

### AI & RAG Examples

- **[rag_conversation_system.py](rag_conversation_system.py)** - RAG System
  (NEW)

  ```bash
  python rag_conversation_system.py
  ```

  Complete RAG (Retrieval-Augmented Generation) example showing:
  - Message storage with automatic embeddings
  - Cross-conversation hybrid search (semantic + keyword)
  - Context-aware AI responses using search results
  - Self-improving knowledge base
  - Dynamic search configurations per conversation

## Running Examples

1. Start the ekoDB server
2. Set your JWT token in the example file or use environment variables
3. Run an example:

   ```bash
   python document_ttl.py
   ```

## Important Notes

- Use `asyncio` for WebSocket operations
- Always set `bypassCache: true` in WebSocket subscriptions for real-time data
- Use proper error handling in production code
- Check the [TTL Documentation](../../documentation/TTL.md) for best practices
