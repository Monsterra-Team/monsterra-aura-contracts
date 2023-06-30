use cosmwasm_std::{to_binary, Addr, Binary, MessageInfo, Response, Storage};
// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus
use cw_storage_plus::{Item, Map};

use crate::error::MonsterraNFTError;

pub const ADMIN: Map<Addr, bool> = Map::new("admin");
pub const SIGNER: Item<Binary> = Item::new("signer");

pub const BASE_URI: Item<String> = Item::new("base_uri");

pub const STAKE_OWNERS: Map<String, String> = Map::new("stake_owners");
pub const USED_NONCES: Map<String, bool> = Map::new("used_nonces");

pub fn set_admin(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    user: Addr,
    status: bool,
) -> Result<Response, MonsterraNFTError> {
    match cw_ownable::assert_owner(storage, &info.sender) {
        Ok(_) => {}
        Err(error) => {
            return Err(MonsterraNFTError::CW721(
                cw721_base::ContractError::Ownership(error),
            ))
        }
    };

    let result = ADMIN.save(storage, user.clone(), &status);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_admin")
            .add_attribute("user", user)
            .add_attribute("status", status.to_string())),
        Err(_) => Err(MonsterraNFTError::Internal {}),
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
) -> Result<Response, MonsterraNFTError> {
    if !is_admin(storage, info.sender.clone()) {
        return Err(MonsterraNFTError::Unauthorized {});
    }

    let result = SIGNER.save(storage, &public_key);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_signer")
            .add_attribute("public_key", public_key.to_string())),
        Err(_) => Err(MonsterraNFTError::Internal {}),
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
) -> Result<Response, MonsterraNFTError> {
    let result = USED_NONCES.save(storage, nonce.clone(), &value);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_used_nonce")
            .add_attribute("nonce", nonce)
            .add_attribute("status", value.to_string())),
        Err(_) => Err(MonsterraNFTError::Internal {}),
    }
}

pub fn is_used_nonce(storage: &dyn Storage, nonce: String) -> bool {
    let result = USED_NONCES.load(storage, nonce);
    match result {
        Ok(value) => value,
        Err(_) => false,
    }
}

pub fn set_base_uri(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    base_uri: String,
) -> Result<Response, MonsterraNFTError> {
    if !is_admin(storage, info.sender.clone()) {
        return Err(MonsterraNFTError::Unauthorized {});
    }

    let result = BASE_URI.save(storage, &base_uri);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_base_uri")
            .add_attribute("base_uri", base_uri)),
        Err(_) => Err(MonsterraNFTError::Internal {}),
    }
}

pub fn get_base_uri(storage: &dyn Storage) -> String {
    let result = BASE_URI.load(storage);
    match result {
        Ok(value) => value,
        Err(_) => String::from(""),
    }
}
