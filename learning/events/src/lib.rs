#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, String};

#[contract]
pub struct EventsContract;

#[contractimpl]
impl EventsContract {
    pub fn emit_greeting(env: Env, name: String) {
        env.events().publish((symbol_short!("greet"),), name);
    }
}

#[cfg(test)]
mod test;
