#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test_call_me_fails() {
    let env = Env::default();

    let level3_contract_id = env.register(level3::Contract, ());
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let result = client.try_call_me(&level3_contract_id);
    assert_eq!(result, Err(Ok(Error::AlwaysFails)));
}
