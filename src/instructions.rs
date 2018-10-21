//! Machine language and byte code instructions

/// Absolute memory address
///
/// Valid addresses are within `0x0` .. `0xFFF`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Addr(u16);

impl From<u16> for Addr {
    fn from(bits: u16) -> Addr {
        Addr(bits & 0x0FFF)
    }
}

/// Byte code instruction
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    /// Jumps to machine routine at `Addr`
    ///
    /// `0nnn` - `SYS addr`
    Sys(Addr),
    /// Clears the display
    ///
    /// `00E0` - `CLS`
    Clear,
    /// Returns from a subroutine
    ///
    /// `00EE` - `RET`
    Return,
}

impl Instruction {
    /// Decodes raw `bits` into a valid `Instruction`
    pub fn decode(bits: u16) -> Option<Instruction> {
        use self::Instruction::*;

        let nnn = bits & 0x0FFF;
        let high_nibble = (bits & 0xF000) >> 12;
        let k = bits & 0x00FF;

        match high_nibble {
            0x0 => match k {
                0xE0 => Some(Clear),
                0xEE => Some(Return),
                _ => Some(Sys(Addr(nnn))),
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

    #[test]
    fn decode_return() {
        assert_eq!(Instruction::decode(0x00EE), Some(Instruction::Return));
    }
    #[test]
    fn decode_sys() {
        assert_eq!(
            Instruction::decode(0x0123),
            Some(Instruction::Sys(Addr(0x0123)))
        );
    }
}
