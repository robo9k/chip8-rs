//! Crate error types

#[cfg(feature = "std")]
use thiserror::Error;

/// Error type for all errors in this crate
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Error))]
pub enum Chip8Error {
    /// Invalid register definition
    #[cfg_attr(feature = "std", error("invalid register {0:?}"))]
    InvalidRegister(u8),

    /// Unknown instruction
    #[cfg_attr(feature = "std", error("unknown instruction {0:?}"))]
    UnknownInstruction(u16),

    /// Known but unimplemented instruction
    #[cfg_attr(feature = "std", error("unimplemented instruction {0:?}"))]
    UnimplementedInstruction(crate::instructions::Instruction),

    /// Invalid key definition
    #[cfg_attr(feature = "std", error("invalid key {0:?}"))]
    InvalidKey(u8),

    /// Value is out of valid range
    #[cfg_attr(feature = "std", error("out of range {0:?}"))]
    OutOfRange(u16),
}

/// Result alias
pub type Result<T> = core::result::Result<T, Chip8Error>;
