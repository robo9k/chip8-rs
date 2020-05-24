//! Keys and keypad

use bitflags::bitflags;

bitflags! {
    /// 16-key hexadecimal keypad
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
    #[derive(Default)]
    pub struct Keypad: u8 {
        /// Key `0`
        // see notes about zero flags in bitflags! docs
        const KEY_0 = 0x1;
        /// Key `1`
        const KEY_1 = 0x2;
        /// Key `2`
        const KEY_2 = 0x3;
        /// Key `3`
        const KEY_3 = 0x4;
        /// Key `4`
        const KEY_4 = 0x5;
        /// Key `5`
        const KEY_5 = 0x6;
        /// Key `6`
        const KEY_6 = 0x7;
        /// Key `7`
        const KEY_7 = 0x8;
        /// Key `8`
        const KEY_8 = 0x9;
        /// Key `9`
        const KEY_9 = 0xA;
        /// Key `A`
        const KEY_A = 0xB;
        /// Key `B`
        const KEY_B = 0xC;
        /// Key `C`
        const KEY_C = 0xD;
        /// Key `D`
        const KEY_D = 0xE;
        /// Key `E`
        const KEY_E = 0xF;
        /// Key `F`
        const KEY_F = 0x10;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypad_default() {
        let keypad = Keypad::default();

        assert!(keypad.is_empty());
    }
}
