#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct HelloSoroban;

#[contractimpl]
impl HelloSoroban {
    pub fn greet(env: Env) -> String {
        String::from_str(&env, "Hello from Soroban examples!")
    }
}

#[cfg(test)]
mod test;
