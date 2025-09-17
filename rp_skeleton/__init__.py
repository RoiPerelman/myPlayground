"""Application package initialization.

Expose the FastAPI app via `from rp_skeleton import create_app`.
"""

from rp_skeleton.server import create_app, get_app


__all__ = ["create_app", "get_app"]
