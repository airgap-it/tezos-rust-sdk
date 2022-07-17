use serde::{Deserialize, Serialize};
use tezos_core::types::encoded::BlockHash;

#[derive(Serialize, Deserialize)]
pub struct Checkpoint {
    /// A block identifier (Base58Check-encoded)
    pub block_hash: BlockHash,
    /// The block level
    pub level: u64,
}

#[cfg(test)]
mod test {
    use crate::error::Error;

    use super::*;

    #[test]
    fn test_serde_serialize() -> Result<(), Error> {
        let invalid_block = Checkpoint {
            block_hash: "BLsqrZ5VimZ5ZJf4s256PH9JP4GAsKnaLsb8BxTkZJN2ijq77KA"
                .try_into()
                .unwrap(),
            level: 1,
        };
        let json = serde_json::to_string(&invalid_block)?;

        assert_eq!(
            json,
            "{\"block_hash\":\"BLsqrZ5VimZ5ZJf4s256PH9JP4GAsKnaLsb8BxTkZJN2ijq77KA\",\"level\":1}"
        );

        Ok(())
    }

    #[test]
    fn test_serde_deserialize() -> Result<(), crate::error::Error> {
        let json =
            "{\"block_hash\":\"BLsqrZ5VimZ5ZJf4s256PH9JP4GAsKnaLsb8BxTkZJN2ijq77KA\",\"level\":1}";

        let invalid_block: Checkpoint = serde_json::from_str(&json)?;

        assert_eq!(
            invalid_block.block_hash,
            "BLsqrZ5VimZ5ZJf4s256PH9JP4GAsKnaLsb8BxTkZJN2ijq77KA"
                .try_into()
                .unwrap()
        );
        assert_eq!(invalid_block.level, 1);

        Ok(())
    }
}
