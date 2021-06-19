FROM rust:1.53 as builder
WORKDIR /usr/src/crash_reporter
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/mpf-crash-reporter /usr/local/bin/mpf-crash-reporter
CMD ["mpf-crash-reporter"]
