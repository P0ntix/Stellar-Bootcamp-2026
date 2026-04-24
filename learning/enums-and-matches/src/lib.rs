#![no_std]
use soroban_sdk::{contract, contractimpl};

#[derive(Clone, Copy)]
pub enum Status {
    Active,
    Paused,
}

#[contract]
pub struct EnumsMatchesContract;

#[contractimpl]
impl EnumsMatchesContract {
    pub fn status_code(is_active: bool) -> u32 {
        let status = if is_active { Status::Active } else { Status::Paused };
        match status {
            Status::Active => 1,
            Status::Paused => 0,
        }
    }
}

#[cfg(test)]
mod test;
