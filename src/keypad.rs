//! Keys and keypad

use crate::errors::Chip8Error;
use std::convert::TryFrom;
use std::ops::{Index, IndexMut};

/// Possible state for each key
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyState {
    /// Key not pressed
    NotPressed = 0,
    /// Key pressed
    Pressed = 1,
}

impl Default for KeyState {
    #[must_use]
    fn default() -> Self {
        Self::NotPressed
    }
}

/// Individual key on the [`Keypad`]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    /// Key `0`
    Key0 = 0x0,
    /// Key `1`
    Key1 = 0x1,
    /// Key `2`
    Key2 = 0x2,
    /// Key `3`
    Key3 = 0x3,
    /// Key `4`
    Key4 = 0x4,
    /// Key `5`
    Key5 = 0x5,
    /// Key `6`
    Key6 = 0x6,
    /// Key `7`
    Key7 = 0x7,
    /// Key `8`
    Key8 = 0x8,
    /// Key `9`
    Key9 = 0x9,
    /// Key `A`
    KeyA = 0xA,
    /// Key `B`
    KeyB = 0xB,
    /// Key `C`
    KeyC = 0xC,
    /// Key `D`
    KeyD = 0xD,
    /// Key `E`
    KeyE = 0xE,
    /// Key `F`
    KeyF = 0xF,
}

impl TryFrom<u8> for Key {
    type Error = Chip8Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Self::Key0),
            0x1 => Ok(Self::Key1),
            0x2 => Ok(Self::Key2),
            0x3 => Ok(Self::Key3),
            0x4 => Ok(Self::Key4),
            0x5 => Ok(Self::Key5),
            0x6 => Ok(Self::Key6),
            0x7 => Ok(Self::Key7),
            0x8 => Ok(Self::Key8),
            0x9 => Ok(Self::Key9),
            0xA => Ok(Self::KeyA),
            0xB => Ok(Self::KeyB),
            0xC => Ok(Self::KeyC),
            0xD => Ok(Self::KeyD),
            0xE => Ok(Self::KeyE),
            0xF => Ok(Self::KeyF),

            _ => Err(Chip8Error::InvalidKey(value)),
        }
    }
}

/// 16-key hexadecimal keypad
///
/// # Key layout
// table without thead requires html
/// <table>
///     <tr>
///         <td>1
///         <td>2
///         <td>3
///         <td>C
///     <tr>
///         <td>4
///         <td>5
///         <td>6
///         <td>D
///     <tr>
///         <td>7
///         <td>8
///         <td>9
///         <td>E
///     <tr>
///         <td>A
///         <td>0
///         <td>B
///         <td>F
/// </table>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Keypad {
    state: [KeyState; 16],
}

impl Keypad {
    /// Creates a new instance with default state for each key
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: [KeyState::default(); 16],
        }
    }
}

impl Index<Key> for Keypad {
    type Output = KeyState;

    #[must_use]
    fn index(&self, index: Key) -> &Self::Output {
        &self.state[index as usize]
    }
}

impl IndexMut<Key> for Keypad {
    #[must_use]
    fn index_mut(&mut self, index: Key) -> &mut Self::Output {
        &mut self.state[index as usize]
    }
}

impl Default for Keypad {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypad_default() {
        use super::{Key::*, KeyState::*};

        let keypad = Keypad::default();

        assert_eq!(keypad[Key0], NotPressed);
        assert_eq!(keypad[Key1], NotPressed);
        assert_eq!(keypad[Key2], NotPressed);
        assert_eq!(keypad[Key3], NotPressed);
        assert_eq!(keypad[Key4], NotPressed);
        assert_eq!(keypad[Key5], NotPressed);
        assert_eq!(keypad[Key6], NotPressed);
        assert_eq!(keypad[Key7], NotPressed);
        assert_eq!(keypad[Key8], NotPressed);
        assert_eq!(keypad[Key9], NotPressed);
        assert_eq!(keypad[KeyA], NotPressed);
        assert_eq!(keypad[KeyB], NotPressed);
        assert_eq!(keypad[KeyC], NotPressed);
        assert_eq!(keypad[KeyD], NotPressed);
        assert_eq!(keypad[KeyE], NotPressed);
        assert_eq!(keypad[KeyF], NotPressed);
    }
}
