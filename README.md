…

## Example

```rust,skt-instruction-decode
use chip_8::instructions::{Instruction, VRegister::VA};

let decoded_instruction = Instruction::decode(0xCAFE)?;

assert_eq!(decoded_instruction, Instruction::Random(VA, 0xFE));
```

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
chip_8 = "0.1.0"
```