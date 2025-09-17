"""FastAPI application factory and lifetime management (renamed package)."""

from contextlib import asynccontextmanager
import logging

from fastapi import FastAPI

from rp_skeleton.api import health
from rp_skeleton.database import ping_database


_app_singleton: FastAPI | None = None


def create_app() -> FastAPI:
    """Create and configure a new FastAPI application instance."""

    @asynccontextmanager
    async def lifespan(app: FastAPI):  # type: ignore[unused-ignore]
        # Startup
        if not ping_database():
            logging.getLogger(__name__).warning("Database not reachable at startup")
        yield
        # (Optional) add shutdown logic here later

    app = FastAPI(title="rp_skeleton API", version="0.1.0", lifespan=lifespan)

    # Routers
    app.include_router(health.router)
    return app


def get_app() -> FastAPI:
    """Return a singleton app instance (used by ASGI servers)."""
    global _app_singleton
    if _app_singleton is None:
        _app_singleton = create_app()
    return _app_singleton
