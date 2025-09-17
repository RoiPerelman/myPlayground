from __future__ import annotations

from pathlib import Path
import sys

from fastapi.testclient import TestClient
import pytest


# Ensure the project root (parent directory) is first on sys.path so that
# the local package is imported instead of any similarly named installed package.
PROJECT_ROOT = Path(__file__).resolve().parent.parent
if str(PROJECT_ROOT) not in sys.path:
    sys.path.insert(0, str(PROJECT_ROOT))


@pytest.fixture(scope="session")
def client() -> TestClient:  # type: ignore[override]
    # Import inside the fixture to ensure coverage can see executed lines
    # and that path adjustments above have taken effect.
    from rp_skeleton.server import create_app  # local import: ensures path adjustments applied

    app = create_app()
    return TestClient(app)
