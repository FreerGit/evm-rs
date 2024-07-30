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
            if r2.is_zero() {
                *r2 = U256::ZERO;
            } else {
                *r2 = r1.wrapping_div(*r2);
            }
        }
        Err(result) => interpreter.instruction_result = result.into(),
    }
    interpreter.pc += 3;
}
