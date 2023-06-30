use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw20_base::state::{BALANCES, TOKEN_INFO};
use cw20_base::ContractError as CW20Error;

use sha2::{Digest, Sha256};

use crate::state::{is_used_nonce, set_used_nonce};
use crate::{
    error::ContractError,
    msg::{MintMsg, MintPayload},
    state::get_signer,
};

pub fn mint_with_signature(
    deps: DepsMut,
    env: Env,
    info: &MessageInfo,
    msg: MintMsg,
    signature: Binary,
) -> Result<Response, ContractError> {
    let MintMsg {
        amount,
        nonce,
        timestamp,
    } = msg;

    if timestamp.plus_seconds(60 * 2).gt(&env.block.time) {
        return Err(ContractError::TimeExpired {});
    }

    if is_used_nonce(deps.storage, nonce.clone()) {
        return Err(ContractError::NonceUsed {});
    }
    set_used_nonce(deps.storage, nonce.clone(), true)?;

    if !verify_sig(
        deps.as_ref(),
        &MintPayload {
            sender: info.sender.clone(),
            amount,
            nonce,
            timestamp,
        },
        signature,
    ) {
        return Err(ContractError::InvalidSignature {});
    }

    let mut config = TOKEN_INFO
        .may_load(deps.storage)?
        .ok_or(ContractError::Unauthorized {})?;

    // update supply and enforce cap
    config.total_supply += amount;
    if let Some(limit) = config.get_cap() {
        if config.total_supply > limit {
            return Err(ContractError::CW20(CW20Error::CannotExceedCap {}));
        }
    }
    TOKEN_INFO.save(deps.storage, &config)?;

    // add amount to recipient balance
    let rcpt_addr = deps.api.addr_validate(&info.sender.to_string())?;
    BALANCES.update(
        deps.storage,
        &rcpt_addr,
        |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + amount) },
    )?;

    let res = Response::new()
        .add_attribute("action", "mint")
        .add_attribute("to", info.sender.to_string())
        .add_attribute("amount", amount);
    Ok(res)
}

fn verify_sig(deps: Deps, unstake_payload: &MintPayload, signature: Binary) -> bool {
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
