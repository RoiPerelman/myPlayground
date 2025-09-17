"""Test server lifespan functionality."""

from __future__ import annotations

from unittest.mock import MagicMock, patch

from fastapi import FastAPI
from fastapi.testclient import TestClient

from rp_skeleton.server import create_app


def test_lifespan_startup_db_success():
    """Test lifespan startup when database ping succeeds."""
    with patch("rp_skeleton.server.ping_database") as mock_ping:
        mock_ping.return_value = True

        app = create_app()
        assert isinstance(app, FastAPI)

        # Test the app works
        with TestClient(app) as client:
            resp = client.get("/health")
            assert resp.status_code == 200


def test_lifespan_startup_db_failure():
    """Test lifespan startup when database ping fails (logs warning but continues)."""
    with (
        patch("rp_skeleton.server.ping_database") as mock_ping,
        patch("rp_skeleton.server.logging.getLogger") as mock_get_logger,
    ):
        mock_ping.return_value = False
        mock_logger = MagicMock()
        mock_get_logger.return_value = mock_logger

        app = create_app()
        assert isinstance(app, FastAPI)

        # Test the app still works even with DB warning
        with TestClient(app) as client:
            resp = client.get("/health")
            assert resp.status_code == 200

        # Verify warning was logged
        mock_logger.warning.assert_called_once()
