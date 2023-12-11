use crate::msg::{ConvertMsg, ConvertPayload, MintMsg, MonsterraNFTMsg};
use crate::state::{get_signer, is_box_contract};
use crate::ContractError;
use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, DepsMut, Env, MessageInfo, QueryRequest, Response,
    StdResult, WasmMsg, WasmQuery,
};
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg};
use sha2::{Digest, Sha256};

pub fn convert(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ConvertMsg,
    signature: Binary,
) -> Result<Response, ContractError> {
    let ConvertMsg {
        boxes,
        nfts,
        timestamp,
    } = msg.clone();

    if timestamp.plus_seconds(60 * 2).gt(&env.block.time) {
        return Err(ContractError::TimeExpired {});
    }

    // check signature
    if !verify_sig(
        &deps,
        &&ConvertPayload {
            sender: info.sender.clone(),
            boxes: boxes.clone(),
            nfts: nfts.clone(),
            timestamp,
        },
        signature,
    ) {
        return Err(ContractError::InvalidSignature {});
    }

    // sub message vector
    let mut messages: Vec<CosmosMsg> = vec![];

    if boxes.len() == 0 {
        return Err(ContractError::InvalidNftInfo {});
    }

    // verify input nfts
    for box_nft in boxes.iter() {
        // verify input box contract
        if !is_box_contract(deps.storage, box_nft.contract_addr.clone()) {
            return Err(ContractError::InvalidBoxContract {});
        }

        let owner: String = query_nft_owner(
            &deps,
            box_nft.contract_addr.clone(),
            box_nft.token_id.clone(),
        )?;

        if owner != info.sender.to_string() {
            return Err(ContractError::NotOwnedNFT {});
        }

        // check if spender allowed this contract use the NFT
        // check_nft_approval(
        //     &deps,
        //     box_nft.contract_addr.clone(),
        //     box_nft.token_id.clone(),
        //     env.contract.address.clone(),
        // )?;
        // burn nfts
        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: box_nft.contract_addr.to_string().clone(),
            msg: to_binary(&Cw721ExecuteMsg::Burn {
                token_id: box_nft.token_id.clone(),
            })?,
            funds: vec![],
        }));
    }

    // mint nfts
    if nfts.len() == 0 {
        return Err(ContractError::InvalidNftInfo {});
    }

    for nft in nfts.iter() {
        // check if this contract be minter
        check_admin(
            &deps,
            nft.contract_addr.clone(),
            env.contract.address.clone(),
        )?;

        // let's mint this NFT
        let min_msg = MintMsg {
            token_id: nft.token_id.to_string(),
            owner: info.sender.to_string(),
        };
        let min_batch_mesage = MonsterraNFTMsg::MintBatch {
            msgs: vec![min_msg],
        };
        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: nft.contract_addr.clone().to_string(),
            msg: to_binary(&min_batch_mesage)?,
            funds: vec![],
        }));
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "convert")
        .add_attribute("sender", info.sender))
}

fn query_nft_owner(
    deps: &DepsMut,
    contract_addr: Addr,
    token_id: String,
) -> Result<String, ContractError> {
    let query_own_msg = Cw721QueryMsg::OwnerOf {
        token_id: token_id.clone(),
        include_expired: None,
    };

    let owner_response: StdResult<cw721::OwnerOfResponse> =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: contract_addr.to_string(),
            msg: to_binary(&query_own_msg)?,
        }));

    match owner_response {
        Ok(owner) => {
            return Ok(owner.owner);
        }
        Err(_) => {
            return Err(ContractError::NotExistedNFT {});
        }
    }
}

fn check_admin(deps: &DepsMut, contract_addr: Addr, user: Addr) -> Result<bool, ContractError> {
    let query_admin_msg: MonsterraNFTMsg = MonsterraNFTMsg::IsAdmin {
        user: user.to_string(),
    };

    let response: StdResult<bool> = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: contract_addr.to_string(),
        msg: to_binary(&query_admin_msg)?,
    }));

    match response {
        Ok(response) => {
            if response {
                return Ok(true);
            } else {
                return Err(ContractError::NotAdmin {});
            }
        }
        Err(e) => {
            return Err(ContractError::CustomError { val: e.to_string() });
        }
    }
}

fn verify_sig(deps: &DepsMut, convert_payload: &ConvertPayload, signature: Binary) -> bool {
    let result = to_binary(convert_payload);
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
