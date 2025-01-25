FROM rust:1.82-alpine as builder

RUN apk update && apk add --no-cache build-base openssl libressl-dev musl-dev bash curl npm libc-dev binaryen
RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

FROM rust:1.82-alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/server /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

CMD ["/app/server"]
