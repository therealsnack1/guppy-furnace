use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Addr, CodeInfoResponse, Coin, Empty, HexBinary, OwnedDeps,
    Querier, QuerierResult, QueryRequest, SystemError, SystemResult, Uint128, WasmQuery,
};

use std::collections::HashMap;

use cw20::{BalanceResponse, Cw20QueryMsg, TokenInfoResponse};

/// mock_dependencies is a drop-in replacement for cosmwasm_std::testing::mock_dependencies
pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier: WasmMockQuerier =
        WasmMockQuerier::new(MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: Default::default(),
    }
}
enum QueryHandler {
    Default,
    Cw20,
}

pub struct WasmMockQuerier {
    query_handler: DefaultQueryHandler,
    cw20_query_handler: CW20QueryHandler,
    handler: QueryHandler,
}

#[derive(Clone, Default)]
pub struct TokenQuerier {
    /// This lets us iterate over all pairs that match the first string
    balances: HashMap<String, HashMap<String, Uint128>>,
}

impl TokenQuerier {
    pub fn new(balances: &[(&String, &[(&String, &Uint128)])]) -> Self {
        TokenQuerier {
            balances: balances_to_map(balances),
        }
    }
}

pub(crate) fn balances_to_map(
    balances: &[(&String, &[(&String, &Uint128)])],
) -> HashMap<String, HashMap<String, Uint128>> {
    let mut balances_map: HashMap<String, HashMap<String, Uint128>> = HashMap::new();
    for (contract_addr, balances) in balances.iter() {
        let mut contract_balances_map: HashMap<String, Uint128> = HashMap::new();
        for (addr, balance) in balances.iter() {
            contract_balances_map.insert(addr.to_string(), **balance);
        }

        balances_map.insert(contract_addr.to_string(), contract_balances_map);
    }
    balances_map
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                });
            }
        };
        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match self.handler {
            QueryHandler::Default => self.query_handler.execute(request),
            QueryHandler::Cw20 => self.cw20_query_handler.execute(request),
        }
    }
}

struct CW20QueryHandler {
    token_querier: TokenQuerier,
}

impl CW20QueryHandler {
    pub fn execute(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                match from_binary(msg).unwrap() {
                    Cw20QueryMsg::TokenInfo {} => {
                        let balances: &HashMap<String, Uint128> =
                            match self.token_querier.balances.get(contract_addr) {
                                Some(balances) => balances,
                                None => {
                                    return SystemResult::Err(SystemError::Unknown {});
                                }
                            };

                        let mut total_supply = Uint128::zero();

                        for balance in balances {
                            total_supply += *balance.1;
                        }

                        SystemResult::Ok(
                            to_binary(&TokenInfoResponse {
                                name: "mAPPL".to_string(),
                                symbol: "mAPPL".to_string(),
                                decimals: 6,
                                total_supply,
                            })
                            .into(),
                        )
                    }
                    Cw20QueryMsg::Balance { address } => {
                        let balances: &HashMap<String, Uint128> =
                            match self.token_querier.balances.get(contract_addr) {
                                Some(balances) => balances,
                                None => {
                                    return SystemResult::Err(SystemError::Unknown {});
                                }
                            };

                        let balance = match balances.get(&address) {
                            Some(v) => v,
                            None => {
                                return SystemResult::Err(SystemError::Unknown {});
                            }
                        };

                        SystemResult::Ok(to_binary(&BalanceResponse { balance: *balance }).into())
                    }
                    _ => panic!("DO NOT ENTER HERE"),
                }
            }
            _ => panic!("DO NOT ENTER HERE"),
        }
    }
}

struct DefaultQueryHandler {
    base: MockQuerier<Empty>,
}

impl DefaultQueryHandler {
    pub fn execute(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::CodeInfo { code_id }) => {
                let mut default = CodeInfoResponse::default();

                match code_id {
                    1 => {
                        default.code_id = 67;
                        default.creator = Addr::unchecked("creator").to_string();
                        default.checksum = HexBinary::from_hex(
                            "f7bb7b18fb01bbf425cf4ed2cd4b7fb26a019a7fc75a4dc87e8a0b768c501f00",
                        )
                        .unwrap();
                        SystemResult::Ok(to_binary(&default).into())
                    }
                    _ => {
                        return SystemResult::Err(SystemError::Unknown {});
                    }
                }
            }
            _ => self.base.handle_query(request),
        }
    }
}

impl WasmMockQuerier {
    pub fn new(base: MockQuerier<Empty>) -> Self {
        WasmMockQuerier {
            query_handler: DefaultQueryHandler { base },
            cw20_query_handler: CW20QueryHandler {
                token_querier: TokenQuerier::default(),
            },
            handler: QueryHandler::Default,
        }
    }

    // Configure the mint whitelist mock querier
    pub fn with_token_balances(&mut self, balances: &[(&String, &[(&String, &Uint128)])]) {
        self.cw20_query_handler.token_querier = TokenQuerier::new(balances);
    }

    pub fn with_default_query_handler(&mut self) {
        self.handler = QueryHandler::Default;
    }

    pub fn with_cw20_query_handler(&mut self) {
        self.handler = QueryHandler::Cw20;
    }
}
