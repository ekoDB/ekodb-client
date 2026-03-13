"""
Tests for WebSocket and token management Python bindings.

These are import/API shape tests since actual WebSocket tests require a running server.
"""

import pytest
from ekodb_client import (
    Client,
    WebSocketClient,
    SubscriptionReceiver,
    ChatStreamReceiver,
)


class TestWebSocketImports:
    """Test that all WebSocket types are properly exported."""

    def test_websocket_client_exported(self):
        assert WebSocketClient is not None

    def test_subscription_receiver_exported(self):
        assert SubscriptionReceiver is not None

    def test_chat_stream_receiver_exported(self):
        assert ChatStreamReceiver is not None


class TestClientTokenMethods:
    """Test that token management methods exist on Client."""

    def test_client_has_refresh_token(self):
        """Client should have a refresh_token method."""
        assert hasattr(Client, "refresh_token")

    def test_client_has_clear_token_cache(self):
        """Client should have a clear_token_cache method."""
        assert hasattr(Client, "clear_token_cache")


class TestClientWebSocketMethod:
    """Test that WebSocket method exists on Client."""

    def test_client_has_websocket_method(self):
        """Client should have a websocket method."""
        assert hasattr(Client, "websocket")


class TestWebSocketClientMethods:
    """Test that all WebSocket methods exist."""

    def test_has_find_all(self):
        assert hasattr(WebSocketClient, "find_all")

    def test_has_subscribe(self):
        assert hasattr(WebSocketClient, "subscribe")

    def test_has_chat_send(self):
        assert hasattr(WebSocketClient, "chat_send")

    def test_has_register_client_tools(self):
        assert hasattr(WebSocketClient, "register_client_tools")

    def test_has_send_tool_result(self):
        assert hasattr(WebSocketClient, "send_tool_result")


class TestSubscriptionReceiverMethods:
    """Test SubscriptionReceiver API."""

    def test_has_recv(self):
        assert hasattr(SubscriptionReceiver, "recv")


class TestChatStreamReceiverMethods:
    """Test ChatStreamReceiver API."""

    def test_has_recv(self):
        assert hasattr(ChatStreamReceiver, "recv")
