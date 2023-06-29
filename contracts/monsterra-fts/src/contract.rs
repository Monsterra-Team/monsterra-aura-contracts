#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw20_base::msg::{InstantiateMsg, MigrateMsg};
use cw20_base::ContractError as CW20Error;

use crate::error::ContractError;
use crate::execute::mint;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::state::{
    get_owner, get_signer, is_admin, is_used_nonce, set_admin, set_new_owner, set_signer, OWNER,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, CW20Error> {
    cw20_base::contract::migrate(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    OWNER.save(deps.storage, &info.sender)?;
    set_admin(deps.storage, &info, info.sender.clone(), true)?;
    match cw20_base::contract::instantiate(deps, env, info, msg) {
        Ok(res) => Ok(res),
        Err(error) => Err(ContractError::CW20(error)),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TransferOwnerShip { user } => set_new_owner(deps.storage, &info, user),
        ExecuteMsg::SetAdmin { user, status } => set_admin(deps.storage, &info, user, status),
        ExecuteMsg::SetSigner { public_key } => set_signer(deps.storage, &info, public_key),
        ExecuteMsg::MintWithSignature { msg, signature } => mint(deps, env, &info, msg, signature),
        _ => match cw20_base::contract::execute(deps, env, info, msg.into()) {
            Ok(res) => Ok(res),
            Err(error) => Err(ContractError::CW20(error)),
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&get_owner(deps.storage)),
        QueryMsg::IsAdmin { user } => to_binary(&is_admin(deps.storage, user)),
        QueryMsg::GetSigner {} => to_binary(&get_signer(deps.storage)),
        QueryMsg::IsUsedNonce { nonce } => to_binary(&is_used_nonce(deps.storage, nonce)),
        _ => cw20_base::contract::query(deps, _env, msg.into()),
    }
}
