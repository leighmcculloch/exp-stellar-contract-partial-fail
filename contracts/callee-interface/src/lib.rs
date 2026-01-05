#![no_std]
use soroban_sdk::{contractclient, contracterror, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlwaysFails = 1,
}

#[contractclient(name = "Client")]
pub trait Interface {
    fn call_me(env: Env) -> Result<(), Error>;
}
