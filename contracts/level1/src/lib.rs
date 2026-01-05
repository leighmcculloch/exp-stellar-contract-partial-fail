#![no_std]
use soroban_sdk::{contract, contractimpl, log, Address, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn do_something(env: Env, level2: Address, level3: Address) {
        let client = level2_interface::Client::new(&env, &level2);
        match client.try_call_me(&level3) {
            Ok(_) => log!(&env, "level2 succeeded"),
            Err(_) => log!(&env, "level2 failed"),
        }
    }
}

mod test;
