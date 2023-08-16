FROM rustlang/rust:nightly-buster as builder
WORKDIR /usr/src/myapp
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src && echo "// Empty" > src/lib.rs && cargo build --release && rm -rf src/

COPY src/ src/
RUN cargo build --release

FROM alpine:latest
RUN apk update && apk add --no-interactive ca-certificates && rm -rf /var/lib/apt/lists/*
COPY .env /.env
COPY --from=builder /usr/src/myapp/target/release/discord-jira-integration /usr/local/bin/discord-jira-integration
EXPOSE 6969
CMD ["discord-jira-integration"]