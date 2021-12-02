//! Virtual machine for the CHIP-8 programming language

#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![doc(html_root_url = "https://docs.rs/chip_8/0.2.0")]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate alloc;

pub mod display;
pub mod errors;
mod font;
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
