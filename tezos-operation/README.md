# Tezos Rust SDK: Operation

`tezos-operation` provides Tezos Operation structures as defined in [the P2P message format](https://tezos.gitlab.io/shell/p2p_api.html) and actions that can be performed on them, e.g. `forge`/`unforge` and `sign`/`verify`.

## Setup

Add the following dependency to your Cargo manifest:

```toml
[dependencies]
tezos-operation = { git = "https://github.com/airgap-it/tezos-rust-sdk.git", tag = "0.1.1" }
```

## Features

### ed25519

Enables to sign and verify Tezos operations with `edsk` and `edpk` keys.

### secp256_k1

Enables to sign and verify Tezos operations with `spsk` and `sppk` keys.

### p256

Enables to sign and verify Tezos operations with `p2sk` and `p2pk` keys.

### full_crypto

Enables `ed25519`, `secp256_k1` and `p256` at once.
