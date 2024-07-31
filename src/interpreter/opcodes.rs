use crate::instructions::{arithmetic, stack};

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
        OPCODE_JUMPTABLE[self.0 as usize].unwrap()
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

    /// Returns the additional items placed on the stack (α)
    pub fn inputs(&self) -> u8 {
        self.inputs
    }

    /// Returns the number of items removed from the stack (δ)
    pub fn outputs(&self) -> u8 {
        self.outputs
    }
}

fn todo_instr(_: &mut Interpreter) {
    todo!()
}

macro_rules! opcodes {
    (
        $(
            $name:ident, $value:literal, $instr:expr, $inputs:expr, $outputs:expr;
        )*
    ) => {
        $(
            pub const $name: u8 = $value;
        )*

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

opcodes! {
    // 10s: Comparison & Bitwise Logic Operations
    STOP,           0x00, todo_instr, 0, 0;
    ADD,            0x01, arithmetic::add, 2, 1;
    MUL,            0x02, arithmetic::mul, 2, 1;
    SUB,            0x03, arithmetic::sub, 2, 1;
    DIV,            0x04, arithmetic::div, 2, 1;
    SDIV,           0x05, todo_instr, 2, 1;
    MOD,            0x06, arithmetic::rem, 2, 1;
    SMOD,           0x07, todo_instr, 2, 1;
    ADDMOD,         0x08, arithmetic::addmod, 3, 1;
    MULMOD,         0x09, arithmetic::mulmod, 3, 1;
    EXP,            0x0a, arithmetic::exp, 2, 1;
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
    PUSH1,          0x60, stack::push::<1>, 0, 1;
    PUSH2,          0x61, stack::push::<2>, 0, 1;
    PUSH3,          0x62, stack::push::<3>, 0, 1;
    PUSH4,          0x63, stack::push::<4>, 0, 1;
    PUSH5,          0x64, stack::push::<5>, 0, 1;
    PUSH6,          0x65, stack::push::<6>, 0, 1;
    PUSH7,          0x66, stack::push::<7>, 0, 1;
    PUSH8,          0x67, stack::push::<8>, 0, 1;
    PUSH9,          0x68, stack::push::<9>, 0, 1;
    PUSH10,         0x69, stack::push::<10>, 0, 1;
    PUSH11,         0x6a, stack::push::<11>, 0, 1;
    PUSH12,         0x6b, stack::push::<12>, 0, 1;
    PUSH13,         0x6c, stack::push::<13>, 0, 1;
    PUSH14,         0x6d, stack::push::<14>, 0, 1;
    PUSH15,         0x6e, stack::push::<15>, 0, 1;
    PUSH16,         0x6f, stack::push::<16>, 0, 1;
    PUSH17,         0x70, stack::push::<17>, 0, 1;
    PUSH18,         0x71, stack::push::<18>, 0, 1;
    PUSH19,         0x72, stack::push::<19>, 0, 1;
    PUSH20,         0x73, stack::push::<20>, 0, 1;
    PUSH21,         0x74, stack::push::<21>, 0, 1;
    PUSH22,         0x75, stack::push::<22>, 0, 1;
    PUSH23,         0x76, stack::push::<23>, 0, 1;
    PUSH24,         0x77, stack::push::<24>, 0, 1;
    PUSH25,         0x78, stack::push::<25>, 0, 1;
    PUSH26,         0x79, stack::push::<26>, 0, 1;
    PUSH27,         0x7a, stack::push::<27>, 0, 1;
    PUSH28,         0x7b, stack::push::<28>, 0, 1;
    PUSH29,         0x7c, stack::push::<29>, 0, 1;
    PUSH30,         0x7d, stack::push::<30>, 0, 1;
    PUSH31,         0x7e, stack::push::<31>, 0, 1;
    PUSH32,         0x7f, stack::push::<32>, 0, 1;

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
        let add_op = super::PUSH0;
        assert_eq!(Opcode::new(add_op).unwrap().0, add_op);
    }
}
