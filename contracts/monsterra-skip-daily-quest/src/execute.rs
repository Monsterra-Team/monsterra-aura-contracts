use cosmwasm_std::{MessageInfo, Response, Storage};

use crate::ContractError;

pub fn skip_daily_quest(
    _storage: &mut dyn Storage,
    info: MessageInfo,
    index: u8,
) -> Result<Response, ContractError> {
    Ok(Response::new()
        .add_attribute("action", "skip_daily_quest")
        .add_attribute("sender", info.sender)
        .add_attribute("index", index.to_string()))
}
