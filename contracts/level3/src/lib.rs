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
    pub fn do_work(env: Env) {
        log!(&env, "level3: doing work");
        WorkDone { success: true }.publish(&env);
    }
}

mod test;
