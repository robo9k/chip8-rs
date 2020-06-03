//! ROM built-in font sprites

/// Number of rows for a font sprite
pub(crate) const FONT_SPRITE_ROWS: usize = 5;

/// Font sprite data for `0`
///
/// ```plain
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓░░▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ```
pub(crate) const SPRITE_DATA_0: [u8; FONT_SPRITE_ROWS] =
    [0b11110000, 0b10010000, 0b10010000, 0b10010000, 0b11110000];

/// Font sprite data for `1`
///
/// ```plain
/// ░░▓░░░░░
/// ░▓▓░░░░░
/// ░░▓░░░░░
/// ░░▓░░░░░
/// ░▓▓▓░░░░
/// /// ```
pub(crate) const SPRITE_DATA_1: [u8; FONT_SPRITE_ROWS] =
    [0b00100000, 0b01100000, 0b00100000, 0b00100000, 0b01110000];

/// Font sprite data for `2`
///
/// ```plain
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ```
pub(crate) const SPRITE_DATA_2: [u8; FONT_SPRITE_ROWS] =
    [0b11110000, 0b00010000, 0b11110000, 0b10000000, 0b11110000];

/// Font sprite data for `3`
///
/// ```plain
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ▓▓▓▓░░░░
/// ```
pub(crate) const SPRITE_DATA_3: [u8; FONT_SPRITE_ROWS] =
    [0b11110000, 0b00010000, 0b11110000, 0b00010000, 0b11110000];

/// Font sprite data for `4`
///
/// ```plain
/// ▓░░▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ░░░▓░░░░
/// ```
pub(crate) const SPRITE_DATA_4: [u8; FONT_SPRITE_ROWS] =
    [0b10010000, 0b10010000, 0b11110000, 0b00010000, 0b00010000];

/// Font sprite data for `5`
///
/// ```plain
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ▓▓▓▓░░░░
/// ```
pub(crate) const SPRITE_DATA_5: [u8; FONT_SPRITE_ROWS] =
    [0b11110000, 0b10000000, 0b11110000, 0b00010000, 0b11110000];

/// Font sprite data for `6`
///
/// ```plain
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ```
pub(crate) const SPRITE_DATA_6: [u8; FONT_SPRITE_ROWS] =
    [0b11110000, 0b10000000, 0b11110000, 0b10010000, 0b11110000];

/// Font sprite data for `7`
///
/// ```plain
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ░░▓░░░░░
/// ░▓░░░░░░
/// ░▓░░░░░░
/// ```
pub(crate) const SPRITE_DATA_7: [u8; FONT_SPRITE_ROWS] =
    [0b11110000, 0b00010000, 0b00100000, 0b01000000, 0b01000000];

/// Font sprite data for `8`
///
/// ```plain
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ```
pub(crate) const SPRITE_DATA_8: [u8; FONT_SPRITE_ROWS] =
    [0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b11110000];

/// Font sprite data for `9`
///
/// ```plain
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ▓▓▓▓░░░░
/// ```
pub(crate) const SPRITE_DATA_9: [u8; FONT_SPRITE_ROWS] =
    [0b11110000, 0b10010000, 0b11110000, 0b00010000, 0b11110000];

/// Font sprite data for `A`
///
/// ```plain
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓░░▓░░░░
/// ```
pub(crate) const SPRITE_DATA_A: [u8; FONT_SPRITE_ROWS] =
    [0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b10010000];

/// Font sprite data for `B`
///
/// ```plain
/// ▓▓▓░░░░░
/// ▓░░▓░░░░
/// ▓▓▓░░░░░
/// ▓░░▓░░░░
/// ▓▓▓░░░░░
/// ```
pub(crate) const SPRITE_DATA_B: [u8; FONT_SPRITE_ROWS] =
    [0b11100000, 0b10010000, 0b11100000, 0b10010000, 0b11100000];

/// Font sprite data for `C`
///
/// ```plain
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓░░░░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ```
pub(crate) const SPRITE_DATA_C: [u8; FONT_SPRITE_ROWS] =
    [0b11110000, 0b10000000, 0b10000000, 0b10000000, 0b11110000];

/// Font sprite data for `D`
///
/// ```plain
/// ▓▓▓░░░░░
/// ▓░░▓░░░░
/// ▓░░▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓░░░░░
/// ```
pub(crate) const SPRITE_DATA_D: [u8; FONT_SPRITE_ROWS] =
    [0b11100000, 0b10010000, 0b10010000, 0b10010000, 0b11100000];

/// Font sprite data for `E`
///
/// ```plain
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ```
pub(crate) const SPRITE_DATA_E: [u8; FONT_SPRITE_ROWS] =
    [0b11110000, 0b10000000, 0b11110000, 0b10000000, 0b11110000];

/// Font sprite data for `F`
///
/// ```plain
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓░░░░░░░
/// ```
pub(crate) const SPRITE_DATA_F: [u8; FONT_SPRITE_ROWS] =
    [0b11110000, 0b10000000, 0b11110000, 0b10000000, 0b10000000];

pub(crate) fn font_as_bytes_iter() -> impl Iterator<Item = &'static u8> {
    SPRITE_DATA_0
        .iter()
        .chain(SPRITE_DATA_1.iter())
        .chain(SPRITE_DATA_2.iter())
        .chain(SPRITE_DATA_3.iter())
        .chain(SPRITE_DATA_4.iter())
        .chain(SPRITE_DATA_5.iter())
        .chain(SPRITE_DATA_6.iter())
        .chain(SPRITE_DATA_7.iter())
        .chain(SPRITE_DATA_8.iter())
        .chain(SPRITE_DATA_9.iter())
        .chain(SPRITE_DATA_A.iter())
        .chain(SPRITE_DATA_B.iter())
        .chain(SPRITE_DATA_C.iter())
        .chain(SPRITE_DATA_D.iter())
        .chain(SPRITE_DATA_E.iter())
        .chain(SPRITE_DATA_F.iter())
}
