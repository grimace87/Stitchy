
FROM rust:1.31

COPY ./ ./

RUN rustup self update
RUN rustup default stable
RUN cargo test
