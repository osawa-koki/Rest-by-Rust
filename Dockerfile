FROM rust:1.66
WORKDIR /app
EXPOSE 8080
COPY . .
RUN cargo build --release --target-dir ./bin
CMD ["./bin/release/rest-by-rust"]
