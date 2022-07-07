use serde::{Deserialize, Serialize};

/// If `Unsynced`, the node is not currently synchronized with of its
/// peers (it is probably still bootstrapping and its head is lagging
/// behind the chain's).
///
/// If `Synced`, the node considers itself synchronized with its peers and
/// the current head timestamp is recent.
///
/// If `Stuck`, the node considers itself synchronized with its peers but
/// the chain seems to be halted from its viewpoint.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChainStatus {
    Stuck,
    Synced,
    Unsynced,
}

#[derive(Serialize, Deserialize)]
pub struct BootstrappedStatus {
    pub bootstrapped: bool,
    pub sync_state: ChainStatus,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Error;

    #[test]
    fn test_serde_serialize() -> Result<(), Error> {
        let status = BootstrappedStatus {
            bootstrapped: false,
            sync_state: ChainStatus::Stuck,
        };
        let json = serde_json::to_string(&status)?;

        assert_eq!(json, "{\"bootstrapped\":false,\"sync_state\":\"stuck\"}");

        Ok(())
    }

    #[test]
    fn test_serde_deserialize() -> Result<(), crate::error::Error> {
        let json = "{\"bootstrapped\":false,\"sync_state\":\"stuck\"}";

        let status: BootstrappedStatus = serde_json::from_str(&json)?;

        assert_eq!(status.bootstrapped, false);
        assert_eq!(status.sync_state, ChainStatus::Stuck);

        Ok(())
    }
}
