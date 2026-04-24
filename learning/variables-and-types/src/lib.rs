#![no_std]
use soroban_sdk::{contract, contractimpl, Vec, Env};

#[contract]
pub struct VariablesContract;

#[contractimpl]
impl VariablesContract {
    pub fn make_vector(env: Env) -> Vec<u32> {
        let mut values = Vec::new(&env);
        let base: u32 = 10;
        values.push_back(base);
        values.push_back(base + 5);
        values
    }
}

#[cfg(test)]
mod test;
