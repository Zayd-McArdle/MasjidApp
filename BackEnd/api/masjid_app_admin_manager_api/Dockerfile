# Stage 1: Build
FROM rust:latest@sha256:931c2c0531a8b143e044f0f3914dfe802f15bb3d1955f0b657e9359327ac2ffc AS builder

# 1. First copy the library
WORKDIR /lib
COPY masjid_app_api_library/Cargo.toml .
COPY masjid_app_api_library/Cargo.lock .
COPY masjid_app_api_library/src ./src

# 2. Then copy the main app
WORKDIR /app
COPY masjid_app_admin_manager_api/Cargo.toml .
COPY masjid_app_admin_manager_api/Cargo.lock .
COPY masjid_app_admin_manager_api/src ./src

# 3. Update the library path in Cargo.toml
RUN sed -i 's|path = ".*masjid_app_api_library"|path = "/lib"|' Cargo.toml

# 4. Build the release
RUN cargo build --release

# Verify the binary exists
RUN ls -la /app/target/release/

# Stage 2: Runtime
FROM debian:stable-slim@sha256:a717a33469731894e77e912ea7d714e27c2dc38ee6554fd1a3102e92999cfc66
RUN apt-get update && apt-get install -y libssl3
COPY --from=builder /app/target/release/masjid_app_admin_manager_api /usr/local/bin/
EXPOSE 3000
CMD ["/usr/local/bin/masjid_app_admin_manager_api"]