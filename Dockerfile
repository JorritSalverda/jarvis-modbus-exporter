# Step 1: Build the application
FROM rust:1.51  as builder

WORKDIR app
RUN apt-get update && apt-get install -y --no-install-recommends musl-tools
RUN rustup target add x86_64-unknown-linux-musl

COPY . .
RUN cargo check
RUN cargo test
RUN cargo build --release --target x86_64-unknown-linux-musl

# Step 2: Create the runtime container image
FROM scratch AS runtime

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/jarvis-modbus-exporter .

USER 1000

ENTRYPOINT ["./jarvis-modbus-exporter"]