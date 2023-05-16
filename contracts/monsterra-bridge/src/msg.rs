use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128, Binary};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
   
} 

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    TransferOwnerShip {user: Addr},
    SetAdmin {user: Addr, status: bool},
    SetAcceptedToken {token: String, status: bool},
    SetAcceptedDesToken {token:String, status: bool},
    SetSigner {public_key: Binary},
    Mint {swap_message: SwapMessage, signature: Binary},
    Burn{swap_message: SwapMessage},
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] 
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetOwner {},
    IsAdmin {user: Addr},
    IsAcceptedToken {token: String},
    IsAcceptedDesToken {token: String},
    GetSigner{},
    GetSwapData{transaction_id: String},
    Test{}

}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapMessage{
    pub transaction_id: String,
    pub cur_token: String,
    pub des_token: String,
    pub cur_user: String,
    pub des_user: String,
    pub amount: Uint128
}
