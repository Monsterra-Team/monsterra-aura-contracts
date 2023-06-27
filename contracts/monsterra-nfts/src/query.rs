use cosmwasm_std::{Deps, StdResult};
use cw721::NftInfoResponse;

use crate::{state::get_base_uri, Data, MonsterraNFT};

pub fn nft_info(deps: Deps, token_id: String) -> StdResult<NftInfoResponse<Option<Data>>> {
    let tract = MonsterraNFT::default();
    let info = tract.tokens.load(deps.storage, &token_id)?;
    let base_uri = get_base_uri(deps.storage);
    Ok(NftInfoResponse {
        token_uri: match info.token_uri {
            Some(value) => Some(value),
            None => Some(base_uri + &token_id),
        },
        extension: info.extension,
    })
}
