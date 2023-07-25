use std::{u16, u32};

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, QuerierWrapper, Response, StdResult};

#[allow(unused_imports)]
use crate::state::ContractInfo;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ContractInfo)]
    GamePaymentContractInfo {},

    #[returns(ContractSupportResponse)]
    ContractSupportInfo { contract_address: Addr },

    #[returns(bool)]
    IsTokenSupport {
        contract_address: Addr,
        payment_contract: Addr,
    },
}

pub enum ExecuteMsg {
    TransferNft {
        contract_address: Addr,
        token_id: String,
        recipient: Addr,
    },
}

#[cw_serde]
pub struct ContractSupportResponse {
    pub contract_address: Addr,
    pub fee: u16,
    pub is_cw721: bool,
    pub status: bool,
}

#[cw_serde]
pub struct GamePaymentContractResponse {
    pub name: String,
    pub symbol: String,
    pub total_contract_supported: u32,
    pub owner: Addr,
}
pub trait GamePaymentQuerier {
    fn contract_support_info(
        &self,
        feed_address: Addr,
        contract_address: Addr,
    ) -> StdResult<ContractSupportResponse>;
    fn is_token_support(
        &self,
        feed_address: Addr,
        contract_address: Addr,
        payment_contract: Addr,
    ) -> StdResult<bool>;
}

impl<'a> GamePaymentQuerier for QuerierWrapper<'a> {
    fn contract_support_info(
        &self,
        feed_address: Addr,
        contract_address: Addr,
    ) -> StdResult<ContractSupportResponse> {
        self.query_wasm_smart(
            feed_address,
            &QueryMsg::ContractSupportInfo { contract_address },
        )
    }

    fn is_token_support(
        &self,
        feed_address: Addr,
        contract_address: Addr,
        payment_contract: Addr,
    ) -> StdResult<bool> {
        self.query_wasm_smart(
            feed_address,
            &QueryMsg::IsTokenSupport {
                contract_address,
                payment_contract,
            },
        )
    }
}

pub trait GameMarketExecute {
    type Err: ToString;

    fn transfer_nft(
        &self,
        contract_address: Addr,
        token_id: String,
        recipient: Addr,
    ) -> Result<Response, Self::Err>;
}
