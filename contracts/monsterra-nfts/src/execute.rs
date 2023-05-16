use crate::MonsterraNFT;

use cosmwasm_std::{
  Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
  Storage,
};
use cw721::{Cw721Execute};
use cw721_base::state::TokenInfo;
use cw721_base::{ContractError};

use crate::msg::{MintBatchMsg, MintMsg};
use crate::error::MonsterraNFTError;
use crate::state::{TransferLog, LOG_COUNTER, STAKE_OWNERS, TRANSFER_LOGS};
use crate::Extension;

pub fn mint_batch(
  mut deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: MintBatchMsg<Extension>,
) -> Result<Response, ContractError> {
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
) -> Result<Response, ContractError> {
  for token_id in token_ids {
    transfer_nft(
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

pub fn burn(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  token_id: String,
) -> Result<Response<Empty>, MonsterraNFTError> {
  let contract = MonsterraNFT::default();
  let token = contract.tokens.load(deps.storage, &token_id)?;

  check_can_send(deps.as_ref(), &env, &info, &token)?;

  contract.tokens.remove(deps.storage, &token_id)?;
  decrement_tokens(deps.storage)?;

  log(
    deps.storage,
    info.sender.clone().into_string(),
    String::from("0"),
    token_id.clone(),
  )?;

  Ok(
    Response::new()
      .add_attribute("action", "burn")
      .add_attribute("sender", info.sender)
      .add_attribute("token_id", token_id.clone()),
  )
}

pub fn mint(
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

      let res = contract.mint(deps.branch(), info.clone(), msg.token_id.clone(), msg.owner.clone(), msg.token_uri.clone(), msg.extension.clone())?;

      log(
        deps.storage,
        String::from(env.contract.address.clone().into_string()),
        msg.owner.clone(),
        msg.clone().token_id,
      )?;

      STAKE_OWNERS.remove(deps.storage, msg.token_id.clone());
      Ok(res)
    }
    None => {
      let res = contract.mint(deps.branch(), info.clone(), msg.token_id.clone(), msg.owner.clone(), msg.token_uri.clone(), msg.extension.clone())?;
      log(
        deps.storage,
        String::from("0"),
        msg.owner.clone(),
        msg.clone().token_id,
      )?;
      Ok(res)
    }
  }
}

pub fn transfer_nft(
  mut deps: DepsMut,
  env: Env,
  info: MessageInfo,
  recipient: String,
  token_id: String,
) -> Result<Response<Empty>, ContractError> {
  let contract = MonsterraNFT::default();
  let res = contract.transfer_nft(
    deps.branch(),
    env.clone(),
    info.clone(),
    recipient.clone(),
    token_id.clone(),
  )?;

  log(
    deps.storage,
    info.sender.clone().into_string(),
    recipient.clone(),
    token_id.clone(),
  )?;
  Ok(res)
}

pub fn send_nft(
  mut deps: DepsMut,
  env: Env,
  info: MessageInfo,
  contract_addr: String,
  token_id: String,
  msg: Binary,
) -> Result<Response<Empty>, ContractError> {
  let contract = MonsterraNFT::default();
  let res = contract.send_nft(
    deps.branch(),
    env.clone(),
    info.clone(),
    contract_addr.clone(),
    token_id.clone(),
    msg.clone(),
  )?;
  log(
    deps.storage,
    info.sender.clone().into_string(),
    contract_addr.clone(),
    token_id.clone().clone(),
  )?;
  Ok(res)
}

/// returns true iff the sender can transfer ownership of the token
pub fn check_can_send(
  deps: Deps,
  env: &Env,
  info: &MessageInfo,
  token: &TokenInfo<Extension>,
) -> Result<(), MonsterraNFTError> {
  let contract = MonsterraNFT::default();
  // owner can send
  if token.owner == info.sender {
    return Ok(());
  }

  // any non-expired token approval can send
  if token
    .approvals
    .iter()
    .any(|apr| apr.spender == info.sender && !apr.is_expired(&env.block))
  {
    return Ok(());
  }

  // operator can send
  let op = contract
    .operators
    .may_load(deps.storage, (&token.owner, &info.sender))?;
  match op {
    Some(ex) => {
      if ex.is_expired(&env.block) {
        Err(MonsterraNFTError::Unauthorized {})
      } else {
        Ok(())
      }
    }
    None => Err(MonsterraNFTError::Unauthorized {}),
  }
}

pub fn decrement_tokens(storage: &mut dyn Storage) -> StdResult<u64> {
  let contract = MonsterraNFT::default();
  let val = contract.token_count(storage)? - 1;
  contract.token_count.save(storage, &val)?;
  Ok(val)
}

pub fn log_counter(storage: &dyn Storage) -> StdResult<u64> {
  Ok(LOG_COUNTER.may_load(storage)?.unwrap_or_default())
}

pub fn increment_log_counter(storage: &mut dyn Storage) -> StdResult<u64> {
  let val = log_counter(storage)? + 1;
  LOG_COUNTER.save(storage, &val)?;
  Ok(val)
}

pub fn log(
  storage: &mut dyn Storage,
  from: String,
  to: String,
  token_id: String,
) -> StdResult<TransferLog> {
  increment_log_counter(storage)?;
  let log = |d: Option<TransferLog>| -> StdResult<TransferLog> {
    match d {
      Some(_one) => Ok(TransferLog {
        from: from.clone(),
        to: to.clone(),
        token_id: token_id.clone(),
      }),
      None => Ok(TransferLog {
        from: from.clone(),
        to: to.clone(),
        token_id: token_id.clone(),
      }),
    }
  };

  let current_counter = log_counter(storage)?;
  Ok(TRANSFER_LOGS.update(storage, current_counter, log)?)
}
