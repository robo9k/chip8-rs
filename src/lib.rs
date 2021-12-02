//! Virtual machine for the CHIP-8 programming language
//!
//! # Features
//! This crate uses [Cargo "features"](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section) for conditional compilation.
//! - `std`: Enables usage of [Rust's standard library `std`](https://doc.rust-lang.org/std/)
//!
//! Functionality affected by features should have a `rustdoc` hint in this documentation, e.g.:
//! > This is supported on **crate feature `std`** only.
//!
//! If this is not possible for technical reasons there should be a "Features" heading describing the details instead.
//!
//! ## Feature `std`
//! This crate is [`no_std`](https://github.com/rust-lang/rfcs/blob/master/text/1184-stabilize-no_std.md) compatible if you disable this feature.
//!
//! This is a [default feature](https://doc.rust-lang.org/cargo/reference/features.html#the-default-feature) and can be disabled with `default-features = false` in your `chip_8` [dependency declaration](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features).
//!
//! Even if disabled this crate still requires the [Rust core allocation and collections library `alloc`](https://doc.rust-lang.org/alloc/), i.e. a global allocator.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![doc(html_root_url = "https://docs.rs/chip_8/0.3.0")]
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
