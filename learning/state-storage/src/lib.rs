#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNT_KEY: Symbol = symbol_short!("count");

#[contract]
pub struct StateStorageContract;

#[contractimpl]
impl StateStorageContract {
    pub fn increment(env: Env) -> u32 {
        let current = env.storage().persistent().get(&COUNT_KEY).unwrap_or(0u32);
        let next = current + 1;
        env.storage().persistent().set(&COUNT_KEY, &next);
        next
    }

    pub fn get_count(env: Env) -> u32 {
        env.storage().persistent().get(&COUNT_KEY).unwrap_or(0u32)
    }
}

#[cfg(test)]
mod test;
