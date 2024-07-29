pub mod opcodes;
pub mod stack;

use bytes::Bytes;
use stack::{Stack, StackError};

use crate::{
    domain::{bytecode::Bytecode, constants::U256},
    interpreter::opcodes::Opcode,
};

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
        self.pc += 1;
    }

    pub fn run(&mut self) {
        // while self.pc < self.code.len() {
        //     let opcode = self.code.bytes_slice();

        //     match Opcode::new(opcode[self.pc]) {
        //         Some(Opcode::PUSH0) => {
        //             let context = Opcode::context(Opcode::PUSH0);
        //             // opcode.instruction(&mut self);
        //         }
        //         Some(Opcode::POP) => {
        //             let context = Opcode::context(Opcode::POP);
        //             context.instruction(self).unwrap();
        //         }
        //         None => todo!(),
        //         _ => todo!(),
        //     }
        //     self.pc += 1;
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let code = Bytes::from_static(&[opcodes::PUSH1, 0x2A, opcodes::POP]);
        let mut evm = Interpreter::new(code);
        evm.execute();

        assert_eq!(evm.stack.len(), 0);
    }
}
