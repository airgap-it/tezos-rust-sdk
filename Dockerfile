FROM rust:1.60.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/tezos-rust-sdk

ADD . ./

RUN rustup component add rustfmt
RUN cargo build --release
