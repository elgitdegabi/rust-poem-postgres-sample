# build
# based on rust official image https://hub.docker.com/_/rust
# IMPORTANT: you should set / export DATABASE_URL environment variable
FROM rust:alpine AS builder
RUN apk add musl-dev gcc mariadb-dev

RUN pwd
RUN mkdir -p /log/config/
ADD logging_config.yaml /log/config/logging_config.yaml

RUN pwd
WORKDIR /src
COPY . .

RUN cargo build --release

# run app
FROM alpine
WORKDIR /app
COPY --from=builder /log/config/logging_config.yaml /log/config/logging_config.yaml
COPY --from=builder /src/target/release/rust-poem-postgres-api-sample .
ENTRYPOINT [ "./rust-poem-postgres-api-sample" ]
