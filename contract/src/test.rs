use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR};
use cosmwasm_std::CosmosMsg::Stargate;
use cosmwasm_std::{
    coin, from_binary, to_binary, Addr, BankMsg, Binary, CosmosMsg, Decimal, ReplyOn, SubMsg,
    Uint128,
};
use cw20::Cw20ReceiveMsg;

use crate::contract::{execute, instantiate, query, MINT_SYMBOL};
use crate::denom::{MsgCreateDenom, MsgMint};
use crate::mock_querier::mock_dependencies as mock_dependencies_querier;
use crate::msg::ExecuteMsg::UpdateConfig;
use crate::msg::{Cw20HookMsg, ExecuteMsg, InstantiateMsg, LeaderboardResponse, QueryMsg};
use crate::state::{Config, LEADERBOARD};
use crate::{denom, ContractError};
#[cfg(test)]
mod tests {}

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(
        "addr0000",
        &[cosmwasm_std::Coin {
            denom: "uwhale".to_string(),
            amount: Uint128::from(100u32),
        }],
    );
    let msg = InstantiateMsg {
        fee_collector_addr: "addr0000".to_string(),
        burn_fee: Some(Decimal::percent(1)),
        use_cw20: None,
        token_code_id: None,
        burn_cw20_addr: None,
        native_denom: Some("uwhale".to_string()),
    };
    let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    assert_eq!(
        res.messages,
        vec![SubMsg {
            id: 0,
            msg: Stargate {
                type_url: "/cosmwasm.tokenfactory.v1beta1.MsgCreateDenom".to_string(),
                value: Binary::from(MsgCreateDenom {
                    sender: env.contract.address.to_string(),
                    subdenom: MINT_SYMBOL.to_string(),
                }),
            },
            gas_limit: None,
            reply_on: ReplyOn::Never,
        }]
    );
}

//test burn message
#[test]
fn burn_execute() {
    let lp_denom = format!("{}/{MOCK_CONTRACT_ADDR}/{MINT_SYMBOL}", "factory");

    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(
        "addr0000",
        &[cosmwasm_std::Coin {
            denom: "uwhale".to_string(),
            amount: Uint128::from(100u32),
        }],
    );
    let msg = InstantiateMsg {
        fee_collector_addr: "addr0000".to_string(),
        burn_fee: Some(Decimal::percent(1)),
        use_cw20: None,
        token_code_id: None,
        burn_cw20_addr: None,
        native_denom: Some("uwhale".to_string()),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    let msg = ExecuteMsg::Burn {};
    let env = mock_env();
    let info = mock_info(
        "addr0000",
        &[cosmwasm_std::Coin {
            denom: "uwhale".to_string(),
            amount: Uint128::from(1000u32),
        }],
    );
    let res = execute(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(res.messages.len(), 4usize);

    let burn_whale_msg = res.messages.get(0).expect("no message").clone().msg;
    let mint_msg = res.messages.get(2).expect("no message").clone().msg;
    let bank_send_msg = res.messages.get(3).expect("no message").clone().msg;

    let burn_whale_msg_expected = CosmosMsg::Bank(BankMsg::Burn {
        amount: vec![coin(1000u128, "uwhale".to_string())],
    });

    let mint_msg_expected = <MsgMint as Into<CosmosMsg>>::into(MsgMint {
        sender: MOCK_CONTRACT_ADDR.to_string(),
        amount: Some(denom::Coin {
            denom: lp_denom.clone(),
            amount: "1000".to_string(),
        }),
    });

    let bank_send_msg_expected = CosmosMsg::Bank(BankMsg::Send {
        to_address: "addr0000".to_string(),
        amount: vec![coin(1000u128, lp_denom.clone())],
    });

    assert_eq!(burn_whale_msg, burn_whale_msg_expected);

    assert_eq!(mint_msg, mint_msg_expected);

    assert_eq!(bank_send_msg, bank_send_msg_expected);
}

// test with a cw20 as the burn asset instead of native tokens
#[test]
fn burn_cw20_for_native_ash() {
    let mut deps = mock_dependencies_querier(&[]);

    deps.querier.with_token_balances(&[
        (
            &String::from("mytoken"),
            &[(
                &String::from("addr0000"),
                &Uint128::new(100_000000000000000000),
            )],
        ),
        (
            &String::from("mytoken"),
            &[(
                &String::from("addr0000"),
                &Uint128::new(100_000000000000000000),
            )],
        ),
    ]);

    let env = mock_env();
    let info = mock_info(
        "addr0000",
        &[cosmwasm_std::Coin {
            denom: "uwhale".to_string(),
            amount: Uint128::from(100u32),
        }],
    );
    let msg = InstantiateMsg {
        fee_collector_addr: "addr0000".to_string(),
        burn_fee: Some(Decimal::percent(1)),
        use_cw20: Some(true),
        token_code_id: Some(1u64),
        burn_cw20_addr: Some("mock_cw20".to_string()),
        native_denom: Some("uwhale".to_string()),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    let receive_msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::from(100u32),
        msg: to_binary(&Cw20HookMsg::BurnCw20 {}).unwrap(),
    });
    let env = mock_env();
    let info = mock_info("addr0000", &[]);
    let res = execute(deps.as_mut(), env, info, receive_msg).unwrap();

    assert_eq!(res.messages.len(), 4usize);

    let burn_whale_msg = res.messages.get(0).expect("no message").clone().msg;
    let mint_msg = res.messages.get(2).expect("no message").clone().msg;
    let bank_send_msg = res.messages.get(3).expect("no message").clone().msg;

    let burn_whale_msg_expected = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: "mock_cw20".to_string(),
        msg: to_binary(&cw20::Cw20ExecuteMsg::Burn {
            amount: Uint128::from(100u128),
        })
        .unwrap(),
        funds: [].into(),
    });

    let mint_msg_expected = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: "02B929945C3E637AC76AD062C703D3ED37171A198CB4E20ABCEC8540D32F947B"
            .to_string(),
        msg: to_binary(&cw20::Cw20ExecuteMsg::Mint {
            recipient: "cosmos2contract".to_string(),
            amount: Uint128::from(100u128),
        })
        .unwrap(),
        funds: [].into(),
    });

    let bank_send_msg_expected = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: "02B929945C3E637AC76AD062C703D3ED37171A198CB4E20ABCEC8540D32F947B"
            .to_string(),
        msg: to_binary(&cw20::Cw20ExecuteMsg::Transfer {
            recipient: "addr0000".to_string(),
            amount: Uint128::from(100u128),
        })
        .unwrap(),
        funds: [].into(),
    });

    assert_eq!(burn_whale_msg, burn_whale_msg_expected);

    assert_eq!(mint_msg, mint_msg_expected);

    assert_eq!(bank_send_msg, bank_send_msg_expected);
}

//test burn message
#[test]
fn burn_execute_cw20_received() {
    let mut deps = mock_dependencies_querier(&[]);

    deps.querier.with_token_balances(&[
        (
            &String::from("mytoken"),
            &[(
                &String::from("addr0000"),
                &Uint128::new(100_000000000000000000),
            )],
        ),
        (
            &String::from("mytoken"),
            &[(
                &String::from("addr0000"),
                &Uint128::new(100_000000000000000000),
            )],
        ),
    ]);

    let env = mock_env();
    let info = mock_info(
        "addr0000",
        &[cosmwasm_std::Coin {
            denom: "uwhale".to_string(),
            amount: Uint128::from(100u32),
        }],
    );
    let msg = InstantiateMsg {
        fee_collector_addr: "addr0000".to_string(),
        burn_fee: Some(Decimal::percent(1)),
        use_cw20: Some(true),
        token_code_id: Some(1u64),
        burn_cw20_addr: None,
        native_denom: Some("uwhale".to_string()),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    let receive_msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::from(100u32),
        msg: to_binary(&Cw20HookMsg::BurnCw20 {}).unwrap(),
    });
    let env = mock_env();
    let info = mock_info(
        "addr0000",
        &[cosmwasm_std::Coin {
            denom: "uwhale".to_string(),
            amount: Uint128::from(1000u32),
        }],
    );
    let res = execute(deps.as_mut(), env, info, receive_msg).unwrap();

    assert_eq!(res.messages.len(), 4usize);

    let burn_whale_msg = res.messages.get(0).expect("no message").clone().msg;
    let mint_msg = res.messages.get(2).expect("no message").clone().msg;
    let bank_send_msg = res.messages.get(3).expect("no message").clone().msg;

    let burn_whale_msg_expected = CosmosMsg::Bank(BankMsg::Burn {
        amount: vec![coin(1000u128, "uwhale".to_string())],
    });

    let mint_msg_expected = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: "02B929945C3E637AC76AD062C703D3ED37171A198CB4E20ABCEC8540D32F947B"
            .to_string(),
        msg: to_binary(&cw20::Cw20ExecuteMsg::Mint {
            recipient: "cosmos2contract".to_string(),
            amount: Uint128::from(100u128),
        })
        .unwrap(),
        funds: [].into(),
    });

    let bank_send_msg_expected = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: "02B929945C3E637AC76AD062C703D3ED37171A198CB4E20ABCEC8540D32F947B"
            .to_string(),
        msg: to_binary(&cw20::Cw20ExecuteMsg::Transfer {
            recipient: "addr0000".to_string(),
            amount: Uint128::from(100u128),
        })
        .unwrap(),
        funds: [].into(),
    });

    assert_eq!(burn_whale_msg, burn_whale_msg_expected);

    assert_eq!(mint_msg, mint_msg_expected);

    assert_eq!(bank_send_msg, bank_send_msg_expected);
}

#[test]
fn burn_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(
        "addr0000",
        &[cosmwasm_std::Coin {
            denom: "uwhale".to_string(),
            amount: Uint128::from(100u32),
        }],
    );
    let msg = InstantiateMsg {
        fee_collector_addr: "addr0000".to_string(),
        burn_fee: Some(Decimal::percent(1)),
        use_cw20: None,
        token_code_id: None,
        burn_cw20_addr: None,
        native_denom: Some("uwhale".to_string()),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    let msg = ExecuteMsg::Burn {};
    let env = mock_env();
    let info = mock_info(
        "addr0000",
        &[cosmwasm_std::Coin {
            denom: "other".to_string(),
            amount: Uint128::from(1000u32),
        }],
    );

    let res = execute(deps.as_mut(), env.clone(), info, msg.clone()).unwrap_err();
    assert_eq!(res, ContractError::IncorrectToken {});

    let info = mock_info(
        "addr0000",
        &[
            cosmwasm_std::Coin {
                denom: "uwhale".to_string(),
                amount: Uint128::from(1000u32),
            },
            cosmwasm_std::Coin {
                denom: "other".to_string(),
                amount: Uint128::from(1000u32),
            },
        ],
    );

    let res = execute(deps.as_mut(), env, info, msg).unwrap_err();
    assert_eq!(res, ContractError::IncorrectTokenQuantity {});
}

#[test]
fn test_update_config() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(
        "addr0000",
        &[cosmwasm_std::Coin {
            denom: "uwhale".to_string(),
            amount: Uint128::from(100u32),
        }],
    );
    let msg = InstantiateMsg {
        fee_collector_addr: "addr0000".to_string(),
        burn_fee: Some(Decimal::percent(1)),
        use_cw20: None,
        token_code_id: None,
        burn_cw20_addr: None,
        native_denom: Some("uwhale".to_string()),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    let config: Config =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()).unwrap();

    assert_eq!(config.owner, Addr::unchecked("addr0000"));

    let info = mock_info("unauthorized", &[]);

    let update_config_message = UpdateConfig {
        owner: Some("new_owner".to_string()),
        fee_collector_addr: None,
        burn_fee: None,
    };

    let res = execute(
        deps.as_mut(),
        env.clone(),
        info,
        update_config_message.clone(),
    )
    .unwrap_err();
    assert_eq!(res, ContractError::Unauthorized {});

    let info = mock_info("addr0000", &[]);

    let _res = execute(deps.as_mut(), env, info, update_config_message.clone());

    // confirm contract owner is changed.
    let config: Config =
        from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()).unwrap();
    assert_eq!(config.owner, Addr::unchecked("new_owner"));
}

#[test]
fn test_leaderboard_query() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(
        "addr0000",
        &[cosmwasm_std::Coin {
            denom: "uwhale".to_string(),
            amount: Uint128::from(100u32),
        }],
    );
    let msg = InstantiateMsg {
        fee_collector_addr: "addr0000".to_string(),
        burn_fee: Some(Decimal::percent(1)),
        use_cw20: None,
        token_code_id: None,
        burn_cw20_addr: None,
        native_denom: Some("uwhale".to_string()),
    };
    instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    for i in 0..20 {
        let address = "address".to_string() + i.to_string().as_str();
        LEADERBOARD
            .save(
                &mut deps.storage,
                &Addr::unchecked(address),
                &Uint128::new(20 - i),
            )
            .unwrap();
    }

    let leaderboard_1: LeaderboardResponse = from_binary(
        &query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Leaderboard {
                start_after: None,
                limit: Some(10u32),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(leaderboard_1.len(), 10usize);

    let last = leaderboard_1.last().unwrap().clone().0;

    let leaderboard_2: LeaderboardResponse = from_binary(
        &query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Leaderboard {
                start_after: Some(last),
                limit: Some(10u32),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(leaderboard_2.len(), 10usize);

    let leaderboard_3: LeaderboardResponse = from_binary(
        &query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Leaderboard {
                start_after: None,
                limit: Some(30u32),
            },
        )
        .unwrap(),
    )
    .unwrap();

    let merged = leaderboard_1
        .into_iter()
        .chain(leaderboard_2.into_iter())
        .collect::<Vec<(Addr, Uint128)>>();
    assert_eq!(merged, leaderboard_3);
}

#[test]
fn test_burn_tax_feature() {
    let lp_denom = format!("{}/{MOCK_CONTRACT_ADDR}/{MINT_SYMBOL}", "factory");

    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(
        "addr0000",
        &[cosmwasm_std::Coin {
            denom: "uwhale".to_string(),
            amount: Uint128::from(100u32),
        }],
    );
    let msg = InstantiateMsg {
        fee_collector_addr: "addr0001".to_string(),
        burn_fee: Some(Decimal::percent(1)),
        use_cw20: None,
        token_code_id: None,
        burn_cw20_addr: None,
        native_denom: Some("uwhale".to_string()),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    let msg = ExecuteMsg::Burn {};
    let env = mock_env();
    let info = mock_info(
        "addr0000",
        &[cosmwasm_std::Coin {
            denom: "uwhale".to_string(),
            amount: Uint128::from(1000u32),
        }],
    );

    let res = execute(deps.as_mut(), env.clone(), info, msg.clone()).unwrap();

    let burn_whale_msg = res.messages.get(0).expect("no message").clone().msg;
    let collect_msg = res.messages.get(1).expect("no message").clone().msg;

    let mint_msg = res.messages.get(2).expect("no message").clone().msg;
    let bank_send_msg = res.messages.get(3).expect("no message").clone().msg;

    let burn_whale_msg_expected = CosmosMsg::Bank(BankMsg::Burn {
        amount: vec![coin(1000u128, "uwhale".to_string())],
    });

    let collect_msg_expected = CosmosMsg::Bank(BankMsg::Send {
        to_address: "addr0001".to_string(),
        amount: vec![coin(10u128, lp_denom.clone())],
    });

    let mint_msg_expected = <MsgMint as Into<CosmosMsg>>::into(MsgMint {
        sender: MOCK_CONTRACT_ADDR.to_string(),
        amount: Some(denom::Coin {
            denom: lp_denom.clone(),
            amount: "1000".to_string(),
        }),
    });

    let bank_send_msg_expected = CosmosMsg::Bank(BankMsg::Send {
        to_address: "addr0000".to_string(),
        amount: vec![coin(1000u128, lp_denom.clone())],
    });

    assert_eq!(burn_whale_msg, burn_whale_msg_expected);

    assert_eq!(collect_msg, collect_msg_expected);
    assert_eq!(mint_msg, mint_msg_expected);

    assert_eq!(bank_send_msg, bank_send_msg_expected);
}
