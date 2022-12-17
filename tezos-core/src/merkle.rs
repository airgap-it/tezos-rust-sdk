use crate::types::encoded::{
    OperationHash,
    OperationListHash,
    OperationListListHash,
    BlockHash,
    BlockPayloadHash,
    Encoded
};
use crate::internal::crypto::blake2b;
use crate::{Result, Error};

const DIGEST_SIZE: usize = 32;

pub fn hash_digest(message: &[u8]) -> [u8; DIGEST_SIZE] {
    let digest = blake2b(message, DIGEST_SIZE).unwrap();
    debug_assert_eq!(DIGEST_SIZE, digest.len());
    let mut result = [0; DIGEST_SIZE];
    result.copy_from_slice(&digest.as_slice());
    result
}

pub fn get_root_hash(inputs: Vec<Vec<u8>>) -> [u8; DIGEST_SIZE] {
    match inputs.len() {
        0 => hash_digest(&[]),
        1 => hash_digest(&inputs[0]),
        _ => {
            let mut leaves: Vec<[u8; DIGEST_SIZE]> = inputs
                .into_iter()
                .map(|x| hash_digest(x.as_slice()))
                .collect();
            leaves.push(leaves.last().expect("Expected at least 2 leaves").clone());

            fn step(n: usize, leaves: &mut Vec<[u8; DIGEST_SIZE]>) -> [u8; DIGEST_SIZE] {
                let m = (n + 1) / 2;
                for i in 0..m {
                    leaves[i] = hash_digest([leaves[2 * i], leaves[2 * i + 1]].concat().as_slice())
                }
                leaves[m] = hash_digest([leaves[n], leaves[n]].concat().as_slice());
                match m {
                    1 => leaves[0],
                    _ if m % 2 == 0 => step(m, leaves),
                    _ => {
                        leaves[m + 1] = leaves[m];
                        step(m + 1, leaves)
                    }                    
                }
            }

            step(leaves.len() - 1, &mut leaves)
        }
    }
}

fn calc_list_hash<F, T>(hashes: Vec<F>) -> Result<T> where T: Encoded, F: Encoded {
    let inputs = hashes
        .iter()
        .map(|x| x.to_bytes().unwrap())
        .collect();
    let root_hash = get_root_hash(inputs);
    T::from_bytes(&root_hash)
}

impl TryFrom<Vec<OperationHash>> for OperationListHash {
    type Error = Error;

    fn try_from(hashes: Vec<OperationHash>) -> Result<Self> {
        calc_list_hash(hashes)
    }
}

impl TryFrom<Vec<OperationListHash>> for OperationListListHash {
    type Error = Error;

    fn try_from(hashes: Vec<OperationListHash>) -> Result<Self> {
        calc_list_hash(hashes)
    }
}

impl TryFrom<Vec<Vec<OperationHash>>> for OperationListListHash {
    type Error = Error;

    fn try_from(hashes: Vec<Vec<OperationHash>>) -> Result<Self> {
        calc_list_hash(
            hashes
                .into_iter()
                .map(|x| OperationListHash::try_from(x).unwrap())
                .collect()
        )
    }
}

impl BlockPayloadHash {
    /// Calculates block payload hash given:
    ///     - predecessor [BlockHash]
    ///     - payload round (typically 0)
    ///     - flat list of non-consensus (validation pass > 0) [OperationHash]
    /// 
    /// For each level, Tenderbake proceeds in rounds. Each round represents an attempt by the validators to agree on the
    /// content of the block for the current level, that is, on the sequence of non-consensus operations the block contains.
    /// We call this sequence the block’s payload.
    /// 
    /// Reference:
    /// https://gitlab.com/tezos/tezos/-/blob/master/src/proto_012_Psithaca/lib_protocol/block_payload_repr.ml#L41
    /// 
    pub fn from_parts(predecessor: BlockHash, round: i32, hashes: Vec<OperationHash>) -> Result<Self> {
        let payload = [
            predecessor.to_bytes()?,
            round.to_be_bytes().to_vec(),
            OperationListHash::try_from(hashes)?.to_bytes()?
        ].concat();
        let digest = hash_digest(payload.as_slice());
        Self::from_bytes(&digest)
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::Result;

    use super::*;

    #[test]
    fn test_operation_list_list_hash() -> Result<()> {
        // https://rpc.tzkt.io/mainnet/chains/main/blocks/2223648
        let inputs = vec![
            vec![
                "oofVGFJuMPd6XpSvXKRVU6AuE6DtAbBibyfUzXdqfrCDTPuraJQ",
                "oo8iiB6A3WJBzEwc74jMRtb9gyzbEenKuLLwUFmQ4xS98HyFJTu",
                "ooGn6pyqjMSmjSGRdm2fS8SAzzVh1JEz3qDiviLdHAcKViXWycB",
                "opFfqxztw69M4m7MPqD63sfZdwF74A8ThDricTQKgiTbNd4ydwc",
                "op5RZXfy4duRbw69qYW99YJvfMCDjyvuDs4s33qjYXBgxmCRp4f",
                "ooyDEUZAFxEpd2HY9VNb94ahj2HYsLUHCcNYSqKepiP3bw6RpnK",
                "opMe31pUFbaEtD5KJask8Uc8yUZf6anhvNdMG6FYuxmXrnAWSnx",
                "opF1AskkJxmA8yZ9cSTFFPBSEr4DGS7QTrRxFQtDTyQovqLt1yF",
                "ooA93x3EopoPQXfmqZEMw4SV5rHiLAkLUKWmer2t3bSSKtJMkrj",
                "ooa9bxcu3KVkz8VL7E3L8iKbacKLFZDDm16QX5q78w6TWqWV1rA",
                "oowjubMor6aVxSALd7cuEQNBmKw44CeqNYqjuQdfQ25conXcddB",
                "opRiXgV2AHhVKvs3rDG9G4rcPUBDJZxAHz3knbgHWkhxzxunxCV",
                "ooay46SGffH5uDrwTzZncG9ddtCRXWsDTsb5vB9AXZoqkPe1LwJ",
                "ooVWhZr6MX9mNwMzdDPpf5ymKEvzrCR5aVRVqAk1XDhFSHAWAGG",
                "op116mqfnbSFjutJYKgTutrqTrjDzLTL34RaJ2gfQF6z92B1GEd",
                "ooJNd7bFHgMxuH2yNJZk5UWAPZQE4Q9rBYnCWg4j2Ew1uBx47Hj",
                "ooxFdyNjjGW3179jxQPjQupFmgAaaYhy5pNFxK2tAwNJoh8ESTD",
                "ooDPfEsSiicmxWBWZ8r9ZJL2YgFYiE4pFFtJzLtyrF8NCjsQ1xA",
                "opVY4fhUQHSRDMDbfNzEn298cZu9ebb3pT617gDi9KyLcmTg6ET",
                "ooRQEKVpWAApe6nycqTSu3KUy3ytZvDzfwbWAckqzHaHa663iak",
                "ooHrrz1nuhQnpztLJs89XCAGKTeYYZp7LoumZhbqTrqdxNTuJSX",
                "onoxNGDu2NBnJxZVMoUhtSqhjsWbEBPaYFneEGTv3DowzDNnxzf",
                "ooziT9HehywMZy7kwEVHsyignQH5MfcdWofoCjBzuX56QnnkrLW",
                "oodvdNyGYuZvDetJTyrKei6V1VtwUt74bdKrbjAqEyCJ1T5WH6t",
                "opW8MQ94tjkezHMsJ2GSbhLkYrcnVQHB5bPpYouWH2GJcBtbV89",
                "oo3TMGKAC67bw47dwtKLfPiNtYP9MPAnnuLKbRQNphE1Mtd9ScJ",
                "opMiDtu78kBSWnLBjJRqgH82gt5TNHejvpDXMhqxZ1K9bVwrETd",
                "ooDM5VKmKF5dLYVeqkhdSN7F3eB4vy3SZu3RWFEPzpyzfgrjrha",
                "opSiekJZ9T7ZErqxANG8dahR79xyXV1f4ikkunv6SaPcHqN2jrt",
                "opQHcRjqtJcBaqSUTtV5f1rtbwZNy3K8nypSwcVYHG5EyG6ifQD",
                "opHTzUG8nymqCxCh626JBbu8ihdHvapNBGVk4xchYFzMQJDJUq3",
                "opGrWMeR2brmv8ufXvXDw9sUX2nXRSDGYYwciPcRHHRiQvdDavp",
                "opDxagryp2fbytCz9zDPKXT9XSy19CZopK8wGsGXvE3Fftf2UiU",
                "oopYPg1PMyRi9ruAH2z1gcxDfMYNLbkSH1F2wb42zF4pMiJpeue",
                "ooTcfv286MjhBEUTHMtRUxQSdtaJ8jMmKqKCX8FVS14KhgShqXu",
                "ooHZMczKqhMUdgtwSc1xLgN2tQwmhQD3KNVfCdCRuavFJ8Fdmw3",
                "ooEqSoXmyuiXLExrs2yGKaokcP2H6353tyP3KmHdhavuDDnTqbB",
                "ooBWF9QmkBowBW84UwThBZ2LGWu9p6Yerfh6takdnUpFP7AqYrU",
                "oo8tq9D4Wg6zmL6E9UhhbBVgT13mpJSpf4gwVYVSbYHbV1uRy8N",
                "oo7ZrkX9ZJcYWC4NisVXHgbKK6ov1aXbQHHfLata27jWrdfJXFX",
                "onzq9eUPiXq35uXch5sa7T1dWaT1iZWjYiNePYVcsJS4WMQBo2h",
                "onuqm2tjCZWqthQBEi4Y1cVtLwYAr4YaJLU2aYLTcpNjnUgtLYg",
                "onumF5NSwJSFpp2zVyQKq6BWCwmfYnpLWzQEJjLb8bnSqETxwPQ",
                "onrnSo4UCu7KC93tmfWYeHvmr6xjAYWdUysxDuxd1CwT5bCg9Xg",
            ],
            vec![],
            vec![],
            vec![
                "ooUkXSA4JEBGj3vq8KdoaLXKrGXRDyJuQNjVVacLUMqWMGEzgoD",
                "onrERtizR9K9BZo5E3XdsJsXTragYfYaLBmMgZ3HrkaU2WpCts5",
                "onpxSxRpTygyLqrAA62PBzZKT14vnsb9GfUY19sNJ5PjruQMzJQ",
                "op7fZ26fv6bfPPNh51m4vuDJZqsCAh91bJ26huPmZdMugr7dynF",
                "ooZmDSaT8JGLdRNGJqdvE8FC37w4hHQmRD1U4ZbZaZ3hP7j522s",
                "ooA3srdnj1rsmQogHVyVKu6j2LaWb5dotCcaLW6vvMNLcZ8f88x",
                "oovnjhrHMeMBpkvTHxSNvapV7iQ6hFsSSL6LzmwkHCXatWNFF1q",
                "opDXgb3ZCM7eH4zkByLbQc3Y3HA7tqgbec2C9sEbjki4dWSkfW3",
                "opaBjse3Y3DzCKUS5bJGDs54HMBtvsKS6DEePWX2mU5keVy8YJk",
                "oobZRdMm1Ps42zmJRE5h4jaXoEiySrUgExSKe5VZT27w26EY5Zd",
                "ooXuUPBCVwgws7ETBxzYUMjKq3JMamKiiUkoDUHNPmntjzfkwKp",
                "opBawsSsFxMhR46whEhfNX558vCW4CWc5AddCuNtcyGd8dk9h6F",
                "ood6iz7LVikCx3T6rMviLSNz6oJUxbNGJKFVbHGqiBLN7r87zs8",
                "onq7Fe3NABrTkb2PZKBvrAefu5J6TxfhJNDCzAGpXc59bCKcEvB",
                "ooHggHarwD3kzuPzt3WWV4PaPjbzh18egp4NR3fJTYYt3vpiw5e",
                "op4bzVsUnWzsfR3tf7gwETmReqFjPwSbWrAtcJv7z6VaFQadax3",
                "opWc6Zop56wKAqxRR1phvDYqCRQJyZiF9agWnKJ4QsE2cVxjFNe",
                "oopugtVGMbreEiikGdUrBSkkbdPPuEANorEs89iptxPKiBq8SnV",
                "onksk5ESiNVvrvbXc6xPJhtRKbnXsNJRzteW73cJv7nMCG4kezX",
                "opWQr3ZpadegC5SUfUw8A8W4p3bQUSX4DS6mygY7FzG9xH5QDgT",
                "ooWbP6PQp1FreE3ikDfTvL9zXVA23A1BKT91RJYYkBEF9nHuYoQ",
                "onkWcVNEprcxu4dfXJvrPvWPixnTjXkErGA32XnonNVEriskXNC",
            ],
        ];
        let expected = OperationListListHash::new("LLoaQ8vjCwYooVy6nRSUP9VRBk1YHaHgqWrHw3J9j4VmEvVZCbaYB".into())?;

        let hashes: Vec<Vec<OperationHash>> = inputs
            .iter()
            .map(|hashes| hashes
                .iter()
                .map(|h| OperationHash::new(h.to_string()).unwrap())
                .collect())
            .collect();

        let actual = OperationListListHash::try_from(hashes)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_payload_hash_no_ops() -> Result<()> {
        // https://rpc.tzkt.io/ithacanet/chains/main/blocks/10000/header
        let predecessor = BlockHash::new("BLLiiqQeQ1N35S6VXeNcsyQPphoM17ZCXm9M6ek3xPYXj1tX2pE".into())?;
        let expected = BlockPayloadHash::new("vh3XZvx7wgTBp92mUVJ9jBNC1NQ79d6DBJm9pappux3exakXJaUU".into())?;
        
        let actual = BlockPayloadHash::from_parts(predecessor, 0, vec![])?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_payload_hash() -> Result<()> {
        // https://rpc.tzkt.io/ithacanet/chains/main/blocks/288671
        let predecessor = BlockHash::new("BL1whyhJA8fUF2ziNZj1MnHFQNLD6QTZTTHiG1oL8LSFwdJQ43z".into())?;
        let hashes = vec![OperationHash::new("ooa2pnEHguRveoV8WMYswpuSkyvxKTA9hyHDAsVgc9qnXtcDxd7".into())?];
        let expected = BlockPayloadHash::new("vh29w4KZGVb3A9QyjzDetftoWiCfvRugwAiaQ5Z3FFScy7QzjmH9".into())?;

        let actual = BlockPayloadHash::from_parts(predecessor, 0, hashes)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}