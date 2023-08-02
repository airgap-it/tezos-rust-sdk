use chrono::NaiveDateTime;
use num_derive::{FromPrimitive, ToPrimitive};
use tezos_core::types::{
    encoded::{
        BlockHash, BlockPayloadHash, ContextHash, NonceHash, OperationListListHash, Signature,
    },
    hex_string::HexString,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub level: i32,
    pub proto: u8,
    pub predecessor: BlockHash,
    pub timestamp: NaiveDateTime,
    pub validation_pass: u8,
    pub operations_hash: OperationListListHash,
    pub fitness: Vec<HexString>,
    pub context: ContextHash,
    pub payload_hash: BlockPayloadHash,
    pub payload_round: i32,
    pub proof_of_work_nonce: HexString,
    pub seed_nonce_hash: Option<NonceHash>,
    pub liquidity_baking_toggle_vote: LiquidityBakingToggleVote,
    pub signature: Signature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum LiquidityBakingToggleVote {
    On = 0,
    Off = 1,
    Pass = 2,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        internal::coder::operation_content_bytes_coder::OperationContentBytesCoder, Result,
    };
    use chrono::DateTime;
    use tezos_core::internal::coder::Encoder;
    use tezos_core::internal::crypto::Crypto;
    use tezos_core::types::encoded::Encoded;

    #[test]
    fn test_forge_block_header() -> Result<()> {
        // https://rpc.tzkt.io/mainnet/chains/main/blocks/3104767/header
        let header = BlockHeader {
            level: 3104767,
            proto: 15,
            predecessor: "BLASooYKQfVBxAMCeGNeuPBiAmqFuSSJ9dumxPQAdtuUeyVxAfY".try_into()?,
            timestamp: DateTime::parse_from_rfc3339("2023-01-31T09:07:59Z").unwrap().naive_utc(),
            validation_pass: 4,
            operations_hash: "LLoZVrv3iM3VLVsDhwDcFYoX5hB3HkaZNErWeojWN6k4dC58fEmnw".try_into()?,
            fitness: vec![
                "02".try_into()?,
                "002f5fff".try_into()?,
                "".try_into()?,
                "ffffffff".try_into()?,
                "00000000".try_into()?
            ],
            context: "CoUqCtZTUuzbDz5rKdnp3rcdNyFUWehzPnF9xiTj1CeshUroG8NX".try_into()?,
            payload_hash: "vh24mP1nXosJ66NDR3VbGrLVBRjUCLAtkX4BjMp1VtXveKxf5XSQ".try_into()?,
            payload_round: 0,
            proof_of_work_nonce: "763259c58b970300".try_into()?,
            liquidity_baking_toggle_vote: LiquidityBakingToggleVote::On,
            signature: "sigka6VxFLGg5pNDYQ1ocPbKpngZ2gXVE97q8gmoNzwEsT7ejYdLTMouJtYKdiaFJXdqDhkWcPgwPzjuA6HbbgPe9bqeSfQn".try_into()?,
            seed_nonce_hash: None
        };

        let payload = OperationContentBytesCoder::encode(&header)?;
        let hash = Crypto::new(None, None, None).blake2b(payload.as_slice(), 32)?;

        let actual = BlockHash::from_bytes(&hash)?;
        let expected: BlockHash =
            "BLh9tisRvpgbQbwJXhenLEPG14mehnvyebni2mzuEMjTq9hvQME".try_into()?;
        assert_eq!(expected, actual);

        Ok(())
    }
}
