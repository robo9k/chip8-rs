//! Machine language and byte code instructions

/// Byte code instruction
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    /// Clears the display
    ///
    /// `00E0` - `CLS`
    Clear,
}

impl Instruction {
    /// Decodes raw `bits` into a valid `Instruction`
    pub fn decode(bits: u16) -> Option<Instruction> {
        use self::Instruction::*;

        let high_nibble = (bits & 0xF000) >> 12;
        let k = bits & 0x00FF;

        match high_nibble {
            0x0 => match k {
                0xE0 => Some(Clear),
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_clear() {
        assert_eq!(Instruction::decode(0x00E0), Some(Instruction::Clear));
    }
}
