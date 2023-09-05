use crate::denom::{Coin, MsgCreateDenom, MsgMint};
use crate::error::ContractError;
use crate::helpers::{
    build_burn_cw20_token_msg, build_mint_cw20_token_msg, build_transfer_cw20_token_msg,
};
use crate::msg::{
    ConfigResponse, Cw20HookMsg, ExecuteMsg, InstantiateMsg, LeaderboardResponse, MigrateMsg,
    QueryMsg,
};
use crate::state::{Config, TokenInstantiateMsg, CONFIG, LEADERBOARD};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, from_binary, instantiate2_address, to_binary, Addr, BankMsg, Binary, CodeInfoResponse,
    CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
    Uint128, WasmMsg,
};
use cw2::{get_contract_version, set_contract_version};
use cw20::{Cw20ReceiveMsg, MinterResponse};
use cw_storage_plus::Bound;
use semver::Version;

// version info for migration info
const CONTRACT_NAME: &str = "white_whale_furnace";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MINT_SYMBOL: &str = "ash";
const DEFAULT_BURN_FEE: Decimal = Decimal::one();
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let mut config = Config {
        owner: deps.api.addr_validate(info.sender.as_str())?,
        mint_denom: format!("{}/{}/{}", "factory", env.contract.address, MINT_SYMBOL),
        use_cw20: msg.use_cw20.unwrap_or(false),
        burn_cw20_addr: None,
        native_denom: "".to_string(),
        fee_collector_addr: deps.api.addr_validate(&msg.fee_collector_addr)?,
        burn_fee: msg.burn_fee.unwrap_or(DEFAULT_BURN_FEE),
    };

    // If msg.use_cw20 is true, we will use the LP token technique
    // Otherwise we will use CreateDenom, if tokenfactory is not available you should use_cw20
    match msg.burn_cw20_addr {
        Some(burn_cw20_addr) => {
            config.burn_cw20_addr = Some(deps.api.addr_validate(&burn_cw20_addr)?);
        }
        None => match msg.native_denom {
            Some(native_denom) => {
                config.native_denom = native_denom;
            }
            None => {
                return Err(ContractError::MissingNativeDenom {});
            }
        },
    }
    let mut messages: Vec<CosmosMsg> = vec![];

    match msg.use_cw20.unwrap_or(false) {
        true => {
            let ash_token_name = format!("ASH-{}", config.mint_denom);
            // Create the LP token using instantiate2
            let creator = deps.api.addr_canonicalize(env.contract.address.as_str())?;
            let code_id = msg.token_code_id.unwrap_or(0);
            let CodeInfoResponse { checksum, .. } = deps.querier.query_wasm_code_info(code_id)?;
            let seed = format!(
                "{}{}{}",
                config.mint_denom,
                info.sender.into_string(),
                env.block.height
            );
            let salt = Binary::from(seed.as_bytes());
            // TODO: RM this for the commented out stuff when there is better testing/mocking around CodeData and Instantiate2
            // let pool_lp_address = deps.api.addr_humanize(
            //     &instantiate2_address(&checksum, &creator, &salt)
            //         .map_err(|e| StdError::generic_err(e.to_string()))?,
            // )?;
            let pool_lp_address = Addr::unchecked(
                &instantiate2_address(&checksum, &creator, &salt)
                    .map_err(|e| StdError::generic_err(e.to_string()))?
                    .to_string(),
            );

            let message = CosmosMsg::Wasm(WasmMsg::Instantiate2 {
                admin: None,
                code_id,
                label: ash_token_name.to_owned(),
                msg: to_binary(&TokenInstantiateMsg {
                    name: ash_token_name.clone(),
                    symbol: format!("{ash_token_name}-ASH").to_string(),
                    decimals: 6,
                    initial_balances: vec![],
                    mint: Some(MinterResponse {
                        minter: env.contract.address.to_string(),
                        cap: None,
                    }),
                })?,
                funds: vec![],
                salt,
            });
            messages.push(message);
            // Overwrite mint_denom with the token address
            config.mint_denom = pool_lp_address.to_string();
        }
        false => {
            messages.push(<MsgCreateDenom as Into<CosmosMsg>>::into(MsgCreateDenom {
                sender: env.contract.address.to_string(),
                subdenom: MINT_SYMBOL.to_string(),
            }));
        }
    }
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_messages(messages))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::UpdateConfig {
            owner,
            fee_collector_addr,
            burn_fee,
        } => update_config(deps, info, owner, fee_collector_addr, burn_fee),
        ExecuteMsg::Burn {} => burn(deps, env, info, None),
    }
}

/// Updates the contract's [Config]
pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::BurnCw20 {}) => burn(deps, env, info, Some(cw20_msg)),
        Err(err) => Err(ContractError::Std(err)),
    }
}

/// Updates the contract's [Config]
pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    owner: Option<String>,
    fee_collector_addr: Option<String>,
    burn_fee: Option<Decimal>,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if deps.api.addr_validate(info.sender.as_str())? != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        // validate address format
        let _ = deps.api.addr_validate(&owner)?;
        config.owner = deps.api.addr_validate(&owner)?;
    }

    if let Some(fee_collector_addr) = fee_collector_addr {
        // validate address format
        config.fee_collector_addr = deps.api.addr_validate(&fee_collector_addr)?;
    }

    if let Some(burn_fee) = burn_fee {
        // Note there is no real validation on bounds for this
        // This would be an info issue in audit.
        // To remedy, someone needs to specify and upper and lower bound and it needs to be enforced here
        // Given the burn fee is inflationary, it is not a huge concern as it will only affect the ash token
        config.burn_fee = burn_fee;
    }
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

/// Burns native tokens or CW20 tokens based on the contract's [Config] setup during instantiate
/// If the contract is setup to use CW20 input tokens, the CW20 token address must be set in the config
/// If the contract is setup to use native input tokens, the CW20 token address must not be set in the config
/// If the contract is setup to mint native ASH tokens on burn, config.use_cw20 must be false
/// If the contract is setup to mint CW20 ASH tokens on burn, config.use_cw20 must be true
///
/// In either case 3 main actions happen assuming good data
/// 1. A burn message of the native input token (or cw20 if setup right)
/// 2. A mint message of the native ASH token (or cw20 if setup right)
/// 3. A transfer message of the native ASH token (or cw20 if setup right)
pub fn burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Option<Cw20ReceiveMsg>,
) -> Result<Response, ContractError> {
    //check the only one token is sent
    let config: Config = CONFIG.load(deps.storage)?;

    let mut messages: Vec<CosmosMsg> = vec![];

    match config.burn_cw20_addr {
        Some(burn_cw20_addr) => {
            // If the BurnCw20_addr was set, we need to not check the 'funds'
            // We can check the input amount from the cw20_msg
            // If the BurnCw20_addr is set, we need to check the cw20_msg
            if cw20_msg.is_none() {
                return Err(ContractError::IncorrectTokenQuantity {});
            }
            let cw20_msg = cw20_msg.clone().unwrap();

            if cw20_msg.amount.is_zero() {
                return Err(ContractError::IncorrectTokenQuantity {});
            }
            messages.push(build_burn_cw20_token_msg(
                burn_cw20_addr.to_string(),
                cw20_msg.amount,
            )?);
        }
        None => {
            if info.funds.len() != 1 {
                return Err(ContractError::IncorrectTokenQuantity {});
            }
            //Only burn whale tokens
            if info.funds[0].denom != config.native_denom {
                return Err(ContractError::IncorrectToken {});
            }
            messages.push(CosmosMsg::from(BankMsg::Burn {
                amount: info.funds.clone(),
            }));
        }
    }
    // Big deviation here lets break it down, end result we want the amount burned and all the messages ready
    // If using cw20 (first case) we will need to use the CW20 spec and rely on the token address being stored in Config
    // Otherwise we are proceeding as normal using Native tokens and minting
    let (amount, ash_amount) = match config.use_cw20 {
        true => {
            let provided_amount = cw20_msg.unwrap().amount;

            // But dont return, after this we are going to mint the amount to the sender
            if config.fee_collector_addr != Addr::unchecked("") {
                let fee = provided_amount * config.burn_fee;

                messages.push(build_transfer_cw20_token_msg(
                    config.fee_collector_addr.clone(),
                    config.mint_denom.clone(),
                    fee,
                )?);
            }

            let ash_amount = coins(provided_amount.u128(), config.mint_denom.as_str());

            // Now mint the ash to the sender, first do a CW20 Mint, this contract already has minter rights
            messages.push(build_mint_cw20_token_msg(
                env.contract.address,
                config.mint_denom.clone(),
                provided_amount,
            )?);
            // Now send ash to the sender
            messages.push(build_transfer_cw20_token_msg(
                info.sender.clone(),
                config.mint_denom.clone(),
                provided_amount,
            )?);
            (provided_amount, ash_amount)
        }
        false => {
            if info.funds.len() != 1 {
                return Err(ContractError::IncorrectTokenQuantity {});
            }
            if info.funds[0].denom != config.native_denom {
                return Err(ContractError::IncorrectToken {});
            }

            let amount = info.funds[0].amount;

            // If the fee_collect_addr is set, inflate the amount to mint, send the difference (fee) to the fee_collector_addr
            // But dont return, after this we are going to mint the amount to the sender
            if config.fee_collector_addr != Addr::unchecked("") {
                let fee = amount * config.burn_fee;
                let fee_amount = coins(fee.u128(), config.mint_denom.as_str());
                messages.push(CosmosMsg::Bank(BankMsg::Send {
                    to_address: config.fee_collector_addr.to_string(),
                    amount: fee_amount.clone(),
                }));
            }

            //mint ASH and transfer to sender
            messages.push(<MsgMint as Into<CosmosMsg>>::into(MsgMint {
                sender: env.contract.address.to_string(),
                amount: Some(Coin {
                    denom: config.mint_denom.clone(),
                    amount: amount.to_string(),
                }),
            }));
            let ash_amount = coins(amount.u128(), config.mint_denom.as_str());
            messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: info.sender.to_string(),
                amount: ash_amount.clone(),
            }));
            (amount, ash_amount)
        }
    };

    let amount_burnt_by_sender = LEADERBOARD.may_load(deps.storage, &info.sender)?;
    if let Some(amount_burnt_by_sender) = amount_burnt_by_sender {
        LEADERBOARD.save(
            deps.storage,
            &info.sender,
            &(amount_burnt_by_sender.checked_add(amount)?),
        )?;
    } else {
        LEADERBOARD.save(deps.storage, &info.sender, &amount)?;
    }

    // If info.funds has an entry in 0 position, we are using native tokens return this asset otherwise the cw20 asset
    let asset = match config.use_cw20 {
        true => cosmwasm_std::Coin {
            denom: config.mint_denom.clone(),
            amount,
        },
        false => info.funds[0].clone(),
    };

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("sender", info.sender.as_str()),
        ("action", "burn"),
        ("asset", &format!("{}", asset)),
        ("action", "mint"),
        ("asset", &format!("{}", ash_amount[0])),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Config {} => Ok(to_binary(&query_config(deps)?)?),
        QueryMsg::Leaderboard { start_after, limit } => {
            Ok(to_binary(&query_leaderboard(deps, start_after, limit)?)?)
        }
    }
}

/// Queries the [Config], which contains the owner, and minting denom
pub fn query_config(deps: Deps) -> Result<ConfigResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

// Queries the leaderboard, unsorted
pub fn query_leaderboard(
    deps: Deps,
    start_after: Option<Addr>,
    limit: Option<u32>,
) -> StdResult<LeaderboardResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = calc_range_start(start_after).map(Bound::ExclusiveRaw);

    LEADERBOARD
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| Ok(item?))
        .collect()
}

// this will set the first key after the provided key, by appending a 1 byte
fn calc_range_start(start_after: Option<Addr>) -> Option<Vec<u8>> {
    start_after.map(|addr| {
        let mut v = addr.as_bytes().to_vec();
        v.push(1);
        v
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let version: Version = CONTRACT_VERSION.parse()?;
    let storage_version: Version = get_contract_version(deps.storage)?.version.parse()?;
    let old_config = CONFIG.load(deps.storage)?; //load old config
    let config = Config {
        // setup new one
        owner: old_config.owner,
        mint_denom: old_config.mint_denom,
        // The new fee_collector_addr and burn fee need to be set in MigrateMsg when migrating
        // The burn fee is non optional rn just for simplicity when migrating
        fee_collector_addr: deps.api.addr_validate(&msg.fee_collector_addr)?,
        burn_fee: msg.burn_fee.unwrap_or(DEFAULT_BURN_FEE),
        use_cw20: false,
        burn_cw20_addr: None,
        native_denom: msg.native_denom.unwrap_or("".to_string()),
    };

    CONFIG.save(deps.storage, &config)?;
    if storage_version >= version {
        return Err(ContractError::Unauthorized {});
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    //fix addresses that had already burned whale into the contract
    fill_missing_addresses(deps)?;

    Ok(Response::default().add_attributes(vec![("action", "migrate".to_string())]))
}

fn fill_missing_addresses(deps: DepsMut) -> Result<(), ContractError> {
    let addr = deps
        .api
        .addr_validate("migaloo17w97atfwdnjpe6wywwsjjw09050aq9s78jjjsmrmhhqtg7nevpmq0u8t9v")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(6953356))?;
    }
    let addr = deps
        .api
        .addr_validate("migaloo1u4npx7xvprwanpru7utv8haq99rtfmdzzw6p3hpfc38n7zmzm42q8ydga3")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(100000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1z89funaazn4ka8vrmmw4q27csdykz63hep4ay8q2dmlspc6wtdgq92u369")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(1))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1l4wszj0d93phzr55ne3v6cf7v63p04pukq524r")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(100000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1hcx5ysepz25lstnp5qrwwrcll5s3q4hlqgdqtw")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(30000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1czk55m7ayx00q90yj9awnm9cqspmgc485hsv5x")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(650000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1cgekxjacxnnv0lc0ddk8cduprm4wzejdywzneu")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(69000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1eruep05azfgss7y43ah7nxvmhsmxjc4dwut2r0")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(15000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo16jz85zvr3p233hrw69sj7hvep7knl024vep59k")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(5000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1663yhw0rre70egdwez3sw2e0ery94vgngjn7s0")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(5000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1mm7037ryl3udnzz2rtx7xcmfw9zlnv0wh2zjxc")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(10000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1ug5h3wn9qlmn00h7dvws8h68vdeh8c0paazv8z")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(10000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1u4sf8rj5ung6kgxjs6pgr76v9ztztp6xyfj4c7")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(155000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo17v368dmh5q8thhg09uce3su66xazw57etsptvl")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(2000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1k5w5fpu3qt4va27vsarddnruumqqprs8mu7y3v")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(570000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1dc0y4de52q2rfxwejmd4y2sfftvnvmq2zpn6mn")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(39000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1dpx7ytug647wefe7ajxmg5ejt68gxcfvw35f4e")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(214899999))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1v8n2jpmq8a97xtv9j28c557q3ztqn7rkrxtykg")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(102000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1tj69t56h3levwepq4cdmvf8fn59rzhdcqakkc6")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(31807000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1fkyvccs92hsrflefg9u030xpq7gflr96c3n5zg")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(10000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1fn2nfv3mpv372uhq9hv7tz2eyv8eykqrphd3gd")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(12000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1fw0xmf4574hccxsvagc6dzzg37r2gwxtdqdemp")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(99999999900))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1fdfgyjjdx2xffzctjzgmvpp5spakj8gqygenfl")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(2980000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1gkh3p7thnpsg5hctlxgrvqafr9umjsqpxdjvdf")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(5000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1g5jjwws9wazwsvmuwc4mh3awfau88ugj98ntv3")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(9000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo18a9m9stu3dyvewwcq9qmp85euxqcvln5mefync")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(6942069000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo185n0twqz7wjcduvj4z3jpz9wg2ujvy7r6d5v27")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(10000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1xcu60yphyx5pux5035wqtgf6smvyydcwftz5h5")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(1000000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1xwx9xzwqzc323e3s3ksxjywvusqf6j44a4hump")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(20000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1xt98hx52dgv7xf39cmagdm52km9xre9c08kq52")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(500000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo195afg0rc5zj2m66h5gfxkyuxauy5trpsy87827")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(140000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo19rnt6nmlzn6awdnq07hksygltt0kzvka6qv329")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(1080000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1q0wc3ps7ehwayy372slg2eqf6fzgqdrnu2hlgw")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(1000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1yvt3zn7wenew453utlc89kz7revr0kk7vka0x2")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(100000000))?;
    }

    let addr = deps
        .api
        .addr_validate("migaloo1ylx03vqp2zkpjayudpx2k0psgdx4uc3dmus82d")?;
    let addr_option = LEADERBOARD.may_load(deps.storage, &addr)?;
    if addr_option.is_none() {
        LEADERBOARD.save(deps.storage, &addr, &Uint128::new(9000000))?;
    }

    Ok(())
}
