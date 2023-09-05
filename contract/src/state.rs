use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw20::{Cw20Coin, MinterResponse};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub mint_denom: String,
    pub fee_collector_addr: Addr,
    pub burn_fee: Decimal,
    pub use_cw20: bool,
    pub burn_cw20_addr: Option<Addr>,
    pub native_denom: String,
}

/// TokenContract InstantiateMsg
#[cw_serde]
pub struct TokenInstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
}

pub const CONFIG: Item<Config> = Item::new("config");
// key: (address, totalburned) -> value: totalburned
pub const LEADERBOARD: Map<&Addr, Uint128> = Map::new("leaderboard");
