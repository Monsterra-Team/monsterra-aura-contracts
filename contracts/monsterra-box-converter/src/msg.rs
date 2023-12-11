use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Timestamp, Addr};
#[cw_serde]
pub struct ConvertMsg {
    pub boxes: Vec<NftInfo>,
    pub nfts: Vec<NftInfo>,
    pub timestamp: Timestamp,
}

#[cw_serde]
pub struct ConvertPayload {
    pub sender: Addr,
    pub boxes: Vec<NftInfo>,
    pub nfts: Vec<NftInfo>,
    pub timestamp: Timestamp,
}

#[cw_serde]
pub struct NftInfo {
    pub contract_addr: Addr,
    pub token_id: String
}

#[cw_serde]
pub struct MintMsg {
    pub token_id: String,
    pub owner: String,
}

#[cw_serde]
pub enum MonsterraNFTMsg {
    IsAdmin {
        user: String,
    },
    MintBatch {
        msgs: Vec<MintMsg>
    }
}

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Convert {
        msg: ConvertMsg,
        signature: Binary,
    },

    SetAdmin {
        user: Addr,
        status: bool,
    },

    SetSigner {
        public_key: Binary,
    },

    SetBoxContract {
        box_contract: Addr,
        status: bool,
    }
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub struct MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(bool)]
    IsAdmin { user: Addr },

    #[returns(Binary)]
    GetSigner {},

    #[returns(bool)]
    IsBoxContract { box_contract: Addr },
}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct YourQueryResponse {}
