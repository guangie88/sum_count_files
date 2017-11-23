# compilation + test image
FROM clux/muslrust:1.23.0-nightly-2017-11-21

WORKDIR /app/src
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked -v
COPY . ./
RUN cargo build --features clippy --frozen -v
