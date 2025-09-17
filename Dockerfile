############################################################
############################################################
# Production multi-stage Dockerfile (updated for rp_skeleton)
# (This file previously held dev content; ensure prod Dockerfile is used.)
############################################################
############################################################

ARG BASE_IMAGE_TAG
FROM ${BASE_IMAGE_TAG}

ENV DEBIAN_FRONTEND=noninteractive \
    PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    PIP_NO_CACHE_DIR=1 \
    LD_LIBRARY_PATH=/usr/local/lib

# System dependencies (adjust as needed for debugging / dev tooling)
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 python3-venv python3-pip \
    && rm -rf /var/lib/apt/lists/*

# one python to rule them all
RUN ln -sf /usr/bin/python3 /usr/bin/python

# create venv
RUN python3 -m venv /venv

# add venv to path to use venv
ENV PATH="/venv/bin:$PATH"

# Install uv (fast resolver) and upgrade pip (kept for tooling fallback)
RUN pip install --upgrade pip && pip install uv

# Copy lock + pyproject for dependency layer caching
COPY pyproject.toml uv.lock ./

# # Sync dev environment (includes tooling & runtime deps) then install project in editable mode
RUN bash -c 'source /venv/bin/activate && uv sync --extra dev --frozen --active'

# # Set application working directory (project root)
WORKDIR /venv/lib/python3.11/site-packages/rp_skeleton
