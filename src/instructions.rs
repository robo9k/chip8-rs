//! Machine language and byte code instructions

/// Byte code instruction
pub enum Instruction {}

impl Instruction {
    /// Decodes raw `bits` into a valid `Instruction`
    pub fn decode(_bits: u16) -> Option<Instruction> {
        None
    }
}
