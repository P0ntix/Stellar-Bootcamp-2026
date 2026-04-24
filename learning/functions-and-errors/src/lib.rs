#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MathError {
    DivisionByZero = 1,
}

#[contract]
pub struct FunctionsContract;

#[contractimpl]
impl FunctionsContract {
    pub fn safe_divide(a: u32, b: u32) -> Result<u32, MathError> {
        if b == 0 {
            return Err(MathError::DivisionByZero);
        }
        Ok(a / b)
    }
}

#[cfg(test)]
mod test;
