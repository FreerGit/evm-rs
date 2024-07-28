#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Opcode {
    opcode: u8,
    inputs: u8,
    outputs: u8,
}

pub const ABC: u8 = 0x55;

#[macro_export]
macro_rules! opcodes {
    ( $( $name:ident, $value:expr, $inputs:expr, $outputs:expr; )* ) => {

        $(
            pub const $name: u8 = $value;
        )*

        impl Opcode {
            pub fn get(self) -> u8 {
                self.opcode
            }

            pub fn new(value: u8) -> Option<Self> {
                match value {
                    $(
                        $value => Some(Opcode {
                            opcode: $value,
                            inputs: $inputs,
                            outputs: $outputs,
                        }),
                    )*
                    _ => None,
                }
            }

            /// AKA mnemonic
            pub fn name_str(&self) -> &'static str {
                match self.opcode {
                    $(
                        $value => stringify!($name),
                    )*
                    _ => "Unknown Opcode",
                }
            }
        }
    }
}

// TODO: assign the fn calls to each opcode
// Maybe a lookup table?
// Maybe just let it be a generated function?
// not sure what/how to do that.

opcodes! {
    // 10s: Comparison & Bitwise Logic Operations
    STOP,           0x00, 0, 0;
    ADD,            0x01, 2, 1;
    MUL,            0x02, 2, 1;
    SUB,            0x03, 2, 1;
    DIV,            0x04, 2, 1;
    SDIV,           0x05, 2, 1;
    MOD,            0x06, 2, 1;
    SMOD,           0x07, 2, 1;
    ADDMOD,         0x08, 3, 1;
    MULMOD,         0x09, 3, 1;
    EXP,            0x0a, 2, 1;
    SIGNEXTEND,     0x0b, 2, 1;
    LT,             0x10, 2, 1;
    GT,             0x11, 2, 1;
    SLT,            0x12, 2, 1;
    SGT,            0x13, 2, 1;
    EQ,             0x14, 2, 1;
    ISZERO,         0x15, 1, 1;
    AND,            0x16, 2, 1;
    OR,             0x17, 2, 1;
    XOR,            0x18, 2, 1;
    NOT,            0x19, 1, 1;
    BYTE,           0x1a, 2, 1;
    SHL,            0x1b, 2, 1;
    SHR,            0x1c, 2, 1;
    SAR,            0x1d, 2, 1;

    // 20s: KECCAK256 (sometimes refered to as SHA3)
    KECCAK256,      0x20, 2, 1;

    // 30s: Environmental Information
    ADRESS,         0x30, 0, 1;
    BALANCE,        0x31, 1, 1;
    ORIGIN,         0x32, 0, 1;
    CALLER,         0x33, 0, 1;
    CALLVALUE,      0x34, 0, 1;
    CALLDATALOAD,   0x35, 1, 1;
    CALLDATASIZE,   0x35, 1, 1;
    CALLDATACOPY,   0x37, 3, 0;
    CODESIZE,       0x38, 0, 1;
    CODECOPY,       0x39, 3, 0;
    GASPRICE,       0x3a, 0, 1;
    EXTCODESIZE,    0x3b, 1, 1;
    EXTCODECOPY,    0x3c, 4, 0;
    RETURNDATASIZE, 0x3d, 0, 1;
    RETURNDATACOPY, 0x3e, 3, 0;
    EXTCODEHASH,    0x3f, 1, 1;

    // 40s: Block Information
    BLOCKHASH,      0x40, 1, 1;
    COINBASE,       0x41, 0, 1;
    TIMESTAMP,      0x42, 0, 1;
    NUMBER,         0x43, 0, 1;
    PREVRANDAO,     0x44, 0, 1;
    GASLIMIT,       0x45, 0, 1;
    CHAINID,        0x46, 0, 1;
    SELFBALANCE,    0x47, 0, 1;
    BASEFEE,        0x48, 0, 1;

    // 50s: Stack, Memory, Storage and Flow Operations
    POP,            0x50, 1, 0;
    MLOAD,          0x51, 1, 1;
    MSTORE,         0x52, 2, 0;
    SLOAD,          0x54, 1, 1;
    SSTORE,         0x55, 2, 0;
    JUMP,           0x56, 1, 0;
    JUMPI,          0x57, 2, 0;
    PC,             0x58, 0, 1;
    MSIZE,          0x59, 0, 1;
    GAS,            0x5a, 0, 1;
    JUMPDEST,       0x5b, 0, 0;

    // 5f, 60s & 70s: Push Operations
    PUSH0,          0x5f, 0, 1;
    PUSH1,          0x60, 0, 1;
    PUSH2,          0x61, 0, 1;
    PUSH3,          0x62, 0, 1;
    PUSH4,          0x63, 0, 1;
    PUSH5,          0x64, 0, 1;
    PUSH6,          0x65, 0, 1;
    PUSH7,          0x66, 0, 1;
    PUSH8,          0x67, 0, 1;
    PUSH9,          0x68, 0, 1;
    PUSH10,         0x69, 0, 1;
    PUSH11,         0x6a, 0, 1;
    PUSH12,         0x6b, 0, 1;
    PUSH13,         0x6c, 0, 1;
    PUSH14,         0x6d, 0, 1;
    PUSH15,         0x6e, 0, 1;
    PUSH16,         0x6f, 0, 1;
    PUSH17,         0x70, 0, 1;
    PUSH18,         0x71, 0, 1;
    PUSH19,         0x72, 0, 1;
    PUSH20,         0x73, 0, 1;
    PUSH21,         0x74, 0, 1;
    PUSH22,         0x75, 0, 1;
    PUSH23,         0x76, 0, 1;
    PUSH24,         0x77, 0, 1;
    PUSH25,         0x78, 0, 1;
    PUSH26,         0x79, 0, 1;
    PUSH27,         0x7a, 0, 1;
    PUSH28,         0x7b, 0, 1;
    PUSH29,         0x7c, 0, 1;
    PUSH30,         0x7d, 0, 1;
    PUSH31,         0x7e, 0, 1;
    PUSH32,         0x7f, 0, 1;

    // 80s: Duplication Operations
    DUP1,           0x80, 1, 2;
    DUP2,           0x81, 2, 3;
    DUP3,           0x82, 3, 4;
    DUP4,           0x83, 4, 5;
    DUP5,           0x84, 5, 6;
    DUP6,           0x85, 6, 7;
    DUP7,           0x86, 7, 8;
    DUP8,           0x87, 8, 9;
    DUP9,           0x88, 9, 10;
    DUP10,          0x89, 10, 11;
    DUP11,          0x8a, 11, 12;
    DUP12,          0x8b, 12, 13;
    DUP13,          0x8c, 13, 14;
    DUP14,          0x8d, 14, 15;
    DUP15,          0x8e, 15, 16;
    DUP16,          0x8f, 16, 17;

    // 90s: Exchange Operations
    SWAP1,          0x90, 2, 2;
    SWAP2,          0x91, 3, 3;
    SWAP3,          0x92, 4, 4;
    SWAP4,          0x93, 5, 5;
    SWAP5,          0x94, 6, 6;
    SWAP6,          0x95, 7, 7;
    SWAP7,          0x96, 8, 8;
    SWAP8,          0x97, 9, 9;
    SWAP9,          0x98, 10, 10;
    SWAP10,         0x99, 11, 11;
    SWAP11,         0x9a, 12, 12;
    SWAP12,         0x9b, 13, 13;
    SWAP13,         0x9c, 14, 14;
    SWAP14,         0x9d, 15, 15;
    SWAP15,         0x9e, 16, 16;
    SWAP16,         0x9f, 17, 17;

    // a0s: Logging Operations
    LOG0,           0xa0, 2, 0;
    LOG1,           0xa1, 3, 0;
    LOG2,           0xa2, 4, 0;
    LOG3,           0xa3, 5, 0;
    LOG4,           0xa4, 6, 0;

    // f0s: System Operations
    CREATE,         0xf0, 3, 1;
    CALL,           0xf1, 7, 1;
    CALLCODE,       0xf2, 7, 1;
    RETURN,         0xf3, 2, 0;
    DELEGATECALL,   0xf4, 6, 1;
    CRATE32,        0xf5, 4, 1;
    STATICCALL,     0xfa, 6, 1;
    REVERT,         0xfd, 2, 0;
    INVALID,        0xfe, 0, 0;
    SELFDESTRUCT,   0xff, 1, 0;

}

// pub struct OperationContext {
//     immediate_size: u8,
//     input: u8,
//     output: u8,
//     is_terminating: bool,
//     // set EVM revision
// }

// impl OperationContext {
//     pub fn get_instruction(self) -> fn() -> () {
//         match self {
//             Operation::ADD => arithemtic::add(),
//             _ => todo!(),
//         }
//     }
// }

// pub struct OperationInfo {
//     inputs: u8,
//     outputs: u8,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion() {
        let add: u8 = 0x01;
        let add_op = Operation::ADD;
        assert_eq!(Operation::new(add).unwrap(), add_op);
        assert_eq!(Operation::get(add_op), add);
    }
}
