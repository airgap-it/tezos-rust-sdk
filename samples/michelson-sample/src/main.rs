use tezos_michelson::{
    micheline::Micheline,
    michelson::{
        data::{int, left, nat, pair, some, try_bytes, try_string, unit},
        types, Michelson,
    },
    Error,
};

fn main() -> Result<(), Error> {
    // int conversion from Michelson to Micheline and to its JSON representation.
    let int_michelson: Michelson = int(10i8);
    let int_micheline: Micheline = int_michelson.into();
    let int_json = serde_json::to_string_pretty(&int_micheline).map_err(|_| Error::Internal {
        description: "JSON serialization failed".into(),
    })?;
    println!("int JSON: {}", int_json);

    // string conversion from Michelson to Micheline and to its JSON representation.
    let string_michelson: Michelson = try_string("string")?;
    let string_micheline: Micheline = string_michelson.into();
    let string_json =
        serde_json::to_string_pretty(&string_micheline).map_err(|_| Error::Internal {
            description: "JSON serialization failed".into(),
        })?;
    println!("string JSON: {}", string_json);

    // bytes conversion from Michelson to Micheline and to its JSON representation.
    let bytes_michelson: Michelson = try_bytes("0a")?;
    let bytes_micheline: Micheline = bytes_michelson.into();
    let bytes_json =
        serde_json::to_string_pretty(&bytes_micheline).map_err(|_| Error::Internal {
            description: "JSON serialization failed".into(),
        })?;
    println!("bytes JSON: {}", bytes_json);

    // a more complex pair structure
    let pair_michelson: Michelson = types::pair(vec![
        types::option(types::nat::<types::Nat>().with_field_annotation("nat".into())),
        types::or(
            types::unit(),
            types::map(
                types::string(),
                types::pair(vec![
                    types::bytes::<types::Bytes>().with_field_annotation("bytes".into()),
                    types::address::<types::Address>().with_field_annotation("address".into()),
                ]),
            ),
        ),
    ]);
    let pair_micheline: Micheline = pair_michelson.into();
    let pair_json = serde_json::to_string_pretty(&pair_micheline).map_err(|_| Error::Internal {
        description: "JSON serialization failed".into(),
    })?;
    println!("pair JSON: {}", pair_json);

    // create a value for the type schema above and pack it to bytes
    let pair_value_michelson: Michelson = pair(vec![some(nat(10u8)), left(unit())]);
    let pair_value_micheline: Micheline = pair_value_michelson.into();
    let pair_value_json =
        serde_json::to_string_pretty(&pair_value_micheline).map_err(|_| Error::Internal {
            description: "JSON serialization failed".into(),
        })?;
    println!("pair value JSON: {}", pair_value_json);
    let pair_value_packed = pair_value_micheline.pack(Some(&pair_micheline))?;

    println!("pair value packed: {}", hex::encode(pair_value_packed));

    Ok(())
}
