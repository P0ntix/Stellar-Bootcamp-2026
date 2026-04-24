#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const VALUE_KEY: Symbol = symbol_short!("value");

#[contract]
pub struct StorageTtlContract;

#[contractimpl]
impl StorageTtlContract {
    pub fn set_value(env: Env, value: u32) {
        env.storage().temporary().set(&VALUE_KEY, &value);
    }

    pub fn get_value(env: Env) -> u32 {
        env.storage().temporary().get(&VALUE_KEY).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
