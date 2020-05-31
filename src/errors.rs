//! Crate error types

use thiserror::Error;

/// Error type for all errors in this crate
#[derive(Error, Debug, Eq, PartialEq)]
pub enum Chip8Error {
    /// Invalid register definition
    #[error("invalid register {0:?}")]
    InvalidRegister(u8),

    /// Unknown instruction
    #[error("unknown instruction {0:?}")]
    UnknownInstruction(u16),

    /// Known but unimplemented instruction
    #[error("unimplemented instruction {0:?}")]
    UnimplementedInstruction(crate::instructions::Instruction),

    /// Invalid key definition
    #[error("invalid key {0:?}")]
    InvalidKey(u8),

    /// Value is out of valid range
    #[error("out of range {0:?}")]
    OutOfRange(u16),
}

/// Result alias
pub type Result<T> = std::result::Result<T, Chip8Error>;
