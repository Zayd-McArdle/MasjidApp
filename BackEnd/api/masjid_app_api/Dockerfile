FROM rust:latest AS builder
WORKDIR /masjid_app_api
COPY Cargo.lock Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:stable-slim AS runner
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /masjid_app_api/target/release/masjid_app_api /usr/local/bin/masjid_app_api
EXPOSE 3000
CMD ["/usr/local/bin/masjid_app_api"]
