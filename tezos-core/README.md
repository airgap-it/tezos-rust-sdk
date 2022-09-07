# Tezos Rust SDK: Core

`tezos-core` provides base Tezos types and actions that can be performed on them.

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

To generate and open the Rust documentation for this crate, use the following command at the root of the crate:

```shell
crago doc --open
```

## Setup

Add the following dependency to your Cargo manifest:

```toml
[dependencies]
tezos-core = { git = "https://github.com/airgap-it/tezos-rust-sdk.git", tag = "0.1.2" }
```

## Features

### ed25519

Enables the default implementation of the ed25519 curve crypto primitives. This allows crates like `tezos-operation` to sign operations with `edsk` private keys.

### secp256_k1

Enables the default implementation of the ed25519 curve crypto primitives. This allows crates like `tezos-operation` to sign operations with `spsk` private keys.

### p256

Enables the default implementation of the ed25519 curve crypto primitives. This allows crates like `tezos-operation` to sign operations with `p2sk` private keys.

### full_crypto

Enables `ed25519`, `secp256_k1` and `p256` at once.

### serde

Enables serialization and deserialization of the structures defined in the `types` module through the [serde](https://serde.rs/) library.
