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
