# Tezos Rust SDK: Michelson

`tezos-michelson` provides [Michelson](https://tezos.gitlab.io/active/michelson.html) and [Micheline](https://tezos.gitlab.io/shell/micheline.html) types and actions, e.g. `pack`/`unpack`.

## Setup

Add the following dependency to your Cargo manifest:

```toml
[dependencies]
tezos-michelson = { git = "https://github.com/airgap-it/tezos-rust-sdk.git", tag = "0.1.0" }
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