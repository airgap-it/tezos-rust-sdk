use std::convert::TryInto;

use tezos_core::types::{encoded::SecretKey, number::Nat};
use tezos_operation::operations::{Transaction, UnsignedOperation};
use tezos_rpc::{
    client::TezosRpc,
    models::{block::BlockId, operation::Operation},
};

use derive_more::{Display, Error as DError, From};

#[derive(DError, Display, Debug, From)]
pub enum Error {
    Core { source: tezos_core::Error },
    Operation { source: tezos_operation::Error },
    Rpc { source: tezos_rpc::Error },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // crate transaction and simulate the operation injection using the pre-apply rpc endpoint
    let rpc = TezosRpc::new("https://ghostnet.smartpy.io".into());

    let branch = rpc
        .get_block_hash()
        .block_id(&BlockId::Level(-2))
        .send()
        .await?;
    let counter: Nat = (rpc
        .get_contract_counter(&("tz1PwXjsrgYBi9wpe3tFhazJpt7JMTVzBp5c".try_into()?))
        .send()
        .await?
        + 1u8)
        .into();

    let unsigned_operation = UnsignedOperation::new(
        branch,
        vec![Transaction::new(
            "tz1PwXjsrgYBi9wpe3tFhazJpt7JMTVzBp5c".try_into()?,
            0u8.into(),
            counter,
            0u8.into(),
            0u8.into(),
            1000u16.into(),
            "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into()?,
            None,
        )
        .into()],
    );

    // estimate fee
    let unsigned_with_fee = rpc.min_fee(unsigned_operation, None).await?;

    // sign
    let secret_key: SecretKey = "edskSA787kkcN7ZF9imyuArKxSYApt6YmQ2oNwKKH9PxFAux8fzrFmE6tzBecnbTRCW1jqN7S8crRoSbrczRxy3TwnycPCJKNr".try_into()?;
    let signed_operation = unsigned_with_fee.into_signed_operation(&secret_key)?;
    let mut rpc_operation: Operation = signed_operation.into();
    rpc_operation.protocol = Some(rpc.get_block().send().await?.protocol);

    let preapply_result = rpc
        .preapply_operations(&vec![&rpc_operation])
        .send()
        .await?;

    println!("Preapply result: {:?}", preapply_result);

    Ok(())
}
