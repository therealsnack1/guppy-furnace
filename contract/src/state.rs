use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub mint_denom: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
// key: (address, totalburned) -> value: totalburned
pub const LEADERBOARD: Map<&Addr, Uint128> = Map::new("leaderboard");
