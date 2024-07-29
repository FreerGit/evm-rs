use crate::{domain::constants::U256, interpreter::Interpreter};

pub fn push0(interpreter: &mut Interpreter) {
    interpreter.pc += 1;
    if let Err(result) = interpreter.stack.push(U256::ZERO) {
        interpreter.instruction_result = result.into();
    }
}

pub fn pop(interpreter: &mut Interpreter) {
    interpreter.pc += 1;
    if let Err(result) = interpreter.stack.pop() {
        interpreter.instruction_result = result.into();
    }
}
