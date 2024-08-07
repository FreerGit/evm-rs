/// EVM interpreter stack limit (of U256)
pub const STACK_LIMIT: usize = 1024;
use std::vec::Vec;

use ruint::aliases::U256;

use super::InstructionResult;

pub type Result<T> = std::result::Result<T, StackError>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StackError {
    Overflow,
    Underflow,
}

impl From<StackError> for InstructionResult {
    fn from(value: StackError) -> Self {
        InstructionResult::StackError(value)
    }
}

#[derive(Debug)]
pub struct Stack {
    data: Vec<U256>,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(STACK_LIMIT),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push(&mut self, value: U256) -> Result<()> {
        if self.data.len() >= STACK_LIMIT {
            return Err(StackError::Overflow);
        }
        self.data.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<U256> {
        self.data.pop().ok_or(StackError::Underflow)
    }

    pub fn top(&mut self) -> Result<&mut U256> {
        let len = self.data.len();
        if len > 0 {
            Ok(&mut self.data[len - 1])
        } else {
            Err(StackError::Underflow)
        }
    }

    /// Pops a item from the stack then returns a reference to the top of the stack.
    /// This is equal to calling `pop()` -> `top()`.
    pub fn pop_top(&mut self) -> Result<(U256, &mut U256)> {
        Ok((self.pop()?, self.top()?))
    }

    /// Pops two items from the stack then returns a reference to the top of the stack.
    /// This is equal to calling `pop()` -> `pop()` -> `top()`.
    pub fn pop2_top(&mut self) -> Result<(U256, U256, &mut U256)> {
        Ok((self.pop()?, self.pop()?, self.top()?))
    }
}
