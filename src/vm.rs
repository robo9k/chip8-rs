//! Virtual machine

use crate::display::{Display, DrawResult, XCoordinate, YCoordinate};
use crate::instructions::{Addr, Instruction, VRegister};
use crate::keypad::{Key, KeyState};
use crate::memory::Memory;
use rand::Rng;
use core::convert::TryFrom;
use core::ops::{Index, IndexMut};
use alloc::vec::Vec;

/// Type of a general purpose register in the VM
type VRegisterValue = u8;

/// Type of the address register `I`
type IRegisterValue = u16;

/// Type of the program counter register
type PCRegisterValue = u16;

/// Memory address for programm (ROM) start
const PROGRAM_START: PCRegisterValue = 0x200;

/// CPU registers
///
/// General purpose `V0`..`VF` and `I`
#[derive(Debug)]
struct Registers {
    vregisters: [VRegisterValue; 16], // TODO: Use variant_count for constant?
    i: IRegisterValue,
    pc: PCRegisterValue,
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    /// Creates a new instance with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            vregisters: [0; 16],
            i: 0,
            pc: PROGRAM_START,
        }
    }
}

impl Index<VRegister> for Registers {
    type Output = VRegisterValue;

    fn index(&self, index: VRegister) -> &Self::Output {
        &self.vregisters[index as usize]
    }
}

impl IndexMut<VRegister> for Registers {
    fn index_mut(&mut self, index: VRegister) -> &mut Self::Output {
        &mut self.vregisters[index as usize]
    }
}

const FONT_RAW_ADDR: u16 = 0x0;

/// Virtual machine
//#[derive(Debug)]
pub struct VM<R: Rng> {
    registers: Registers,
    rng: R,
    sys_fn: fn(&mut Self, crate::instructions::Addr) -> crate::errors::Result<()>,
    keypad: crate::keypad::Keypad,
    waiting_on_any_keypress: Option<VRegister>,
    memory: Memory,
    display: Display,
}

#[cfg(feature = "std")]
impl Default for VM<rand::rngs::ThreadRng> {
    /// Creates a new instance with thread-local random number generator
    #[must_use]
    fn default() -> Self {
        Self::new(rand::thread_rng(), |_, addr| {
            Err(crate::errors::Chip8Error::UnimplementedInstruction(
                Instruction::Sys(addr),
            ))
        })
    }
}

impl<R> VM<R>
where
    R: rand::Rng,
{
    /// Creates a new instance with the given RNG
    #[must_use]
    pub fn new(
        rng: R,
        sys_fn: fn(&mut Self, crate::instructions::Addr) -> crate::errors::Result<()>,
    ) -> Self {
        let mut vm = Self {
            registers: Registers::new(),
            rng,
            sys_fn,
            keypad: crate::keypad::Keypad::default(),
            waiting_on_any_keypress: None,
            memory: Memory::default(),
            display: Display::default(),
        };

        for (offs, font_byte) in crate::font::font_as_bytes_iter().enumerate() {
            let addr = Addr::new(FONT_RAW_ADDR + offs as u16).expect("built-in font fits into RAM");
            vm.memory.write(addr, *font_byte);
        }

        vm
    }

    #[must_use]
    fn execute_instruction(&mut self, instruction: &Instruction) -> crate::errors::Result<()> {
        match *instruction {
            Instruction::Sys(addr) => return (self.sys_fn)(self, addr),
            Instruction::Clear => self.display.clear(),
            // Return
            Instruction::Jump(addr) => self.registers.pc = addr.into(),
            // Call(Addr)
            Instruction::SkipEqualOperand(vx, byte) => {
                if self.registers[vx] == byte {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipNotEqualOperand(vx, byte) => {
                if self.registers[vx] != byte {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipEqual(vx, vy) => {
                if self.registers[vx] == self.registers[vy] {
                    self.registers.pc += 2
                }
            }
            Instruction::LoadOperand(vx, byte) => self.registers[vx] = byte,
            Instruction::AddOperand(vx, byte) => {
                self.registers[vx] = self.registers[vx].wrapping_add(byte)
            }
            Instruction::Load(vx, vy) => self.registers[vx] = self.registers[vy],
            Instruction::Or(vx, vy) => self.registers[vx] |= self.registers[vy],
            Instruction::And(vx, vy) => self.registers[vx] &= self.registers[vy],
            Instruction::XOr(vx, vy) => self.registers[vx] ^= self.registers[vy],
            Instruction::Add(vx, vy) => {
                let x = self.registers[vx] as u16;
                let y = self.registers[vy] as u16;

                let res = x + y;

                // VF is carryover
                self.registers[VRegister::VF] =
                    (res > VRegisterValue::MAX as u16) as VRegisterValue;

                self.registers[vx] = res as VRegisterValue;
            }
            Instruction::Sub(vx, vy) => {
                let x = self.registers[vx];
                let y = self.registers[vy];

                // VF is Not Borrow i.e. x > y
                self.registers[VRegister::VF] = (x > y) as VRegisterValue;

                self.registers[vx] = x.wrapping_sub(y);
            }
            Instruction::ShiftRight(vx, vy) => {
                let y = self.registers[vy];

                // VF is LSB before shift
                self.registers[VRegister::VF] = y & 0x1;

                self.registers[vx] = y >> 1;
            }
            Instruction::SubNegated(vx, vy) => {
                let x = self.registers[vx];
                let y = self.registers[vy];

                // VF is not borrow i.e. y > x
                self.registers[VRegister::VF] = (y > x) as VRegisterValue;

                self.registers[vx] = y.wrapping_sub(x);
            }
            Instruction::ShiftLeft(vx, vy) => {
                let y = self.registers[vy];

                // VF is MSB before shift
                self.registers[VRegister::VF] = y >> 7;

                self.registers[vx] = y << 1;
            }
            Instruction::SkipNotEqual(vx, vy) => {
                if self.registers[vx] != self.registers[vy] {
                    self.registers.pc += 2
                }
            }
            Instruction::LoadI(addr) => self.registers.i = addr.into(),
            Instruction::LongJump(addr) => {
                let addr: PCRegisterValue = addr.into();
                self.registers.pc = self.registers[VRegister::V0] as PCRegisterValue + addr;
            }
            Instruction::Random(vx, byte) => {
                self.registers[vx] = self.rng.gen::<VRegisterValue>() & byte
            }
            Instruction::Draw(vx, vy, nibble) => {
                let x = XCoordinate::new(vx as usize);
                let y = YCoordinate::new(vy as usize);

                let mut sprite_data = Vec::with_capacity(nibble.into());
                for offs in 0..nibble.into() {
                    let addr = Addr::new(self.registers.i + offs as u16)?;
                    sprite_data.push(self.memory.read(addr));
                }
                let sprite = &sprite_data[..].into();
                let draw_result = self.display.draw(sprite, x, y);
                self.registers[VRegister::VF] = match draw_result {
                    DrawResult::Drawn => 0,
                    DrawResult::Overdrawn => 1,
                };
            }
            Instruction::SkipKeyPressed(vx) => {
                let key_idx = self.registers[vx];
                let key = Key::try_from(key_idx)?;
                if self.keypad[key] == KeyState::Pressed {
                    self.registers.pc += 2;
                }
            }
            Instruction::SkipKeyNotPressed(vx) => {
                let key_idx = self.registers[vx];
                let key = Key::try_from(key_idx)?;
                if self.keypad[key] == KeyState::NotPressed {
                    self.registers.pc += 2;
                }
            }
            // LoadRegisterDelayTimer(Vx)
            Instruction::LoadKey(vx) => self.waiting_on_any_keypress = Some(vx),
            // LoadDelayTimerRegister(Vx)
            // LoadSoundTimerRegister(Vx)
            Instruction::AddI(vx) => self.registers.i += self.registers[vx] as IRegisterValue,
            Instruction::LoadSprite(vx) => {
                let x = self.registers[vx] as u16;
                self.registers.i =
                    Addr::new(FONT_RAW_ADDR + x * crate::font::FONT_SPRITE_ROWS as u16)?.into();
            }
            Instruction::LoadBinaryCodedDecimal(vx) => {
                let mut num = self.registers[vx];

                for (i, place) in vec![100, 10, 1].iter().enumerate() {
                    let bcd = num / place;
                    self.memory
                        .write(Addr::new(self.registers.i + i as u16)?, bcd);
                    num -= bcd * place;
                }
            }
            Instruction::LoadMemoryRegisters(vx) => {
                for (offs, reg) in VRegister::iter_to(vx).enumerate() {
                    let addr = Addr::new(self.registers.i + offs as u16)?;
                    self.memory.write(addr, self.registers[reg]);
                }

                self.registers.i = Addr::new(self.registers.i + vx as u16 + 1)?.into();
            }
            Instruction::LoadRegistersMemory(vx) => {
                for (offs, reg) in VRegister::iter_to(vx).enumerate() {
                    let addr = Addr::new(self.registers.i + offs as u16)?;
                    self.registers[reg] = self.memory.read(addr);
                }

                self.registers.i = Addr::new(self.registers.i + vx as u16 + 1)?.into();
            }
            other => return Err(crate::errors::Chip8Error::UnimplementedInstruction(other)),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::{Instruction::*, VRegister::*};
    use crate::keypad::{Key::*, KeyState::*};

    fn test_vm_default() -> VM<rand::rngs::mock::StepRng> {
        let rng = rand::rngs::mock::StepRng::new(4, 0);
        VM::new(rng, |_, _| Ok(()))
    }

    #[test]
    fn vregisters_set_get() {
        let mut registers = Registers::new();
        println!("Created registers: {:?}", registers);

        registers[V0] = 42;

        assert_eq!(registers[V0], 42, "register value get equals set");
    }

    macro_rules! registers_test {
        (
            $test_name:ident {
                instruction: $instruction:expr,
                registers_before: {$($register_before:expr => $register_before_value:expr),+},
                registers_after: {$($register_after:expr => $register_after_value:expr),+},
                register_overflow: $register_overflow_value:expr,
            }
        ) => {
            #[test]
            fn $test_name() -> crate::errors::Result<()> {
                let mut vm = test_vm_default();
                $(
                  vm.registers[$register_before] = $register_before_value;
                )+
                //println!("Created VM: {:?}", vm);

                vm.execute_instruction(&$instruction)?;
                println!("Executed instruction: {:?}", $instruction);

                $(
                    assert_eq!(
                        vm.registers[$register_after],
                        $register_after_value,
                        "register value {:?} expected {}, actual {}",
                        $register_after,
                        $register_after_value,
                        vm.registers[$register_after],
                    );
                )+
                assert_eq!(
                    vm.registers[$crate::instructions::VRegister::VF],
                    $register_overflow_value,
                    "overflow expected {}, actual {}",
                    $register_overflow_value,
                    vm.registers[$crate::instructions::VRegister::VF],
                );

                Ok(())
            }
        };
    }

    #[cfg(feature = "std")]
    #[test]
    fn vm_execute_instruction_sys_default() -> crate::errors::Result<()> {
        let mut vm = VM::default();

        let res = vm.execute_instruction(&Instruction::Sys(0x0FFF.into()));

        assert_eq!(
            res,
            Err(crate::errors::Chip8Error::UnimplementedInstruction(
                Instruction::Sys(0x0FFF.into())
            ))
        );
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_sys_sysfn() -> crate::errors::Result<()> {
        let mut vm = VM::new(rand::thread_rng(), |_, addr| {
            assert_eq!(addr, 0x0FFF.into());
            Ok(())
        });

        vm.execute_instruction(&Instruction::Sys(0x0FFF.into()))?;

        // There's nothing useful to assert in the current implementation
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_jump() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers.pc = 0x0;

        vm.execute_instruction(&Instruction::Jump(0x0FFF.into()))?;

        assert_eq!(vm.registers.pc, 0x0FFF);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_skipequaloperand() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers.pc = 0x0;
        vm.registers[V0] = 0xFF;

        vm.execute_instruction(&Instruction::SkipEqualOperand(V0, 0xFF))?;

        assert_eq!(vm.registers.pc, 0x0002);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_skipnotequaloperand() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers.pc = 0x0;
        vm.registers[V0] = 0xFF;

        vm.execute_instruction(&Instruction::SkipNotEqualOperand(V0, 0xEE))?;

        assert_eq!(vm.registers.pc, 0x0002);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_skipequal() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers.pc = 0x0;
        vm.registers[V0] = 0xFF;
        vm.registers[VF] = 0xFF;

        vm.execute_instruction(&Instruction::SkipEqual(V0, VF))?;

        assert_eq!(vm.registers.pc, 0x0002);
        Ok(())
    }

    registers_test!(
        vm_execute_instruction_load_operand {
            instruction: LoadOperand(V2, 0xFF),
            registers_before: {V2 => 0xEE},
            registers_after: {V2 => 0xFF},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_add_operand {
            instruction: AddOperand(V2, 0xFF),
            registers_before: {V2 => 0x00},
            registers_after: {V2 => 0xFF},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_add_operand_wrapping {
            instruction: AddOperand(V2, 0x01),
            registers_before: {V2 => 0xFF},
            registers_after: {V2 => 0x00},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_load {
            instruction: Load(V2, V3),
            registers_before: {V2 => 0x00, V3 => 0xFF},
            registers_after: {V2 => 0xFF, V3 => 0xFF},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_or {
            instruction: Or(V2, V3),
            registers_before: {V2 => 0x01, V3 => 0x10},
            registers_after: {V2 => 0x11},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_and {
            instruction: And(V2, V3),
            registers_before: {V2 => 0x01, V3 => 0x11},
            registers_after: {V2 => 0x01},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_xor {
            instruction: XOr(V2, V3),
            registers_before: {V2 => 0x01, V3 => 0x11},
            registers_after: {V2 => 0x10},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_add {
            instruction: Add(V2, V3),
            registers_before: {V2 => 0xFE, V3 => 0x01},
            registers_after: {V2 => 0xFF, V3 => 0x01},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_add_overflow {
            instruction: Add(V2, V3),
            registers_before: {V2 => 0xFF, V3 => 0x01},
            registers_after: {V2 => 0x00, V3 => 0x01},
            register_overflow: 1,
        }
    );

    registers_test!(
        vm_execute_instruction_sub {
            instruction: Sub(V2, V3),
            registers_before: {V2 => 0x3, V3 => 0x2},
            registers_after: {V2 => 0x1, V3 => 0x2},
            register_overflow: 1,
        }
    );

    registers_test!(
        vm_execute_instruction_sub_borrow {
            instruction: Sub(V2, V3),
            registers_before: {V2 => 0x3, V3 => 0x4},
            registers_after: {V2 => 0xFF, V3 => 0x4},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_shiftright {
            instruction: ShiftRight(V2, V3),
            registers_before: {V2 => 0b00, V3 => 0b10},
            registers_after: {V2 => 0b01, V3 => 0b10},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_shiftright_inplace {
            instruction: ShiftRight(V2, V2),
            registers_before: {V2 => 0b10},
            registers_after: {V2 => 0b01},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_shiftright_inplace_overflow {
            instruction: ShiftRight(V2, V2),
            registers_before: {V2 => 0b1111_1111},
            registers_after: {V2 => 0b0111_1111},
            register_overflow: 1,
        }
    );

    registers_test!(
        vm_execute_instruction_subnegated {
            instruction: SubNegated(V2, V3),
            registers_before: {V2 => 0x2, V3 => 0x3},
            registers_after: {V2 => 0x1, V3 => 0x3},
            register_overflow: 1,
        }
    );

    registers_test!(
        vm_execute_instruction_subnegated_borrow {
            instruction: SubNegated(V2, V3),
            registers_before: {V2 => 0x5, V3 => 0x3},
            registers_after: {V2 => 0xFE, V3 => 0x3},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_shiftleft {
            instruction: ShiftLeft(V2, V3),
            registers_before: {V2 => 0b00, V3 => 0b01},
            registers_after: {V2 => 0b10, V3 => 0b01},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_shiftleft_inplace {
            instruction: ShiftLeft(V2, V2),
            registers_before: {V2 => 0b0111_0111},
            registers_after: {V2 => 0b1110_1110},
            register_overflow: 0,
        }
    );

    registers_test!(
        vm_execute_instruction_shiftleft_inplace_overflow {
            instruction: ShiftLeft(V2, V2),
            registers_before: {V2 => 0b1111_0111},
            registers_after: {V2 => 0b1110_1110},
            register_overflow: 1,
        }
    );

    #[test]
    fn vm_execute_instruction_skipnotequal() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers.pc = 0x0;
        vm.registers[V0] = 0xFF;
        vm.registers[VF] = 0xEE;

        vm.execute_instruction(&Instruction::SkipNotEqual(V0, VF))?;

        assert_eq!(vm.registers.pc, 0x0002);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_loadi() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers.i = 0xF0F0;

        vm.execute_instruction(&LoadI(0x0AAA.into()))?;

        assert_eq!(vm.registers.i, 0x0AAA);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_longjump() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers.pc = 0x0111;
        vm.registers[V0] = 0x11;

        vm.execute_instruction(&LongJump(0x0111.into()))?;

        assert_eq!(vm.registers.pc, 0x0122);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_skipkeypressed() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers.pc = 0x0111;
        vm.registers[V6] = 0x4;
        vm.keypad[Key4] = Pressed;

        vm.execute_instruction(&SkipKeyPressed(V6))?;

        assert_eq!(vm.registers.pc, 0x0113);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_skipkeynotpressed() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers.pc = 0x0111;
        vm.registers[V6] = 0x4;
        vm.keypad[Key4] = NotPressed;

        vm.execute_instruction(&SkipKeyNotPressed(V6))?;

        assert_eq!(vm.registers.pc, 0x0113);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_loadkey() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.waiting_on_any_keypress = None;

        vm.execute_instruction(&LoadKey(V6))?;

        assert_eq!(vm.waiting_on_any_keypress, Some(V6));
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_addi() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers[V0] = 0x1;
        vm.registers.i = 0x0AAA;

        vm.execute_instruction(&AddI(V0))?;

        assert_eq!(vm.registers.i, 0x0AAB);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_random() -> crate::errors::Result<()> {
        let rng = rand::rngs::mock::StepRng::new(0b1000_0000, 0);
        let mut vm = VM::new(rng, |_, _| Ok(()));
        vm.registers[V0] = 0x00;

        vm.execute_instruction(&Instruction::Random(V0, 0b1100_0000))?;

        assert_eq!(vm.registers[V0], 0b1000_0000);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_draw() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers[V0] = 0xF;
        vm.registers[V1] = 0x0;
        vm.registers.i = 0x0111;
        vm.memory.write((0x0111 + 0).into(), 0b11111111);
        vm.memory.write((0x0111 + 1).into(), 0b10000000);
        vm.memory.write((0x0111 + 2).into(), 0b11111100);
        vm.memory.write((0x0111 + 3).into(), 0b10000000);
        vm.memory.write((0x0111 + 4).into(), 0b10000000);

        vm.execute_instruction(&Instruction::Draw(V0, V1, 5.into()))?;
        vm.execute_instruction(&Instruction::Draw(V0, V1, 5.into()))?;

        assert!(vm.registers[VF] != 0);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_loadsprite() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers[V0] = 0xF;

        vm.execute_instruction(&Instruction::LoadSprite(V0))?;

        // This is actually an implementation detail
        assert_eq!(
            vm.registers.i,
            FONT_RAW_ADDR + 0xF * crate::font::FONT_SPRITE_ROWS as u16
        );
        let mut sprite_data = vec![];
        for offs in 0..crate::font::FONT_SPRITE_ROWS {
            let addr = Addr::new(vm.registers.i + offs as u16)?;
            let row = vm.memory.read(addr);
            sprite_data.push(row);
        }
        assert_eq!(sprite_data[..], crate::font::SPRITE_DATA_F);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_loadbinarycodeddecimal() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers[V0] = 123;
        vm.registers.i = 0x0111;

        vm.execute_instruction(&Instruction::LoadBinaryCodedDecimal(V0))?;

        assert_eq!(vm.memory.read((0x0111 + 0x0).into()), 1);
        assert_eq!(vm.memory.read((0x0111 + 0x1).into()), 2);
        assert_eq!(vm.memory.read((0x0111 + 0x2).into()), 3);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_loadmemoryregisters_all() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers[V0] = 0x0;
        vm.registers[V1] = 0x1;
        vm.registers[V2] = 0x2;
        vm.registers[V3] = 0x3;
        vm.registers[V4] = 0x4;
        vm.registers[V5] = 0x5;
        vm.registers[V6] = 0x6;
        vm.registers[V7] = 0x7;
        vm.registers[V8] = 0x8;
        vm.registers[V9] = 0x9;
        vm.registers[VA] = 0xA;
        vm.registers[VB] = 0xB;
        vm.registers[VC] = 0xC;
        vm.registers[VD] = 0xD;
        vm.registers[VE] = 0xE;
        vm.registers[VF] = 0xF;
        vm.registers.i = 0x0111;

        vm.execute_instruction(&LoadMemoryRegisters(VF))?;

        assert_eq!(vm.memory.read((0x0111 + 0x0).into()), 0x0);
        assert_eq!(vm.memory.read((0x0111 + 0x1).into()), 0x1);
        assert_eq!(vm.memory.read((0x0111 + 0x2).into()), 0x2);
        assert_eq!(vm.memory.read((0x0111 + 0x3).into()), 0x3);
        assert_eq!(vm.memory.read((0x0111 + 0x4).into()), 0x4);
        assert_eq!(vm.memory.read((0x0111 + 0x5).into()), 0x5);
        assert_eq!(vm.memory.read((0x0111 + 0x6).into()), 0x6);
        assert_eq!(vm.memory.read((0x0111 + 0x7).into()), 0x7);
        assert_eq!(vm.memory.read((0x0111 + 0x8).into()), 0x8);
        assert_eq!(vm.memory.read((0x0111 + 0x9).into()), 0x9);
        assert_eq!(vm.memory.read((0x0111 + 0xA).into()), 0xA);
        assert_eq!(vm.memory.read((0x0111 + 0xB).into()), 0xB);
        assert_eq!(vm.memory.read((0x0111 + 0xC).into()), 0xC);
        assert_eq!(vm.memory.read((0x0111 + 0xD).into()), 0xD);
        assert_eq!(vm.memory.read((0x0111 + 0xE).into()), 0xE);
        assert_eq!(vm.memory.read((0x0111 + 0xF).into()), 0xF);
        assert_eq!(vm.registers.i, 0x0111 + 0xF + 1);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_loadmemoryregisters_one() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers[V0] = 0xAA;
        vm.registers[V1] = 0xBB;
        vm.registers.i = 0x0111;

        vm.execute_instruction(&LoadMemoryRegisters(V0))?;

        assert_eq!(vm.memory.read((0x0111 + 0).into()), 0xAA);
        assert_eq!(vm.memory.read((0x0111 + 1).into()), 0x00);
        assert_eq!(vm.registers.i, 0x0111 + 1);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_loadregistersmemory_all() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers.i = 0x0111;
        vm.memory.write((0x0111 + 0x0).into(), 0x0);
        vm.memory.write((0x0111 + 0x1).into(), 0x1);
        vm.memory.write((0x0111 + 0x2).into(), 0x2);
        vm.memory.write((0x0111 + 0x3).into(), 0x3);
        vm.memory.write((0x0111 + 0x4).into(), 0x4);
        vm.memory.write((0x0111 + 0x5).into(), 0x5);
        vm.memory.write((0x0111 + 0x6).into(), 0x6);
        vm.memory.write((0x0111 + 0x7).into(), 0x7);
        vm.memory.write((0x0111 + 0x8).into(), 0x8);
        vm.memory.write((0x0111 + 0x9).into(), 0x9);
        vm.memory.write((0x0111 + 0xA).into(), 0xA);
        vm.memory.write((0x0111 + 0xB).into(), 0xB);
        vm.memory.write((0x0111 + 0xC).into(), 0xC);
        vm.memory.write((0x0111 + 0xD).into(), 0xD);
        vm.memory.write((0x0111 + 0xE).into(), 0xE);
        vm.memory.write((0x0111 + 0xF).into(), 0xF);

        vm.execute_instruction(&LoadRegistersMemory(VF))?;

        assert_eq!(vm.registers[V0], 0x0);
        assert_eq!(vm.registers[V1], 0x1);
        assert_eq!(vm.registers[V2], 0x2);
        assert_eq!(vm.registers[V3], 0x3);
        assert_eq!(vm.registers[V4], 0x4);
        assert_eq!(vm.registers[V5], 0x5);
        assert_eq!(vm.registers[V6], 0x6);
        assert_eq!(vm.registers[V7], 0x7);
        assert_eq!(vm.registers[V8], 0x8);
        assert_eq!(vm.registers[V9], 0x9);
        assert_eq!(vm.registers[VA], 0xA);
        assert_eq!(vm.registers[VB], 0xB);
        assert_eq!(vm.registers[VC], 0xC);
        assert_eq!(vm.registers[VD], 0xD);
        assert_eq!(vm.registers[VE], 0xE);
        assert_eq!(vm.registers[VF], 0xF);
        assert_eq!(vm.registers.i, 0x0111 + 0xF + 1);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_loadregistersmemory_one() -> crate::errors::Result<()> {
        let mut vm = test_vm_default();
        vm.registers.i = 0x0111;
        vm.memory.write((0x0111 + 0x0).into(), 0x0);
        vm.memory.write((0x0111 + 0x1).into(), 0x1);

        vm.execute_instruction(&LoadRegistersMemory(V0))?;

        assert_eq!(vm.registers[V0], 0x0);
        assert_eq!(vm.registers[V1], 0x0);
        assert_eq!(vm.registers.i, 0x0111 + 1);
        Ok(())
    }
}
