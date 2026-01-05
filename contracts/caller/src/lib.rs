#![no_std]
use soroban_sdk::{contract, contractimpl, log, Address, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn do_something(env: Env, contract: Address) {
        let client = callee_interface::Client::new(&env, &contract);
        match client.try_call_me() {
            Ok(_) => log!(&env, "called contract succeeded"),
            Err(_) => log!(&env, "called contract failed"),
        }
    }
}

mod test;
