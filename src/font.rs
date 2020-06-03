//! ROM built-in font sprites

/// Number of rows for a font sprite
const FONT_SPRITE_ROWS: usize = 5;

/// Font sprite data for `0`
/// 
/// ```
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓░░▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ```
const SPRITE_0: [u8; FONT_SPRITE_ROWS] = [
    0b11110000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11110000,
];

/// Font sprite data for `1`
/// 
/// ```
/// ░░▓░░░░░
/// ░▓▓░░░░░
/// ░░▓░░░░░
/// ░░▓░░░░░
/// ░▓▓▓░░░░
/// /// ```
const SPRITE_1: [u8; FONT_SPRITE_ROWS] = [
    0b00100000,
    0b01100000,
    0b00100000,
    0b00100000,
    0b01110000,
];

/// Font sprite data for `2`
/// 
/// ```
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ```
const SPRITE_2: [u8; FONT_SPRITE_ROWS] = [
    0b11110000,
    0b00010000,
    0b11110000,
    0b10000000,
    0b11110000,
];

/// Font sprite data for `3`
/// 
/// ```
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ▓▓▓▓░░░░
/// ```
const SPRITE_3: [u8; FONT_SPRITE_ROWS] = [
    0b11110000,
    0b00010000,
    0b11110000,
    0b00010000,
    0b11110000,
];

/// Font sprite data for `4`
/// 
/// ```
/// ▓░░▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ░░░▓░░░░
/// ```
const SPRITE_4: [u8; FONT_SPRITE_ROWS] = [
    0b10010000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b00010000,
];

/// Font sprite data for `5`
/// 
/// ```
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ▓▓▓▓░░░░
/// ```
const SPRITE_5: [u8; FONT_SPRITE_ROWS] = [
    0b11110000,
    0b10000000,
    0b11110000,
    0b00010000,
    0b11110000,
];

/// Font sprite data for `6`
/// 
/// ```
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ```
const SPRITE_6: [u8; FONT_SPRITE_ROWS] = [
    0b11110000,
    0b10000000,
    0b11110000,
    0b10010000,
    0b11110000,
];

/// Font sprite data for `7`
/// 
/// ```
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ░░▓░░░░░
/// ░▓░░░░░░
/// ░▓░░░░░░
/// ```
const SPRITE_7: [u8; FONT_SPRITE_ROWS] = [
    0b11110000,
    0b00010000,
    0b00100000,
    0b01000000,
    0b01000000,
];

/// Font sprite data for `8`
/// 
/// ```
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ```
const SPRITE_8: [u8; FONT_SPRITE_ROWS] = [
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b11110000,
];

/// Font sprite data for `9`
/// 
/// ```
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ░░░▓░░░░
/// ▓▓▓▓░░░░
/// ```
const SPRITE_9: [u8; FONT_SPRITE_ROWS] = [
    0b11110000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b11110000,
];

/// Font sprite data for `A`
/// 
/// ```
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓▓░░░░
/// ▓░░▓░░░░
/// ▓░░▓░░░░
/// ```
const SPRITE_A: [u8; FONT_SPRITE_ROWS] = [
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b10010000,
];

/// Font sprite data for `B`
/// 
/// ```
/// ▓▓▓░░░░░
/// ▓░░▓░░░░
/// ▓▓▓░░░░░
/// ▓░░▓░░░░
/// ▓▓▓░░░░░
/// ```
const SPRITE_B: [u8; FONT_SPRITE_ROWS] = [
    0b11100000,
    0b10010000,
    0b11100000,
    0b10010000,
    0b11100000,
];

/// Font sprite data for `C`
/// 
/// ```
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓░░░░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ```
const SPRITE_C: [u8; FONT_SPRITE_ROWS] = [
    0b11110000,
    0b10000000,
    0b10000000,
    0b10000000,
    0b11110000,
];

/// Font sprite data for `D`
/// 
/// ```
/// ▓▓▓░░░░░
/// ▓░░▓░░░░
/// ▓░░▓░░░░
/// ▓░░▓░░░░
/// ▓▓▓░░░░░
/// ```
const SPRITE_D: [u8; FONT_SPRITE_ROWS] = [
    0b11100000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11100000,
];

/// Font sprite data for `E`
/// 
/// ```
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ```
const SPRITE_E: [u8; FONT_SPRITE_ROWS] = [
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b11110000,
];

/// Font sprite data for `F`
/// 
/// ```
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓▓▓▓░░░░
/// ▓░░░░░░░
/// ▓░░░░░░░
/// ```
const SPRITE_F: [u8; FONT_SPRITE_ROWS] = [
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b10000000,
];

const FONT: [[u8; FONT_SPRITE_ROWS]; 16] = [
    SPRITE_0,
    SPRITE_1,
    SPRITE_2,
    SPRITE_3,
    SPRITE_4,
    SPRITE_5,
    SPRITE_6,
    SPRITE_7,
    SPRITE_8,
    SPRITE_9,
    SPRITE_A,
    SPRITE_B,
    SPRITE_C,
    SPRITE_D,
    SPRITE_E,
    SPRITE_F,
];

fn font_as_bytes_iter() -> impl Iterator<Item=&'static u8> {
    SPRITE_0.iter().chain(SPRITE_1.iter()).chain(SPRITE_2.iter()).chain(SPRITE_3.iter()).chain(SPRITE_4.iter()).chain(SPRITE_5.iter()).chain(SPRITE_6.iter()).chain(SPRITE_7.iter()).chain(SPRITE_8.iter()).chain(SPRITE_9.iter()).chain(SPRITE_A.iter()).chain(SPRITE_B.iter()).chain(SPRITE_C.iter()).chain(SPRITE_D.iter()).chain(SPRITE_E.iter()).chain(SPRITE_F.iter())
}