// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, Binary, MessageInfo, Response, Storage, Timestamp, Uint128, Uint256,
};
use cw_storage_plus::{Item, Map};

use crate::ContractError;

#[cw_serde]
pub struct StakeData {
    pub amount: Uint128,
    pub duration: Uint256,
    pub token: Addr,
    pub time: Timestamp,
}

pub const OWNER: Item<Addr> = Item::new("owner");
pub const ADMIN: Map<Addr, bool> = Map::new("admin");
pub const SIGNER: Item<Binary> = Item::new("signer");

pub const ACCEPTED_TOKENS: Map<Addr, bool> = Map::new("accepted_token");
pub const MAX_STAKE_DURATION: Item<Uint256> = Item::new("max_stake_duration");
pub const TOTAL_STAKED: Map<Addr, Uint128> = Map::new("total_staked");
pub const STAKED_DATA: Map<Addr, Vec<StakeData>> = Map::new("total_staked");
pub const USED_NONCES: Map<String, bool> = Map::new("used_nonces");

pub fn set_new_owner(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    user: Addr,
) -> Result<Response, ContractError> {
    if info.sender != get_owner(storage) {
        return Err(ContractError::Unauthorized {});
    }

    let result = OWNER.save(storage, &user);
    match result {
        Ok(_) => Ok(Response::new().add_attribute("method", "transfer_ownership")),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn get_owner(storage: &dyn Storage) -> Addr {
    OWNER.load(storage).unwrap()
}

pub fn set_admin(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    user: Addr,
    status: bool,
) -> Result<Response, ContractError> {
    if info.sender != get_owner(storage) {
        return Err(ContractError::Unauthorized {});
    }

    let result = ADMIN.save(storage, user.clone(), &status);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_admin")
            .add_attribute("user", user)
            .add_attribute("status", status.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn is_admin(storage: &dyn Storage, user: Addr) -> bool {
    let result = ADMIN.load(storage, user);
    match result {
        Ok(value) => value,
        Err(_) => false,
    }
}

pub fn set_signer(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    public_key: Binary,
) -> Result<Response, ContractError> {
    if !is_admin(storage, info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    let result = SIGNER.save(storage, &public_key);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_signer")
            .add_attribute("public_key", public_key.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn get_signer(storage: &dyn Storage) -> Binary {
    let result = SIGNER.load(storage);
    match result {
        Ok(value) => value,
        Err(_) => {
            let mes = "";
            to_binary(&mes).unwrap()
        }
    }
}

pub fn set_accepted_token(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    token: &Addr,
    status: bool,
) -> Result<Response, ContractError> {
    if !is_admin(storage, info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    let result = ACCEPTED_TOKENS.save(storage, token.clone(), &status);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_accepted_token")
            .add_attribute("accpeted_token", token.to_string())
            .add_attribute("status", status.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn is_accepted_token(storage: &dyn Storage, token: Addr) -> bool {
    let result = ACCEPTED_TOKENS.load(storage, token);
    match result {
        Ok(value) => value,
        Err(_) => false,
    }
}

pub fn set_used_nonce(
    storage: &mut dyn Storage,
    nonce: String,
    value: bool,
) -> Result<Response, ContractError> {
    let result = USED_NONCES.save(storage, nonce.clone(), &value);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_used_nonce")
            .add_attribute("nonce", nonce)
            .add_attribute("status", value.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn is_used_nonce(storage: &dyn Storage, nonce: String) -> bool {
    let result = USED_NONCES.load(storage, nonce);
    match result {
        Ok(value) => value,
        Err(_) => false,
    }
}

pub fn set_total_staked(
    storage: &mut dyn Storage,
    user: &Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let result = TOTAL_STAKED.save(storage, user.clone(), &amount);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_used_nonce")
            .add_attribute("user", user.to_string())
            .add_attribute("amount", amount.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn get_total_staked(storage: &dyn Storage, user: Addr) -> Uint128 {
    let result = TOTAL_STAKED.load(storage, user);
    match result {
        Ok(value) => value,
        Err(_) => Uint128::zero(),
    }
}

pub fn set_staked_data(
    storage: &mut dyn Storage,
    user: &Addr,
    data: StakeData,
) -> Result<Response, ContractError> {
    let mut staked_data = match STAKED_DATA.load(storage, user.clone()) {
        Ok(vec) => vec,
        Err(_) => Vec::new(),
    };

    staked_data.push(data.clone());

    let result = STAKED_DATA.save(storage, user.clone(), &staked_data);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_staked_data")
            .add_attribute("user", user.to_string())
            .add_attribute("amount", data.amount.to_string())
            .add_attribute("duration", data.duration.to_string())
            .add_attribute("token", data.token.to_string())
            .add_attribute("time", data.time.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn get_staked_data(storage: &dyn Storage, user: Addr) -> Vec<StakeData> {
    let result = STAKED_DATA.load(storage, user);
    match result {
        Ok(value) => value,
        Err(_) => Vec::new(),
    }
}
