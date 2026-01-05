#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test_do_something_succeeds() {
    let env = Env::default();

    // Register the callee contract
    let callee_contract_id = env.register(callee::Contract, ());

    // Register the caller contract
    let caller_contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &caller_contract_id);

    // This should succeed even though the callee contract fails
    client.do_something(&callee_contract_id);
}
