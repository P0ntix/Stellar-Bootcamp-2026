#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, String};

#[contract]
pub struct EventsDemo;

#[contractimpl]
impl EventsDemo {
    pub fn emit_note(env: Env, note: String) {
        env.events().publish((symbol_short!("note"),), note);
    }
}

#[cfg(test)]
mod test;
