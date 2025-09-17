"""SQLAlchemy engine singleton and session helpers.

Loads configuration from environment variables or an optional .env file
placed at project root. Uses psycopg (psycopg3) driver.

Environment variables supported:
  DATABASE_URL             Full SQLAlchemy URL. If set, takes precedence.
  DB_USER (default: postgres)
  DB_PASSWORD (default: test123)
  DB_HOST (default: db)
  DB_PORT (default: 5432)
  DB_NAME (default: postgres)
  DB_ECHO (default: false)  If 'true'/'1', enables SQL echo logging

Example URL form produced if DATABASE_URL not provided:
  postgresql+psycopg://user:password@host:port/dbname
"""

from __future__ import annotations

import os
import threading
from typing import Generator, Optional

from sqlalchemy import create_engine, text
from sqlalchemy.engine import Engine
from sqlalchemy.orm import Session, sessionmaker


# Lazy optional .env loading without extra dependency; simple parser.
# (If python-dotenv is desired later, can replace this.)
_env_loaded = False
_ENV_FILE_CANDIDATES = (".env",)  # Could extend with .env.local etc.

_engine_lock = threading.Lock()
_engine_singleton: Optional[Engine] = None
_SessionFactory: Optional[sessionmaker[Session]] = None


def _load_env_once() -> None:
    global _env_loaded
    if _env_loaded:
        return
    for fname in _ENV_FILE_CANDIDATES:
        if os.path.exists(fname):
            try:
                with open(fname, "r", encoding="utf-8") as fh:
                    for line in fh:
                        line = line.strip()
                        if not line or line.startswith("#"):
                            continue
                        if "=" not in line:
                            continue
                        key, value = line.split("=", 1)
                        key = key.strip()
                        value = value.strip().strip('"').strip("'")
                        # Do not overwrite existing env
                        os.environ.setdefault(key, value)
            except OSError:
                pass
    _env_loaded = True


def _build_url() -> str:
    _load_env_once()
    if url := os.environ.get("DATABASE_URL"):
        return url
    user = os.environ.get("DB_USER", "postgres")
    password = os.environ.get("DB_PASSWORD", "test123")
    host = os.environ.get("DB_HOST", "db")
    port = os.environ.get("DB_PORT", "5432")
    name = os.environ.get("DB_NAME", "postgres")
    return f"postgresql+psycopg://{user}:{password}@{host}:{port}/{name}"


def get_engine() -> Engine:
    """Return a process-wide singleton SQLAlchemy Engine.

    Thread-safe double-checked locking to avoid race in highly concurrent
    startup scenarios (rare for ASGI but inexpensive here).
    """
    global _engine_singleton, _SessionFactory
    if _engine_singleton is None:
        with _engine_lock:
            if _engine_singleton is None:  # double-checked
                url = _build_url()
                echo_flag = os.environ.get("DB_ECHO", "false").lower() in {"1", "true", "yes", "on"}
                engine = create_engine(url, echo=echo_flag, future=True, pool_pre_ping=True)
                _engine_singleton = engine
                _SessionFactory = sessionmaker(bind=engine, class_=Session, expire_on_commit=False, autoflush=False)
    return _engine_singleton  # type: ignore[return-value]


def get_session() -> Generator[Session, None, None]:
    """FastAPI-style dependency that yields a database session.

    Usage:
        from fastapi import Depends
        from rp_skeleton.database import get_session

        @router.get("/items")
        def list_items(db: Session = Depends(get_session)):
            ...
    """
    get_engine()  # ensure engine/session factory initialized
    if _SessionFactory is None:  # pragma: no cover - defensive
        raise RuntimeError("Session factory not initialized; engine not created")
    session: Session = _SessionFactory()  # type: ignore[call-arg]
    try:
        yield session
    finally:
        session.close()


def ping_database(timeout_seconds: float = 2.0) -> bool:
    """Attempt a lightweight connectivity test (SELECT 1)."""
    engine = get_engine()
    try:
        with engine.connect() as conn:
            conn.execute(text("SELECT 1"))
        return True
    except Exception:  # pragma: no cover - network/driver errors environment-specific
        return False
