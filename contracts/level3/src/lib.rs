#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, log, Env};

#[contractevent]
pub struct WorkDone {
    pub success: bool,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn level3(env: Env) {
        log!(&env, "level3");
        WorkDone { success: true }.publish(&env);
    }
}

mod test;
