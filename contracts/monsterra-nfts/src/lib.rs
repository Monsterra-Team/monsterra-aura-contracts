pub mod error;
pub mod execute;
pub mod msg;
pub mod query;
pub mod state;

use cosmwasm_schema::cw_serde;

use cosmwasm_std::Empty;

pub use cw721_base::{ContractError, Cw721Contract, InstantiateMsg, MinterResponse};

use execute::{mint_batch, stake_batch};

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
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
    use crate::error::MonsterraNFTError;
    use crate::state::{get_signer, is_admin, is_used_nonce, set_admin, set_signer};

    use super::*;

    use cosmwasm_std::{entry_point, to_binary};
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
    ) -> Result<Response, MonsterraNFTError> {
        match msg {
            ExecuteMsg::MintBatch(msg) => mint_batch(deps, env, info, msg),
            ExecuteMsg::StakeBatch { token_ids } => stake_batch(deps, env, info, token_ids),
            ExecuteMsg::SetAdmin { user, status } => set_admin(deps.storage, &info, user, status),
            ExecuteMsg::SetSigner { public_key } => set_signer(deps.storage, &info, public_key),
            _ => match MonsterraNFT::default().execute(deps, env, info, msg.into()) {
                Ok(result) => Ok(result),
                Err(error) => Err(MonsterraNFTError::CW721(error)),
            },
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::IsUsedNonce { nonce } => to_binary(&is_used_nonce(deps.storage, nonce)),
            QueryMsg::IsAdmin { user } => to_binary(&is_admin(deps.storage, user)),
            QueryMsg::GetSigner {} => to_binary(&get_signer(deps.storage)),
            _ => MonsterraNFT::default().query(deps, env, msg.into()),
        }
    }

    #[entry_point]
    pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
        // No state migrations performed, just returned a Response
        Ok(Response::default())
    }
}
