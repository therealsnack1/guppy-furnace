use crate::denom::{Coin, MsgCreateDenom, MsgMint};
use crate::error::ContractError;
use crate::msg::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, LeaderboardResponse, MigrateMsg, QueryMsg,
};
use crate::state::{Config, CONFIG, LEADERBOARD};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdResult, Uint128, Decimal,
};
use cw2::{get_contract_version, set_contract_version};
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

    let config = Config {
        owner: deps.api.addr_validate(info.sender.as_str())?,
        mint_denom: format!("{}/{}/{}", "factory", env.contract.address, MINT_SYMBOL),
        fee_collector_addr: deps.api.addr_validate(&msg.fee_collector_addr)?,
        burn_fee: msg.burn_fee.unwrap_or(DEFAULT_BURN_FEE),
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(
        Response::new().add_message(<MsgCreateDenom as Into<CosmosMsg>>::into(MsgCreateDenom {
            sender: env.contract.address.to_string(),
            subdenom: MINT_SYMBOL.to_string(),
        })),
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { owner, fee_collector_addr, burn_fee } => update_config(deps, info, owner, fee_collector_addr, burn_fee),
        ExecuteMsg::Burn {} => burn(deps, env, info),
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

pub fn burn(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    //check the only one token is sent
    if info.funds.len() != 1 {
        return Err(ContractError::IncorrectTokenQuantity {});
    }

    //Only burn whale tokens
    if info.funds[0].denom != "uwhale" {
        return Err(ContractError::IncorrectToken {});
    }
    let config: Config = CONFIG.load(deps.storage)?;
    let mut messages: Vec<CosmosMsg> = vec![];
    let amount = info.funds[0].amount;

    //burn the whale sent
    messages.push(CosmosMsg::from(BankMsg::Burn {
        amount: info.funds.clone(),
    }));

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

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("sender", info.sender.as_str()),
        ("action", "burn"),
        ("asset", &format!("{}", info.funds[0])),
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
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let version: Version = CONTRACT_VERSION.parse()?;
    let storage_version: Version = get_contract_version(deps.storage)?.version.parse()?;

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
