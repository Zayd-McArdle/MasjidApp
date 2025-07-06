# Stage 1: Build
FROM rust:latest AS builder

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
FROM debian:stable-slim
RUN apt-get update && apt-get install -y libssl3
COPY --from=builder /app/target/release/masjid_app_public_api /usr/local/bin/
EXPOSE 3000
CMD ["/usr/local/bin/masjid_app_public_api"]