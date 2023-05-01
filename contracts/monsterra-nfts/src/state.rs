// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map, U64Key};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransferLog {
    pub from: String,
    pub to: String,
    pub token_id: String,
    // pub block_height: u64,
    // pub tx_hash: String,
}

pub const TRANSFER_LOGS: Map<U64Key, TransferLog> = Map::new("transfer_logs");
pub const LOG_COUNTER: Item<u64> = Item::new("log_counter");

pub const STAKE_OWNERS: Map<String, String> = Map::new("stake_owners");
