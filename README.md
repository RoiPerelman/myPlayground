## Development

### Run the API locally

```bash
uvicorn rp_skeleton.main:app --reload
```

Visit `http://127.0.0.1:8000/health` for the health endpoint.

### Tooling

This project uses:

- `ruff` for formatting & linting
- `pyright` for type checking (strict mode)
- `pytest` for tests
- `tox` to orchestrate environments
- `pre-commit` to enforce quality on commits & pushes

### Install dev dependencies

If you're using `uv` (recommended):

```bash
uv sync --group dev
```

Or with plain pip (creates a venv first):

```bash
python -m venv .venv
source .venv/bin/activate
pip install -e .[dev]
```

### Pre-commit

Install hooks once:

```bash
pre-commit install --install-hooks
pre-commit install --hook-type pre-push
```

Run on all files manually:

```bash
pre-commit run --all-files
```

### Lint & format

```bash
ruff format .
ruff check .
```

### Type check

```bash
pyright
```

### Tests

```bash
pytest -q
```

### Tox

Run everything:

```bash
tox
```

Or a specific env:

```bash
tox -e lint
tox -e type
tox -e tests
```

### Production ASGI

In production deploy the ASGI app `rp_skeleton.main:app` using your process manager (e.g. uvicorn, gunicorn with uvicorn workers, hypercorn).

Example:

```bash
uvicorn rp_skeleton.main:app --host 0.0.0.0 --port 8000
```

### Containers (Dev & Prod)

This repo provides two Dockerfiles:

- `Dockerfile.dev`: Developer experience (hot reload, editable install, dev dependencies). Uses build arg `INSPEKTO_DEBIAN_BASE_IMAGE_TAG` (default example: `debian:bookworm-slim`).
- `Dockerfile` (multi-stage prod): Reproducible build using `uv.lock` in a builder stage (with `uv`), producing a wheel installed into a lean runtime. Uses build arg `INSPEKTO_AGENT_BASE_IMAGE_TAG`.

#### Build & run (development image)

```bash
docker build \
	--build-arg INSPEKTO_DEBIAN_BASE_IMAGE_TAG=debian:bookworm-slim \
	-f Dockerfile.dev -t myplayground-dev .

docker run --rm -p 8000:8000 myplayground-dev
```

Hot reload is enabled; bind-mount with compose (see below) for live code editing.

#### Build & run (production image)

```bash
docker build \
	--build-arg INSPEKTO_AGENT_BASE_IMAGE_TAG=debian:bookworm-slim \
	-f Dockerfile -t myplayground:latest .

docker run --rm -p 8000:8000 myplayground:latest
```

#### docker-compose

Simplest dev workflow:

```bash
docker compose up --build api-dev
```

Run prod variant locally:

```bash
docker compose up --build api
```

#### Notes on reproducibility

- Dependency versions are pinned via `uv.lock`.
- The production builder stage uses `uv sync --frozen` (without installing the project package) before building a wheel, ensuring deterministic resolution.
- The runtime stage installs only the wheel + its pinned transitive dependencies.
- Dev image installs with `--group dev` for tooling (ruff, pyright, pytest, etc.).

#### Updating dependencies

Modify `pyproject.toml`, then regenerate the lock:

```bash
uv lock
```

Rebuild the images to apply changes.

