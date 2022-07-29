# Tezos Rust SDK: Core

`tezos-core` provides base Tezos types and actions that can be performed on them.

## Setup

Add the following dependency to your Cargo manifest:

```toml
[dependencies]
tezos-core = "0.1"
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
