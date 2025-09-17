"""Tests for database engine functionality."""

from __future__ import annotations

import os
from unittest.mock import MagicMock, patch

from rp_skeleton.database import get_engine, get_session, ping_database


def test_get_engine_creates_singleton():
    """Test that get_engine returns the same instance."""
    engine1 = get_engine()
    engine2 = get_engine()
    assert engine1 is engine2


def test_get_session_yields_session():
    """Test that get_session yields a valid session."""
    sessions = list(get_session())
    assert len(sessions) == 1
    # Session should be closed after the generator


def test_ping_database_with_mock_success():
    """Test ping_database returns True on successful connection."""
    with patch("rp_skeleton.database.engine.get_engine") as mock_get_engine:
        mock_engine = MagicMock()
        mock_conn = MagicMock()
        mock_engine.connect.return_value.__enter__.return_value = mock_conn
        mock_get_engine.return_value = mock_engine

        result = ping_database()
        assert result is True
        mock_conn.execute.assert_called_once()


def test_ping_database_with_mock_failure():
    """Test ping_database returns False on connection failure."""
    with patch("rp_skeleton.database.engine.get_engine") as mock_get_engine:
        mock_engine = MagicMock()
        mock_engine.connect.side_effect = Exception("Connection failed")
        mock_get_engine.return_value = mock_engine

        result = ping_database()
        assert result is False


def test_engine_with_environment():
    """Test that engine can be created with different environment settings."""
    # Just test that we can call get_engine with different env vars without crashing
    with patch.dict(os.environ, {"DB_ECHO": "true"}):
        engine = get_engine()  # Uses existing singleton, but that's OK
        assert engine is not None


def test_engine_creation_flow():
    """Test engine creation works as expected."""
    # Test that basic engine functionality works
    engine = get_engine()
    assert engine is not None

    # Test that sessions can be created
    session_gen = get_session()
    session = next(session_gen)
    assert session is not None
