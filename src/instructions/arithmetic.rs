use ruint::aliases::U256;

use crate::interpreter::Interpreter;

pub fn add(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((r1, r2)) => *r2 = r1.wrapping_add(*r2),
        Err(result) => interpreter.instruction_result = result.into(),
    }
    interpreter.pc += 3;
}

pub fn mul(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((r1, r2)) => *r2 = r1.wrapping_mul(*r2),
        Err(result) => interpreter.instruction_result = result.into(),
    }
    interpreter.pc += 3;
}

pub fn sub(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((r1, r2)) => *r2 = r1.wrapping_sub(*r2),
        Err(result) => interpreter.instruction_result = result.into(),
    }
    interpreter.pc += 3;
}

pub fn div(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((r1, r2)) => {
            if *r2 != U256::ZERO {
                *r2 = r1.wrapping_div(*r2);
            }
        }
        Err(result) => interpreter.instruction_result = result.into(),
    }
    interpreter.pc += 3;
}

pub fn rem(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((r1, r2)) => {
            if *r2 != U256::ZERO {
                *r2 = r1.wrapping_rem(*r2);
            }
        }
        Err(result) => interpreter.instruction_result = result.into(),
    }
    interpreter.pc += 3;
}

pub fn smod(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((_, _)) => todo!(),
        Err(result) => interpreter.instruction_result = result.into(),
    }
    interpreter.pc += 3;
}

pub fn addmod(interpreter: &mut Interpreter) {
    match interpreter.stack.pop2_top() {
        Ok((r1, r2, r3)) => *r3 = r1.add_mod(r2, *r3),
        Err(result) => interpreter.instruction_result = result.into(),
    }
    interpreter.pc += 4;
}

pub fn mulmod(interpreter: &mut Interpreter) {
    match interpreter.stack.pop2_top() {
        Ok((r1, r2, r3)) => *r3 = r1.mul_mod(r2, *r3),
        Err(result) => interpreter.instruction_result = result.into(),
    }
    interpreter.pc += 4;
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
        evm
    }

    #[test]
    fn add() {
        let mut evm = build_evm(&[opcodes::ADD], &[U256::from(1), U256::from(2)]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.len(), 1);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(3));
    }

    #[test]
    fn mul() {
        let mut evm = build_evm(&[opcodes::MUL], &[U256::from(3), U256::from(3)]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(9));
    }

    #[test]
    fn sub() {
        let mut evm = build_evm(&[opcodes::SUB], &[U256::from(3), U256::from(2)]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(1));

        let mut evm = build_evm(&[opcodes::SUB], &[U256::from(2), U256::from(3)]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::MAX);
    }

    #[test]
    fn div() {
        let mut evm = build_evm(&[opcodes::DIV], &[U256::from(3), U256::from(3)]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(1));

        evm = build_evm(&[opcodes::DIV], &[U256::from(0), U256::from(3)]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(0));
    }

    #[test]
    fn rem() {
        let mut evm = build_evm(&[opcodes::MOD], &[U256::from(3), U256::from(3)]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(0));

        evm = build_evm(&[opcodes::MOD], &[U256::from(6), U256::from(4)]);
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(2));
    }

    #[test]
    fn add_mod() {
        let mut evm = build_evm(
            &[opcodes::ADDMOD],
            &[U256::from(4), U256::from(4), U256::from(3)],
        );
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(2));

        evm = build_evm(
            &[opcodes::MOD],
            &[U256::from(1), U256::from(1), U256::from(2)],
        );
        let instr_res = evm.run();
        assert_eq!(instr_res, InstructionResult::Stop);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(0));
    }
}
