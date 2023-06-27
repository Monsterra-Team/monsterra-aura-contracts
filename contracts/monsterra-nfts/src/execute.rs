use crate::error::MonsterraNFTError;
use crate::MonsterraNFT;

use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use cw721::Cw721Execute;
use cw721_base::ContractError;
use sha2::{Digest, Sha256};

use crate::msg::{MintBatchMsg, MintBatchWithSignatureMsg, MintBatchWithSignaturePayload, MintMsg};
use crate::state::{get_signer, is_admin, is_used_nonce, set_used_nonce, STAKE_OWNERS};
use crate::Extension;

pub fn mint_batch(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MintBatchMsg<Extension>,
) -> Result<Response, MonsterraNFTError> {
    if !is_admin(deps.storage, info.sender.clone()) {
        return Err(MonsterraNFTError::Unauthorized {});
    }

    for msg in msg.msgs {
        mint(deps.branch(), env.clone(), info.clone(), msg.clone())?;
    }
    Ok(Response::new().add_attribute("action", "mint_batch"))
}

pub fn stake_batch(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_ids: Vec<String>,
) -> Result<Response, MonsterraNFTError> {
    let contract = MonsterraNFT::default();
    for token_id in token_ids {
        contract.transfer_nft(
            deps.branch(),
            env.clone(),
            info.clone(),
            env.clone().contract.address.into_string(),
            token_id.clone(),
        )?;

        let log = |d: Option<String>| -> StdResult<String> {
            match d {
                Some(_one) => Ok(info.sender.clone().into_string()),
                None => Ok(info.sender.clone().into_string()),
            }
        };

        STAKE_OWNERS.update(deps.storage, token_id.clone(), log)?;
    }
    Ok(Response::new().add_attribute("action", "stake_batch"))
}

fn mint(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MintMsg<Extension>,
) -> Result<Response<Empty>, ContractError> {
    let contract = MonsterraNFT::default();

    let owner = STAKE_OWNERS.may_load(deps.storage, msg.token_id.clone())?;

    match owner {
        Some(_) => {
            // let transfer_msg = MonsterraExecuteMsg::<Extension>::TransferNft {
            //   recipient: String::from("merlin"),
            //   token_id: msg.token_id.clone(),
            // };

            // let res = CosmosMsg::Wasm(WasmMsg::Execute {
            //   contract_addr: env.contract.address.clone().into_string(),
            //   msg: to_binary(&transfer_msg)?,
            //   funds: vec![],
            // });

            contract
                .tokens
                .remove(deps.storage, msg.token_id.as_str())?;

            let res = contract.mint(
                deps.branch(),
                info.clone(),
                msg.token_id.clone(),
                msg.owner.clone(),
                msg.token_uri.clone(),
                msg.extension.clone(),
            )?;

            STAKE_OWNERS.remove(deps.storage, msg.token_id.clone());
            Ok(res)
        }
        None => {
            let res = contract.mint(
                deps.branch(),
                info.clone(),
                msg.token_id.clone(),
                msg.owner.clone(),
                msg.token_uri.clone(),
                msg.extension.clone(),
            )?;
            Ok(res)
        }
    }
}

pub fn mint_batch_with_signature(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MintBatchWithSignatureMsg<Extension>,
    signature: Binary,
) -> Result<Response, MonsterraNFTError> {
    let MintBatchWithSignatureMsg {
        msgs,
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
            msgs: msgs.clone(),
        },
        signature,
    ) {
        return Err(MonsterraNFTError::InvalidSignature {});
    }

    for msg in msgs {
        if info.sender != msg.owner {
            return Err(MonsterraNFTError::Unauthorized {});
        }
        mint(deps.branch(), env.clone(), info.clone(), msg)?;
    }
    Ok(Response::new().add_attribute("action", "mint_batch"))
}

fn verify_sig(
    deps: Deps,
    mint_payload: &MintBatchWithSignaturePayload<Extension>,
    signature: Binary,
) -> bool {
    let msg: Binary;
    let result = to_binary(mint_payload);
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
