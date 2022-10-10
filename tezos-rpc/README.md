# Tezos Rust SDK: RPC

`tezos-rpc` provides a Tezos RPC client which can be used to interact with Tezos nodes.

It allows to:
- interact with a Tezos node
- estimate the operation fee

## Requirements

Rust 1.60.0 or above.

Install the `rustc` compiler and the `cargo` command line tool through [rustup](https://rustup.rs).

## Build

```shell
cargo build --release
```

## Test

```shell
cargo test
```

## Rust Documentation

To generate and open the Rust documentation for this crate, use the following command at the root of the crate:

```shell
crago doc --open
```

## Setup

Add the following dependency to your Cargo manifest:

```toml
[dependencies]
tezos-rpc = { git = "https://github.com/airgap-it/tezos-rust-sdk.git", tag = "0.1.2" }
```

## Features

### http

Enables the default http provider. This features is enabled by default and uses the [reqwest](https://github.com/seanmonstar/reqwest) crate as the http client.
If you want to provide your own http client, disable the default features and provide an implementation of the `Http` trait.

## Shell RPC's

[rpc-openapi.json](https://gitlab.com/tezos/tezos/-/blob/master/docs/api/rpc-openapi.json)

| Path | Methods | Implemented |
|:-|:-|:-|
| `/chains/{chain_id}` | `patch` | :heavy_check_mark: |
| `/chains/{chain_id}/blocks` | `get` | :heavy_check_mark: |
| `/chains/{chain_id}/chain_id` | `get` | :heavy_check_mark: |
| `/chains/{chain_id}/checkpoint` | `get` | **`DEPRECATED`** |
| `/chains/{chain_id}/invalid_blocks` | `get` | :heavy_check_mark: |
| `/chains/{chain_id}/invalid_blocks/{block_hash}` | `get`, `delete` | :heavy_check_mark: |
| `/chains/{chain_id}/is_bootstrapped` | `get` | :heavy_check_mark: |
| `/chains/{chain_id}/levels/caboose` | `get` | :heavy_check_mark: |
| `/chains/{chain_id}/levels/checkpoint` | `get` | :heavy_check_mark: |
| `/chains/{chain_id}/levels/savepoint` | `get` | :heavy_check_mark: |
| `/config` | `get` | |
| `/config/history_mode` | `get` | |
| `/config/network/user_activated_protocol_overrides` | `get` | |
| `/config/network/user_activated_upgrades` | `get` | |
| `/errors` | `get` | |
| `/fetch_protocol/{Protocol_hash}` | `get` | |
| `/injection/block` | `post` | :heavy_check_mark: |
| `/injection/operation` | `post` | :heavy_check_mark: |
| `/injection/protocol` | `post` | |
| `/monitor/active_chains` | `get` | |
| `/monitor/bootstrapped` | `get` | |
| `/monitor/commit_hash` | `get` | **`DEPRECATED`** |
| `/monitor/heads/{chain_id}` | `get` | |
| `/monitor/protocols` | `get` | |
| `/monitor/valid_blocks` | `get` | |
| `/network/connections` | `get` | |
| `/network/connections/{peer_id}` | `get`, `delete` | |
| `/network/greylist` | `delete` | |
| `/network/greylist/clear` | `get` | **`DEPRECATED`** |
| `/network/greylist/ips` | `get` | |
| `/network/greylist/peers` | `get` | |
| `/network/log` | `get` | |
| `/network/peers` | `get` | |
| `/network/peers/{peer_id}` | `get`, `patch` | |
| `/network/peers/{peer_id}/ban` | `get` | **`DEPRECATED`** |
| `/network/peers/{peer_id}/banned` | `get` | |
| `/network/peers/{peer_id}/log` | `get` | |
| `/network/peers/{peer_id}/trust` | `get` | **`DEPRECATED`** |
| `/network/peers/{peer_id}/unban` | `get` | **`DEPRECATED`** |
| `/network/peers/{peer_id}/untrust` | `get` | **`DEPRECATED`** |
| `/network/points` | `get` | |
| `/network/points/{point}` | `get`, `put`, `patch` | |
| `/network/points/{point}/ban` | `get` | **`DEPRECATED`** |
| `/network/points/{point}/banned` | `get` | |
| `/network/points/{point}/log` | `get` | |
| `/network/points/{point}/trust` | `get` | **`DEPRECATED`** |
| `/network/points/{point}/unban` | `get` | **`DEPRECATED`** |
| `/network/points/{point}/untrust` | `get` | **`DEPRECATED`** |
| `/network/self` | `get` | |
| `/network/stat` | `get` | |
| `/network/version` | `get` | **`DEPRECATED`** |
| `/network/versions` | `get` | **`DEPRECATED`** |
| `/protocols` | `get` | |
| `/protocols/{Protocol_hash}` | `get` | |
| `/protocols/{Protocol_hash}/environment` | `get` | |
| `/stats/gc` | `get` | |
| `/stats/memory` | `get` | |
| `/version` | `get` | |
| `/workers/block_validator` | `get` | |
| `/workers/chain_validators` | `get` | |
| `/workers/chain_validators/{chain_id}` | `get` | |
| `/workers/chain_validators/{chain_id}/ddb` | `get` | |
| `/workers/chain_validators/{chain_id}/peers_validators` | `get` | |
| `/workers/chain_validators/{chain_id}/peers_validators/{peer_id}` | `get` | |
| `/workers/prevalidators` | `get` | |
| `/workers/prevalidators/{chain_id}` | `get` | |

## Protocol RPC's

[jakarta-openapi.json](https://gitlab.com/tezos/tezos/-/blob/master/docs/api/jakarta-openapi.json)

| Path | Methods | Implemented |
|:-|:-|:-|
| `/chains/<chain_id>/blocks/<block_id>/` | `get` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/big_maps/{big_map_id}` | `get` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/big_maps/{big_map_id}/{script_expr}` | `get` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/big_maps/{big_map_id}/{script_expr}/normalized` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/context/cache/contracts/all` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/cache/contracts/rank` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/context/cache/contracts/size` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/cache/contracts/size_limit` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/constants` | `get`  | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/constants/errors` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}` | `get` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/balance` | `get` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/balance_and_frozen_bonds` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/big_map_get` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/counter` | `get` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/delegate` | `get` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/entrypoints` | `get` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/entrypoints/{entrypoint}` | `get` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/frozen_bonds` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/manager_key` | `get` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/script` | `get` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/script/normalized` | `post` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/single_sapling_get_diff` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/storage` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/contracts/{contract_id}/storage/normalized` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}/current_frozen_deposits` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}/deactivated` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}/delegated_balance` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}/delegated_contracts` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}/frozen_deposits` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}/frozen_deposits_limit` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}/full_balance` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}/grace_period` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}/participation` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}/staking_balance` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}/voting_info` | `get` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/context/delegates/{pkh}/voting_power` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/liquidity_baking/cpmm_address` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/merkle_tree` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/nonces/{block_level}` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/raw/bytes` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/sapling/{sapling_state_id}/get_diff` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/sc_rollup` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/sc_rollup/{Sc_rollup_hash}/inbox` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/sc_rollup/{Sc_rollup_hash}/initial_level` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/sc_rollup/{Sc_rollup_hash}/kind` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/seed` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/context/selected_snapshot` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/tx_rollup/{tx_rollup_id}/commitment/{block_level}` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/tx_rollup/{tx_rollup_id}/inbox/{block_level}` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/tx_rollup/{tx_rollup_id}/pending_bonded_commitments/{pkh}` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/context/tx_rollup/{tx_rollup_id}/state` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/hash` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/header` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/header/protocol_data` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/header/protocol_data/raw` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/header/raw` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/header/shell` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/baking_rights` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/complete/{prefix}` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/current_level` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/endorsing_rights` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/forge/operations` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/forge/protocol_data` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/forge/tx_rollup/commitment/merkle_tree_hash` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/forge/tx_rollup/commitment/merkle_tree_path` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/forge/tx_rollup/inbox/merkle_tree_hash` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/forge/tx_rollup/inbox/merkle_tree_path` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/forge/tx_rollup/inbox/message_hash` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/forge_block_header` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/levels_in_current_cycle` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/parse/block` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/parse/operations` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/preapply/block` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/preapply/operations` | `post` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/helpers/round` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/entrypoint` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/entrypoints` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/normalize_data` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/normalize_script` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/normalize_type` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/pack_data` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/run_code` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/run_operation` | `post` | :heavy_check_mark: |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/run_view` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/script_size` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/simulate_operation` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/trace_code` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/typecheck_code` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/scripts/typecheck_data` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/helpers/validators` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/live_blocks` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/metadata` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/metadata_hash` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/operation_hashes` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/operation_hashes/{list_offset}` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/operation_hashes/{list_offset}/{operation_offset}` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/operation_metadata_hashes` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/operation_metadata_hashes/{list_offset}` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/operation_metadata_hashes/{list_offset}/{operation_offset}` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/operations` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/operations/{list_offset}` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/operations/{list_offset}/{operation_offset}` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/operations_metadata_hash` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/protocols` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/votes/ballot_list` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/votes/ballots` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/votes/current_period` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/votes/current_proposal` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/votes/current_quorum` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/votes/listings` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/votes/proposals` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/votes/successor_period` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/votes/total_voting_power` | `get` | |

## Mempool RPC's

[jakarta-openapi.json](https://gitlab.com/tezos/tezos/-/blob/master/docs/api/jakarta-mempool-openapi.json)

| Path | Methods | Implemented |
|:-|:-|:-|
| `/chains/<chain_id>/blocks/<block_id>/ban_operation` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/filter` | `get`, `post` | |
| `/chains/<chain_id>/blocks/<block_id>/monitor_operations` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/pending_operations` | `get` | |
| `/chains/<chain_id>/blocks/<block_id>/request_operations` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/unban_all_operations` | `post` | |
| `/chains/<chain_id>/blocks/<block_id>/unban_operation` | `post` | |
