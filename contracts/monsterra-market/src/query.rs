use cosmwasm_std::{to_binary, Addr, Binary, Deps, Env, StdResult};

use crate::interfaces::{ContractSupportResponse, QueryMsg::*};
use crate::msg::QueryMsg;
use crate::state::{Bid, Bundle, ContractInfo, GameMarketContract, Order};

impl<'a> GameMarketQuery for GameMarketContract<'a> {
    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfo> {
        self.contract_info.load(deps.storage)
    }

    fn order_info(&self, deps: Deps, order_id: String) -> StdResult<Order> {
        let info = self.orders.load(deps.storage, &order_id)?;
        Ok(info)
    }

    fn bundle_info(&self, deps: Deps, bundle_id: String) -> StdResult<Bundle> {
        let info = self.bundles.load(deps.storage, &bundle_id)?;
        Ok(info)
    }

    fn bid_info(&self, deps: Deps, bid_id: String) -> StdResult<Bid> {
        let info = self.bids.load(deps.storage, &bid_id)?;
        Ok(info)
    }

    fn contract_support_info(&self, deps: Deps, contract_address: Addr) -> StdResult<ContractSupportResponse> {
        let contract_info = self.contract_info.load(deps.storage)?;
        let info = deps.querier.query_wasm_smart(
            contract_info.game_market_payment_contract,
            &ContractSupportInfo { contract_address },
        )?;
        Ok(info)
    }

    fn is_token_support(
        &self,
        deps: Deps,
        contract_address: Addr,
        payment_contract: Addr,
    ) -> StdResult<bool> {
        let contract_info = self.contract_info.load(deps.storage)?;
        let info = deps.querier.query_wasm_smart(
            contract_info.game_market_payment_contract,
            &IsTokenSupport {
                contract_address,
                payment_contract,
            },
        )?;
        Ok(info)
    }
}

pub trait GameMarketQuery {
    fn order_info(&self, deps: Deps, order_id: String) -> StdResult<Order>;

    fn bid_info(&self, deps: Deps, bid_id: String) -> StdResult<Bid>;

    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfo>;

    fn bundle_info(&self, deps: Deps, bundle_id: String) -> StdResult<Bundle>;

    fn contract_support_info(&self, deps: Deps, contract_address: Addr) -> StdResult<ContractSupportResponse>;
    fn is_token_support(
        &self,
        deps: Deps,
        contract_address: Addr,
        payment_contract: Addr,
    ) -> StdResult<bool>;
}

impl<'a> GameMarketContract<'a> {
    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::ContractInfo {} => to_binary(&self.contract_info(deps)?),

            QueryMsg::OrderInfo { order_id } => to_binary(&self.order_info(deps, order_id)?),
            QueryMsg::BidInfo { bid_id } => to_binary(&self.bid_info(deps, bid_id)?),
            QueryMsg::BundleInfo { bundle_id } => to_binary(&self.bundle_info(deps, bundle_id)?),
            QueryMsg::ContractSupportInfo { contract_address } => {
                to_binary(&self.contract_support_info(deps, contract_address)?)
            }
            QueryMsg::IsTokenSupport {
                contract_address,
                payment_contract,
            } => to_binary(&self.is_token_support(deps, contract_address, payment_contract)?),
        }
    }
}
