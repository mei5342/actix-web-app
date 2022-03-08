# FROM rust:1.56 as builder
# WORKDIR /usr/src/app
# COPY . .

# ENV USER=mei5342
# RUN cargo init .

# RUN cargo install --path .

# # FROM debian:buster-slim
# # RUN apt-get update && rm -rf /var/lib/apt/lists/*
# # COPY --from=builder /usr/local/cargo/bin/app .
# CMD ["./app"]

FROM rust:latest
WORKDIR /usr/src/

RUN cargo new app --bin
COPY . .
RUN cd /usr/src/app && \
    cargo build
WORKDIR /usr/src/app

EXPOSE 80

CMD ["cargo", "run"]