use cosmwasm_schema::write_api;

use monsterra_fungible_token::msg::{InstantiateMsg, MigrateMsg, ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        migrate: MigrateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
