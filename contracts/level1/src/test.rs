#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test_level1_succeeds() {
    let env = Env::default();

    // Register level3 contract (innermost, succeeds)
    let level3_contract_id = env.register(level3::Contract, ());

    // Register level2 contract (middle, fails after calling level3)
    let level2_contract_id = env.register(level2::Contract, ());

    // Register level1 contract (outermost, catches level2's failure)
    let level1_contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &level1_contract_id);

    // This should succeed even though level2 fails
    client.level1(&level2_contract_id, &level3_contract_id);
}
