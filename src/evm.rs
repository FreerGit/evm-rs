use bytes::Bytes;

use crate::{
    domain::{bytecode::Bytecode, constants::U256},
    interpreter::opcodes::{self, Opcode},
    stack::Stack,
};

pub struct EVM {
    code: Bytecode,
    stack: Stack,
    pc: usize,
}

impl EVM {
    pub fn new(code: Bytes) -> Self {
        Self {
            code: Bytecode::new_legacy(code),
            stack: Stack::new(),
            pc: 0,
        }
    }

    pub fn execute(&mut self) {
        while self.pc < self.code.len() {
            let opcode = self.code.bytes_slice();
            match Opcode::new(opcode[self.pc]) {
                Some(opcode) => {
                    // how do I do this lmfaoooo
                    opcode.instruction()
                    self.pc += 1;
                    let value = opcode[self.pc];
                    self.stack.push(U256::from(value)).unwrap();
                }
                Some(opcodes::POP) => {
                    self.stack.pop().unwrap();
                }
                None => todo!(),
                _ => todo!(),
            }
            self.pc += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let code = Bytes::from_static(&[opcodes::PUSH1, 0x2A, opcodes::POP]);
        let mut evm = EVM::new(code);
        evm.execute();

        assert_eq!(evm.stack.len(), 0);
    }
}
