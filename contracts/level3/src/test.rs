#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test_level3_succeeds() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    client.level3();
}
