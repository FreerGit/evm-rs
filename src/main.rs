use bytes::Bytes;
use evm_rs::evm::EVM;

fn main() {
    let code = Bytes::from_static(&[0x60, 0x2A, 0x50]);
    let mut evm = EVM::new(code);
    evm.execute();
}
