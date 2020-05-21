//! Crate error types

use thiserror::Error;

/// Error type for all errors in this crate
#[derive(Error, Debug)]
pub enum Chip8Error {
    /// Invalid register definition
    #[error("invalid register {0:?}")]
    InvalidRegister(u8),

    /// Unknown instruction
    #[error("unknown instruction {0:?}")]
    UnknownInstruction(u16),
}

/// Result alias
pub type Result<T> = std::result::Result<T, Chip8Error>;
