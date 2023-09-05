use cw_multi_test::{App, Contract, ContractWrapper, Executor};
fn contract_furnace(app: &mut App) -> u64 {
    let contract = Box::new(ContractWrapper::new_with_empty(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    ));

    app.store_code(contract)
}

// Using our suite lets test create pair
// and add liquidity to it
#[test]
fn north_star() {
    let mut app = App::default();
    let code_id = contract_furnace(&mut app);

    // Set up the 
    app.init_modules(|router, _, storage| {

        for (addr, coins) in balances {
            router
                .bank
                .init_balance(storage, &addr, coins)
                .expect("init balance");
        }
    });
    }