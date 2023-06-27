#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{
    execute_set_accepted_token, execute_set_admin, execute_set_signer, execute_stake,
    execute_transfer_ownership, execute_unstake,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::{
    query_accepted_token, query_admin, query_owner, query_signer, query_staked_data,
    query_total_staked, query_used_nonce,
};
use crate::state::{set_admin, OWNER};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:monsterra-staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
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

    // With `Response` type, it is possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TransferOwnerShip { user } => {
            execute_transfer_ownership(deps.storage, info, user)
        }
        ExecuteMsg::SetAdmin { user, status } => {
            execute_set_admin(deps.storage, info, user, status)
        }
        ExecuteMsg::SetSigner { public_key } => execute_set_signer(deps.storage, info, public_key),
        ExecuteMsg::SetAcceptedToken { token, status } => {
            execute_set_accepted_token(deps.storage, info, token, status)
        }
        ExecuteMsg::Stake {
            token,
            amount,
            duration,
        } => execute_stake(deps.storage, env, info, &token, amount, duration),
        ExecuteMsg::Unstake { msg, signature } => execute_unstake(deps, env, info, msg, signature),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps.storage)),
        QueryMsg::IsAdmin { user } => to_binary(&query_admin(deps.storage, user)),
        QueryMsg::GetSigner {} => to_binary(&query_signer(deps.storage)),
        QueryMsg::IsAcceptedToken { token } => {
            to_binary(&query_accepted_token(deps.storage, token))
        }
        QueryMsg::GetTotalStaked { user } => to_binary(&query_total_staked(deps.storage, user)),
        QueryMsg::GetStakeData { user } => to_binary(&query_staked_data(deps.storage, user)),
        QueryMsg::IsUsedNonce { nonce } => to_binary(&query_used_nonce(deps.storage, nonce)),
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}
