from importlib import reload

from rp_skeleton import server


def test_get_app_singleton() -> None:
    first = server.get_app()
    second = server.get_app()
    assert first is second


def test_main_import() -> None:
    # Importing main should obtain an app instance
    import rp_skeleton.main as main  # noqa: WPS433

    assert hasattr(main, "app")
    assert main.app is server.get_app()

    # Reload server module to ensure singleton path exercised safely
    reload(server)
    new_app = server.get_app()
    assert new_app is server.get_app()
