FROM rust:1.78-slim

# Install common build dependencies and tools used in examples
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    pkg-config \
    libssl-dev \
    llvm \
    clang \
    git \
  && rm -rf /var/lib/apt/lists/*

# Ensure rustfmt and clippy are available
RUN rustup component add rustfmt clippy

ENV USER=vscode
ENV HOME=/home/vscode

RUN useradd -m ${USER}
WORKDIR /workspace

USER ${USER}
