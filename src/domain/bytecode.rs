use bytes::Bytes;
use hex::FromHexError;

use crate::interpreter::operation::Operation;

#[derive(Debug, PartialEq, Eq)]
pub enum Bytecode {
    Legacy(Bytes),
    // TODO: read up on Eof, uncertain how this actually looks like in practice.
    Eof(Bytes),
}

impl Bytecode {
    pub fn new_legacy(bytes: Bytes) -> Self {
        Self::Legacy(bytes)
    }

    pub fn parse(value: &str) -> Result<Self, FromHexError> {
        let hex = hex::decode(value.strip_prefix("0x").unwrap_or(value))?;
        Ok(Bytecode::Legacy(Bytes::from(hex)))
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::Bytecode;

    #[test]
    fn parse() {
        let expected = Bytes::from_static(&[0x01, 0x60, 0x01, 0x40]);
        assert_eq!(
            Bytecode::parse(&"0x01600140").unwrap(),
            Bytecode::new_legacy(expected.clone())
        );
        assert_eq!(
            Bytecode::parse(&"01600140").unwrap(),
            Bytecode::new_legacy(expected.clone())
        );
    }
}
