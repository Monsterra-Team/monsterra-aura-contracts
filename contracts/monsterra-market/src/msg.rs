use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[allow(unused_imports)]
use crate::{
    interfaces::ContractSupportResponse,
    state::{Bid, Bundle, ContractInfo, Order},
};

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub bundle_fee: u16,
    pub game_market_payment_contract: Addr,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateOrder {
        token_address: Addr,
        payment_contract: Addr,
        token_id: String,
        price: Uint128,
        quantity: Uint128,
    },
    UpdateOrder {
        order_id: String,
        quantity: Uint128,
        price: Uint128,
    },
    CancelOrder {
        order_id: String,
    },
    BuyOrder {
        order_id: String,
        quantity: Uint128,
    },
    CreateBid {
        token_address: Addr,
        payment_contract: Addr,
        token_id: String,
        price: Uint128,
        expired: u64,
    },
    UpdateBid {
        bid_id: String,
        price: Uint128,
        expired: u64,
    },
    CancelBid {
        bid_id: String,
    },
    AcceptBid {
        bid_id: String,
    },
    CreateBundle {
        list_token_address: Vec<Addr>,
        list_token_id: Vec<String>,
        payment_contract: Addr,
        price: Uint128,
    },
    BuyBundle {
        bundle_id: String,
    },
    CancelBundle {
        bundle_id: String,
    },
    UpdateBundle {
        bundle_id: String,
        price: Uint128
    },
    UpdateBundleFee {
        bundle_fee: u16,
    },
    UpdateGameMarketPaymentContract {
        game_market_payment_contract: Addr,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(ContractInfo)]
    ContractInfo {},

    #[returns(Order)]
    OrderInfo { order_id: String },

    #[returns(Bid)]
    BidInfo { bid_id: String },

    #[returns(Bundle)]
    BundleInfo { bundle_id: String },

    #[returns(ContractSupportResponse)]
    ContractSupportInfo { contract_address: Addr },

    #[returns(bool)]
    IsTokenSupport {
        contract_address: Addr,
        payment_contract: Addr,
    },
}
