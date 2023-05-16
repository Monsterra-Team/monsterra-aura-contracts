pub mod execute;
pub mod msg;
pub mod query;
pub mod state;
pub mod error;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Empty};

pub use cw721_base::{ContractError, Cw721Contract, InstantiateMsg, MinterResponse};

use execute::{mint_batch, send_nft, stake_batch, transfer_nft};

// see: https://docs.opensea.io/docs/metadata-standards
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Data {
  pub data: Option<String>,
  pub nft_type: Option<u8>,
}

pub type Extension = Option<Data>;

pub type MonsterraNFT<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = crate::msg::MonsterraNFTExecuteMsg<Extension, Empty>;
pub type QueryMsg = crate::msg::MonsterraNFTQueryMsg<Empty>;
pub type MigrateMsg = crate::msg::MonsterraNFTMigrateMsg;

#[cfg(not(feature = "library"))]
pub mod entry {
  use super::*;

  use cosmwasm_std::entry_point;
  use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

  // This is a simple type to let us handle empty extensions

  // This makes a conscious choice on the various generics used by the contract
  #[entry_point]
  pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
  ) -> StdResult<Response> {
    MonsterraNFT::default().instantiate(deps, env, info, msg)
  }

  #[entry_point]
  pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
  ) -> Result<Response, ContractError> {
    match msg {
      ExecuteMsg::MintBatch(msg) => mint_batch(deps, env, info, msg),
      ExecuteMsg::StakeBatch { token_ids } => stake_batch(deps, env, info, token_ids),
      ExecuteMsg::TransferNft {
        recipient,
        token_id,
      } => transfer_nft(deps, env, info, recipient, token_id),
      ExecuteMsg::SendNft {
        contract,
        token_id,
        msg,
      } => send_nft(deps, env, info, contract, token_id, msg),
      _ => MonsterraNFT::default().execute(deps, env, info, msg.into()),
    }
  }

  #[entry_point]
  pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
      QueryMsg::AllTransferLogs { start_after, limit } => {
        to_binary(&query::query_transfer_log(deps, start_after, limit)?)
      }
      _ => MonsterraNFT::default().query(deps, env, msg.into()),
    }
  }

  #[entry_point]
  pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    // No state migrations performed, just returned a Response
    Ok(Response::default())
  }
}
