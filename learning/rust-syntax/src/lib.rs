#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct SyntaxContract;

#[contractimpl]
impl SyntaxContract {
    pub fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}

#[cfg(test)]
mod test;
