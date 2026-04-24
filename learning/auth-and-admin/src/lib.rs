#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

const ADMIN_KEY: Symbol = symbol_short!("admin");

#[contract]
pub struct AuthAdminContract;

#[contractimpl]
impl AuthAdminContract {
    pub fn init_admin(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN_KEY) {
            panic!("admin already initialized");
        }
        env.storage().instance().set(&ADMIN_KEY, &admin);
    }

    pub fn protected_action(env: Env, caller: Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN_KEY)
            .expect("admin not initialized");
        if caller != admin {
            panic!("not admin");
        }
    }
}

#[cfg(test)]
mod test;
