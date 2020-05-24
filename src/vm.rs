//! Virtual machine

use crate::instructions::{Instruction, VRegister};
use rand::Rng;
use std::ops::{Index, IndexMut};

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

/// Virtual machine
//#[derive(Debug)]
pub struct VM<R: Rng> {
    registers: Registers,
    rng: R,
    sys_fn: fn(&mut Self, crate::instructions::Addr) -> crate::errors::Result<()>,
    keypad: crate::keypad::Keypad,
}

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
        Self {
            registers: Registers::new(),
            rng,
            sys_fn,
            keypad: crate::keypad::Keypad::default(),
        }
    }

    #[must_use]
    fn execute_instruction(&mut self, instruction: &Instruction) -> crate::errors::Result<()> {
        match *instruction {
            Instruction::Sys(addr) => return (self.sys_fn)(self, addr),
            // Clear
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
            // Draw(Vx, Vy, Nibble)
            // SkipKeyPressed(Vx)
            // SkipKeyNotPressed(Vx)
            // LoadRegisterDelayTimer(Vx)
            // LoadKey(Vx)
            // LoadDelayTimerRegister(Vx)
            // LoadSoundTimerRegister(Vx)
            Instruction::AddI(vx) => self.registers.i += self.registers[vx] as IRegisterValue,
            // LoadSprite(Vx)
            // LoadBinaryCodedDecimal(Vx)
            // LoadMemoryRegisters(Vx)
            // LoadRegistersMemory(Vx)
            other => return Err(crate::errors::Chip8Error::UnimplementedInstruction(other)),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::{Instruction::*, VRegister::*};

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
                let mut vm = $crate::vm::VM::default();
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
        let mut vm = VM::default();
        vm.registers.pc = 0x0;

        vm.execute_instruction(&Instruction::Jump(0x0FFF.into()))?;

        assert_eq!(vm.registers.pc, 0x0FFF);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_skipequaloperand() -> crate::errors::Result<()> {
        let mut vm = VM::default();
        vm.registers.pc = 0x0;
        vm.registers[V0] = 0xFF;

        vm.execute_instruction(&Instruction::SkipEqualOperand(V0, 0xFF))?;

        assert_eq!(vm.registers.pc, 0x0002);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_skipnotequaloperand() -> crate::errors::Result<()> {
        let mut vm = VM::default();
        vm.registers.pc = 0x0;
        vm.registers[V0] = 0xFF;

        vm.execute_instruction(&Instruction::SkipNotEqualOperand(V0, 0xEE))?;

        assert_eq!(vm.registers.pc, 0x0002);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_skipequal() -> crate::errors::Result<()> {
        let mut vm = VM::default();
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
        let mut vm = VM::default();
        vm.registers.pc = 0x0;
        vm.registers[V0] = 0xFF;
        vm.registers[VF] = 0xEE;

        vm.execute_instruction(&Instruction::SkipNotEqual(V0, VF))?;

        assert_eq!(vm.registers.pc, 0x0002);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_loadi() -> crate::errors::Result<()> {
        let mut vm = VM::default();
        vm.registers.i = 0xF0F0;

        vm.execute_instruction(&LoadI(0x0AAA.into()))?;

        assert_eq!(vm.registers.i, 0x0AAA);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_longjump() -> crate::errors::Result<()> {
        let mut vm = VM::default();
        vm.registers.pc = 0x0111;
        vm.registers[V0] = 0x11;

        vm.execute_instruction(&LongJump(0x0111.into()))?;

        assert_eq!(vm.registers.pc, 0x0122);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_addi() -> crate::errors::Result<()> {
        let mut vm = VM::default();
        vm.registers[V0] = 0x1;
        vm.registers.i = 0x0AAA;

        vm.execute_instruction(&AddI(V0))?;

        assert_eq!(vm.registers.i, 0x0AAB);
        Ok(())
    }

    #[test]
    fn vm_execute_instruction_random() -> crate::errors::Result<()> {
        let rng = bufrng::BufRng::new(&[0, 0, 0, 0b1000_0000]);
        let mut vm = VM::new(rng, |_, _| Ok(()));
        vm.registers[V0] = 0x00;

        vm.execute_instruction(&Instruction::Random(V0, 0b1100_0000))?;

        assert_eq!(vm.registers[V0], 0b1000_0000);
        Ok(())
    }
}
