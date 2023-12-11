use cosmwasm_std::{to_binary, Addr, Binary, MessageInfo, Response, Storage};
// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus
use cw_storage_plus::{Item, Map};

use crate::error::ContractError;

pub const ADMIN: Map<Addr, bool> = Map::new("admin");
pub const SIGNER: Item<Binary> = Item::new("signer");
pub const BOX_CONTRACTS: Map<Addr, bool> = Map::new("box_contracts");

pub fn set_admin(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    user: Addr,
    status: bool,
) -> Result<Response, ContractError> {
    match cw_ownable::assert_owner(storage, &info.sender) {
        Ok(_) => {}
        Err(error) => return Err(ContractError::Ownership(error)),
    };

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
    result.unwrap_or(false)
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

pub fn set_box_contract(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    box_contract: Addr,
    status: bool,
) -> Result<Response, ContractError> {
    if !is_admin(storage, info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    let result = BOX_CONTRACTS.save(storage, box_contract.clone(), &status);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_box_contract")
            .add_attribute("contract", box_contract)
            .add_attribute("status", status.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn is_box_contract(storage: &dyn Storage, box_contract: Addr) -> bool {
    let result = BOX_CONTRACTS.load(storage, box_contract);
    result.unwrap_or(false)
}
