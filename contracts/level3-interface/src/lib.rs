#![no_std]
use soroban_sdk::{contractclient, Env};

#[contractclient(name = "Client")]
pub trait Interface {
    fn do_work(env: Env);
}
