#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map, Symbol, Vec};

const USERS_KEY: Symbol = symbol_short!("users");

#[contract]
pub struct MapsKeysContract;

#[contractimpl]
impl MapsKeysContract {
    pub fn register(env: Env, user: Address, score: u32) {
        let mut users: Map<Address, u32> = env
            .storage()
            .persistent()
            .get(&USERS_KEY)
            .unwrap_or(Map::new(&env));
        users.set(user, score);
        env.storage().persistent().set(&USERS_KEY, &users);
    }

    pub fn list_scores(env: Env) -> Vec<u32> {
        let users: Map<Address, u32> = env
            .storage()
            .persistent()
            .get(&USERS_KEY)
            .unwrap_or(Map::new(&env));
        let mut out = Vec::new(&env);
        for (_, value) in users.iter() {
            out.push_back(value);
        }
        out
    }
}

#[cfg(test)]
mod test;
