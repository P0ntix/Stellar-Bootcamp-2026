#![no_std]
use soroban_sdk::{contract, contractimpl, Address};

#[contract]
pub struct AuthCheck;

#[contractimpl]
impl AuthCheck {
    pub fn require_owner(owner: Address) {
        owner.require_auth();
    }
}

#[cfg(test)]
mod test;
