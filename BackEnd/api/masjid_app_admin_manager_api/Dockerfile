# Stage 1: Build
FROM rust:latest@sha256:09027f873d45b8ecd1dc86f1fa13b7a62b875c961f017a0e331267b350430137 AS builder

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
FROM debian:stable-slim@sha256:13e0a2e78aa0554ad5ff66e4e260c6dbcdff807eb1f39d43b9fb0288aae88b04
RUN apt-get update && apt-get install -y libssl3
COPY --from=builder /app/target/release/masjid_app_admin_manager_api /usr/local/bin/
EXPOSE 3000
CMD ["/usr/local/bin/masjid_app_admin_manager_api"]