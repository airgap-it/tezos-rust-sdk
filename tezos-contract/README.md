# Tezos Rust SDK: Contract

`tezos-contract` provides the main entry point to interact with Tezos contracts.

It allows to:
- read a contract's storage
- read BigMap values
- prepare contract calls

## Requirements

Rust 1.60.0 or above.

Install the `rustc` compiler and the `cargo` command line tool through [rustup](https://rustup.rs).

## Build

```shell
cargo build --release
```

## Test

```shell
cargo test
```

## Rust Documentation

Use the following command from the root folder of one of the crates to generate and open the Rust documentation:

```shell
cargo doc --open
```

## Setup

Add the following dependency to your Cargo manifest:

```toml
[dependencies]
tezos-contract = { git = "https://github.com/airgap-it/tezos-rust-sdk.git", tag = "0.1.2" }
```
