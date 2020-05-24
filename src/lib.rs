//! Virtual machine for the CHIP-8 programming language

#![doc(html_root_url = "https://docs.rs/chip_8/0.2.0")]
#![warn(missing_docs)]

pub mod errors;
pub mod instructions;
pub mod keypad;
pub mod vm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
