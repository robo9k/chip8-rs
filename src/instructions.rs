//! Machine language and byte code instructions

use crate::errors::Chip8Error;
use std::convert::TryFrom;

/// General purpose register
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
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

impl TryFrom<u8> for VRegister {
    type Error = Chip8Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Self::V0),
            0x1 => Ok(Self::V1),
            0x2 => Ok(Self::V2),
            0x3 => Ok(Self::V3),
            0x4 => Ok(Self::V4),
            0x5 => Ok(Self::V5),
            0x6 => Ok(Self::V6),
            0x7 => Ok(Self::V7),
            0x8 => Ok(Self::V8),
            0x9 => Ok(Self::V9),
            0xA => Ok(Self::VA),
            0xB => Ok(Self::VB),
            0xC => Ok(Self::VC),
            0xD => Ok(Self::VD),
            0xE => Ok(Self::VE),
            0xF => Ok(Self::VF),

            _ => Err(Chip8Error::InvalidRegister(value)),
        }
    }
}

struct VRegisterRangeIter {
    next: Option<VRegister>,
    range: Option<VRegister>,
}

impl Iterator for VRegisterRangeIter {
    type Item = VRegister;

    fn next(&mut self) -> Option<Self::Item> {
        use VRegister::*;

        if self.range.is_none() {
            return None;
        }

        let next = self.next;
        self.next = match self.next {
            Some(V0) => Some(V1),
            Some(V1) => Some(V2),
            Some(V2) => Some(V3),
            Some(V3) => Some(V4),
            Some(V4) => Some(V5),
            Some(V5) => Some(V6),
            Some(V6) => Some(V7),
            Some(V7) => Some(V8),
            Some(V8) => Some(V9),
            Some(V9) => Some(VA),
            Some(VA) => Some(VB),
            Some(VB) => Some(VC),
            Some(VC) => Some(VD),
            Some(VD) => Some(VE),
            Some(VE) => Some(VF),
            Some(VF) => None,
            None => None,
        };
        if next == self.range {
            self.range = None;
        }

        next
    }
}

impl VRegister {
    /// Returns an `Iterator` from `V0` up to including `upper_bound`
    // NOTE: This could be implemented much nicer with `std::iter::Step`
    pub fn iter_to(upper_bound: VRegister) -> impl Iterator<Item = VRegister> {
        VRegisterRangeIter {
            next: Some(Self::V0),
            range: Some(upper_bound),
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Addr(u16);

impl Addr {
    /// Creates a new instance if `bits` is valid
    /// # Errors
    ///
    /// Will return [`Chip8Error::OutOfRange`](crate::errors::Chip8Error::OutOfRange)
    /// if given `bits` are out of range for an `Addr`.
    pub fn new(bits: u16) -> crate::errors::Result<Self> {
        if bits > 0x0FFF {
            Err(Chip8Error::OutOfRange(bits))
        } else {
            Ok(Self(bits & 0x0FFF))
        }
    }
}

impl From<u16> for Addr {
    fn from(bits: u16) -> Self {
        Self(bits & 0x0FFF)
    }
}

impl From<Addr> for u16 {
    fn from(addr: Addr) -> u16 {
        addr.0
    }
}

impl From<Addr> for usize {
    fn from(addr: Addr) -> usize {
        addr.0 as usize
    }
}

/// Hex digit
///
/// Valid values are within `0x0` .. `0xF`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Nibble(u8);

impl From<u8> for Nibble {
    fn from(bits: u8) -> Self {
        Self(bits & 0xF)
    }
}

impl From<Nibble> for usize {
    fn from(nibble: Nibble) -> Self {
        nibble.0 as usize
    }
}

/// Byte code instruction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    /// Sets `Vx` to random number AND `kk`
    ///
    /// `Cxkk` - `RND Vx, byte`
    Random(Vx, Byte),
    /// Read `n` bytes of memory from address `I`, draw it at `Vx` and `Vy` screen coordinates and set `VF` for erased pixels
    ///
    /// `Dxyn` - `DRW Vx, Vy, nibble`
    Draw(Vx, Vy, Nibble),
    /// Skip next instruction if key `Vx` is pressed
    ///
    /// `Ex9E` - `SKP Vx`
    SkipKeyPressed(Vx),
    /// Skip next instruction if key `Vx` is not pressed
    ///
    /// `ExA1` - `SKNP Vx`
    SkipKeyNotPressed(Vx),
    /// Set `Vx` to delay timer value
    ///
    /// `Fx07` - `LD Vx, DT`
    LoadRegisterDelayTimer(Vx),
    /// Wait for key press and store it in `Vx`
    ///
    /// `Fx0A` - `LD Vx, K`
    LoadKey(Vx),
    /// Set delay timer to `Vx`
    ///
    /// `Fx15` - `LD DT, Vx`
    LoadDelayTimerRegister(Vx),
    /// Set sound timer to `Vx`
    ///
    /// `Fx18` - `LD ST, Vx`
    LoadSoundTimerRegister(Vx),
    /// Add `Vx` to `I`
    ///
    /// `Fx1E` - `ADD I, Vx`
    AddI(Vx),
    /// Set `I` to the address of the sprite `Vx`
    ///
    /// `Fx29` - `LD F, Vx`
    LoadSprite(Vx),
    /// Store binary-coded decimal (BCD) at `I`, `I`+1 and `I`+2
    ///
    /// `Fx33` - `LD B, Vx`
    LoadBinaryCodedDecimal(Vx),
    /// Store registers `V0`..`Vx` in memory at `I`
    ///
    /// `Fx55` - `LD [I], Vx`
    LoadMemoryRegisters(Vx),
    /// Read registers `V0`..`Vx` from memory at `I`
    ///
    /// `Fx65` - `LD Vx, [I]`
    LoadRegistersMemory(Vx),
}

impl Instruction {
    /// Decodes raw `bits` into a valid `Instruction`
    ///
    /// # Errors
    ///
    /// Will return [`Chip8Error::UnknownInstruction`](crate::errors::Chip8Error::UnknownInstruction)
    /// if given `bits` don't match any known instruction.
    pub fn decode(bits: u16) -> crate::errors::Result<Self> {
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
            0x0 => match nnn {
                0x0E0 => Ok(Clear),
                0x0EE => Ok(Return),
                _ => Ok(Sys(nnn.into())),
            },
            0x1 => Ok(Jump(nnn.into())),
            0x2 => Ok(Call(nnn.into())),
            0x3 => Ok(SkipEqualOperand(VRegister::try_from(x)?, kk)),
            0x4 => Ok(SkipNotEqualOperand(VRegister::try_from(x)?, kk)),
            0x5 => match low_nibble {
                0x0 => Ok(SkipEqual(VRegister::try_from(x)?, VRegister::try_from(y)?)),
                _ => Err(Chip8Error::UnknownInstruction(bits)),
            },
            0x6 => Ok(LoadOperand(VRegister::try_from(x)?, kk)),
            0x7 => Ok(AddOperand(VRegister::try_from(x)?, kk)),
            0x8 => match low_nibble {
                0x0 => Ok(Load(VRegister::try_from(x)?, VRegister::try_from(y)?)),
                0x1 => Ok(Or(VRegister::try_from(x)?, VRegister::try_from(y)?)),
                0x2 => Ok(And(VRegister::try_from(x)?, VRegister::try_from(y)?)),
                0x3 => Ok(XOr(VRegister::try_from(x)?, VRegister::try_from(y)?)),
                0x4 => Ok(Add(VRegister::try_from(x)?, VRegister::try_from(y)?)),
                0x5 => Ok(Sub(VRegister::try_from(x)?, VRegister::try_from(y)?)),
                0x6 => Ok(ShiftRight(VRegister::try_from(x)?, VRegister::try_from(y)?)),
                0x7 => Ok(SubNegated(VRegister::try_from(x)?, VRegister::try_from(y)?)),
                0xE => Ok(ShiftLeft(VRegister::try_from(x)?, VRegister::try_from(y)?)),

                _ => Err(Chip8Error::UnknownInstruction(bits)),
            },
            0x9 => match low_nibble {
                0x0 => Ok(SkipNotEqual(
                    VRegister::try_from(x)?,
                    VRegister::try_from(y)?,
                )),
                _ => Err(Chip8Error::UnknownInstruction(bits)),
            },
            0xA => Ok(LoadI(nnn.into())),
            0xB => Ok(LongJump(nnn.into())),
            0xC => Ok(Random(VRegister::try_from(x)?, kk)),
            0xD => Ok(Draw(
                VRegister::try_from(x)?,
                VRegister::try_from(y)?,
                Nibble::from(low_nibble),
            )),
            0xE => match kk {
                0x9E => Ok(SkipKeyPressed(VRegister::try_from(x)?)),
                0xA1 => Ok(SkipKeyNotPressed(VRegister::try_from(x)?)),
                _ => Err(Chip8Error::UnknownInstruction(bits)),
            },
            0xF => match kk {
                0x07 => Ok(LoadRegisterDelayTimer(VRegister::try_from(x)?)),
                0x0A => Ok(LoadKey(VRegister::try_from(x)?)),
                0x15 => Ok(LoadDelayTimerRegister(VRegister::try_from(x)?)),
                0x18 => Ok(LoadSoundTimerRegister(VRegister::try_from(x)?)),
                0x1E => Ok(AddI(VRegister::try_from(x)?)),
                0x29 => Ok(LoadSprite(VRegister::try_from(x)?)),
                0x33 => Ok(LoadBinaryCodedDecimal(VRegister::try_from(x)?)),
                0x55 => Ok(LoadMemoryRegisters(VRegister::try_from(x)?)),
                0x65 => Ok(LoadRegistersMemory(VRegister::try_from(x)?)),
                _ => Err(Chip8Error::UnknownInstruction(bits)),
            },

            _ => Err(Chip8Error::UnknownInstruction(bits)),
        }
    }

    /// Encodes a valid `Instruction` into raw bits
    pub fn encode(&self) -> u16 {
        fn x(reg: VRegister) -> u16 {
            (reg as u16) << 8
        }

        fn y(reg: VRegister) -> u16 {
            (reg as u16) << 4
        }

        fn kk(byte: Byte) -> u16 {
            byte as u16
        }

        fn nnn(addr: Addr) -> u16 {
            addr.0 as u16
        }

        fn n(nibble: Nibble) -> u16 {
            nibble.0 as u16
        }

        match *self {
            Self::Sys(addr) => 0x0000 | nnn(addr),
            Self::Clear => 0x00E0,
            Self::Return => 0x00EE,
            Self::Jump(addr) => 0x1000 | nnn(addr),
            Self::Call(addr) => 0x2000 | nnn(addr),
            Self::SkipEqualOperand(vx, byte) => 0x3000 | x(vx) | kk(byte),
            Self::SkipNotEqualOperand(vx, byte) => 0x4000 | x(vx) | kk(byte),
            Self::SkipEqual(vx, vy) => 0x5000 | x(vx) | y(vy) | 0x0,
            Self::LoadOperand(vx, byte) => 0x6000 | x(vx) | kk(byte),
            Self::AddOperand(vx, byte) => 0x7000 | x(vx) | kk(byte),
            Self::Load(vx, vy) => 0x8000 | x(vx) | y(vy) | 0x0,
            Self::Or(vx, vy) => 0x8000 | x(vx) | y(vy) | 0x1,
            Self::And(vx, vy) => 0x8000 | x(vx) | y(vy) | 0x2,
            Self::XOr(vx, vy) => 0x8000 | x(vx) | y(vy) | 0x3,
            Self::Add(vx, vy) => 0x8000 | x(vx) | y(vy) | 0x4,
            Self::Sub(vx, vy) => 0x8000 | x(vx) | y(vy) | 0x5,
            Self::ShiftRight(vx, vy) => 0x8000 | x(vx) | y(vy) | 0x6,
            Self::SubNegated(vx, vy) => 0x8000 | x(vx) | y(vy) | 0x7,
            Self::ShiftLeft(vx, vy) => 0x8000 | x(vx) | y(vy) | 0xE,
            Self::SkipNotEqual(vx, vy) => 0x9000 | x(vx) | y(vy) | 0x0,
            Self::LoadI(addr) => 0xA000 | nnn(addr),
            Self::LongJump(addr) => 0xB000 | nnn(addr),
            Self::Random(vx, byte) => 0xC000 | x(vx) | kk(byte),
            Self::Draw(vx, vy, nibble) => 0xD000 | x(vx) | y(vy) | n(nibble),
            Self::SkipKeyPressed(vx) => 0xE000 | x(vx) | 0x9E,
            Self::SkipKeyNotPressed(vx) => 0xE000 | x(vx) | 0xA1,
            Self::LoadRegisterDelayTimer(vx) => 0xF000 | x(vx) | 0x07,
            Self::LoadKey(vx) => 0xF000 | x(vx) | 0x0A,
            Self::LoadDelayTimerRegister(vx) => 0xF000 | x(vx) | 0x15,
            Self::LoadSoundTimerRegister(vx) => 0xF000 | x(vx) | 0x18,
            Self::AddI(vx) => 0xF000 | x(vx) | 0x1E,
            Self::LoadSprite(vx) => 0xF000 | x(vx) | 0x29,
            Self::LoadBinaryCodedDecimal(vx) => 0xF000 | x(vx) | 0x33,
            Self::LoadMemoryRegisters(vx) => 0xF000 | x(vx) | 0x55,
            Self::LoadRegistersMemory(vx) => 0xF000 | x(vx) | 0x65,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn vregister_tryfrom_ok() {
        assert_eq!(VRegister::try_from(0x0), Ok(VRegister::V0));
    }

    #[test]
    fn vregister_tryfrom_err() {
        assert_eq!(
            VRegister::try_from(0x10),
            Err(Chip8Error::InvalidRegister(0x10))
        );
    }

    #[test]
    fn decode_clear() {
        assert_eq!(Instruction::decode(0x00E0), Ok(Instruction::Clear));
    }

    #[test]
    fn decode_return() {
        assert_eq!(Instruction::decode(0x00EE), Ok(Instruction::Return));
    }

    #[test]
    fn decode_sys() {
        assert_eq!(
            Instruction::decode(0x0123),
            Ok(Instruction::Sys(0x0123.into()))
        );
    }

    #[test]
    fn decode_jump() {
        assert_eq!(
            Instruction::decode(0x1234),
            Ok(Instruction::Jump(0x0234.into()))
        );
    }

    #[test]
    fn decode_call() {
        assert_eq!(
            Instruction::decode(0x2345),
            Ok(Instruction::Call(0x0345.into()))
        );
    }

    #[test]
    fn decode_skip_equal_operand() {
        assert_eq!(
            Instruction::decode(0x30FF),
            Ok(Instruction::SkipEqualOperand(VRegister::V0, 0xFF))
        );
    }

    #[test]
    fn decode_skip_not_equal_operand() {
        assert_eq!(
            Instruction::decode(0x40FF),
            Ok(Instruction::SkipNotEqualOperand(VRegister::V0, 0xFF))
        );
    }

    #[test]
    fn decode_skip_equal() {
        assert_eq!(
            Instruction::decode(0x50F0),
            Ok(Instruction::SkipEqual(VRegister::V0, VRegister::VF))
        );
    }

    #[test]
    fn decode_load_operand() {
        assert_eq!(
            Instruction::decode(0x60FF),
            Ok(Instruction::LoadOperand(VRegister::V0, 0xFF))
        );
    }

    #[test]
    fn decode_add_operand() {
        assert_eq!(
            Instruction::decode(0x70FF),
            Ok(Instruction::AddOperand(VRegister::V0, 0xFF))
        );
    }

    #[test]
    fn decode_load() {
        assert_eq!(
            Instruction::decode(0x8120),
            Ok(Instruction::Load(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_or() {
        assert_eq!(
            Instruction::decode(0x8121),
            Ok(Instruction::Or(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_and() {
        assert_eq!(
            Instruction::decode(0x8122),
            Ok(Instruction::And(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_xor() {
        assert_eq!(
            Instruction::decode(0x8123),
            Ok(Instruction::XOr(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_add() {
        assert_eq!(
            Instruction::decode(0x8124),
            Ok(Instruction::Add(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_sub() {
        assert_eq!(
            Instruction::decode(0x8125),
            Ok(Instruction::Sub(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_shift_right() {
        assert_eq!(
            Instruction::decode(0x8126),
            Ok(Instruction::ShiftRight(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_sub_negated() {
        assert_eq!(
            Instruction::decode(0x8127),
            Ok(Instruction::SubNegated(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_shift_left() {
        assert_eq!(
            Instruction::decode(0x812E),
            Ok(Instruction::ShiftLeft(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_skip_not_equal() {
        assert_eq!(
            Instruction::decode(0x9120),
            Ok(Instruction::SkipNotEqual(VRegister::V1, VRegister::V2))
        );
    }

    #[test]
    fn decode_load_i() {
        assert_eq!(
            Instruction::decode(0xA123),
            Ok(Instruction::LoadI(0x123.into()))
        );
    }

    #[test]
    fn decode_long_jump() {
        assert_eq!(
            Instruction::decode(0xB123),
            Ok(Instruction::LongJump(0x123.into()))
        );
    }

    #[test]
    fn decode_random() {
        assert_eq!(
            Instruction::decode(0xC0FF),
            Ok(Instruction::Random(VRegister::V0, 0xFF))
        );
    }

    #[test]
    fn decode_draw() {
        assert_eq!(
            Instruction::decode(0xD0FA),
            Ok(Instruction::Draw(VRegister::V0, VRegister::VF, 0xA.into()))
        );
    }

    #[test]
    fn decode_skipkeypressed() {
        assert_eq!(
            Instruction::decode(0xE19E),
            Ok(Instruction::SkipKeyPressed(VRegister::V1))
        );
    }

    #[test]
    fn decode_skipkeynotpressed() {
        assert_eq!(
            Instruction::decode(0xE1A1),
            Ok(Instruction::SkipKeyNotPressed(VRegister::V1))
        );
    }
    #[test]
    fn decode_loadregisterdelaytimer() {
        assert_eq!(
            Instruction::decode(0xF207),
            Ok(Instruction::LoadRegisterDelayTimer(VRegister::V2))
        );
    }

    #[test]
    fn decode_loadkey() {
        assert_eq!(
            Instruction::decode(0xF30A),
            Ok(Instruction::LoadKey(VRegister::V3))
        );
    }

    #[test]
    fn decode_loaddelaytimerregister() {
        assert_eq!(
            Instruction::decode(0xF415),
            Ok(Instruction::LoadDelayTimerRegister(VRegister::V4))
        );
    }

    #[test]
    fn decode_loadsounddimerregister() {
        assert_eq!(
            Instruction::decode(0xF518),
            Ok(Instruction::LoadSoundTimerRegister(VRegister::V5))
        );
    }

    #[test]
    fn decode_addi() {
        assert_eq!(
            Instruction::decode(0xF61E),
            Ok(Instruction::AddI(VRegister::V6))
        );
    }

    #[test]
    fn decode_loadsprite() {
        assert_eq!(
            Instruction::decode(0xF729),
            Ok(Instruction::LoadSprite(VRegister::V7))
        );
    }

    #[test]
    fn decode_loadbinarycodeddecimal() {
        assert_eq!(
            Instruction::decode(0xF833),
            Ok(Instruction::LoadBinaryCodedDecimal(VRegister::V8))
        );
    }

    #[test]
    fn decode_loadmemoryregisters() {
        assert_eq!(
            Instruction::decode(0xF955),
            Ok(Instruction::LoadMemoryRegisters(VRegister::V9))
        );
    }

    #[test]
    fn decode_loadregistersmemory() {
        assert_eq!(
            Instruction::decode(0xFA65),
            Ok(Instruction::LoadRegistersMemory(VRegister::VA))
        );
    }

    #[test]
    fn vregister_iter_to_v0() {
        use super::VRegister::*;
        assert!(VRegister::iter_to(V0).eq(vec![V0].iter().map(|r| *r)));
    }

    #[test]
    fn vregister_iter_to_va() {
        use super::VRegister::*;
        assert!(
            VRegister::iter_to(VA).eq(vec![V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, VA]
                .iter()
                .map(|r| *r))
        );
    }

    #[test]
    fn vregister_iter_to_vf() {
        use super::VRegister::*;
        assert!(VRegister::iter_to(VF).eq(vec![
            V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, VA, VB, VC, VD, VE, VF
        ]
        .iter()
        .map(|r| *r)));
    }

    proptest! {
        #[test]
        fn instruction_decode_encode(bits in u16::MIN..=u16::MAX) {
            let decoded = Instruction::decode(bits);
            if let Ok(decoded) = decoded  {
                println!("{:0>#06X} decoded: {:?}", bits, decoded);
                let encoded = decoded.encode();
                prop_assert_eq!(bits, encoded, "bits = {:0>#06X}, encoded = {:0>#06X}", bits, encoded);
            }
        }
    }
}
