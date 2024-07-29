use bytes::Bytes;
use evm_rs::interpreter::Interpreter;

fn main() {
    let code = Bytes::from_static(&[0x5f, 0x50]);
    let mut evm = Interpreter::new(code);
    evm.run();
}
