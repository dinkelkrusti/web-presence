FROM rust:1.96-slim-bookworm AS build

RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

COPY ./event-service/src ./src
COPY ./event-service/Cargo.lock .
COPY ./event-service/Cargo.toml .

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    "event-service"

RUN cargo build --target x86_64-unknown-linux-musl --release


FROM rust:1.96-alpine3.21

COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /etc/group /etc/group

USER event-service:event-service

COPY --from=build --chown=event-service:event-service ./target/x86_64-unknown-linux-musl/release/event-service /app/event-service

EXPOSE 8080

ENTRYPOINT ["./app/event-service"]