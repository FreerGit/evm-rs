pub mod opcodes;
pub mod stack;

use bytes::Bytes;
use stack::{Stack, StackError};

use crate::{domain::bytecode::Bytecode, interpreter::opcodes::Opcode};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum InstructionResult {
    /// The default, any other value signals exit of execution (error).
    #[default]
    Continue,
    Stop,

    StackError(StackError),
}

// pub type Result<T> = std::result::Result<T, InterpreterError>;

#[derive(Debug)]
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

    pub fn run(&mut self) -> InstructionResult {
        while self.instruction_result == InstructionResult::Continue {
            self.step();
        }

        self.instruction_result
    }
}

#[cfg(test)]
mod tests {
    use ruint::aliases::U256;

    use super::*;

    fn build_evm(bytes: &'static [u8]) -> Interpreter {
        let code = Bytes::from_static(bytes);
        Interpreter::new(code)
    }

    #[test]
    fn test_push_pop() {
        let mut evm = build_evm(&[opcodes::PUSH0, opcodes::PUSH0, opcodes::POP, opcodes::POP]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.len(), 0);
    }

    #[test]
    fn add() {
        let mut evm = build_evm(&[opcodes::ADD]);
        evm.stack.push(U256::from(1)).unwrap();
        evm.stack.push(U256::from(2)).unwrap();
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.len(), 1);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(3));
    }

    #[test]
    fn mul() {
        let mut evm = build_evm(&[opcodes::MUL]);
        evm.stack.push(U256::from(3)).unwrap();
        evm.stack.push(U256::from(3)).unwrap();
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(9));
    }

    #[test]
    fn sub() {
        let mut evm = build_evm(&[opcodes::SUB]);
        evm.stack.push(U256::from(3)).unwrap();
        evm.stack.push(U256::from(2)).unwrap();
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::MAX);
    }

    #[test]
    fn div() {
        let mut evm = build_evm(&[opcodes::DIV]);
        evm.stack.push(U256::from(3)).unwrap();
        evm.stack.push(U256::from(3)).unwrap();
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(1));

        evm = build_evm(&[opcodes::DIV]);
        evm.stack.push(U256::from(0)).unwrap();
        evm.stack.push(U256::from(3)).unwrap();
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(0));
    }
}
