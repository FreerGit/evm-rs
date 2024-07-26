use bytes::Bytes;

use crate::{
    domain::{bytecode::Bytecode, constants::U256},
    interpreter::operation::Operation,
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
            match Operation::new(opcode[self.pc]) {
                Some(Operation::PUSH1) => {
                    self.pc += 1;
                    let value = opcode[self.pc];
                    self.stack.push(U256::from(value)).unwrap();
                }
                Some(Operation::POP) => {
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
        let code = Bytes::from_static(&[0x60, 0x2A, 0x50]);
        let mut evm = EVM::new(code);
        evm.execute();

        assert_eq!(evm.stack.len(), 0);
    }
}
