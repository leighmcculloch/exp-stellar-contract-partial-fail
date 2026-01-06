#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test_level2_fails() {
    let env = Env::default();

    let level3_contract_id = env.register(level3::Contract, ());
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let result = client.try_level2(&level3_contract_id);
    assert_eq!(result, Err(Ok(Error::AlwaysFails)));
}
