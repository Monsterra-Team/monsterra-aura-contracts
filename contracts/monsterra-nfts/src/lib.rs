pub mod error;
pub mod execute;
pub mod msg;
pub mod query;
pub mod state;

use cosmwasm_schema::cw_serde;

use cosmwasm_std::{entry_point, to_binary};
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};

use cw721_base::Extension;
pub use cw721_base::{ContractError, Cw721Contract, MinterResponse};

use execute::{mint_batch, mint_batch_with_signature, stake_batch};
use query::nft_info;

use error::MonsterraNFTError;
use state::{
    get_base_uri, get_signer, is_admin, is_used_nonce, set_admin, set_base_uri, set_signer,
};

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
pub struct Data {
    // pub data: Option<String>,
    // pub nft_type: Option<u8>,
}

pub type MonsterraNFT<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = crate::msg::MonsterraNFTExecuteMsg<Extension, Empty>;
pub type QueryMsg = crate::msg::MonsterraNFTQueryMsg<Empty>;
pub type MigrateMsg = crate::msg::MonsterraNFTMigrateMsg;
pub type InstantiateMsg = crate::msg::MonsterraNFTInstantiateMsg;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    // This is a simple type to let us handle empty extensions

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, MonsterraNFTError> {
        let base_uri = msg.base_uri.clone();
        let res = MonsterraNFT::default().instantiate(deps.branch(), env, info.clone(), msg.into());
        set_admin(deps.storage, &info, info.sender.clone(), true)?;
        set_base_uri(deps.storage, &info, base_uri)?;
        match res {
            Ok(result) => Ok(result),
            Err(error) => Err(MonsterraNFTError::Std(error)),
        }
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, MonsterraNFTError> {
        match msg {
            ExecuteMsg::MintBatch(msg) => mint_batch(deps, env, info, msg),
            ExecuteMsg::StakeBatch { token_ids } => stake_batch(deps, env, info, token_ids),
            ExecuteMsg::MintBatchWithSignature { msg, signature } => {
                mint_batch_with_signature(deps, env, info, msg, signature)
            }
            ExecuteMsg::SetAdmin { user, status } => set_admin(deps.storage, &info, user, status),
            ExecuteMsg::SetSigner { public_key } => set_signer(deps.storage, &info, public_key),
            ExecuteMsg::SetBaseUri { base_uri } => set_base_uri(deps.storage, &info, base_uri),
            _ => match MonsterraNFT::default().execute(deps, env, info, msg.into()) {
                Ok(result) => Ok(result),
                Err(error) => Err(MonsterraNFTError::CW721(error)),
            },
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::NftInfo { token_id } => to_binary(&nft_info(deps, token_id)?),
            QueryMsg::IsUsedNonce { nonce } => to_binary(&is_used_nonce(deps.storage, nonce)),
            QueryMsg::IsAdmin { user } => to_binary(&is_admin(deps.storage, user)),
            QueryMsg::GetSigner {} => to_binary(&get_signer(deps.storage)),
            QueryMsg::GetBaseURI {} => to_binary(&get_base_uri(deps.storage)),
            _ => MonsterraNFT::default().query(deps, env, msg.into()),
        }
    }

    #[entry_point]
    pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
        // No state migrations performed, just returned a Response
        Ok(Response::default())
    }
}
