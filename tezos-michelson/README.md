# Tezos Rust SDK: Michelson

`tezos-michelson` provides [Michelson](https://tezos.gitlab.io/active/michelson.html) and [Micheline](https://tezos.gitlab.io/shell/micheline.html) types and actions.

It allowes to:

- parse or create Micheline structures
- convert Micheline from/to JSON
- pack and unpack Micheline
- convert Micheline to typed Michelson and vice versa

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
tezos-michelson = { git = "https://github.com/airgap-it/tezos-rust-sdk.git", tag = "0.1.2" }
```

## Features

### serde

Enables serialization and deserialization of the `Michelson` and `Micheline` structures through the [serde](https://serde.rs/) library.

## Example

```rust
use tezos_michelson::michelson::{data, Michelson, types};

let michelson: Michelson = data::pair(vec![data::int(0), data::int(2)]);
let schema = types::pair(vec![types::nat(), types::nat()]);
let packed = michelson.pack(Some(&schema));
```