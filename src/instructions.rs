//! Machine language and byte code instructions

/// General purpose register
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRegister {
    #[allow(missing_docs)]
    V0 = 0x0,
    #[allow(missing_docs)]
    V1 = 0x1,
    #[allow(missing_docs)]
    V2 = 0x2,
    #[allow(missing_docs)]
    V3 = 0x3,
    #[allow(missing_docs)]
    V4 = 0x4,
    #[allow(missing_docs)]
    V5 = 0x5,
    #[allow(missing_docs)]
    V6 = 0x6,
    #[allow(missing_docs)]
    V7 = 0x7,
    #[allow(missing_docs)]
    V8 = 0x8,
    #[allow(missing_docs)]
    V9 = 0x9,
    #[allow(missing_docs)]
    VA = 0xA,
    #[allow(missing_docs)]
    VB = 0xB,
    #[allow(missing_docs)]
    VC = 0xC,
    #[allow(missing_docs)]
    VD = 0xD,
    #[allow(missing_docs)]
    VE = 0xE,
    #[allow(missing_docs)]
    VF = 0xF,
}

impl VRegister {
    /// Matches `bits` to an `VRegister`
    pub fn from(bits: u8) -> Option<VRegister> {
        use self::VRegister::*;

        match bits {
            0x0 => Some(V0),
            0x1 => Some(V1),
            0x2 => Some(V2),
            0x3 => Some(V3),
            0x4 => Some(V4),
            0x5 => Some(V5),
            0x6 => Some(V6),
            0x7 => Some(V7),
            0x8 => Some(V8),
            0x9 => Some(V9),
            0xA => Some(VA),
            0xB => Some(VB),
            0xC => Some(VC),
            0xD => Some(VD),
            0xE => Some(VE),
            0xF => Some(VF),

            _ => None,
        }
    }
}

/// First register in an instruction
pub type Vx = VRegister;

/// Second register in an instruction
pub type Vy = VRegister;

/// A byte
pub type Byte = u8;

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
    /// Jumps to `Addr`
    ///
    /// `1nnn` - `JP addr`
    Jump(Addr),
    /// Calls subroutine at `Addr`
    ///
    /// `2nnn` - `CALL addr`
    Call(Addr),
    /// Skips next instruction if `Vx` equals `byte`
    ///
    /// `3xkk` - `SE Vx, byte`
    SkipEqualOperand(Vx, Byte),
    /// Skips next instruction if `Vx` is not equal to `byte`
    ///
    /// `4xkk` - `SNE Vx, byte`
    SkipNotEqualOperand(Vx, Byte),
    /// Skips next instruction if `Vy` is equal to `Vy`
    ///
    /// `5xy0` - `SE Vx, Vy`
    SkipEqual(Vx, Vy),
}

impl Instruction {
    /// Decodes raw `bits` into a valid `Instruction`
    pub fn decode(bits: u16) -> Option<Instruction> {
        use self::Instruction::*;

        // lowest 12 bits
        let nnn = (bits & 0x0FFF) as u16;
        // highest 4 bits of high byte
        let high_nibble = ((bits & 0xF000) >> 12) as u8;
        // lowest 4 bits of low byte
        let low_nibble = (bits & 0x000F) as u8;
        // lower 4 bits of high byte
        let x = ((bits & 0x0F00) >> 8) as u8;
        // higher 4 bits of lower byte
        let y = ((bits & 0x00F0) >> 4) as u8;
        // lower 8 bits
        let kk = (bits & 0x00FF) as u8;

        match high_nibble {
            0x0 => match kk {
                0xE0 => Some(Clear),
                0xEE => Some(Return),
                _ => Some(Sys(Addr(nnn))),
            },
            0x1 => Some(Jump(Addr(nnn))),
            0x2 => Some(Call(Addr(nnn))),
            0x3 => Some(SkipEqualOperand(VRegister::from(x).unwrap(), kk)),
            0x4 => Some(SkipNotEqualOperand(VRegister::from(x).unwrap(), kk)),
            0x5 => match low_nibble {
                0x0 => Some(SkipEqual(
                    VRegister::from(x).unwrap(),
                    VRegister::from(y).unwrap(),
                )),
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

    #[test]
    fn decode_jump() {
        assert_eq!(
            Instruction::decode(0x1234),
            Some(Instruction::Jump(Addr(0x0234)))
        );
    }

    #[test]
    fn decode_call() {
        assert_eq!(
            Instruction::decode(0x2345),
            Some(Instruction::Call(Addr(0x0345)))
        );
    }

    #[test]
    fn decode_skip_equal_operand() {
        assert_eq!(
            Instruction::decode(0x30FF),
            Some(Instruction::SkipEqualOperand(VRegister::V0, 0xFF))
        );
    }

    #[test]
    fn decode_skip_not_equal_operand() {
        assert_eq!(
            Instruction::decode(0x40FF),
            Some(Instruction::SkipNotEqualOperand(VRegister::V0, 0xFF))
        );
    }

    #[test]
    fn decode_skip_equal() {
        assert_eq!(
            Instruction::decode(0x50F0),
            Some(Instruction::SkipEqual(VRegister::V0, VRegister::VF))
        );
    }

}
