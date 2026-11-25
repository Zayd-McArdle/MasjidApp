# Stage 1: Build
FROM rust:latest@sha256:e84068000361bf3d4a5ba2b7647f62893528b1eb9f847d7a23b493f5ce8ea254 AS builder

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
FROM debian:stable-slim@sha256:a64d9dbf633e2a2503afac76be521dc25d9b5765150b30d31a3914d4ae969cab
RUN apt-get update && apt-get install -y libssl3
COPY --from=builder /app/target/release/masjid_app_admin_manager_api /usr/local/bin/
EXPOSE 3000
CMD ["/usr/local/bin/masjid_app_admin_manager_api"]