#![no_std]
use level2_interface::Error;
use soroban_sdk::{contract, contractevent, contractimpl, log, symbol_short, Address, Env, Symbol};

#[contractevent]
pub struct Failing {
    pub reason: Symbol,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn call_me(env: Env, level3: Address) -> Result<(), Error> {
        log!(&env, "level2: calling level3");
        let client = level3_interface::Client::new(&env, &level3);
        client.do_work();
        log!(&env, "level2: about to fail");
        Failing {
            reason: symbol_short!("always"),
        }
        .publish(&env);
        Err(Error::AlwaysFails)
    }
}

mod test;
