//! Virtual machine for the CHIP-8 programming language

#![doc(html_root_url = "https://docs.rs/chip_8/0.2.0")]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod display;
mod font;
pub mod errors;
pub mod instructions;
pub mod keypad;
pub mod memory;
pub mod vm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
