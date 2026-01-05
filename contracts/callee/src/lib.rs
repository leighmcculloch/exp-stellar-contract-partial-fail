#![no_std]
use callee_interface::Error;
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn call_me() -> Result<(), Error> {
        Err(Error::AlwaysFails)
    }
}

mod test;
