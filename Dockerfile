FROM rust:1.48 as builder
WORKDIR /application

# 1 - Install Rust's static linking dependencies to permit the smallest possible binary
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

# 2 - Install dependencies and compile the binary
RUN USER=root cargo new fib_chal
WORKDIR /application/fib_chal
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# 3 - Copy the binary to an alpine image
FROM alpine
COPY --from=builder /usr/local/cargo/bin/fib_chal /usr/bin
CMD fib_chal
EXPOSE 8080
