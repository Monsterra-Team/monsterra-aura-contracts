use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[allow(unused_imports)]
use crate::state::{ContractInfo, ContractSupport};

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddContractSupport {
        contract_address: Addr,
        payment_contract: Addr,
        fee: u16,
        is_cw721: bool,
    },
    UpdateFee {
        contract_address: Addr,
        fee: u16,
    },
    SetPaymentMethod {
        contract_address: Addr,
        payment_contract: Addr,
        status: bool,
    },
    RemoveContractSupport {
        contract_address: Addr,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ContractInfo)]
    GamePaymentContractInfo {},

    #[returns(ContractSupport)]
    ContractSupportInfo { contract_address: Addr },

    #[returns(bool)]
    IsTokenSupport {
        contract_address: Addr,
        payment_contract: Addr,
    },
    #[returns(u16)]
    GetContractFee { contract_address: Addr },
}
