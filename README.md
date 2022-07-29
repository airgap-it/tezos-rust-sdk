# Tezos Rust SDK

A set of Rust libraries to interact with the Tezos blockchain.

## Packages

Tezos Rust SDK is a multi-crate project. It has been designed to allow its users to use only the required minimum of functionality that meets their needs, thus optimizing the amount of redundant and unwanted code and dependencies.

| Package            | Description                                                          | Dependencies        |
|--------------------|----------------------------------------------------------------------|---------------------|
| `tezos-core`      | Provides base Tezos types and actions that can be performed on them. | ✖️                   |
| `tezos-michelson` | Provides [Michelson](https://tezos.gitlab.io/active/michelson.html) and [Micheline](https://tezos.gitlab.io/shell/micheline.html) types and actions, e.g. `pack`/`unpack`.                                    | `tezos-core`                                                                        |
| `tezos-operation` | Provides Tezos Operation structures as defined in [the P2P message format](https://tezos.gitlab.io/shell/p2p_api.html) and actions that can be performed on them, e.g. `forge`/`unforge` and `sign`/`verify`. | `tezos-core` <br /> `tezos-michelson`                                              |
| `tezos-rpc`       | Provides a Tezos RPC client which should be used to interact with Tezos nodes.                                                                                                                                | `tezos-core` <br /> `tezos-michelson` <br /> `tezos-operation`                    |
| `tezos-contract`  | Provides a Tezos contract handler which should be used to interact with Tezos contracts.                                                                                                                      | `tezos-core` <br /> `tezos-michelson` <br /> `tezos-operation` <br />`tezos-rpc` |