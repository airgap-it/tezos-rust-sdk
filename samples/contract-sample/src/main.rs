use std::convert::TryInto;

use derive_more::{Display, Error as DError, From};

use tezos_contract::ContractFetcher;
use tezos_michelson::michelson::{data, data::bytes, Michelson};
use tezos_rpc::client::TezosRpc;

#[derive(DError, Display, Debug, From)]
pub enum Error {
    Core { source: tezos_core::Error },
    Contract { source: tezos_contract::Error },
    Rpc { source: tezos_rpc::Error },
    Michelson { source: tezos_michelson::Error },
    Hex { source: hex::FromHexError },
    Utf8 { source: std::string::FromUtf8Error },
    Unknonw,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let rpc = TezosRpc::new("https://ghostnet.smartpy.io".into());
    let tezos_domains = rpc
        .contract_at("KT1GFYUFQRT4RsNbtG2NU23woUyMp5tx9gx2".try_into()?, None)
        .await?;

    // get value from records big map
    let records = tezos_domains
        .storage()
        .big_maps()
        .get_by_name("records")
        .unwrap();
    let record_micheline = records
        .get_value(bytes("lab-void.ith".as_bytes()), None)
        .await?;

    let record_michelson: Michelson = record_micheline.try_into()?;
    let record_pair: data::Pair = record_michelson.try_into()?;
    let record_owner_address: data::String = record_pair
        .flatten()
        .values
        .into_iter()
        .nth(5)
        .unwrap()
        .try_into()?;

    println!("Owner address: {:?}", record_owner_address.to_str());

    // get value from reverse_records big map
    let reverse_records = tezos_domains
        .storage()
        .big_maps()
        .get_by_name("reverse_records")
        .unwrap();
    let reverse_record = reverse_records
        .get_value(record_owner_address.into(), None)
        .await?;

    let reverse_record_michelson: Michelson = reverse_record.try_into()?;
    let reverse_record_pair: data::Pair = reverse_record_michelson.try_into()?;
    let reverse_record_pair: data::Pair = reverse_record_pair.flatten();
    let reverse_record_some_name: data::Some = reverse_record_pair
        .flatten()
        .values
        .into_iter()
        .nth(1)
        .unwrap()
        .try_into()?;
    let reverse_record_name: data::Bytes = (*reverse_record_some_name.value).try_into()?;
    let reverse_record_name_bytes: Vec<u8> =
        hex::decode(reverse_record_name.value()[2..].to_owned())?;
    let name = String::from_utf8(reverse_record_name_bytes)?;

    println!("Name: {}", name);

    Ok(())
}
