FROM rust:1.79

COPY ./ ./

RUN cargo build --release

ENV RUST_LOG=debug

CMD ["./target/release/sphinx_drm"]