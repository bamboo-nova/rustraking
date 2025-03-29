FROM python:3.11-slim AS base

# The Stage to install rust.
FROM base AS rust-install
RUN apt-get update && \
    apt-get install -y curl \
    build-essential g++ cmake pkg-config libssl-dev libopencv-dev && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    . "$HOME/.cargo/env" && \
    rustc --version && cargo --version \
    apt-get clean

# Final stage.
FROM rust-install AS final

# Set path.
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

# Copy only Cargo.toml and Cargo.lock first, then build the dependent libraries (to make use of Docker's cache)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source codes.
COPY . .

# Install python library.
RUN pip install --upgrade pip

RUN pip install poetry
COPY . .
RUN poetry config virtualenvs.create false \
    && poetry install --without ml --no-interaction --no-ansi

RUN rm pyproject.toml poetry.lock

# Build rust codes.
RUN cargo build --release
