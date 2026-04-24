#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct ConditionalsContract;

#[contractimpl]
impl ConditionalsContract {
    pub fn sum_to(limit: u32) -> u32 {
        let mut total = 0;
        let mut i = 0;

        while i <= limit {
            total += i;
            i += 1;
        }

        total
    }
}

#[cfg(test)]
mod test;
