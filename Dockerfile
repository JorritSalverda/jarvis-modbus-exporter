FROM rust:1.51 as build

WORKDIR /usr/src

# Install musl-gcc
RUN apt-get update && apt-get install -y --no-install-recommends musl-tools

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

# Copy the source and build the application.
COPY . ./

RUN cargo test
RUN cargo build --release --target x86_64-unknown-linux-musl

# Copy the statically-linked binary into a scratch container.
FROM scratch

COPY --from=build /usr/src/target/x86_64-unknown-linux-musl/release/jarvis-modbus-exporter .

USER 1000

ENTRYPOINT ["./jarvis-modbus-exporter"]