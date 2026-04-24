#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env};

#[contract]
pub struct TokenLedgerBasicsContract;

#[contractimpl]
impl TokenLedgerBasicsContract {
    pub fn read_balance(env: Env, token_address: Address, user: Address) -> i128 {
        let client = token::Client::new(&env, &token_address);
        client.balance(&user)
    }
}

#[cfg(test)]
mod test;
