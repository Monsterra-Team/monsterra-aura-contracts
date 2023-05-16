// use cosmwasm_std::{Addr, Uint128};
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct InstantiateMsg {
//     pub name: String,
//     pub symbol: String,
//     pub bundle_fee: u16,
//     pub game_market_payment_contract: Addr,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum ExecuteMsg {
//     CreateOrder {
//         token_address: Addr,
//         payment_contract: Addr,
//         token_id: String,
//         price: Uint128,
//         quantity: Uint128,
//     },
//     UpdateOrder {
//         order_id: String,
//         quantity: Uint128,
//         price: Uint128,
//     },
//     CancelOrder {
//         order_id: String,
//     },
//     BuyOrder {
//         order_id: String,
//         quantity: Uint128,
//     },
//     CreateBid {
//         token_address: Addr,
//         payment_contract: Addr,
//         token_id: String,
//         price: Uint128,
//         expired: u64,
//     },
//     UpdateBid {
//         bid_id: String,
//         price: Uint128,
//         expired: u64,
//     },
//     CancelBid {
//         bid_id: String,
//     },
//     AcceptBid {
//         bid_id: String,
//     },
//     CreateBundle {
//         list_token_address: Vec<Addr>,
//         list_token_id: Vec<String>,
//         payment_contract: Addr,
//         price: Uint128,
//     },
//     BuyBundle {
//         bundle_id: String,
//     },
//     CancelBundle {
//         bundle_id: String,
//     },
//     UpdateBundleFee {
//         bundle_fee: u16,
//     },
//     UpdateGameMarketPaymentContract {
//         game_market_payment_contract: Addr,
//     },
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum QueryMsg {
//     // GetCount returns the current count as a json-encoded number
//     ContractInfo {},
//     OrderInfo { order_id: String },
//     BidInfo { bid_id: String },
//     BundleInfo { bundle_id: String },
//     ContractSupportInfo {contract_address: Addr},
//     IsTokenSupport {contract_address: Addr, payment_contract: Addr},
// }
