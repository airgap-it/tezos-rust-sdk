# Tezos Rust SDK

A set of Rust libraries to interact with the Tezos blockchain.

## Use Cases

The Tezos Rust SDK ships with tools that can be used to:

*General*
- convert a Base58 encoded string to bytes and vice versa

*Michelson/Micheline*
- parse or create Micheline structures
- convert Micheline from/to JSON
- pack and unpack Micheline
- convert Micheline to typed Michelson and vice versa

*Operations*
- create an unsigned or signed Tezos operation
- forge and unforge an operation
- sign an operation and verify the signature

*RPC*
- interact with a Tezos node
- estimate the operation fee

*Contract*
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

For example, run the above command inside [tezos-core](tezos-core) to generate the documentation for the `tezos-core`
crate and open it in the browser.

## Samples

See the sample projects in [samples](samples) to see how the various crates can be used.

## Packages

Tezos Rust SDK is a multi-crate project. It has been designed to allow its users to use only the required minimum of functionality that meets their needs, thus optimizing the amount of redundant and unwanted code and dependencies.

| Package            | Description                                                          | Dependencies        |
|--------------------|----------------------------------------------------------------------|---------------------|
| `tezos-core`      | Provides base Tezos types and actions that can be performed on them. | ✖️                   |
| `tezos-michelson` | Provides [Michelson](https://tezos.gitlab.io/active/michelson.html) and [Micheline](https://tezos.gitlab.io/shell/micheline.html) types and actions, e.g. `pack`/`unpack`.                                    | `tezos-core`                                                                        |
| `tezos-operation` | Provides Tezos Operation structures as defined in [the P2P message format](https://tezos.gitlab.io/shell/p2p_api.html) and actions that can be performed on them, e.g. `forge`/`unforge` and `sign`/`verify`. | `tezos-core` <br /> `tezos-michelson`                                              |
| `tezos-rpc`       | Provides a Tezos RPC client which should be used to interact with Tezos nodes.                                                                                                                                | `tezos-core` <br /> `tezos-michelson` <br /> `tezos-operation`                    |
| `tezos-contract`  | Provides a Tezos contract handler which should be used to interact with Tezos contracts.                                                                                                                      | `tezos-core` <br /> `tezos-michelson` <br /> `tezos-operation` <br />`tezos-rpc` |
