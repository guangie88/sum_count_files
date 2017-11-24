# compilation + test image
FROM clux/muslrust:1.23.0-nightly-2017-11-21 as compiler

WORKDIR /app/src
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked -v
COPY . ./
RUN cargo build --release --features clippy --frozen -v

# runtime image
FROM alpine:3.6
WORKDIR /app
COPY --from=compiler \
    /app/src/target/x86_64-unknown-linux-musl/release/sum_count_files \
    ./