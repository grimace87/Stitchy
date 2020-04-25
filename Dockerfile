
FROM rust:1.23

COPY ./ ./

RUN cargo test
