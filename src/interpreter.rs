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
        if self.pc >= self.bytecode.len() {
            self.instruction_result = InstructionResult::Stop;
        } else {
            let opcode = self.bytecode.bytes_slice()[self.pc];
            self.pc += 1;
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
    use bytes::Bytes;
    use ruint::aliases::U256;

    use crate::interpreter::{opcodes, InstructionResult};

    use super::Interpreter;

    fn build_evm(bytes: &'static [u8]) -> Interpreter {
        let code = Bytes::from_static(bytes);
        Interpreter::new(code)
    }

    fn build_evm_w_stack(bytes: &'static [u8], to_push: &[U256]) -> Interpreter {
        let code = Bytes::from_static(bytes);
        let mut evm = Interpreter::new(code);
        for num in to_push.iter().rev() {
            evm.stack.push(*num).unwrap();
        }
        evm
    }

    #[test]
    fn test_push_pop() {
        let mut evm = build_evm(&[opcodes::PUSH0, opcodes::PUSH0, opcodes::POP, opcodes::POP]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.len(), 0);
    }

    #[test]
    fn pc() {
        let mut evm = build_evm_w_stack(
            &[
                opcodes::PUSH0,
                opcodes::POP,
                opcodes::ADD,
                opcodes::ADDMOD,
                opcodes::PUSH0,
                opcodes::POP,
            ],
            &[U256::from(2), U256::from(2), U256::from(3), U256::from(5)],
        );

        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        println!("{:?}", evm);
        assert_eq!(evm.stack.len(), 1);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(2));
    }
}
