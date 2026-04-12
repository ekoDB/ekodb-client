"""
Stage helpers for ekoDB script functions.

Provides helper methods for creating function stage configurations
that can be used in script definitions.
"""

from typing import Any, Dict, List, Optional, Union


def parameter_ref(name: str) -> Dict[str, str]:
    """Build the structural placeholder ``{"type": "Parameter", "name": name}``
    that ekoDB's ``resolve_json_parameters`` recognizes inside
    ``Insert.record``, ``Update.updates``, ``UpdateById.updates``,
    ``FindOneAndUpdate.updates``, ``BatchInsert.records``, and any
    ``QueryExpression`` filter value.

    This is the structural alternative to the text-level ``"{{name}}"``
    placeholder — use it when the parameter is a whole-object record or a
    value whose type would be lost on a raw-JSON round-trip (Binary,
    DateTime, UUID, Decimal, Duration, Number, Set, Vector). Requires
    ekoDB >= 0.41.0 for the mutation-stage parameter-resolution
    consistency fix.

    Example:
        >>> stage = Stage.insert("items", parameter_ref("record"))
        >>> stage["record"]
        {'type': 'Parameter', 'name': 'record'}
    """
    return {"type": "Parameter", "name": name}


class Stage:
    """Helper class for creating function stage configurations."""

    @staticmethod
    def param(name: str) -> Dict[str, str]:
        """Shorthand for :func:`parameter_ref`.

        See :func:`parameter_ref` for the full explanation and example.
        """
        return parameter_ref(name)

    @staticmethod
    def find_all(collection: str) -> Dict[str, Any]:
        """Find all records in a collection."""
        return {"type": "FindAll", "collection": collection}

    @staticmethod
    def query(
        collection: str,
        filter: Optional[Dict[str, Any]] = None,
        sort: Optional[List[Dict[str, Any]]] = None,
        limit: Optional[int] = None,
        skip: Optional[int] = None,
    ) -> Dict[str, Any]:
        """Query records with filter, sort, limit, skip."""
        stage: Dict[str, Any] = {"type": "Query", "collection": collection}
        if filter is not None:
            stage["filter"] = filter
        if sort is not None:
            stage["sort"] = sort
        if limit is not None:
            stage["limit"] = limit
        if skip is not None:
            stage["skip"] = skip
        return stage

    @staticmethod
    def find_by_id(collection: str, record_id: str) -> Dict[str, Any]:
        """Find a record by ID."""
        return {"type": "FindById", "collection": collection, "record_id": record_id}

    @staticmethod
    def insert(
        collection: str,
        record: Dict[str, Any],
        bypass_ripple: bool = False,
        ttl: Optional[str] = None,
    ) -> Dict[str, Any]:
        """Insert a record into a collection."""
        stage = {
            "type": "Insert",
            "collection": collection,
            "record": record,
            "bypass_ripple": bypass_ripple,
        }
        if ttl:
            stage["ttl"] = ttl
        return stage

    @staticmethod
    def update(
        collection: str,
        filter: Dict[str, Any],
        updates: Dict[str, Any],
        bypass_ripple: bool = False,
        ttl: Optional[Union[str, int]] = None,
    ) -> Dict[str, Any]:
        """Update records matching filter."""
        stage: Dict[str, Any] = {
            "type": "Update",
            "collection": collection,
            "filter": filter,
            "updates": updates,
        }
        if bypass_ripple:
            stage["bypass_ripple"] = bypass_ripple
        if ttl is not None:
            stage["ttl"] = ttl
        return stage

    @staticmethod
    def update_by_id(
        collection: str,
        record_id: str,
        updates: Dict[str, Any],
        bypass_ripple: bool = False,
        ttl: Optional[Union[str, int]] = None,
    ) -> Dict[str, Any]:
        """Update a specific record by ID."""
        stage: Dict[str, Any] = {
            "type": "UpdateById",
            "collection": collection,
            "record_id": record_id,
            "updates": updates,
        }
        if bypass_ripple:
            stage["bypass_ripple"] = bypass_ripple
        if ttl is not None:
            stage["ttl"] = ttl
        return stage

    @staticmethod
    def delete(
        collection: str,
        filter: Dict[str, Any],
        bypass_ripple: bool = False,
    ) -> Dict[str, Any]:
        """Delete records matching filter."""
        stage: Dict[str, Any] = {
            "type": "Delete",
            "collection": collection,
            "filter": filter,
        }
        if bypass_ripple:
            stage["bypass_ripple"] = bypass_ripple
        return stage

    @staticmethod
    def delete_by_id(
        collection: str,
        record_id: str,
        bypass_ripple: bool = False,
    ) -> Dict[str, Any]:
        """Delete a specific record by ID."""
        stage: Dict[str, Any] = {
            "type": "DeleteById",
            "collection": collection,
            "record_id": record_id,
        }
        if bypass_ripple:
            stage["bypass_ripple"] = bypass_ripple
        return stage

    @staticmethod
    def project(fields: List[str], exclude: bool = False) -> Dict[str, Any]:
        """Project specific fields from records."""
        return {"type": "Project", "fields": fields, "exclude": exclude}

    @staticmethod
    def count(output_field: str) -> Dict[str, Any]:
        """Count records and store in output field."""
        return {"type": "Count", "output_field": output_field}

    @staticmethod
    def if_condition(
        condition: Dict[str, Any],
        then_functions: List[Dict[str, Any]],
        else_functions: Optional[List[Dict[str, Any]]] = None,
    ) -> Dict[str, Any]:
        """Conditional execution of functions."""
        stage = {
            "type": "If",
            "condition": condition,
            "then_functions": then_functions,
        }
        if else_functions:
            stage["else_functions"] = else_functions
        return stage

    @staticmethod
    def http_request(
        url: str,
        method: str = "GET",
        headers: Optional[Dict[str, str]] = None,
        body: Optional[Any] = None,
        timeout_seconds: Optional[int] = None,
        output_field: Optional[str] = None,
    ) -> Dict[str, Any]:
        """Make an HTTP request."""
        stage = {"type": "HttpRequest", "url": url, "method": method}
        if headers:
            stage["headers"] = headers
        if body is not None:
            stage["body"] = body
        if timeout_seconds is not None:
            stage["timeout_seconds"] = timeout_seconds
        if output_field:
            stage["output_field"] = output_field
        return stage

    @staticmethod
    def call_function(
        function_label: str,
        params: Optional[Dict[str, Any]] = None,
    ) -> Dict[str, Any]:
        """Call a saved UserFunction by label."""
        stage: Dict[str, Any] = {
            "type": "CallFunction",
            "function_label": function_label,
        }
        if params is not None:
            stage["params"] = params
        return stage

    @staticmethod
    def for_each(functions: List[Dict[str, Any]]) -> Dict[str, Any]:
        """For each record, execute functions."""
        return {"type": "ForEach", "functions": functions}

    @staticmethod
    def group(
        by_fields: List[str],
        functions: List[Dict[str, Any]],
    ) -> Dict[str, Any]:
        """Group records and apply aggregation functions."""
        return {"type": "Group", "by_fields": by_fields, "functions": functions}

    @staticmethod
    def kv_get(key: str) -> Dict[str, Any]:
        """Get a value from the KV store.

        Returns {value: <data>} on hit, {value: null} on miss.
        """
        return {"type": "KvGet", "key": key}

    @staticmethod
    def kv_set(
        key: str,
        value: Any,
        ttl: Optional[Union[str, int]] = None,
    ) -> Dict[str, Any]:
        """Set a value in the KV store with optional TTL."""
        stage: Dict[str, Any] = {"type": "KvSet", "key": key, "value": value}
        if ttl is not None:
            stage["ttl"] = ttl
        return stage

    @staticmethod
    def kv_query(
        pattern: Optional[str] = None,
        include_expired: Optional[bool] = None,
    ) -> Dict[str, Any]:
        """Query KV store with pattern matching."""
        stage: Dict[str, Any] = {"type": "KvQuery"}
        if pattern is not None:
            stage["pattern"] = pattern
        if include_expired is not None:
            stage["include_expired"] = include_expired
        return stage

    @staticmethod
    def kv_delete(key: str) -> Dict[str, Any]:
        """Delete a key from the KV store."""
        return {"type": "KvDelete", "key": key}

    @staticmethod
    def kv_exists(key: str, output_field: Optional[str] = None) -> Dict[str, Any]:
        """Check if a key exists in the KV store."""
        stage = {"type": "KvExists", "key": key}
        if output_field:
            stage["output_field"] = output_field
        return stage

    @staticmethod
    def chat(
        messages: List[Dict[str, str]],
        model: str = "gpt-4",
        temperature: float = 0.7,
        max_tokens: Optional[int] = None,
        output_field: Optional[str] = None,
    ) -> Dict[str, Any]:
        """Send a chat completion request to an AI model."""
        stage = {
            "type": "Chat",
            "messages": messages,
            "model": model,
            "temperature": temperature,
        }
        if max_tokens is not None:
            stage["max_tokens"] = max_tokens
        if output_field:
            stage["output_field"] = output_field
        return stage

    @staticmethod
    def embed(
        input_field: str,
        output_field: str,
        model: Optional[str] = None,
    ) -> Dict[str, Any]:
        """Generate embeddings for a field in each record."""
        stage: Dict[str, Any] = {
            "type": "Embed",
            "input_field": input_field,
            "output_field": output_field,
        }
        if model is not None:
            stage["model"] = model
        return stage

    @staticmethod
    def text_search(
        collection: str,
        query_text: str,
        fields: Optional[List[str]] = None,
        limit: Optional[int] = None,
        fuzzy: Optional[bool] = None,
    ) -> Dict[str, Any]:
        """Full-text search."""
        stage: Dict[str, Any] = {
            "type": "TextSearch",
            "collection": collection,
            "query_text": query_text,
        }
        if fields is not None:
            stage["fields"] = fields
        if limit is not None:
            stage["limit"] = limit
        if fuzzy is not None:
            stage["fuzzy"] = fuzzy
        return stage

    @staticmethod
    def vector_search(
        collection: str,
        query_vector: List[float],
        limit: Optional[int] = None,
        threshold: Optional[float] = None,
    ) -> Dict[str, Any]:
        """Vector similarity search."""
        stage: Dict[str, Any] = {
            "type": "VectorSearch",
            "collection": collection,
            "query_vector": query_vector,
        }
        if limit is not None:
            stage["limit"] = limit
        if threshold is not None:
            stage["threshold"] = threshold
        return stage

    @staticmethod
    def hybrid_search(
        collection: str,
        query_text: str,
        query_vector: Optional[List[float]] = None,
        limit: Optional[int] = None,
    ) -> Dict[str, Any]:
        """Hybrid search (vector + text)."""
        stage: Dict[str, Any] = {
            "type": "HybridSearch",
            "collection": collection,
            "query_text": query_text,
        }
        if query_vector is not None:
            stage["query_vector"] = query_vector
        if limit is not None:
            stage["limit"] = limit
        return stage

    @staticmethod
    def find_one(
        collection: str,
        key: str,
        value: Any,
    ) -> Dict[str, Any]:
        """Find one record by key/value pair."""
        return {
            "type": "FindOne",
            "collection": collection,
            "key": key,
            "value": value,
        }

    @staticmethod
    def batch_insert(
        collection: str,
        records: List[Dict[str, Any]],
        bypass_ripple: bool = False,
    ) -> Dict[str, Any]:
        """Insert multiple records."""
        stage: Dict[str, Any] = {
            "type": "BatchInsert",
            "collection": collection,
            "records": records,
        }
        if bypass_ripple:
            stage["bypass_ripple"] = bypass_ripple
        return stage

    @staticmethod
    def batch_delete(
        collection: str,
        record_ids: List[str],
        bypass_ripple: bool = False,
    ) -> Dict[str, Any]:
        """Delete multiple records by IDs."""
        stage: Dict[str, Any] = {
            "type": "BatchDelete",
            "collection": collection,
            "record_ids": record_ids,
        }
        if bypass_ripple:
            stage["bypass_ripple"] = bypass_ripple
        return stage

    @staticmethod
    def create_savepoint(name: str) -> Dict[str, Any]:
        """Create a savepoint for partial rollback."""
        return {"type": "CreateSavepoint", "name": name}

    @staticmethod
    def rollback_to_savepoint(name: str) -> Dict[str, Any]:
        """Rollback to a specific savepoint."""
        return {"type": "RollbackToSavepoint", "name": name}

    @staticmethod
    def release_savepoint(name: str) -> Dict[str, Any]:
        """Release a savepoint."""
        return {"type": "ReleaseSavepoint", "name": name}

    @staticmethod
    def swr(
        cache_key: str,
        ttl: Union[str, int],
        url: str,
        method: str = "GET",
        headers: Optional[Dict[str, str]] = None,
        body: Optional[Any] = None,
        timeout_seconds: Optional[int] = None,
        output_field: Optional[str] = None,
        collection: Optional[str] = None,
    ) -> Dict[str, Any]:
        """
        SWR (Stale-While-Revalidate) pattern for external API caching.
        Automatically handles: KV cache check → HTTP request → KV cache set → optional audit storage.

        Args:
            cache_key: KV key for caching (supports parameter substitution like "user:{{user_id}}")
            ttl: Cache TTL - supports duration strings ("15m", "1h"), integers (seconds), or ISO timestamps
            url: HTTP URL to fetch from (supports parameter substitution)
            method: HTTP method (default: "GET")
            headers: Optional HTTP headers
            body: Optional HTTP request body
            timeout_seconds: Optional HTTP timeout
            output_field: Field name for response in enriched params (default: "response")
            collection: Optional collection for audit trail storage

        Returns:
            Dict representing the SWR function stage
        """
        stage: Dict[str, Any] = {
            "type": "SWR",
            "cache_key": cache_key,
            "ttl": ttl,
            "url": url,
            "method": method,
        }
        if headers is not None:
            stage["headers"] = headers
        if body is not None:
            stage["body"] = body
        if timeout_seconds is not None:
            stage["timeout_seconds"] = timeout_seconds
        if output_field is not None:
            stage["output_field"] = output_field
        if collection is not None:
            stage["collection"] = collection
        return stage

    @staticmethod
    def bcrypt_hash(
        plain: str,
        output_field: str,
        cost: Optional[int] = None,
    ) -> Dict[str, Any]:
        """Bcrypt-hash a plaintext and write the result into every record in
        the working data as ``output_field``. Use in a compound
        ``users_register`` function between input validation and Insert.

        ``plain`` is typically a ``"{{password}}"`` placeholder — the
        substituter replaces it with the call-time param before this stage
        runs. ``cost`` is the bcrypt work factor (4..=31); leave ``None``
        for the ekoDB default (12).

        Requires ekoDB >= 0.41.0.
        """
        stage: Dict[str, Any] = {
            "type": "BcryptHash",
            "plain": plain,
            "output_field": output_field,
        }
        if cost is not None:
            stage["cost"] = cost
        return stage

    @staticmethod
    def bcrypt_verify(
        plain: str,
        hash_field: str,
        output_field: str,
    ) -> Dict[str, Any]:
        """Verify a plaintext against a bcrypt hash stored on the first
        record in the working data; writes a boolean result into
        ``output_field``. Pair with an ``If`` stage to branch on success
        / failure.

        ``plain`` is typically ``"{{password}}"``; ``hash_field`` is the
        name of the field on the current record holding the stored hash
        (e.g. ``"password_hash"``).

        Requires ekoDB >= 0.41.0.
        """
        return {
            "type": "BcryptVerify",
            "plain": plain,
            "hash_field": hash_field,
            "output_field": output_field,
        }

    @staticmethod
    def random_token(
        bytes: int,
        output_field: str,
        encoding: Optional[str] = None,
    ) -> Dict[str, Any]:
        """Generate a cryptographically-random token and add it to every
        record in the working data. ``bytes`` must be in 1..=1024.
        ``encoding`` is one of ``"hex"`` (default), ``"base64"``, or
        ``"base64url"``.

        Requires ekoDB >= 0.41.0.
        """
        stage: Dict[str, Any] = {
            "type": "RandomToken",
            "bytes": bytes,
            "output_field": output_field,
        }
        if encoding is not None:
            stage["encoding"] = encoding
        return stage

    @staticmethod
    def try_catch(
        try_functions: List[Dict[str, Any]],
        catch_functions: List[Dict[str, Any]],
        output_error_field: Optional[str] = None,
    ) -> Dict[str, Any]:
        """Try/Catch error handling for graceful failure recovery.
        Executes ``try_functions``, and if any fail, executes ``catch_functions``.

        Args:
            try_functions: Functions to attempt.
            catch_functions: Functions to execute on failure.
            output_error_field: Field name to store error details (default: "error").
        """
        stage: Dict[str, Any] = {
            "type": "TryCatch",
            "try_functions": try_functions,
            "catch_functions": catch_functions,
        }
        if output_error_field is not None:
            stage["output_error_field"] = output_error_field
        return stage

    @staticmethod
    def parallel(
        functions: List[Dict[str, Any]],
        wait_for_all: bool = True,
    ) -> Dict[str, Any]:
        """Execute multiple functions in parallel (concurrently).
        All functions run simultaneously, results are merged.

        Args:
            functions: Functions to execute concurrently.
            wait_for_all: True = wait for all to complete, False = return on first completion.
        """
        return {
            "type": "Parallel",
            "functions": functions,
            "wait_for_all": wait_for_all,
        }

    @staticmethod
    def sleep(duration_ms: Union[str, int]) -> Dict[str, Any]:
        """Sleep/delay execution for rate limiting or timing control.

        Args:
            duration_ms: Duration in milliseconds: ``1000`` or ``"{{delay_param}}"``.
        """
        return {"type": "Sleep", "duration_ms": duration_ms}

    @staticmethod
    def return_response(
        fields: Dict[str, Any],
        status_code: Optional[int] = None,
    ) -> Dict[str, Any]:
        """Return a shaped response (final output formatting).
        Constructs the final response object from current execution context.

        Args:
            fields: Fields to include in response with ``{{param}}`` substitution.
            status_code: HTTP status code (default: 200).
        """
        stage: Dict[str, Any] = {"type": "Return", "fields": fields}
        if status_code is not None:
            stage["status_code"] = status_code
        return stage

    @staticmethod
    def validate(
        schema: Dict[str, Any],
        data_field: str,
        on_error: Optional[List[Dict[str, Any]]] = None,
    ) -> Dict[str, Any]:
        """Validate data against a JSON schema before processing.

        Args:
            schema: JSON Schema to validate against.
            data_field: Field containing data to validate.
            on_error: Functions to execute on validation failure.
        """
        stage: Dict[str, Any] = {
            "type": "Validate",
            "schema": schema,
            "data_field": data_field,
        }
        if on_error is not None:
            stage["on_error"] = on_error
        return stage


class ChatMessage:
    """Helper class for creating chat messages."""

    @staticmethod
    def system(content: str) -> Dict[str, str]:
        """Create a system message."""
        return {"role": "system", "content": content}

    @staticmethod
    def user(content: str) -> Dict[str, str]:
        """Create a user message."""
        return {"role": "user", "content": content}

    @staticmethod
    def assistant(content: str) -> Dict[str, str]:
        """Create an assistant message."""
        return {"role": "assistant", "content": content}
