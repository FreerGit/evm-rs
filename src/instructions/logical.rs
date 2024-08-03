use crate::interpreter::Interpreter;

pub fn and(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((r1, r2)) => *r2 = r1 & *r2,
        Err(result) => interpreter.instruction_result = result.into(),
    }
}

pub fn or(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((r1, r2)) => *r2 = r1 | *r2,
        Err(result) => interpreter.instruction_result = result.into(),
    }
}

pub fn xor(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((r1, r2)) => *r2 = r1 ^ *r2,
        Err(result) => interpreter.instruction_result = result.into(),
    }
}

pub fn not(interpreter: &mut Interpreter) {
    match interpreter.stack.top() {
        Ok(r1) => *r1 = !(*r1),
        Err(result) => interpreter.instruction_result = result.into(),
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use ruint::aliases::U256;

    use crate::interpreter::{opcodes, InstructionResult};

    use super::*;

    fn build_evm(bytes: &'static [u8], to_push: &[U256]) -> Interpreter {
        let code = Bytes::from_static(bytes);
        let mut evm = Interpreter::new(code);
        for num in to_push.iter().rev() {
            evm.stack.push(*num).unwrap();
        }
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        evm
    }

    #[test]
    fn bitwise_and() {
        let mut evm = build_evm(&[opcodes::AND], &[U256::from(42), U256::from(42)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(42));
        evm = build_evm(&[opcodes::LT], &[U256::from(42), U256::from(24)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(0));
    }

    #[test]
    fn bitwise_or() {
        let mut evm = build_evm(&[opcodes::OR], &[U256::from(42), U256::from(42)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(42));
        evm = build_evm(&[opcodes::OR], &[U256::from(1), U256::from(2)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(3));
    }

    #[test]
    fn bitwise_xor() {
        let mut evm = build_evm(&[opcodes::XOR], &[U256::from(42), U256::from(42)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(0));
        evm = build_evm(&[opcodes::XOR], &[U256::from(5), U256::from(11)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(14));
    }

    #[test]
    fn bitwise_not() {
        let mut evm = build_evm(&[opcodes::NOT], &[U256::from(0)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::MAX);
        evm = build_evm(&[opcodes::NOT], &[U256::from(5)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::MAX - U256::from(5));
    }
}
