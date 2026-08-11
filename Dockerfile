FROM rust:1.94-alpine AS builder

# openssl-libs-static: `native-tls` (used for the K8s exec WebSocket's TLS
# connection in server/src/ws.rs) links against OpenSSL, and this is a
# static-musl build - the plain `openssl` package only ships the shared libs.
# (libressl-dev conflicts with openssl-libs-static - Alpine can't have both
# static providers installed, and openssl is the one native-tls actually
# wants here.)
RUN apk update && apk add --no-cache build-base openssl openssl-dev openssl-libs-static musl-dev bash curl npm libc-dev binaryen
RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
RUN rustup target add wasm32-unknown-unknown

# The quench-* crates (and anything else from forge's private registry) come
# from `ennor`, which the base image has no reason to already know about -
# mirrors forge's own docker/Dockerfile.alpine build-arg setup exactly, since
# `anvil docker build` passes these same two build args regardless of which
# Dockerfile it's building.
ARG CARGO_REGISTRIES_ENNOR_INDEX=sparse+https://ennor.ddns.net/index/
ARG CARGO_REGISTRIES_ENNOR_TOKEN=
RUN mkdir -p /usr/local/cargo && \
    printf "[registries.ennor]\nindex = \"%s\"\n" "${CARGO_REGISTRIES_ENNOR_INDEX}" > /usr/local/cargo/config.toml && \
    if [ -n "${CARGO_REGISTRIES_ENNOR_TOKEN}" ]; then \
      printf "token = \"%s\"\n" "${CARGO_REGISTRIES_ENNOR_TOKEN}" >> /usr/local/cargo/config.toml; \
    fi

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

FROM rust:1.94-alpine AS runner

WORKDIR /app

COPY --from=builder /work/target/release/server /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
# quench_starter::actix::serve()'s own bind address env var, replacing
# leptos_axum's LEPTOS_SITE_ADDR. BASE_PATH and ALLOW_IN_MEMORY_DB are left
# unset here on purpose - the deployment manifest injects those at runtime,
# same as every other forge service (see homecloud's palantir overlay).
ENV SERVER_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

CMD ["/app/server"]
