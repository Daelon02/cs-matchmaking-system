# syntax = docker/dockerfile:experimental
ARG BASE_BUILD_IMAGE=rust:1.88.0
#ARG RUNNER_IMAGE=gcr.io/distroless/cc
ARG RUNNER_IMAGE=ubuntu:22.04

FROM $BASE_BUILD_IMAGE as base_builder
RUN cargo install cargo-chef --version 0.1.51
RUN apt-get update \
    && apt-get install -y --no-install-recommends cmake libssl-dev libsasl2-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

FROM base_builder as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base_builder as cacher
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=ssh,id=default cargo chef cook --target x86_64-unknown-linux-gnu --recipe-path recipe.json
#RUN cargo chef cook --release --target x86_64-unknown-linux-gnu --recipe-path recipe.json

FROM base_builder as builder
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN --mount=type=ssh,id=default cargo build --target x86_64-unknown-linux-gnu

FROM $RUNNER_IMAGE as base_runner
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
      libpq5 cmake pkg-config ca-certificates \
    && update-ca-certificates \
    && apt-get remove pkg-config ca-certificates -y \
    && rm -rf /var/lib/apt/lists/* \
WORKDIR /app

FROM base_runner as runner
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/debug/report-service /usr/local/bin/
#COPY config.yaml ./
ENTRYPOINT ["report-service"]
