#![no_std]
use soroban_sdk::{contract, contractimpl};

#[derive(Clone, Copy)]
pub struct Counter {
    pub value: u32,
}

#[contract]
pub struct StructsMethodsContract;

#[contractimpl]
impl StructsMethodsContract {
    pub fn increment_from(start: u32) -> u32 {
        let counter = Counter { value: start };
        counter.value + 1
    }
}

#[cfg(test)]
mod test;
