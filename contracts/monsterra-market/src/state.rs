use crate::error::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Storage, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};

pub struct GameMarketContract<'a> {
    pub contract_info: Item<'a, ContractInfo>,
    pub owner: Item<'a, Addr>,
    pub orders: IndexedMap<'a, &'a str, Order, OrderIndexes<'a>>,
    pub bids: IndexedMap<'a, &'a str, Bid, BidIndexes<'a>>,
    pub bundles: IndexedMap<'a, &'a str, Bundle, BundleIndexes<'a>>,
    pub can_accept: IndexedMap<'a, &'a str, CanAccept, CanAcceptIndexes<'a>>,
}

impl Default for GameMarketContract<'static> {
    fn default() -> Self {
        Self::new(
            "contract_info",
            "owner",
            "orders_key",
            "order",
            "bids_key",
            "bid",
            "bundles_key",
            "bundle",
            "can_accept_key",
            "can_accept",
        )
    }
}

impl<'a> GameMarketContract<'a> {
    fn new(
        contract_info: &'a str,
        owner: &'a str,
        orders_key: &'a str,
        order: &'a str,
        bids_key: &'a str,
        bid: &'a str,
        bundles_key: &'a str,
        bundle: &'a str,
        can_accept_key: &'a str,
        can_accept: &'a str,
    ) -> Self {
        let indexes_order = OrderIndexes {
            order_id: MultiIndex::new(order_idx, orders_key, order),
        };
        let indexes_bid = BidIndexes {
            bid_id: MultiIndex::new(bid_idx, bids_key, bid),
        };
        let indexes_bundle = BundleIndexes {
            bundle_id: MultiIndex::new(bundle_idx, bundles_key, bundle),
        };
        let indexes_can_accept = CanAcceptIndexes {
            key: MultiIndex::new(can_accept_idx, can_accept_key, can_accept),
        };
        Self {
            contract_info: Item::new(contract_info),
            owner: Item::new(owner),
            orders: IndexedMap::new(orders_key, indexes_order),
            bids: IndexedMap::new(bids_key, indexes_bid),
            bundles: IndexedMap::new(bundles_key, indexes_bundle),
            can_accept: IndexedMap::new(can_accept_key, indexes_can_accept),
        }
    }

    pub fn update_can_accept(
        &self,
        storage: &mut dyn Storage,
        status: bool,
        token_address: &Addr,
        token_id: &String,
        owner: &Addr,
        bundle_id: &String,
        order_id: &String,
    ) {
        let key =
            token_address.clone().to_string() + &token_id.clone() + &owner.clone().to_string();
        let can_accept = CanAccept {
            token_address: token_address.clone(),
            token_id: token_id.clone(),
            owner: owner.clone(),
            status,
            bundle_id: bundle_id.to_owned().clone(),
            order_id: order_id.to_owned().clone(),
        };
        let can_accept_info = self.can_accept.load(storage, &key);

        let is_existed = match can_accept_info {
            Ok(_) => match self.can_accept.save(storage, &key, &can_accept) {
                Ok(_) => true,
                Err(_) => true,
            },
            Err(_) => false,
        };
        if !is_existed {
            match self.can_accept.update(storage, &key, |old| match old {
                Some(_) => Err(ContractError::Added {}),
                None => Ok(can_accept),
            }) {
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
        }
    }
}

#[cw_serde]
pub struct ContractInfo {
    pub name: String,
    pub symbol: String,
    pub owner: Addr,
    pub total_order: u32,
    pub total_bid: u32,
    pub total_bundle: u32,
    pub bundle_fee: u16,
    pub game_market_payment_contract: Addr,
}

#[cw_serde]
pub struct CanAccept {
    pub token_address: Addr,
    pub token_id: String,
    pub owner: Addr,
    pub status: bool,
    pub bundle_id: String,
    pub order_id: String,
}

#[cw_serde]
pub struct Order {
    pub id: String,
    pub owner: Addr,
    pub token_address: Addr,
    pub payment_contract: Addr,
    pub token_id: String,
    pub quantity: Uint128,
    pub price: Uint128,
    pub is_cw721: bool,
    pub status: bool,
}

#[cw_serde]
pub struct Bid {
    pub id: String,
    pub owner: Addr,
    pub token_address: Addr,
    pub payment_contract: Addr,
    pub token_id: String,
    pub quantity: Uint128,
    pub price: Uint128,
    pub expired: u64,
    pub status: bool,
}

#[cw_serde]
pub struct Bundle {
    pub id: String,
    pub owner: Addr,
    pub list_token_address: Vec<Addr>,
    pub payment_contract: Addr,
    pub list_token_id: Vec<String>,
    pub price: Uint128,
    pub status: bool,
}

pub struct OrderIndexes<'a> {
    // pk goes to second tuple element
    pub order_id: MultiIndex<'a, String, Order, String>,
}

impl<'a> IndexList<Order> for OrderIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Order>> + '_> {
        let v: Vec<&dyn Index<Order>> = vec![&self.order_id];
        Box::new(v.into_iter())
    }
}

pub struct BidIndexes<'a> {
    // pk goes to second tuple element
    pub bid_id: MultiIndex<'a, String, Bid, String>,
}

impl<'a> IndexList<Bid> for BidIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Bid>> + '_> {
        let v: Vec<&dyn Index<Bid>> = vec![&self.bid_id];
        Box::new(v.into_iter())
    }
}

pub struct BundleIndexes<'a> {
    // pk goes to second tuple element
    pub bundle_id: MultiIndex<'a, String, Bundle, String>,
}

impl<'a> IndexList<Bundle> for BundleIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Bundle>> + '_> {
        let v: Vec<&dyn Index<Bundle>> = vec![&self.bundle_id];
        Box::new(v.into_iter())
    }
}

pub struct CanAcceptIndexes<'a> {
    // pk goes to second tuple element
    pub key: MultiIndex<'a, String, CanAccept, String>,
}

impl<'a> IndexList<CanAccept> for CanAcceptIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<CanAccept>> + '_> {
        let v: Vec<&dyn Index<CanAccept>> = vec![&self.key];
        Box::new(v.into_iter())
    }
}

pub fn can_accept_idx(d: &CanAccept) -> String {
    d.token_address.clone().to_string() + (&d.token_id.clone()) + (&d.owner.clone().to_string())
}

pub fn order_idx(d: &Order) -> String {
    d.id.clone()
}

pub fn bid_idx(d: &Bid) -> String {
    d.id.clone()
}

pub fn bundle_idx(d: &Bundle) -> String {
    d.id.clone()
}
