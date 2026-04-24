#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const YES_KEY: Symbol = symbol_short!("yes");
const NO_KEY: Symbol = symbol_short!("no");

#[contract]
pub struct SimpleVoting;

#[contractimpl]
impl SimpleVoting {
    pub fn vote_yes(env: Env) -> u32 {
        let current: u32 = env.storage().persistent().get(&YES_KEY).unwrap_or(0);
        let next = current + 1;
        env.storage().persistent().set(&YES_KEY, &next);
        next
    }

    pub fn vote_no(env: Env) -> u32 {
        let current: u32 = env.storage().persistent().get(&NO_KEY).unwrap_or(0);
        let next = current + 1;
        env.storage().persistent().set(&NO_KEY, &next);
        next
    }
}

#[cfg(test)]
mod test;
