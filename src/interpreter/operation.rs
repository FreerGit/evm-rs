#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Operation(u8);

#[macro_export]
macro_rules! operations {
    ( $( $name:ident, $value:expr; )* ) => {
        impl Operation {
            pub fn get(self) -> u8 {
                self.0
            }

            pub fn new(value: u8) -> Option<Self> {
                match value {
                    $(
                        $value => Some(Operation::$name),
                    )*
                    _ => None,
                }
            }

            pub fn name(&self) -> &'static str {
                match self.0 {
                    $(
                        $value => stringify!($name),
                    )*
                    _ => "Unknown Operation",
                }
            }

            $(
                pub const $name: Self = Self($value);
            )*
        }
    }
}

operations! {
    STOP, 0x00;
    ADD, 0x01;
    MUL, 0x02;
    SUB, 0x03;
    DIV, 0x04;
    SDIV, 0x05;
    MOD, 0x06;
    SMOD, 0x07;
    ADDMOD, 0x08;
    MULMOD, 0x09;
    EXP, 0x0A;
    SIGNEXTEND, 0x0B;
    // 0x0C - 0x0F
    LT, 0x10;
    GT, 0x11;
    SLT, 0x12;
    SGT, 0x13;
    EQ, 0x14;
    ISZERO, 0x15;
    AND, 0x16;
    OR, 0x17;
    XOR, 0x18;
    NOT, 0x19;
    BYTE, 0x1A;
    SHL, 0x1B;
    SHR, 0x1C;
    SAR, 0x1D;
    // 0x1E, 0x1F
    SHA3, 0x20;
    // 0x21 - 0x2F
    ADRESS, 0x30;
    BALANCE, 0x31;
    // TODO: SKIPPING A BUNCH FOR NOW
    POP, 0x50;
    // TODO: SKIPPING A BUNCH FOR NOW
    PUSH1, 0x60;
}

pub struct OperationContext {
    immediate_size: u8,
    input: u8,
    output: u8,
    is_terminating: bool,
    // set EVM revision
}

// impl OperationContext {
//     pub fn get_instruction(self) -> fn() -> () {
//         match self {
//             Operation::ADD => arithemtic::add(),
//             _ => todo!(),
//         }
//     }
// }

pub struct OperationInfo {
    inputs: u8,
    outputs: u8,
}

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
