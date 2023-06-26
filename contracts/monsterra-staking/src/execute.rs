use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    Storage, Uint128, Uint256, WasmMsg,
};
use cw20::Cw20ExecuteMsg;
use sha2::{Digest, Sha256};

use crate::{
    msg::{UnstakeMsg, UnstakePayload},
    state::{
        get_signer, get_total_staked, is_accepted_token, is_used_nonce, set_accepted_token,
        set_admin, set_new_owner, set_signer, set_staked_data, set_total_staked, set_used_nonce,
        StakeData,
    },
    ContractError,
};

pub fn execute_transfer_ownership(
    storage: &mut dyn Storage,
    info: MessageInfo,
    user: Addr,
) -> Result<Response, ContractError> {
    set_new_owner(storage, &info, user)
}

pub fn execute_set_admin(
    storage: &mut dyn Storage,
    info: MessageInfo,
    user: Addr,
    status: bool,
) -> Result<Response, ContractError> {
    set_admin(storage, &info, user, status)
}

pub fn execute_set_accepted_token(
    storage: &mut dyn Storage,
    info: MessageInfo,
    token: Addr,
    status: bool,
) -> Result<Response, ContractError> {
    set_accepted_token(storage, &info, &token, status)
}

pub fn execute_set_signer(
    storage: &mut dyn Storage,
    info: MessageInfo,
    public_key: Binary,
) -> Result<Response, ContractError> {
    set_signer(storage, &info, public_key)
}

pub fn execute_stake(
    storage: &mut dyn Storage,
    env: Env,
    info: MessageInfo,
    token: &Addr,
    amount: Uint128,
    duration: Uint256,
) -> Result<Response, ContractError> {
    if !is_accepted_token(storage, token.clone()) {
        return Err(ContractError::NotAcceptedToken {});
    }

    let mut messages: Vec<CosmosMsg> = vec![];
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
            owner: info.sender.to_string(),
            recipient: env.contract.address.to_string(),
            amount,
        })?,
        funds: vec![],
    }));

    set_staked_data(
        storage,
        &info.sender,
        StakeData {
            amount,
            duration,
            token: token.clone(),
            time: env.block.time,
        },
    )?;

    let mut total_staked = get_total_staked(storage, info.sender.clone());

    total_staked = total_staked
        .checked_add(amount)
        .map_err(StdError::overflow)?;

    set_total_staked(storage, &info.sender.clone(), total_staked)?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("method", "stake")
        .add_attributes(vec![
            ("sender", info.sender.to_string()),
            ("token", token.to_string()),
            ("amount", amount.to_string()),
            ("duration", duration.to_string()),
        ]))
}

pub fn execute_unstake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: UnstakeMsg,
    signature: Binary,
) -> Result<Response, ContractError> {
    let UnstakeMsg {
        ref token,
        amount,
        nonce,
        timestamp,
    } = msg;

    if !is_accepted_token(deps.storage, token.clone()) {
        return Err(ContractError::NotAcceptedToken {});
    }

    if timestamp.plus_seconds(60 * 2).gt(&env.block.time) {
        return Err(ContractError::TimeExpired {});
    }

    if is_used_nonce(deps.storage, nonce.clone()) {
        return Err(ContractError::NonceUsed {});
    }

    set_used_nonce(deps.storage, nonce.clone(), true)?;

    if !verify_sig(
        deps.as_ref(),
        &UnstakePayload {
            sender: info.sender.clone(),
            token: token.clone(),
            amount,
            nonce,
            timestamp,
        },
        signature,
    ) {
        return Err(ContractError::InvalidSignature {});
    }

    let mut messages: Vec<CosmosMsg> = vec![];
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: msg.token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount: msg.amount,
        })?,
        funds: vec![],
    }));

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("method", "unstake")
        .add_attributes(vec![
            ("sender", info.sender.to_string()),
            ("token", msg.token.to_string()),
            ("amount", msg.amount.to_string()),
        ]))
}

fn verify_sig(deps: Deps, unstake_payload: &UnstakePayload, signature: Binary) -> bool {
    let msg: Binary;
    let result = to_binary(unstake_payload);
    match result {
        Ok(value) => msg = value,
        Err(_) => return false,
    };
    let signer = get_signer(deps.storage);

    let hash = Sha256::digest(msg);
    let result = deps
        .api
        .secp256k1_verify(hash.as_ref(), &signature, &signer.0);
    match result {
        Ok(value) => value,
        Err(_) => false,
    }
}
