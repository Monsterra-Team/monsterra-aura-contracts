use cosmwasm_schema::write_api;

use cw20_base::msg::{InstantiateMsg, MigrateMsg};
use monsterra_fts::msg::{ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        migrate: MigrateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
