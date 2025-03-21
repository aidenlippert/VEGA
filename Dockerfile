FROM rust:1.75 AS builder
WORKDIR /usr/src/vega
COPY . .
RUN cargo build --release --bin vega-node

FROM debian:buster-slim
COPY --from=builder /usr/src/vega/target/release/vega-node /usr/local/bin/
CMD ["vega-node"]
