FROM rust:1.91

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev libfuse-dev pkg-config clang cmake make \
    && rm -rf /var/lib/apt/lists/*

COPY . /src
WORKDIR /src

# REPOID is consumed by pbs-buildcfg/build.rs in place of `git rev-parse HEAD`.
ENV REPOID=a7eb4e8a1477df960570fa72e9e7e2b8ac58bdab

RUN cargo build --release
