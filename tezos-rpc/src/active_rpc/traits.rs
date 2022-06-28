use async_trait::async_trait;

/// Tezos protocol-dependent RPCs.
///
/// See [RPCs - Reference](https://tezos.gitlab.io/active/rpc.html) for more details.
#[async_trait]
pub trait ActiveRPC {}
