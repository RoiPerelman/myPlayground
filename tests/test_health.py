from __future__ import annotations

from http import HTTPStatus
from typing import Any, Mapping

from fastapi.testclient import TestClient  # type: ignore[import-not-found]


def test_health_ok(client: TestClient) -> None:
    resp = client.get("/health")  # type: ignore[reportUnknownMemberType]
    assert resp.status_code == HTTPStatus.OK  # type: ignore[reportUnknownMemberType]
    raw: Any = resp.json()  # type: ignore[reportUnknownMemberType]
    assert isinstance(raw, Mapping)
    # Type ignore for pyright unknown types in test data
    data = {str(k): str(v) for k, v in raw.items()}  # type: ignore[reportUnknownArgumentType,reportUnknownVariableType]
    assert data["status"] == "ok"
    assert data["service"] == "rp_skeleton"
    assert "T" in data["timestamp"]  # basic isoformat structure
