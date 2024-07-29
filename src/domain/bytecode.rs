use bytes::Bytes;
use hex::FromHexError;

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

    pub fn bytes(&self) -> &Bytes {
        match self {
            Bytecode::Legacy(bytes) => bytes,
            Bytecode::Eof(_) => todo!(),
        }
    }

    pub fn bytes_slice(&self) -> &[u8] {
        match self {
            Bytecode::Legacy(bytes) => bytes,
            Bytecode::Eof(_) => todo!(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Bytecode::Legacy(bytes) => bytes.len(),
            Bytecode::Eof(_) => todo!(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Bytecode::Legacy(bytes) => bytes.is_empty(),
            Bytecode::Eof(_) => todo!(),
        }
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
            Bytecode::parse("0x01600140").unwrap(),
            Bytecode::new_legacy(expected.clone())
        );
        assert_eq!(
            Bytecode::parse("01600140").unwrap(),
            Bytecode::new_legacy(expected.clone())
        );
    }
}
