FROM rust:latest AS build

WORKDIR /build

COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && touch ./src/lib.rs
RUN cargo fetch --target x86_64-unknown-linux-gnu
RUN rm -rf ./src

COPY ./ ./
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM gcr.io/distroless/cc:nonroot

WORKDIR /app
COPY --from=build /build/target/x86_64-unknown-linux-gnu/release/compute-waster /app/compute-waster

CMD ["/app/compute-waster"]
