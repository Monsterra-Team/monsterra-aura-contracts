#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, to_binary};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::convert;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{set_signer, set_admin, get_signer, set_box_contract, is_admin, is_box_contract};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:monsterra-box-converter";
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
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&info.sender.to_string()))?;
    set_admin(deps.storage, &info, info.sender.clone(), true)?;
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
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    // match msg {}
    Ok(Response::default())
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Convert { msg, signature } => convert(deps, _env, info, msg, signature),
        ExecuteMsg::SetAdmin { user, status } => set_admin(deps.storage, &info, user, status),
        ExecuteMsg::SetSigner { public_key } => set_signer(deps.storage, &info, public_key),
        ExecuteMsg::SetBoxContract { box_contract, status } => set_box_contract(deps.storage, &info, box_contract, status),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IsAdmin { user } => to_binary(&is_admin(deps.storage, user)),
        QueryMsg::GetSigner {} => to_binary(&get_signer(deps.storage)),
        QueryMsg::IsBoxContract { box_contract } => to_binary(&is_box_contract(deps.storage, box_contract)),
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
