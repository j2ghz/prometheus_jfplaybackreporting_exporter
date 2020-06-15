FROM rust:1.40 as builder
WORKDIR /usr/src/prometheus_jfplaybackreporting_exporter
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies
COPY --from=builder /usr/local/cargo/bin/prometheus_jfplaybackreporting_exporter /usr/local/bin/prometheus_jfplaybackreporting_exporter
CMD ["prometheus_jfplaybackreporting_exporter"]
