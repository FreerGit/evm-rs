use ruint::aliases::U256;

use crate::interpreter::Interpreter;

pub fn lt(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((r1, r2)) => *r2 = U256::from(r1.lt(r2)),
        Err(result) => interpreter.instruction_result = result.into(),
    }
}

pub fn gt(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((r1, r2)) => *r2 = U256::from(r1.gt(r2)),
        Err(result) => interpreter.instruction_result = result.into(),
    }
}

pub fn eq(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((r1, r2)) => *r2 = U256::from(r1.eq(r2)),
        Err(result) => interpreter.instruction_result = result.into(),
    }
}

pub fn iszero(interpreter: &mut Interpreter) {
    match interpreter.stack.top() {
        Ok(r1) => *r1 = U256::from(r1.is_zero()),
        Err(result) => interpreter.instruction_result = result.into(),
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

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
    fn lt() {
        let mut evm = build_evm(&[opcodes::LT], &[U256::from(3), U256::from(4)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(1));
        evm = build_evm(&[opcodes::LT], &[U256::from(3), U256::from(3)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(0));
    }

    #[test]
    fn gt() {
        let mut evm = build_evm(&[opcodes::GT], &[U256::from(3), U256::from(4)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(0));
        evm = build_evm(&[opcodes::GT], &[U256::from(3), U256::from(3)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(0));
    }

    #[test]
    fn eq() {
        let mut evm = build_evm(&[opcodes::EQ], &[U256::from(3), U256::from(3)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(1));
        evm = build_evm(&[opcodes::EQ], &[U256::from(3), U256::from(2)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(0));
    }

    #[test]
    fn iszero() {
        let mut evm = build_evm(&[opcodes::ISZERO], &[U256::from(0)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(1));
        evm = build_evm(&[opcodes::ISZERO], &[U256::from(55)]);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(0));
    }
}
