# Partial Fail

A Soroban smart contract example demonstrating graceful error handling using the `try_` variant for cross-contract calls.

## Overview

This project contains two contracts:

- **callee** - A contract with a `call_me` function that always fails with a contract error
- **caller** - A contract that calls the callee using `try_call_me()` to handle the error gracefully

The caller contract logs whether the called contract succeeded or failed, demonstrating how to use the `try_` variant to catch errors from cross-contract calls without panicking.

## Project Structure

```text
.
├── contracts
│   ├── callee              # Contract that always fails
│   │   ├── src
│   │   │   ├── lib.rs
│   │   │   └── test.rs
│   │   └── Cargo.toml
│   ├── callee-interface    # Interface crate with error type and client
│   │   ├── src
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── caller              # Contract that calls callee with try variant
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## Building

```bash
stellar contract build
```

## Testing

```bash
cargo test
```

## How It Works

The `callee-interface` crate defines the contract interface separately from the implementation:

```rust
#[contracterror]
pub enum Error {
    AlwaysFails = 1,
}

#[contractclient(name = "Client")]
pub trait Interface {
    fn call_me(env: Env) -> Result<(), Error>;
}
```

The `caller` contract uses the interface to call the callee:

```rust
pub fn do_something(env: Env, contract: Address) {
    let client = callee_interface::Client::new(&env, &contract);
    match client.try_call_me() {
        Ok(_) => log!(&env, "called contract succeeded"),
        Err(_) => log!(&env, "called contract failed"),
    }
}
```

By using `try_call_me()` instead of `call_me()`, the caller receives a `Result` and can handle the error without the transaction failing.
