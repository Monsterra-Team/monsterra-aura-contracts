use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Timestamp, Uint128};

use crate::state::StakeData;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    TransferOwnerShip {
        user: Addr,
    },
    SetAdmin {
        user: Addr,
        status: bool,
    },
    SetAcceptedToken {
        token: Addr,
        status: bool,
    },
    SetSigner {
        public_key: Binary,
    },
    Stake {
        token: Addr,
        amount: Uint128,
        duration: u8,
    },
    Unstake {
        msg: UnstakeMsg,
        signature: Binary,
    },
    Burn {
        token: Addr,
        amount: Uint128,
    },
}

#[cw_serde]
pub struct UnstakeMsg {
    pub token: Addr,
    pub amount: Uint128,
    pub nonce: String,
    pub timestamp: Timestamp,
}

#[cw_serde]
pub struct UnstakePayload {
    pub sender: Addr,
    pub token: Addr,
    pub amount: Uint128,
    pub nonce: String,
    pub timestamp: Timestamp,
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // This example query variant indicates that any client can query the contract
    // using `YourQuery` and it will return `YourQueryResponse`
    // This `returns` information will be included in contract's schema
    // which is used for client code generation.
    //
    // #[returns(YourQueryResponse)]
    // YourQuery {},
    #[returns(Addr)]
    GetOwner {},
    #[returns(bool)]
    IsAdmin { user: Addr },
    #[returns(bool)]
    IsAcceptedToken { token: Addr },
    #[returns(Binary)]
    GetSigner {},
    #[returns(Uint128)]
    GetTotalStaked { user: Addr },
    #[returns(Vec<StakeData>)]
    GetStakeData { user: Addr },
    #[returns(bool)]
    IsUsedNonce { nonce: String },
}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct YourQueryResponse {}

#[cw_serde]
pub struct StakeDataResponse {
    stake_data: Vec<StakeData>,
}
