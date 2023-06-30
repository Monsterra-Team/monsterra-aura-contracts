// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Binary, MessageInfo, Response, Storage, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

use crate::error::ContractError;

#[cw_serde]
pub struct StakeData {
    pub amount: Uint128,
    pub duration: u8,
    pub token: Addr,
    pub time: Timestamp,
}

pub const OWNER: Item<Addr> = Item::new("owner");
pub const ADMIN: Map<Addr, bool> = Map::new("admin");
pub const SIGNER: Item<Binary> = Item::new("signer");

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
