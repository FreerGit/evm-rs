use crate::instructions::stack;

use super::Interpreter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Opcode(u8);

impl Opcode {
    pub fn new(opcode: u8) -> Option<Self> {
        match OPCODE_JUMPTABLE[opcode as usize] {
            Some(_) => Some(Opcode(opcode)),
            _ => None,
        }
    }

    pub fn context(self) -> OpcodeContext {
        return OPCODE_JUMPTABLE[self.0 as usize].unwrap();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OpcodeContext {
    inputs: u8,
    outputs: u8,
    instruction: fn(&mut Interpreter),
}

impl OpcodeContext {
    pub fn instruction(&self, interpreter: &mut Interpreter) {
        (self.instruction)(interpreter)
    }
}

fn todo_instr(i: &mut Interpreter) {
    todo!()
}

macro_rules! opcodes {
    (
        $(
            $name:ident, $value:literal, $instr:expr, $inputs:expr, $outputs:expr;
        )*
    ) => {
        impl Opcode {$(
            pub const $name: Self = Self($value);
        )*}

        /// Get the context of a given opcode
        pub const OPCODE_JUMPTABLE: [Option<OpcodeContext>; 256] = {
            let mut table = [None; 256];
            $(
                table[$value] = Some(OpcodeContext {
                    inputs: $inputs,
                    outputs: $outputs,
                    instruction: $instr,
                });
            )*
            table


        };


        impl Opcode {
            pub fn to_str(&self) -> Option<&'static str> {
                match self.0 {
                    $(
                        $value => Some(stringify!($name)),
                    )*
                    _ => None,
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
    STOP,           0x00, todo_instr, 0, 0;
    ADD,            0x01, todo_instr, 2, 1;
    MUL,            0x02, todo_instr, 2, 1;
    SUB,            0x03, todo_instr, 2, 1;
    DIV,            0x04, todo_instr, 2, 1;
    SDIV,           0x05, todo_instr, 2, 1;
    MOD,            0x06, todo_instr, 2, 1;
    SMOD,           0x07, todo_instr, 2, 1;
    ADDMOD,         0x08, todo_instr, 3, 1;
    MULMOD,         0x09, todo_instr, 3, 1;
    EXP,            0x0a, todo_instr, 2, 1;
    SIGNEXTEND,     0x0b, todo_instr, 2, 1;
    LT,             0x10, todo_instr, 2, 1;
    GT,             0x11, todo_instr, 2, 1;
    SLT,            0x12, todo_instr, 2, 1;
    SGT,            0x13, todo_instr, 2, 1;
    EQ,             0x14, todo_instr, 2, 1;
    ISZERO,         0x15, todo_instr, 1, 1;
    AND,            0x16, todo_instr, 2, 1;
    OR,             0x17, todo_instr, 2, 1;
    XOR,            0x18, todo_instr, 2, 1;
    NOT,            0x19, todo_instr, 1, 1;
    BYTE,           0x1a, todo_instr, 2, 1;
    SHL,            0x1b, todo_instr, 2, 1;
    SHR,            0x1c, todo_instr, 2, 1;
    SAR,            0x1d, todo_instr, 2, 1;

    // 20s: KECCAK256 (sometimes refered to as SHA3)
    KECCAK256,      0x20, todo_instr, 2, 1;

    // 30s: Environmental Information
    ADRESS,         0x30, todo_instr, 0, 1;
    BALANCE,        0x31, todo_instr, 1, 1;
    ORIGIN,         0x32, todo_instr, 0, 1;
    CALLER,         0x33, todo_instr, 0, 1;
    CALLVALUE,      0x34, todo_instr, 0, 1;
    CALLDATALOAD,   0x35, todo_instr, 1, 1;
    CALLDATASIZE,   0x36, todo_instr, 1, 1;
    CALLDATACOPY,   0x37, todo_instr, 3, 0;
    CODESIZE,       0x38, todo_instr, 0, 1;
    CODECOPY,       0x39, todo_instr, 3, 0;
    GASPRICE,       0x3a, todo_instr, 0, 1;
    EXTCODESIZE,    0x3b, todo_instr, 1, 1;
    EXTCODECOPY,    0x3c, todo_instr, 4, 0;
    RETURNDATASIZE, 0x3d, todo_instr, 0, 1;
    RETURNDATACOPY, 0x3e, todo_instr, 3, 0;
    EXTCODEHASH,    0x3f, todo_instr, 1, 1;

    // 40s: Block Information
    BLOCKHASH,      0x40, todo_instr, 1, 1;
    COINBASE,       0x41, todo_instr, 0, 1;
    TIMESTAMP,      0x42, todo_instr, 0, 1;
    NUMBER,         0x43, todo_instr, 0, 1;
    PREVRANDAO,     0x44, todo_instr, 0, 1;
    GASLIMIT,       0x45, todo_instr, 0, 1;
    CHAINID,        0x46, todo_instr, 0, 1;
    SELFBALANCE,    0x47, todo_instr, 0, 1;
    BASEFEE,        0x48, todo_instr, 0, 1;

    // 50s: Stack, Memory, Storage and Flow Operations
    POP,            0x50, stack::pop, 1, 0;
    MLOAD,          0x51, todo_instr, 1, 1;
    MSTORE,         0x52, todo_instr, 2, 0;
    SLOAD,          0x54, todo_instr, 1, 1;
    SSTORE,         0x55, todo_instr, 2, 0;
    JUMP,           0x56, todo_instr, 1, 0;
    JUMPI,          0x57, todo_instr, 2, 0;
    PC,             0x58, todo_instr, 0, 1;
    MSIZE,          0x59, todo_instr, 0, 1;
    GAS,            0x5a, todo_instr, 0, 1;
    JUMPDEST,       0x5b, todo_instr, 0, 0;

    // 5f, 60s & 70s: Push Operations
    PUSH0,          0x5f, stack::push0, 0, 1;
    PUSH1,          0x60, todo_instr, 0, 1;
    PUSH2,          0x61, todo_instr, 0, 1;
    PUSH3,          0x62, todo_instr, 0, 1;
    PUSH4,          0x63, todo_instr, 0, 1;
    PUSH5,          0x64, todo_instr, 0, 1;
    PUSH6,          0x65, todo_instr, 0, 1;
    PUSH7,          0x66, todo_instr, 0, 1;
    PUSH8,          0x67, todo_instr, 0, 1;
    PUSH9,          0x68, todo_instr, 0, 1;
    PUSH10,         0x69, todo_instr, 0, 1;
    PUSH11,         0x6a, todo_instr, 0, 1;
    PUSH12,         0x6b, todo_instr, 0, 1;
    PUSH13,         0x6c, todo_instr, 0, 1;
    PUSH14,         0x6d, todo_instr, 0, 1;
    PUSH15,         0x6e, todo_instr, 0, 1;
    PUSH16,         0x6f, todo_instr, 0, 1;
    PUSH17,         0x70, todo_instr, 0, 1;
    PUSH18,         0x71, todo_instr, 0, 1;
    PUSH19,         0x72, todo_instr, 0, 1;
    PUSH20,         0x73, todo_instr, 0, 1;
    PUSH21,         0x74, todo_instr, 0, 1;
    PUSH22,         0x75, todo_instr, 0, 1;
    PUSH23,         0x76, todo_instr, 0, 1;
    PUSH24,         0x77, todo_instr, 0, 1;
    PUSH25,         0x78, todo_instr, 0, 1;
    PUSH26,         0x79, todo_instr, 0, 1;
    PUSH27,         0x7a, todo_instr, 0, 1;
    PUSH28,         0x7b, todo_instr, 0, 1;
    PUSH29,         0x7c, todo_instr, 0, 1;
    PUSH30,         0x7d, todo_instr, 0, 1;
    PUSH31,         0x7e, todo_instr, 0, 1;
    PUSH32,         0x7f, todo_instr, 0, 1;

    // 80s: Duplication Operations
    DUP1,           0x80, todo_instr, 1, 2;
    DUP2,           0x81, todo_instr, 2, 3;
    DUP3,           0x82, todo_instr, 3, 4;
    DUP4,           0x83, todo_instr, 4, 5;
    DUP5,           0x84, todo_instr, 5, 6;
    DUP6,           0x85, todo_instr, 6, 7;
    DUP7,           0x86, todo_instr, 7, 8;
    DUP8,           0x87, todo_instr, 8, 9;
    DUP9,           0x88, todo_instr, 9, 10;
    DUP10,          0x89, todo_instr, 10, 11;
    DUP11,          0x8a, todo_instr, 11, 12;
    DUP12,          0x8b, todo_instr, 12, 13;
    DUP13,          0x8c, todo_instr, 13, 14;
    DUP14,          0x8d, todo_instr, 14, 15;
    DUP15,          0x8e, todo_instr, 15, 16;
    DUP16,          0x8f, todo_instr, 16, 17;

    // 90s: Exchange Operations
    SWAP1,          0x90, todo_instr, 2, 2;
    SWAP2,          0x91, todo_instr, 3, 3;
    SWAP3,          0x92, todo_instr, 4, 4;
    SWAP4,          0x93, todo_instr, 5, 5;
    SWAP5,          0x94, todo_instr, 6, 6;
    SWAP6,          0x95, todo_instr, 7, 7;
    SWAP7,          0x96, todo_instr, 8, 8;
    SWAP8,          0x97, todo_instr, 9, 9;
    SWAP9,          0x98, todo_instr, 10, 10;
    SWAP10,         0x99, todo_instr, 11, 11;
    SWAP11,         0x9a, todo_instr, 12, 12;
    SWAP12,         0x9b, todo_instr, 13, 13;
    SWAP13,         0x9c, todo_instr, 14, 14;
    SWAP14,         0x9d, todo_instr, 15, 15;
    SWAP15,         0x9e, todo_instr, 16, 16;
    SWAP16,         0x9f, todo_instr, 17, 17;

    // a0s: Logging Operations
    LOG0,           0xa0, todo_instr, 2, 0;
    LOG1,           0xa1, todo_instr, 3, 0;
    LOG2,           0xa2, todo_instr, 4, 0;
    LOG3,           0xa3, todo_instr, 5, 0;
    LOG4,           0xa4, todo_instr, 6, 0;

    // f0s: System Operations
    CREATE,         0xf0, todo_instr, 3, 1;
    CALL,           0xf1, todo_instr, 7, 1;
    CALLCODE,       0xf2, todo_instr, 7, 1;
    RETURN,         0xf3, todo_instr, 2, 0;
    DELEGATECALL,   0xf4, todo_instr, 6, 1;
    CRATE32,        0xf5, todo_instr, 4, 1;
    STATICCALL,     0xfa, todo_instr, 6, 1;
    REVERT,         0xfd, todo_instr, 2, 0;
    INVALID,        0xfe, todo_instr, 0, 0;
    SELFDESTRUCT,   0xff, todo_instr, 1, 0;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion() {
        let add_op = super::Opcode::PUSH0;
        assert_eq!(Opcode::new(add_op.0).unwrap(), add_op);
    }
}
