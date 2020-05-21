//! Virtual machine for the CHIP-8 programming language

#![warn(missing_docs)]

pub mod instructions;
pub mod vm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
