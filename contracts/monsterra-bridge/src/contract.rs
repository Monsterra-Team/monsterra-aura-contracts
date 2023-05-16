#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, Uint128,CosmosMsg,WasmMsg};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg};

use sha2::{Digest, Sha256};
use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg, SwapMessage,MigrateMsg};
use crate::state::{
    set_new_owner, get_owner, OWNER,
    set_admin, is_admin,
    set_signer, get_signer,
    set_accepted_token, is_accepted_token,
    set_accepted_des_token, is_accepted_des_token,
    set_swap_data, get_swap_data, Swapdata
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:monstera-terra";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION"); 

//allow contract migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.save(deps.storage, &info.sender)?;
    set_admin(deps.storage, &info, info.sender.clone(), true)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
       )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TransferOwnerShip {user} => try_transfer_ownership(deps.storage, info, user),
        ExecuteMsg::SetAdmin { user, status } => try_set_admin(deps.storage, info, user, status),
        ExecuteMsg::SetAcceptedToken {token, status} => try_set_accepted_token(deps.storage, info, token, status),
        ExecuteMsg::SetAcceptedDesToken{token, status} => try_set_accepted_des_token(deps.storage, info, token, status),
        ExecuteMsg::SetSigner{public_key} => try_set_signer(deps.storage, info, public_key),
        ExecuteMsg::Mint{swap_message, signature} => try_mint(deps, info,swap_message, signature),
        ExecuteMsg::Burn{swap_message} => try_burn(deps.storage, info, swap_message)
    }
}

pub fn try_transfer_ownership(storage: &mut dyn Storage, info: MessageInfo, user:Addr) -> Result<Response, ContractError> {
    set_new_owner(storage, &info, user)
}

pub fn try_set_admin(storage: &mut dyn Storage, info: MessageInfo, user:Addr, status: bool) -> Result<Response, ContractError> {
    set_admin(storage, &info, user, status)
}

pub fn try_set_accepted_token(storage: &mut dyn Storage, info: MessageInfo, token:String, status: bool) -> Result<Response, ContractError> {
    set_accepted_token(storage, &info, token, status)
}

pub fn try_set_accepted_des_token(storage: &mut dyn Storage, info: MessageInfo, token:String, status: bool) -> Result<Response, ContractError> {
    set_accepted_des_token(storage, &info, token, status)
}

pub fn try_set_signer(storage: &mut dyn Storage, info: MessageInfo, public_key:Binary) -> Result<Response, ContractError> {
    set_signer(storage, &info, public_key)
}

pub fn try_mint(deps: DepsMut, info: MessageInfo, swap_message: SwapMessage, signature: Binary) -> Result<Response, ContractError>{
    //check requirements 
    if is_transaction_existed(deps.storage, swap_message.transaction_id.clone()){
        return Err(ContractError::TransactionExisted{});
    }

    if !_validate_swap_data(deps.storage, info.sender, &swap_message){
        return Err(ContractError::InvalidSwapData{});
    }
     
    if !is_valid_swap_message(deps.as_ref(), &swap_message, signature){
        return Err(ContractError::InvalidSignature{});
    }

    //save swap data
    let swap_data = Swapdata::default(&swap_message, String::from("mint"));
    set_swap_data(deps.storage, swap_message.transaction_id.clone(), swap_data)?;

    //mint new tokens for corresponding accounts 
    let mut messages: Vec<CosmosMsg> = vec![];
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: swap_message.cur_token.clone(),
        msg: to_binary(&Cw20ExecuteMsg::Mint{
            recipient: swap_message.cur_user.clone(),
            amount: swap_message.amount.clone()
        })?,
        funds: vec![],
    }));

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("method", "mint"),
        ("transaction_id", &swap_message.transaction_id),
        ("cur_token", &swap_message.cur_token),
        ("des_token", &swap_message.des_token),
        ("cur_user", &swap_message.cur_user),
        ("des_user", &swap_message.des_user),
        ("swap_amount", &swap_message.amount.to_string()),
        ("side", "mint")
    ]))
}

pub fn try_burn(storage: &mut dyn Storage, info: MessageInfo, swap_message: SwapMessage) -> Result<Response, ContractError>{
    if is_transaction_existed(storage, swap_message.transaction_id.clone()){
        return Err(ContractError::TransactionExisted{});
    }

    if  !_validate_swap_data(storage, info.sender, &swap_message){
        return Err(ContractError::InvalidSwapData{});
    }
    //save swap data
    let swap_data = Swapdata::default(&swap_message, String::from("burn"));
    set_swap_data(storage, swap_message.transaction_id.clone(), swap_data)?;

    //burn tokens of corresponding accounts
    let mut messages: Vec<CosmosMsg> = vec![];
    
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: swap_message.cur_token.clone(),
        msg: to_binary(&Cw20ExecuteMsg::BurnFrom{
            owner: swap_message.cur_user.clone(),
            amount: swap_message.amount.clone()
        })?,
        funds: vec![],
    }));

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("method", "burn"),
        ("transaction_id", &swap_message.transaction_id),
        ("cur_token", &swap_message.cur_token),
        ("des_token", &swap_message.des_token),
        ("cur_user", &swap_message.cur_user),
        ("des_user", &swap_message.des_user),
        ("swap_amount", &swap_message.amount.to_string()),
        ("side", "burn")
    ]))
}


fn is_transaction_existed(storage: &mut dyn Storage, transaction_id:String) -> bool {
    let data = get_swap_data(storage, transaction_id);

    if data.amount == Uint128::new(0) {false} else {true}
}

fn _validate_swap_data(storage: &mut dyn Storage, sender: Addr, swap_message: &SwapMessage)-> bool {
    if swap_message.amount == Uint128::new(0){
        return false;
    }

    if !is_accepted_token(storage, &swap_message.cur_token) || !is_accepted_des_token(storage, &swap_message.des_token) {
        return false;
    }

    if sender != swap_message.cur_user {
        return false;
    }

    return true;
}

fn is_valid_swap_message(deps: Deps, swap_message: &SwapMessage, signature: Binary)-> bool{
    let msg:Binary;
    let result = to_binary(swap_message);   
    match result {
        Ok(value) => {msg = value},
        Err(_) => return false
    };
    let signer = get_signer(deps.storage);

    query_verify_cosmos(deps, &msg, &signature, &signer.0)
}

fn query_verify_cosmos(
    deps: Deps,
    message: &[u8],
    signature: &[u8],
    public_key: &[u8]
) -> bool {
    let hash  = Sha256::digest(message);
    let result = deps.api.secp256k1_verify(hash.as_ref(), signature, public_key);
    match result {
        Ok(value) => value,
        Err(_) => false
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps.storage)),
        QueryMsg::IsAdmin{user} => to_binary(&query_admin(deps.storage, user)),
        QueryMsg::GetSigner{} => to_binary(&query_signer(deps.storage)),
        QueryMsg::IsAcceptedToken{token} => to_binary(&query_accepted_token(deps.storage, token)),
        QueryMsg::IsAcceptedDesToken{token} => to_binary(&query_accepted_des_token(deps.storage, token)),
        QueryMsg::GetSwapData{transaction_id} => to_binary(&query_swap_data(deps.storage, transaction_id)),
        QueryMsg::Test{} => to_binary(&query_test())
    }
}

// test function for migrating 
fn query_test() -> Uint128{
    Uint128::new(100)
}

fn query_owner(storage: &dyn Storage) -> Addr {
    get_owner(storage)
}

fn query_admin(storage: &dyn Storage, user: Addr) -> bool {
    is_admin(storage, user)
}

fn query_signer(storage: &dyn Storage) -> Binary {
    get_signer(storage)
}

fn query_accepted_token(storage: &dyn Storage, token: String) -> bool {
    is_accepted_token(storage, &token)
}

fn query_accepted_des_token(storage: &dyn Storage, token: String) -> bool {
    is_accepted_des_token(storage, &token)
}


fn query_swap_data(storage: &dyn Storage, transaction_id: String) -> Swapdata {
    get_swap_data(storage, transaction_id)
}

