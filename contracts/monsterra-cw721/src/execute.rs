use crate::error::MonsterraNFTError;
use crate::{ExecuteMsg, MonsterraNFT};

use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg,
};
use cw721::Cw721Execute;
use cw721_base::state::TokenInfo;
use cw721_base::ContractError;
use sha2::{Digest, Sha256};

use crate::msg::{MintBatchMsg, MintBatchWithSignatureMsg, MintBatchWithSignaturePayload, MintMsg};
use crate::state::{get_signer, is_admin, is_used_nonce, set_used_nonce, STAKE_OWNERS};
use crate::Extension;

pub fn mint_batch(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MintBatchMsg<Extension>,
) -> Result<Response, MonsterraNFTError> {
    if !is_admin(deps.storage, info.sender.clone()) {
        return Err(MonsterraNFTError::Unauthorized {});
    }

    let mut res = Response::new();

    for msg in msg.msgs {
        res = res.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&ExecuteMsg::InternalMint { msg })?,
            funds: vec![],
        }));
    }
    Ok(res.add_attribute("action", "mint_batch"))
}

pub fn stake_batch(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_ids: Vec<String>,
) -> Result<Response, MonsterraNFTError> {
    let contract = MonsterraNFT::default();
    let mut res = Response::new();
    for token_id in token_ids {
        let token = contract.tokens.load(deps.storage, &token_id)?;
        // we must check permissions here
        contract.check_can_send(deps.as_ref(), &env, &info, &token)?;

        res = res.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&ExecuteMsg::InternalTransfer {
                sender: info.sender.to_string(),
                recipient: env.clone().contract.address.into_string(),
                token_id: token_id.clone(),
            })?,
            funds: vec![],
        }));

        let log =
            |_d: Option<String>| -> StdResult<String> { Ok(info.sender.clone().into_string()) };

        STAKE_OWNERS.update(deps.storage, token_id.clone(), log)?;
    }
    Ok(res.add_attribute("action", "stake_batch"))
}

pub fn mint_batch_with_signature(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MintBatchWithSignatureMsg,
    signature: Binary,
) -> Result<Response, MonsterraNFTError> {
    let MintBatchWithSignatureMsg {
        token_ids,
        nonce,
        timestamp,
    } = msg.clone();

    if timestamp.plus_seconds(60 * 2).gt(&env.block.time) {
        return Err(MonsterraNFTError::TimeExpired {});
    }

    if is_used_nonce(deps.storage, nonce.clone()) {
        return Err(MonsterraNFTError::NonceUsed {});
    }
    set_used_nonce(deps.storage, nonce.clone(), true)?;

    if !verify_sig(
        deps.as_ref(),
        &MintBatchWithSignaturePayload {
            sender: info.sender.clone(),
            nonce,
            timestamp,
            token_ids: token_ids.clone(),
        },
        signature,
    ) {
        return Err(MonsterraNFTError::InvalidSignature {});
    }

    let mut res = Response::new();

    for token_id in token_ids {
        res = res.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&ExecuteMsg::InternalMint {
                msg: MintMsg {
                    token_id,
                    owner: info.sender.to_string(),
                    token_uri: None,
                    extension: None,
                },
            })?,
            funds: vec![],
        }));
    }
    Ok(res.add_attribute("action", "mint_batch"))
}

pub fn internal_mint(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MintMsg<Extension>,
) -> Result<Response, MonsterraNFTError> {
    let contract = MonsterraNFT::default();

    let stake_owner = STAKE_OWNERS.may_load(deps.storage, msg.token_id.clone())?;

    match stake_owner {
        Some(_) => {
            STAKE_OWNERS.remove(deps.storage, msg.token_id.clone());

            Ok(contract.transfer_nft(
                deps.branch(),
                env.clone(),
                info.clone(),
                msg.owner.clone(),
                msg.token_id.clone(),
            )?)
        }
        None => {
            let token = TokenInfo {
                owner: deps.api.addr_validate(&msg.owner)?,
                approvals: vec![],
                token_uri: None,
                extension: None,
            };
            contract
                .tokens
                .update(deps.storage, &msg.token_id, |old| match old {
                    Some(_) => Err(ContractError::Claimed {}),
                    None => Ok(token),
                })?;

            contract.increment_tokens(deps.storage)?;

            Ok(Response::new()
                .add_attribute("action", "mint")
                .add_attribute("minter", info.sender)
                .add_attribute("owner", msg.owner)
                .add_attribute("token_id", msg.token_id))
        }
    }
}

pub fn internal_transfer_nft(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    sender: String,
    recipient: String,
    token_id: String,
) -> Result<Response, MonsterraNFTError> {
    let contract = MonsterraNFT::default();

    let mut token = contract.tokens.load(deps.storage, &token_id)?;

    // set owner and remove existing approvals
    token.owner = deps.api.addr_validate(&recipient)?;
    token.approvals = vec![];
    contract.tokens.save(deps.storage, &token_id, &token)?;

    Ok(Response::new()
        .add_attribute("action", "transfer_nft")
        .add_attribute("sender", sender)
        .add_attribute("recipient", recipient)
        .add_attribute("token_id", token_id))
}

fn verify_sig(deps: Deps, mint_payload: &MintBatchWithSignaturePayload, signature: Binary) -> bool {
    let result = to_binary(mint_payload);
    let msg: Binary = match result {
        Ok(value) => value,
        Err(_) => return false,
    };
    let signer = get_signer(deps.storage);

    let hash = Sha256::digest(msg);
    let result = deps
        .api
        .secp256k1_verify(hash.as_ref(), &signature, &signer.0);
    result.unwrap_or(false)
}
