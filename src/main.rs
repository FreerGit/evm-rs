use bytes::Bytes;
use evm_rs::interpreter::{InstructionResult, Interpreter};

fn main() {
    let code = Bytes::from_static(&[0x5f, 0x50]);
    let mut evm = Interpreter::new(code);
    while evm.instruction_result == InstructionResult::Continue {
        evm.step();
    }
}
