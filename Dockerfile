FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/frags /usr/local/bin/frags
CMD ["frags"]
EXPOSE 8080