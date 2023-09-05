use crate::state::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub fee_collector_addr: String,
    pub burn_fee: Option<Decimal>,
    pub use_cw20: Option<bool>,
    pub token_code_id: Option<u64>,
    pub burn_cw20_addr: Option<String>,
    pub native_denom: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Updates contract's config, i.e. relevant code_ids, fee_collector address and owner
    UpdateConfig {
        owner: Option<String>,
        fee_collector_addr: Option<String>,
        burn_fee: Option<Decimal>,
    },
    Burn {},
    /// Used to trigger the [Cw20HookMsg] messages; burning CW20s instead of native tokens
    Receive(Cw20ReceiveMsg),
}

#[cw_serde]
pub enum Cw20HookMsg {
    /// Burn a already setup CW20 for ASH
    BurnCw20 {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(LeaderboardResponse)]
    Leaderboard {
        /// An optional parameter specifying what address to start searching after.
        start_after: Option<Addr>,
        /// The amount of incentive contracts to return.
        ///
        /// If unspecified, will default to a value specified by the contract.
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct MigrateMsg {
    pub fee_collector_addr: String,
    pub burn_fee: Option<Decimal>,
    pub native_denom: Option<String>,
}

pub type ConfigResponse = Config;
pub type LeaderboardResponse = Vec<(Addr, Uint128)>;
