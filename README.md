[//]: # (README)

# chip8-rs

…

[![ci build status](https://github.com/robo9k/chip8-rs/workflows/test/badge.svg)](https://github.com/robo9k/chip8-rs/actions?query=workflow%3Atest)
[![crate version](https://img.shields.io/crates/v/chip_8)](https://crates.io/crates/chip_8)
[![docs](https://docs.rs/chip_8/badge.svg)](https://docs.rs/chip_8)
[![crate license](https://img.shields.io/crates/l/chip_8)](https://github.com/robo9k/chip8-rs/blob/master/COPYRIGHT)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

## Example

```rust,skt-instruction-decode
use chip_8::instructions::{Instruction, VRegister::VA};

let decoded_instruction = Instruction::decode(0xCAFE)?;

assert_eq!(decoded_instruction, Instruction::Random(VA, 0xFE));
```

## Usage

Add a dependency to your `Cargo.toml` (see [The Cargo Book](https://doc.rust-lang.org/cargo/guide/dependencies.html#adding-a-dependency)):

```toml
[dependencies]
chip_8 = "0.2.0"
```

# License

`chip8-rs` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

`SPDX-License-Identifier: MIT OR Apache-2.0`

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for details.
