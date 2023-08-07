use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Binary};

#[allow(unused_imports)]
use crate::state::Swapdata;

#[cw_serde]
pub struct InstantiateMsg {
   
} 

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    TransferOwnerShip {user: Addr},
    SetAdmin {user: Addr, status: bool},
    SetAcceptedToken {token: String, status: bool},
    SetAcceptedDesToken {token:String, status: bool},
    SetMaxSwapAmount {token:String, max_amount: Uint128},
    SetApproveTransaction {transaction_id: String, status: bool},
    SetSigner {public_key: Binary},
    Mint {swap_message: SwapMessage, signature: Binary},
    Burn{swap_message: SwapMessage},
    
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(Addr)]
    GetOwner {},
    #[returns(bool)]
    IsAdmin {user: Addr},
    #[returns(bool)]
    IsAcceptedToken {token: String},
    #[returns(bool)]
    IsAcceptedDesToken {token: String},
    #[returns(Uint128)]
    GetMaxSwapAmount {token: String},
    #[returns(Binary)]
    IsApproveTransaction {transaction_id: String},
    #[returns(Binary)]
    GetSigner{},
    #[returns(Swapdata)]
    GetSwapData{transaction_id: String},
    #[returns(Uint128)]
    Test{}

}

// We define a custom struct for each query response
#[cw_serde]
pub struct SwapMessage{
    pub transaction_id: String,
    pub cur_token: String,
    pub des_token: String,
    pub cur_user: String,
    pub des_user: String,
    pub amount: Uint128
}
