#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER_KEY: Symbol = symbol_short!("counter");

#[contract]
pub struct CounterStorage;

#[contractimpl]
impl CounterStorage {
    pub fn increment(env: Env) -> u32 {
        let current: u32 = env.storage().persistent().get(&COUNTER_KEY).unwrap_or(0);
        let next = current + 1;
        env.storage().persistent().set(&COUNTER_KEY, &next);
        next
    }
}

#[cfg(test)]
mod test;
