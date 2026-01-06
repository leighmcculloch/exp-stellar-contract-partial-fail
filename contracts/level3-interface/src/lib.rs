#![no_std]
use soroban_sdk::{contractclient, Env};

#[contractclient(name = "Client")]
pub trait Interface {
    fn level3(env: Env);
}
