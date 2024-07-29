pub mod opcodes;
pub mod stack;

use bytes::Bytes;
use stack::{Stack, StackError};

use crate::{domain::bytecode::Bytecode, interpreter::opcodes::Opcode};

#[derive(Debug, Default, Eq, PartialEq)]
pub enum InstructionResult {
    /// The default, any other value signals exit of execution (error).
    #[default]
    Continue,
    Stop,

    StackError(StackError),
}

// pub type Result<T> = std::result::Result<T, InterpreterError>;

pub struct Interpreter {
    /// Bytecode that instruction result will point to
    pub bytecode: Bytecode,
    pub stack: Stack,
    pub pc: usize,
    pub instruction_result: InstructionResult,
}

impl Interpreter {
    pub fn new(code: Bytes) -> Self {
        Self {
            bytecode: Bytecode::new_legacy(code),
            stack: Stack::new(),
            pc: 0,
            instruction_result: InstructionResult::Continue,
        }
    }

    pub fn step(&mut self) {
        // let instr_pointer = self.pc;
        if self.pc >= self.bytecode.len() {
            self.instruction_result = InstructionResult::Stop;
        } else {
            let opcode = self.bytecode.bytes_slice()[self.pc];
            match Opcode::new(opcode) {
                Some(op) => op.context().instruction(self),
                None => todo!(),
            }
        }
    }

    pub fn run(&mut self) {
        while self.instruction_result == InstructionResult::Continue {
            self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let code =
            Bytes::from_static(&[opcodes::PUSH0, opcodes::PUSH0, opcodes::POP, opcodes::POP]);
        let mut evm = Interpreter::new(code);
        evm.run();
        println!("{:?}", evm.stack);
        assert_eq!(evm.stack.len(), 0);
    }
}
