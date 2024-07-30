use crate::interpreter::Interpreter;

pub fn add(interpreter: &mut Interpreter) {
    match interpreter.stack.pop_top() {
        Ok((r1, r2)) => *r2 = r1 + *r2,
        Err(result) => interpreter.instruction_result = result.into(),
    }
    interpreter.pc += 3;
}
