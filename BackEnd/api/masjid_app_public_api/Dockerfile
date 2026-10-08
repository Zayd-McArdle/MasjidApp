# Stage 1: Build
FROM rust:latest@sha256:6ff07edce8775d0f64be7aba9197229407301bddf2054d62c27b541a6238a181 AS builder

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
FROM debian:stable-slim@sha256:eb593cf2c358cacef45ca0a424bbc7d30cfa3466265fc2662b9466a0ca6ba1c5
RUN apt-get update && apt-get install -y libssl3
COPY --from=builder /app/target/release/masjid_app_public_api /usr/local/bin/
EXPOSE 3000
CMD ["/usr/local/bin/masjid_app_public_api"]