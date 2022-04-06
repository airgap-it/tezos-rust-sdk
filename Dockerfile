FROM rust:1.59.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/tezosoxide

ADD . ./

RUN cargo build --release