
FROM rust:latest

COPY ./ ./

RUN cargo test
