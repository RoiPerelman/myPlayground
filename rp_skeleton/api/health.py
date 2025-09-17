"""Health check router (renamed package).

Provides a simple liveness/readiness style endpoint.
"""

from datetime import datetime, timezone

from fastapi import APIRouter
from pydantic import BaseModel


router = APIRouter(tags=["health"], prefix="")


class HealthResponse(BaseModel):
    status: str
    timestamp: str
    service: str


@router.get("/health", response_model=HealthResponse, summary="Basic health check")
async def health() -> HealthResponse:
    """Return service health metadata.

    Could be extended later (db connectivity, external services etc.).
    """
    return HealthResponse(
        status="ok",
        timestamp=datetime.now(timezone.utc).isoformat(),
        service="rp_skeleton",
    )
