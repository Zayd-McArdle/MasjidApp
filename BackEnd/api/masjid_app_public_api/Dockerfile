# Stage 1: Build
FROM rust:latest@sha256:5d05167b28cef0fa3a6c781cd77949386848191f3382e82cf53bd1277a47a98f AS builder

# 1. First copy the library
WORKDIR /lib
COPY masjid_app_api_library/Cargo.toml .
COPY masjid_app_api_library/Cargo.lock .
COPY masjid_app_api_library/src ./src

# 2. Then copy the main app
WORKDIR /app
COPY masjid_app_public_api/Cargo.toml .
COPY masjid_app_public_api/Cargo.lock .
COPY masjid_app_public_api/src ./src

# 3. Update the library path in Cargo.toml
RUN sed -i 's|path = ".*masjid_app_api_library"|path = "/lib"|' Cargo.toml

# 4. Build the release
RUN cargo build --release

# Verify the binary exists
RUN ls -la /app/target/release/

# Stage 2: Runtime
FROM debian:stable-slim@sha256:5bc3287b25407c965a30f38e32603dc253a3869e1b12a21ac09bfc27fd8b13ce
RUN apt-get update && apt-get install -y libssl3
COPY --from=builder /app/target/release/masjid_app_public_api /usr/local/bin/
EXPOSE 3000
CMD ["/usr/local/bin/masjid_app_public_api"]