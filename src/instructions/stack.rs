use ruint::aliases::U256;

use crate::interpreter::Interpreter;

pub fn push0(interpreter: &mut Interpreter) {
    if let Err(result) = interpreter.stack.push(U256::ZERO) {
        interpreter.instruction_result = result.into();
    }
}

pub fn pop(interpreter: &mut Interpreter) {
    if let Err(result) = interpreter.stack.pop() {
        interpreter.instruction_result = result.into();
    }
}

pub fn push<const N: usize>(interpreter: &mut Interpreter) {
    const { assert!(matches!(N, 1..=32)) }; // Now THAT is a gamer move.

    let mut value = [0u8; 32];
    let pc = interpreter.pc;
    if interpreter.pc + N <= interpreter.bytecode.len() {
        value[32 - N..].copy_from_slice(&interpreter.bytecode.bytes()[pc..pc + N])
    } else {
        let available_bytes = interpreter.bytecode.bytes().len() - pc;
        value[32 - available_bytes..].copy_from_slice(&interpreter.bytecode.bytes()[pc..]);
    }

    if let Err(result) = interpreter.stack.push(U256::from_be_bytes(value)) {
        interpreter.instruction_result = result.into();
    }
    interpreter.pc += N;
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::interpreter::{opcodes, stack::StackError, InstructionResult};

    use super::*;

    fn build_evm(bytes: &[u8], to_push: &[U256]) -> Interpreter {
        let code = Bytes::copy_from_slice(bytes);
        let mut evm = Interpreter::new(code);
        for num in to_push.iter().rev() {
            evm.stack.push(*num).unwrap();
        }
        evm
    }

    #[test]
    fn push0() {
        let mut evm = build_evm(&[opcodes::PUSH0], &[]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.len(), 1);
        assert_eq!(evm.stack.pop().unwrap(), U256::ZERO);
    }

    #[test]
    fn pop() {
        let mut evm = build_evm(&[opcodes::PUSH0, opcodes::POP], &[]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.len(), 0);

        evm = build_evm(&[opcodes::POP], &[]);
        let instr_res = evm.run();
        assert_eq!(
            instr_res,
            InstructionResult::StackError(StackError::Underflow)
        );
    }

    #[test]
    fn push() {
        let push32_bytes = "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
        let decoded_bytes = &hex::decode(push32_bytes).expect("Invalid hex string")[..];
        let very_very_large_number = U256::from_be_slice(decoded_bytes);
        let push32 = [
            0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad,
            0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef,
            0xde, 0xad, 0xbe, 0xef,
        ];
        let mut bytecode = Vec::new();
        bytecode.extend_from_slice(&[
            opcodes::PUSH1,
            0x01,
            opcodes::PUSH2,
            0x01,
            0x01,
            opcodes::PUSH32,
        ]);
        bytecode.extend_from_slice(&push32);

        let mut evm = build_evm(&bytecode, &[]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.len(), 3);
        assert_eq!(evm.stack.pop().unwrap(), very_very_large_number);
        assert_eq!(evm.stack.len(), 2);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(257));
        assert_eq!(evm.stack.len(), 1);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(1));
    }
}
