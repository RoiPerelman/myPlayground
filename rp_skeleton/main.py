"""Application ASGI entry point helper (renamed package).

Allows running with: `python -m rp_skeleton.main` for local dev.
Prefer invoking via `uvicorn rp_skeleton.main:app --reload`.
"""

from fastapi import FastAPI

from rp_skeleton.server import get_app


app: FastAPI = get_app()

if __name__ == "__main__":  # pragma: no cover - manual run convenience
    import uvicorn

    uvicorn.run("rp_skeleton.main:app", host="0.0.0.0", port=8000, reload=True)
