#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Operation(u8);

#[macro_export]
macro_rules! operations {
    ( $( $name:ident, $value:expr; )* ) => {
        impl Operation {
            pub fn to_instruction(self) -> u8 {
                self.0
            }

            pub fn from_instruction(value: u8) -> Option<Self> {
                match value {
                    $(
                        $value => Some(Operation::$name),
                    )*
                    _ => None,
                }
            }

            $(
                pub const $name: Self = Self($value);
            )*
        }
    }
}

operations! {
    ADD, 0x01;
    MUL, 0x02;
    SUB, 0x03;
    DIV, 0x04;
}

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
        assert_eq!(Operation::from_instruction(add).unwrap(), add_op);
        assert_eq!(Operation::to_instruction(add_op), add);
    }
}
