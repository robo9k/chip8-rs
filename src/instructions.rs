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
    /// Loads `byte` into `Vx`
    ///
    /// `6xkk` - `LD Vx, byte`
    LoadOperand(Vx, Byte),
    /// Adds `byte` to `Vx`, then stores it in `Vx`
    ///
    /// `7xkk` - `ADD Vx, byte`
    AddOperand(Vx, Byte),
    /// Loads `Vy` into `Vx`
    ///
    /// `8xy0` - `LD Vx, Vy`
    Load(Vx, Vy),
    /// Sets `Vx` to `Vx OR Vy`
    ///
    /// `8xy1` - `OR Vx, Vy`
    Or(Vx, Vy),
    /// Sets `Vx` to `Vx AND Vy`
    ///
    /// `8xy2` - `AND Vx, Vy`
    And(Vx, Vy),
    /// Sets `Vx` to `Vx XOR Vy`
    ///
    /// `8xy3` - `XOR Vx, Vy`
    XOr(Vx, Vy),
    /// Sets `Vx` to `Vx + Vy`, `VF` to carry
    ///
    /// `8xy4` - `ADD Vx, Vy`
    Add(Vx, Vy),
    /// Sets `Vx` to `Vx - Vy`, `VF` to not borrow
    ///
    /// `8xy5` - `SUB Vx, Vy`
    Sub(Vx, Vy),
    /// Sets `Vx` to `Vy SHR 1`
    ///
    /// `8xy6` - `SHR Vx {, Vy}`
    ShiftRight(Vx, Vy),
    /// Sets `Vx` to `Vy - Vx`, `VF` to not borrow
    ///
    /// `8xy7` - `SUBN Vx, Vy`
    SubNegated(Vx, Vy),
    /// Sets `Vx` to `Vy SHL 1`
    ///
    /// `8xyE` - `SHL Vx {, Vy}`
    ShiftLeft(Vx, Vy),
    /// Skips next instruction if `Vx` is not equal to `Vy`
    ///
    /// `9xy0` - `SNE Vx, Vy`
    SkipNotEqual(Vx, Vy),
    /// Loads `Addr` into register `I`
    ///
    /// `Annn` - `LD I, addr`
    LoadI(Addr),
    /// Jumps to `Addr + V0`
    ///
    /// `Bnnn` - `JP V0, addr`
    LongJump(Addr),
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
            0x6 => Some(LoadOperand(VRegister::from(x).unwrap(), kk)),
            0x7 => Some(AddOperand(VRegister::from(x).unwrap(), kk)),
            0x8 => match low_nibble {
                0x0 => Some(Load(
                    VRegister::from(x).unwrap(),
                    VRegister::from(y).unwrap(),
                )),
                0x1 => Some(Or(VRegister::from(x).unwrap(), VRegister::from(y).unwrap())),
                0x2 => Some(And(
                    VRegister::from(x).unwrap(),
                    VRegister::from(y).unwrap(),
                )),
                0x3 => Some(XOr(
                    VRegister::from(x).unwrap(),
                    VRegister::from(y).unwrap(),
                )),
                0x4 => Some(Add(
                    VRegister::from(x).unwrap(),
                    VRegister::from(y).unwrap(),
                )),
                0x5 => Some(Sub(
                    VRegister::from(x).unwrap(),
                    VRegister::from(y).unwrap(),
                )),
                0x6 => Some(ShiftRight(
                    VRegister::from(x).unwrap(),
                    VRegister::from(y).unwrap(),
                )),
                0x7 => Some(SubNegated(
                    VRegister::from(x).unwrap(),
                    VRegister::from(y).unwrap(),
                )),
                0xE => Some(ShiftLeft(
                    VRegister::from(x).unwrap(),
                    VRegister::from(y).unwrap(),
                )),

                _ => None,
            },
            0x9 => match low_nibble {
                0x0 => Some(SkipNotEqual(
                    VRegister::from(x).unwrap(),
                    VRegister::from(y).unwrap(),
                )),
                _ => None,
            },
            0xA => Some(LoadI(Addr(nnn))),
            0xB => Some(LongJump(Addr(nnn))),

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

    #[test]
    fn decode_load_operand() {
        assert_eq!(
            Instruction::decode(0x60FF),
            Some(Instruction::LoadOperand(VRegister::V0, 0xFF))
        );
    }

    #[test]
    fn decode_add_operand() {
        assert_eq!(
            Instruction::decode(0x70FF),
            Some(Instruction::AddOperand(VRegister::V0, 0xFF))
        );
    }

    #[test]
    fn decode_load() {
        assert_eq!(
            Instruction::decode(0x8120),
            Some(Instruction::Load(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_or() {
        assert_eq!(
            Instruction::decode(0x8121),
            Some(Instruction::Or(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_and() {
        assert_eq!(
            Instruction::decode(0x8122),
            Some(Instruction::And(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_xor() {
        assert_eq!(
            Instruction::decode(0x8123),
            Some(Instruction::XOr(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_add() {
        assert_eq!(
            Instruction::decode(0x8124),
            Some(Instruction::Add(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_sub() {
        assert_eq!(
            Instruction::decode(0x8125),
            Some(Instruction::Sub(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_shift_right() {
        assert_eq!(
            Instruction::decode(0x8126),
            Some(Instruction::ShiftRight(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_sub_negated() {
        assert_eq!(
            Instruction::decode(0x8127),
            Some(Instruction::SubNegated(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_shift_left() {
        assert_eq!(
            Instruction::decode(0x812E),
            Some(Instruction::ShiftLeft(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_skip_not_equal() {
        assert_eq!(
            Instruction::decode(0x9120),
            Some(Instruction::SkipNotEqual(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_load_i() {
        assert_eq!(
            Instruction::decode(0xA123),
            Some(Instruction::LoadI(Addr(0x123)))
        );
    }

    #[test]
    fn decode_long_jump() {
        assert_eq!(
            Instruction::decode(0xB123),
            Some(Instruction::LongJump(Addr(0x123)))
        );
    }
}
