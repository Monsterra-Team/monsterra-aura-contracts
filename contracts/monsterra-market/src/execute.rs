use std::ops::{Div, Mul, Sub};

use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, Timestamp, Uint128,
    WasmMsg,
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::interfaces::{ContractSupportResponse, QueryMsg::*};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Bid, Bundle, ContractInfo, GameMarketContract, Order};
use cw20::{BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg};
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg, OwnerOfResponse};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:game-market";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const ZOOM_FEE: u16 = 10000;

impl<'a> GameMarketContract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let contract_info = ContractInfo {
            name: msg.name,
            symbol: msg.symbol,
            owner: _info.sender.clone(),
            total_order: 0,
            total_bid: 0,
            total_bundle: 0,
            bundle_fee: msg.bundle_fee,
            game_market_payment_contract: msg.game_market_payment_contract,
        };
        self.contract_info.save(deps.storage, &contract_info)?;
        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::CreateOrder {
                token_address,
                payment_contract,
                token_id,
                price,
                quantity,
            } => self.create_order(
                deps,
                env,
                info,
                token_address,
                payment_contract,
                token_id,
                price,
                quantity,
            ),
            ExecuteMsg::UpdateOrder {
                order_id,
                quantity,
                price,
            } => self.update_order(deps, env, info, order_id, quantity, price),
            ExecuteMsg::BuyOrder { order_id, quantity } => {
                self.buy_order(deps, env, info, order_id, quantity)
            }
            ExecuteMsg::CancelOrder { order_id } => self.cancel_order(deps, env, info, order_id),

            ExecuteMsg::CreateBid {
                token_address,
                payment_contract,
                token_id,
                price,
                expired,
            } => self.create_bid(
                deps,
                env,
                info,
                token_address,
                payment_contract,
                token_id,
                price,
                expired,
            ),
            ExecuteMsg::UpdateBid {
                bid_id,
                price,
                expired,
            } => self.update_bid(deps, env, info, bid_id, price, expired),
            ExecuteMsg::AcceptBid { bid_id } => self.accept_bid(deps, env, info, bid_id),
            ExecuteMsg::CancelBid { bid_id } => self.cancel_bid(deps, env, info, bid_id),

            ExecuteMsg::CreateBundle {
                list_token_address,
                list_token_id,
                payment_contract,
                price,
            } => self.create_bundle(
                deps,
                env,
                info,
                list_token_address,
                list_token_id,
                payment_contract,
                price,
            ),
            ExecuteMsg::BuyBundle { bundle_id } => self.buy_bundle(deps, env, info, bundle_id),
            ExecuteMsg::CancelBundle { bundle_id } => {
                self.cancel_bundle(deps, env, info, bundle_id)
            }
            ExecuteMsg::UpdateBundleFee { bundle_fee } => {
                self.update_bundle_fee(deps, info, bundle_fee)
            }
            ExecuteMsg::UpdateGameMarketPaymentContract {
                game_market_payment_contract,
            } => self.update_game_market_payment_contract(deps, info, game_market_payment_contract),
        }
    }
}

pub trait GameMarketExecute {
    type Err: ToString;

    fn create_order(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_address: Addr,
        payment_contract: Addr,
        token_id: String,
        price: Uint128,
        quantity: Uint128,
    ) -> Result<Response, Self::Err>;

    fn update_order(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        order_id: String,
        quantity: Uint128,
        price: Uint128,
    ) -> Result<Response, Self::Err>;

    fn buy_order(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        order_id: String,
        quantity: Uint128,
    ) -> Result<Response, Self::Err>;

    fn cancel_order(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        order_id: String,
    ) -> Result<Response, Self::Err>;

    fn create_bid(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_address: Addr,
        payment_contract: Addr,
        token_id: String,
        price: Uint128,
        expired: u64,
    ) -> Result<Response, Self::Err>;

    fn update_bid(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        bid_id: String,
        price: Uint128,
        expired: u64,
    ) -> Result<Response, Self::Err>;

    fn accept_bid(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        bid_id: String,
    ) -> Result<Response, Self::Err>;

    fn cancel_bid(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        bid_id: String,
    ) -> Result<Response, Self::Err>;

    fn create_bundle(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        list_token_address: Vec<Addr>,
        list_token_id: Vec<String>,
        payment_contract: Addr,
        price: Uint128,
    ) -> Result<Response, Self::Err>;

    fn buy_bundle(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        bundle_id: String,
    ) -> Result<Response, Self::Err>;

    fn cancel_bundle(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        bundle_id: String,
    ) -> Result<Response, Self::Err>;

    fn update_bundle_fee(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        bundle_fee: u16,
    ) -> Result<Response, Self::Err>;

    fn update_game_market_payment_contract(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        game_market_payment_contract: Addr,
    ) -> Result<Response, Self::Err>;
}

impl<'a> GameMarketExecute for GameMarketContract<'a> {
    type Err = ContractError;

    fn create_order(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_address: Addr,
        payment_contract: Addr,
        token_id: String,
        price: Uint128,
        quantity: Uint128,
    ) -> Result<Response, ContractError> {
        let mut contract_info = self.contract_info.load(deps.storage)?;
        let is_payment_token_supported: bool = deps.querier.query_wasm_smart(
            contract_info.game_market_payment_contract.clone(),
            &IsTokenSupport {
                contract_address: token_address.clone(),
                payment_contract: payment_contract.clone(),
            },
        )?;
        if !is_payment_token_supported {
            return Err(ContractError::PaymentMethodNotSupport {});
        }
        let data_contract_support: ContractSupportResponse = deps.querier.query_wasm_smart(
            contract_info.game_market_payment_contract.clone(),
            &ContractSupportInfo {
                contract_address: token_address.clone(),
            },
        )?;
        let mut balance = Uint128::zero();
        if data_contract_support.is_cw721 {
            let owner_address: OwnerOfResponse = deps.querier.query_wasm_smart(
                token_address.to_string().clone(),
                &Cw721QueryMsg::OwnerOf {
                    token_id: token_id.clone(),
                    include_expired: None,
                },
            )?;
            if owner_address.owner == info.sender.to_string() {
                balance = Uint128::from(1u128);
            }
        } else {
            let balance_token: BalanceResponse = deps.querier.query_wasm_smart(
                token_address.to_string().clone(),
                &Cw20QueryMsg::Balance {
                    address: info.sender.to_string(),
                },
            )?;
            balance = balance_token.balance
        }
        if Uint128::is_zero(&balance) || balance < quantity {
            return Err(ContractError::InsufficienTokenBalance {});
        }
        if quantity == Uint128::zero() {
            return Err(ContractError::InvalidQuantity {});
        }
        let mut messages: Vec<CosmosMsg> = vec![];
        let id = (contract_info.total_order.clone() + 1).to_string();
        let order = Order {
            id: id.clone(),
            owner: info.sender.clone(),
            token_address: token_address.clone(),
            payment_contract: payment_contract.clone(),
            token_id: token_id.clone(),
            quantity: quantity.clone(),
            price: price.clone(),
            is_cw721: data_contract_support.is_cw721,
            status: true,
        };
        contract_info.total_order += 1;
        self.contract_info.save(deps.storage, &contract_info)?;
        self.orders
            .update(deps.storage, &order.clone().id, |old| match old {
                Some(_) => Err(ContractError::Added {}),
                None => Ok(order),
            })?;
        if !data_contract_support.is_cw721 {
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: token_address.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                    owner: info.sender.to_string(),
                    recipient: env.contract.address.to_string(),
                    amount: quantity,
                })?,
                funds: vec![],
            }))
        } else {
            self.update_can_accept(
                deps.storage,
                true,
                &token_address,
                &token_id,
                &info.sender,
                &String::from("0"),
                &id,
            );
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: token_address.clone().to_string(),
                msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                    recipient: env.contract.address.into_string(),
                    token_id,
                })?,
                funds: vec![],
            }))
        }
        Ok(Response::new()
            .add_messages(messages)
            .add_attribute("action", "create_order")
            .add_attribute("order_id", id))
    }

    fn update_order(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        order_id: String,
        quantity: Uint128,
        price: Uint128,
    ) -> Result<Response, ContractError> {
        let mut order = self.orders.load(deps.storage, &order_id)?;
        if !order.status {
            return Err(ContractError::OrderCanceled {});
        }
        if order.owner != info.sender {
            return Err(ContractError::NotOwner {});
        }
        if quantity == Uint128::zero() {
            return Err(ContractError::InvalidQuantity {});
        }
        let mut messages: Vec<CosmosMsg> = vec![];
        order.price = price;
        if !order.is_cw721 {
            if quantity > order.quantity {
                let balance_token: BalanceResponse = deps.querier.query_wasm_smart(
                    order.token_address.to_string().clone(),
                    &Cw20QueryMsg::Balance {
                        address: info.sender.to_string(),
                    },
                )?;
                if balance_token.balance < Uint128::sub(quantity, order.quantity) {
                    return Err(ContractError::InsufficienTokenBalance {});
                }
                messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: order.token_address.to_string(),
                    msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                        owner: info.sender.to_string(),
                        recipient: env.contract.address.to_string(),
                        amount: Uint128::sub(quantity, order.quantity),
                    })?,
                    funds: vec![],
                }))
            } else if quantity < order.quantity {
                messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: order.token_address.to_string(),
                    msg: to_binary(&Cw20ExecuteMsg::Transfer {
                        recipient: order.owner.to_string(),
                        amount: Uint128::sub(order.quantity, quantity),
                    })?,
                    funds: vec![],
                }))
            }
        }
        order.quantity = quantity;
        self.orders.save(deps.storage, &order_id, &order)?;
        Ok(Response::new()
            .add_messages(messages)
            .add_attribute("action", "update_order")
            .add_attribute("order_id", order_id))
    }

    fn buy_order(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        order_id: String,
        quantity: Uint128,
    ) -> Result<Response, ContractError> {
        let mut order = self.orders.load(deps.storage, &order_id)?;
        let contract_info = self.contract_info.load(deps.storage)?;
        let data_contract_support: ContractSupportResponse = deps.querier.query_wasm_smart(
            contract_info.game_market_payment_contract.clone(),
            &ContractSupportInfo {
                contract_address: order.token_address.clone(),
            },
        )?;
        if !order.status {
            return Err(ContractError::OrderCanceled {});
        }
        if quantity == Uint128::zero() || quantity > order.quantity {
            return Err(ContractError::InvalidQuantity {});
        }
        if quantity == order.quantity {
            order.status = false;
            order.quantity = Uint128::zero();
        } else {
            order.quantity = Uint128::sub(order.quantity, quantity);
        }
        self.orders.save(deps.storage, &order_id, &order)?;
        let mut messages: Vec<CosmosMsg> = vec![];
        if order.is_cw721 {
            self.update_can_accept(
                deps.storage,
                false,
                &order.token_address,
                &order.token_id,
                &order.owner,
                &String::from("0"),
                &order_id,
            );
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: order.token_address.to_string(),
                msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                    recipient: info.sender.to_string(),
                    token_id: order.token_id,
                })?,
                funds: vec![],
            }))
        } else {
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: order.token_address.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: info.sender.to_string(),
                    amount: quantity,
                })?,
                funds: vec![],
            }))
        }
        if order.price > Uint128::zero() {
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: order.payment_contract.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                    owner: info.sender.to_string(),
                    recipient: env.contract.address.to_string(),
                    amount: Uint128::mul(order.price, quantity),
                })?,
                funds: vec![],
            }));
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: order.payment_contract.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: order.owner.to_string(),
                    amount: caculate_amount(
                        Uint128::mul(order.price, quantity),
                        data_contract_support.fee,
                    ),
                })?,
                funds: vec![],
            }));
        }
        Ok(Response::new()
            .add_messages(messages)
            .add_attribute("action", "buy_order")
            .add_attribute("order_id", order.id))
    }

    fn cancel_order(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        order_id: String,
    ) -> Result<Response, ContractError> {
        let mut order = self.orders.load(deps.storage, &order_id)?;
        if !order.status {
            return Err(ContractError::OrderCanceled {});
        }
        if order.owner != info.sender {
            return Err(ContractError::NotOwner {});
        }
        let quantity = order.quantity.clone();
        order.quantity = Uint128::zero();
        order.status = false;
        self.orders.save(deps.storage, &order_id, &order)?;
        let mut messages: Vec<CosmosMsg> = vec![];
        if order.is_cw721 {
            self.update_can_accept(
                deps.storage,
                false,
                &order.token_address,
                &order.token_id,
                &order.owner,
                &String::from("0"),
                &order_id,
            );
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: order.token_address.to_string(),
                msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                    recipient: info.sender.to_string(),
                    token_id: order.token_id,
                })?,
                funds: vec![],
            }))
        } else {
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: order.token_address.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: info.sender.to_string(),
                    amount: quantity,
                })?,
                funds: vec![],
            }))
        }
        Ok(Response::new()
            .add_messages(messages)
            .add_attribute("action", "cancel_order")
            .add_attribute("order_id", order_id))
    }

    fn create_bid(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_address: Addr,
        payment_contract: Addr,
        token_id: String,
        price: Uint128,
        expired: u64,
    ) -> Result<Response, ContractError> {
        let mut contract_info = self.contract_info.load(deps.storage)?;
        let is_payment_token_supported: bool = deps.querier.query_wasm_smart(
            contract_info.game_market_payment_contract.clone(),
            &IsTokenSupport {
                contract_address: token_address.clone(),
                payment_contract: payment_contract.clone(),
            },
        )?;
        if !is_payment_token_supported {
            return Err(ContractError::PaymentMethodNotSupport {});
        }
        if price <= Uint128::zero() {
            return Err(ContractError::InvalidPrice {});
        }
        let data_contract_support: ContractSupportResponse = deps.querier.query_wasm_smart(
            contract_info.game_market_payment_contract.clone(),
            &ContractSupportInfo {
                contract_address: token_address.clone(),
            },
        )?;
        if !data_contract_support.is_cw721 {
            return Err(ContractError::OnlySupportCw721 {});
        }
        let id = (contract_info.total_bid.clone() + 1).to_string();
        let bid = Bid {
            id: id.clone(),
            owner: info.sender.clone(),
            token_address: token_address.clone(),
            payment_contract: payment_contract.clone(),
            token_id: token_id.clone(),
            quantity: Uint128::from(1u128),
            price: price.clone(),
            status: true,
            expired,
        };
        contract_info.total_bid += 1;
        self.contract_info.save(deps.storage, &contract_info)?;
        self.bids
            .update(deps.storage, &bid.clone().id, |old| match old {
                Some(_) => Err(ContractError::Added {}),
                None => Ok(bid),
            })?;
        Ok(Response::new()
            .add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: payment_contract.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                    owner: info.sender.to_string(),
                    recipient: env.contract.address.to_string(),
                    amount: price,
                })?,
                funds: vec![],
            })])
            .add_attribute("action", "create_bid")
            .add_attribute("bid_id", id))
    }

    fn update_bid(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        bid_id: String,
        price: Uint128,
        expired: u64,
    ) -> Result<Response, ContractError> {
        let mut bid = self.bids.load(deps.storage, &bid_id)?;
        if !bid.status {
            return Err(ContractError::BidCanceled {});
        }
        if bid.owner != info.sender {
            return Err(ContractError::NotOwner {});
        }
        if price <= Uint128::zero() {
            return Err(ContractError::InvalidPrice {});
        }
        let mut different_price = Uint128::zero();
        let mut messages: Vec<CosmosMsg> = vec![];
        if bid.price > price {
            different_price = Uint128::sub(bid.price, price);
            let mess = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: bid.payment_contract.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: info.sender.clone().to_string(),
                    amount: different_price,
                })?,
                funds: vec![],
            });
            messages.push(mess)
        } else if bid.price < price {
            different_price = Uint128::sub(price, bid.price);
            let mess = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: bid.payment_contract.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                    owner: info.sender.to_string(),
                    recipient: env.contract.address.to_string(),
                    amount: different_price,
                })?,
                funds: vec![],
            });
            messages.push(mess)
        }
        bid.price = price;
        bid.expired = expired;
        self.bids.save(deps.storage, &bid_id, &bid)?;
        Ok(Response::new()
            .add_messages(messages)
            .add_attribute("action", "update_bid")
            .add_attribute("bid_id", bid_id))
        // }
    }

    fn accept_bid(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        bid_id: String,
    ) -> Result<Response, ContractError> {
        let contract_info = self.contract_info.load(deps.storage)?;
        let mut bid = self.bids.load(deps.storage, &bid_id)?;
        let sender = info.sender.clone();
        let data_contract_support: ContractSupportResponse = deps.querier.query_wasm_smart(
            contract_info.game_market_payment_contract.clone(),
            &ContractSupportInfo {
                contract_address: bid.token_address.clone(),
            },
        )?;
        if Timestamp::from_seconds(bid.expired) < env.block.time {
            return Err(ContractError::BidExpired {});
        }
        if !bid.status {
            return Err(ContractError::BidCanceled {});
        }
        bid.quantity = Uint128::zero();
        bid.status = false;
        let mut order_id = String::from("0");
        let mut bundle_id = String::from("0");
        self.bids.save(deps.storage, &bid_id, &bid.clone())?;
        let mut messages: Vec<CosmosMsg> = vec![];
        let owner_address: OwnerOfResponse = deps.querier.query_wasm_smart(
            bid.token_address.to_string().clone(),
            &Cw721QueryMsg::OwnerOf {
                token_id: bid.token_id.clone(),
                include_expired: None,
            },
        )?;
        if owner_address.owner == info.sender.to_string() {
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: bid.token_address.to_string(),
                msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                    recipient: bid.owner.to_string(),
                    token_id: bid.token_id,
                })?,
                funds: vec![],
            }))
        } else {
            let key = bid.token_address.clone().to_string()
                + &bid.token_id.clone()
                + &info.sender.clone().to_string();
            let can_accept = self.can_accept.load(deps.storage, &key);
            let result = match can_accept {
                Ok(info) => info,
                Err(_) => return Err(ContractError::CanNotAcceptBid {}),
            };
            // Ok(result)
            if !result.status {
                return Err(ContractError::CanNotAcceptBid {});
            }
            if result.order_id != String::from("0") {
                let mut order = self.orders.load(deps.storage, &result.order_id)?;
                order.status = false;
                order.quantity = Uint128::zero();
                self.orders.save(deps.storage, &order.id, &order)?;
                self.update_can_accept(
                    deps.storage,
                    false,
                    &bid.token_address,
                    &bid.token_id,
                    &sender,
                    &result.bundle_id,
                    &result.order_id,
                );
                order_id = result.order_id;
            } else if result.bundle_id != String::from("0") {
                let mut bundle = self.bundles.load(deps.storage, &result.bundle_id)?;
                bundle.status = false;
                for (index, _) in bundle.list_token_address.iter().enumerate() {
                    if bundle.list_token_address[index].clone() != bid.token_address.clone()
                        || bundle.list_token_id[index].clone() != bid.token_id.clone()
                    {
                        let mess = CosmosMsg::Wasm(WasmMsg::Execute {
                            contract_addr: bundle.list_token_address[index].clone().to_string(),
                            msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                                recipient: bundle.owner.clone().to_string(),
                                token_id: bundle.list_token_id[index].clone(),
                            })?,
                            funds: vec![],
                        });
                        messages.push(mess);
                    }
                    self.update_can_accept(
                        deps.storage,
                        false,
                        &bundle.list_token_address[index],
                        &bundle.list_token_id[index],
                        &sender,
                        &result.bundle_id,
                        &result.order_id,
                    )
                }
                self.bundles.save(deps.storage, &bundle.id, &bundle)?;
                bundle_id = result.bundle_id;
            }
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: bid.token_address.to_string(),
                msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                    recipient: bid.owner.to_string(),
                    token_id: bid.token_id,
                })?,
                funds: vec![],
            }))
        }
        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: bid.payment_contract.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount: caculate_amount(bid.price, data_contract_support.fee),
            })?,
            funds: vec![],
        }));
        Ok(Response::new()
            .add_messages(messages)
            .add_attribute("action", "accept_bid")
            .add_attribute("bid_id", bid_id)
            .add_attribute("order_id", order_id)
            .add_attribute("bundle_id", bundle_id))
    }

    fn cancel_bid(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        bid_id: String,
    ) -> Result<Response, ContractError> {
        let mut bid = self.bids.load(deps.storage, &bid_id)?;
        if !bid.status {
            return Err(ContractError::BidCanceled {});
        }
        if bid.owner != info.sender {
            return Err(ContractError::NotOwner {});
        }
        bid.quantity = Uint128::zero();
        bid.status = false;
        self.bids.save(deps.storage, &bid_id, &bid)?;
        Ok(Response::new()
            .add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: bid.payment_contract.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: bid.owner.to_string(),
                    amount: bid.price,
                })?,
                funds: vec![],
            })])
            .add_attribute("action", "cancel_bid")
            .add_attribute("bid_id", bid_id))
    }

    fn create_bundle(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        list_token_address: Vec<Addr>,
        list_token_id: Vec<String>,
        payment_contract: Addr,
        price: Uint128,
    ) -> Result<Response, ContractError> {
        let mut contract_info = self.contract_info.load(deps.storage)?;
        if list_token_address.len() == 0
            || list_token_address.len() != list_token_id.len()
            || list_token_address.len() > 20
        {
            return Err(ContractError::InvalidNumberItem {});
        }
        for (index, _) in list_token_address.iter().enumerate() {
            let is_payment_token_supported: bool = deps.querier.query_wasm_smart(
                contract_info.game_market_payment_contract.clone(),
                &IsTokenSupport {
                    contract_address: list_token_address[index].clone(),
                    payment_contract: payment_contract.clone(),
                },
            )?;
            if !is_payment_token_supported {
                return Err(ContractError::PaymentMethodNotSupport {});
            }
            let data_contract_support: ContractSupportResponse = deps.querier.query_wasm_smart(
                contract_info.game_market_payment_contract.clone(),
                &ContractSupportInfo {
                    contract_address: list_token_address[index].clone(),
                },
            )?;
            if !data_contract_support.is_cw721 {
                return Err(ContractError::OnlySupportCw721 {});
            }
            let owner_address: OwnerOfResponse = deps.querier.query_wasm_smart(
                list_token_address[index].to_string().clone(),
                &Cw721QueryMsg::OwnerOf {
                    token_id: list_token_id[index].clone(),
                    include_expired: None,
                },
            )?;
            if owner_address.owner != info.sender.to_string() {
                return Err(ContractError::InsufficienTokenBalance {});
            }
        }
        let owner = info.sender.clone();
        let id = (contract_info.total_bundle.clone() + 1).to_string();
        let bundle = Bundle {
            id: id.clone(),
            owner: info.sender.clone(),
            list_token_address: list_token_address.clone(),
            list_token_id: list_token_id.clone(),
            price: price.clone(),
            status: true,
            payment_contract,
        };
        contract_info.total_bundle += 1;
        self.contract_info.save(deps.storage, &contract_info)?;
        self.bundles
            .update(deps.storage, &bundle.clone().id, |old| match old {
                Some(_) => Err(ContractError::Added {}),
                None => Ok(bundle),
            })?;
        let mut messages: Vec<CosmosMsg> = vec![];
        for (index, _) in list_token_address.iter().enumerate() {
            self.update_can_accept(
                deps.storage,
                true,
                &list_token_address[index],
                &list_token_id[index],
                &owner,
                &id,
                &String::from("0"),
            );
            let mess = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: list_token_address[index].clone().to_string(),
                msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                    recipient: env.contract.address.clone().to_string(),
                    token_id: list_token_id[index].clone(),
                })?,
                funds: vec![],
            });
            messages.push(mess);
        }
        Ok(Response::new()
            .add_messages(messages)
            .add_attribute("action", "create_bundle")
            .add_attribute("bundle_id", id))
    }

    fn buy_bundle(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        bundle_id: String,
    ) -> Result<Response, ContractError> {
        let contract_info = self.contract_info.load(deps.storage)?;
        let mut bundle = self.bundles.load(deps.storage, &bundle_id)?;
        if !bundle.status {
            return Err(ContractError::BundleCanceled {});
        }
        bundle.status = false;
        let owner = bundle.owner.clone();
        self.bundles.save(deps.storage, &bundle_id, &bundle)?;
        let mut messages: Vec<CosmosMsg> = vec![];
        for (index, _) in bundle.list_token_address.iter().enumerate() {
            self.update_can_accept(
                deps.storage,
                false,
                &bundle.list_token_address[index],
                &bundle.list_token_id[index],
                &owner,
                &bundle_id,
                &String::from("0"),
            );
            let mess = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: bundle.list_token_address[index].clone().to_string(),
                msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                    recipient: info.sender.clone().to_string(),
                    token_id: bundle.list_token_id[index].clone(),
                })?,
                funds: vec![],
            });
            messages.push(mess);
        }
        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: bundle.payment_contract.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                owner: info.sender.to_string(),
                recipient: env.contract.address.to_string(),
                amount: bundle.price,
            })?,
            funds: vec![],
        }));
        if bundle.price > Uint128::zero() {
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: bundle.payment_contract.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: bundle.owner.to_string(),
                    amount: caculate_amount(bundle.price, contract_info.bundle_fee),
                })?,
                funds: vec![],
            }));
        }
        Ok(Response::new()
            .add_messages(messages)
            .add_attribute("action", "buy_bundle")
            .add_attribute("bundle_id", bundle_id))
    }

    fn cancel_bundle(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        bundle_id: String,
    ) -> Result<Response, ContractError> {
        let mut bundle = self.bundles.load(deps.storage, &bundle_id)?;
        if !bundle.status {
            return Err(ContractError::BundleCanceled {});
        }
        if bundle.owner != info.sender {
            return Err(ContractError::NotOwner {});
        }
        bundle.status = false;
        let owner = info.sender.clone();
        self.bundles.save(deps.storage, &bundle_id, &bundle)?;
        let mut messages: Vec<CosmosMsg> = vec![];
        for (index, _) in bundle.list_token_address.iter().enumerate() {
            self.update_can_accept(
                deps.storage,
                false,
                &bundle.list_token_address[index],
                &bundle.list_token_id[index],
                &owner,
                &bundle_id,
                &String::from("0"),
            );
            let mess = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: bundle.list_token_address[index].clone().to_string(),
                msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                    recipient: bundle.owner.clone().to_string(),
                    token_id: bundle.list_token_id[index].clone(),
                })?,
                funds: vec![],
            });
            messages.push(mess);
        }
        Ok(Response::new()
            .add_messages(messages)
            .add_attribute("action", "cancel_bundle")
            .add_attribute("bundle_id", bundle_id))
    }

    fn update_bundle_fee(
        &self,
        deps: DepsMut,
        _info: MessageInfo,
        bundle_fee: u16,
    ) -> Result<Response, ContractError> {
        let mut contract_info = self.contract_info.load(deps.storage)?;
        contract_info.bundle_fee = bundle_fee;
        self.contract_info.save(deps.storage, &contract_info)?;
        Ok(Response::new().add_attribute("action", "update_bundle_fee"))
    }

    fn update_game_market_payment_contract(
        &self,
        deps: DepsMut,
        _info: MessageInfo,
        game_market_payment_contract: Addr,
    ) -> Result<Response, ContractError> {
        let mut contract_info = self.contract_info.load(deps.storage)?;
        contract_info.game_market_payment_contract = game_market_payment_contract;
        self.contract_info.save(deps.storage, &contract_info)?;
        Ok(Response::new().add_attribute("action", "update_game_market_payment_contract"))
    }
}

fn caculate_amount(amount: Uint128, fee: u16) -> Uint128 {
    return Uint128::div(
        Uint128::mul(
            amount,
            Uint128::sub(Uint128::from(ZOOM_FEE), Uint128::from(fee)),
        ),
        Uint128::from(ZOOM_FEE),
    );
}
