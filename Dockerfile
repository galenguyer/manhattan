FROM rust:1.57.0 AS builder

WORKDIR /src/manhattan
COPY ./ ./
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /src/manhattan/target/release/manhattan /
ENTRYPOINT ["/manhattan"]
