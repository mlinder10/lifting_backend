FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

ENV PORT = 8080

EXPOSE 8080

RUN cargo build --release

CMD ["./target/release/rocket"]
