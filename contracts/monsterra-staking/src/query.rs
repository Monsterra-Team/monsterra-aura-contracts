use cosmwasm_std::{Addr, Binary, Storage, Uint128};

use crate::state::{
    get_owner, get_signer, get_staked_data, get_total_staked, is_accepted_token, is_admin,
    StakeData,
};

pub fn query_owner(storage: &dyn Storage) -> Addr {
    get_owner(storage)
}

pub fn query_admin(storage: &dyn Storage, user: Addr) -> bool {
    is_admin(storage, user)
}

pub fn query_signer(storage: &dyn Storage) -> Binary {
    get_signer(storage)
}

pub fn query_accepted_token(storage: &dyn Storage, token: Addr) -> bool {
    is_accepted_token(storage, token)
}

pub fn query_total_staked(storage: &dyn Storage, user: Addr) -> Uint128 {
    get_total_staked(storage, user)
}

pub fn query_staked_data(storage: &dyn Storage, user: Addr) -> Vec<StakeData> {
    get_staked_data(storage, user)
}
