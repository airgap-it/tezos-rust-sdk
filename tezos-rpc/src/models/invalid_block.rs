use serde::{Deserialize, Serialize};
use tezos_core::types::encoded::BlockHash;

use super::error::RpcError;

#[derive(Serialize, Deserialize)]
pub struct InvalidBlock {
    pub block: BlockHash,
    pub level: u64,
    pub errors: Vec<RpcError>,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_serde_serialize() -> Result<(), crate::error::Error> {
        let invalid_block = InvalidBlock {
            block: "BLsqrZ5VimZ5ZJf4s256PH9JP4GAsKnaLsb8BxTkZJN2ijq77KA"
                .try_into()
                .unwrap(),
            level: 1,
            errors: vec![],
        };
        let json = serde_json::to_string(&invalid_block)?;

        assert_eq!(json, "{\"block\":\"BLsqrZ5VimZ5ZJf4s256PH9JP4GAsKnaLsb8BxTkZJN2ijq77KA\",\"level\":1,\"errors\":[]}");

        Ok(())
    }

    #[test]
    fn test_serde_deserialize() -> Result<(), crate::error::Error> {
        let json = "{\"block\":\"BLsqrZ5VimZ5ZJf4s256PH9JP4GAsKnaLsb8BxTkZJN2ijq77KA\",\"level\":1,\"errors\":[]}";

        let invalid_block: InvalidBlock = serde_json::from_str(&json)?;

        assert_eq!(
            invalid_block.block,
            "BLsqrZ5VimZ5ZJf4s256PH9JP4GAsKnaLsb8BxTkZJN2ijq77KA"
                .try_into()
                .unwrap()
        );
        assert_eq!(invalid_block.level, 1);
        assert_eq!(invalid_block.errors, vec![]);

        Ok(())
    }
}
