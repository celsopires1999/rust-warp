FROM rust:1.66.0

RUN useradd -m -u 1000 rust

USER rust

WORKDIR /home/rust/app

RUN rustup component add rustfmt &&\
    cargo install cargo-watch

EXPOSE 8888

CMD ["tail", "-f", "/dev/null"]
