use cosmwasm_std::{to_binary, Addr, BankMsg, Coin, CosmosMsg, StdResult, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;

/// Builds a CW20 transfer message
/// recipient: the address of the recipient
/// token_contract_address: the address of the CW20 contract
/// amount: the amount of tokens to transfer
/// returns a CosmosMsg::Wasm(WasmMsg::Execute) message
/// to transfer CW20 tokens
///
pub fn build_transfer_cw20_token_msg(
    recipient: Addr,
    token_contract_address: String,
    amount: Uint128,
) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token_contract_address,
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: recipient.into(),
            amount,
        })?,
        funds: vec![],
    }))
}

/// Builds a CW20 mint message
/// recipient: the address of the recipient
/// token_contract_address: the address of the CW20 contract
/// amount: the amount of tokens to mint
/// returns a CosmosMsg::Wasm(WasmMsg::Execute) message
/// to mint CW20 tokens
///
pub fn build_mint_cw20_token_msg(
    recipient: Addr,
    token_contract_address: String,
    amount: Uint128,
) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token_contract_address,
        msg: to_binary(&Cw20ExecuteMsg::Mint {
            recipient: recipient.into(),
            amount,
        })?,
        funds: vec![],
    }))
}

/// Builds a CW20 burn message
/// token_contract_address: the address of the CW20 contract
/// amount: the amount of tokens to burn
/// returns a CosmosMsg::Wasm(WasmMsg::Execute) message
/// to burn CW20 tokens
///
pub fn build_burn_cw20_token_msg(
    token_contract_address: String,
    amount: Uint128,
) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token_contract_address,
        msg: to_binary(&Cw20ExecuteMsg::Burn { amount })?,
        funds: vec![],
    }))
}

/// Builds a Native transfer message
/// recipient: the address of the recipient
/// token_contract_address: the denom of the native token
/// amount: the amount of tokens to transfer
/// returns a CosmosMsg::Bank(BankMsg::Send) message
/// to transfer native tokens
pub fn build_native_token_msg(
    recipient: Addr,
    token_contract_address: String,
    amount: Uint128,
) -> StdResult<CosmosMsg> {
    // Return a bankmsg send to transfer native tokens to a recipient address
    Ok(CosmosMsg::Bank(BankMsg::Send {
        to_address: recipient.into(),
        amount: vec![Coin {
            denom: token_contract_address,
            amount,
        }],
    }))
}
