FROM rust:slim AS builder

WORKDIR /usr/src/pg-amqp-bridge
COPY . .
RUN cargo install --path .


FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/pg_amqp_bridge /usr/local/bin/pg_amqp_bridge

CMD ["pg_amqp_bridge"]
