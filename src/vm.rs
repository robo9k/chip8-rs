//! Virtual machine

use crate::instructions::{Instruction, VRegister};
use std::ops::{Index, IndexMut};

/// Type of a general purpose register in the VM
pub type VRegisterValue = u8;

/// General purpose registers
#[derive(Debug)]
pub struct VRegisters {
    vregisters: [VRegisterValue; 16], // TODO: Use variant_count for constant?
}

impl Default for VRegisters {
    fn default() -> Self {
        Self::new()
    }
}

impl VRegisters {
    /// Creates a new instance with default values
    pub fn new() -> Self {
        Self {
            vregisters: [0; 16],
        }
    }

    /// Returns the current value of the register
    pub fn register(&self, register: VRegister) -> &VRegisterValue {
        match register {
            VRegister::V0 => &self.vregisters[0],
            VRegister::V1 => &self.vregisters[1],
            VRegister::V2 => &self.vregisters[2],
            VRegister::V3 => &self.vregisters[3],
            VRegister::V4 => &self.vregisters[4],
            VRegister::V5 => &self.vregisters[5],
            VRegister::V6 => &self.vregisters[6],
            VRegister::V7 => &self.vregisters[7],
            VRegister::V8 => &self.vregisters[8],
            VRegister::V9 => &self.vregisters[9],
            VRegister::VA => &self.vregisters[10],
            VRegister::VB => &self.vregisters[11],
            VRegister::VC => &self.vregisters[12],
            VRegister::VD => &self.vregisters[13],
            VRegister::VE => &self.vregisters[14],
            VRegister::VF => &self.vregisters[15],
        }
    }

    /// Returns the current mutable value of the register
    pub fn register_mut(&mut self, register: VRegister) -> &mut VRegisterValue {
        match register {
            VRegister::V0 => &mut self.vregisters[0],
            VRegister::V1 => &mut self.vregisters[1],
            VRegister::V2 => &mut self.vregisters[2],
            VRegister::V3 => &mut self.vregisters[3],
            VRegister::V4 => &mut self.vregisters[4],
            VRegister::V5 => &mut self.vregisters[5],
            VRegister::V6 => &mut self.vregisters[6],
            VRegister::V7 => &mut self.vregisters[7],
            VRegister::V8 => &mut self.vregisters[8],
            VRegister::V9 => &mut self.vregisters[9],
            VRegister::VA => &mut self.vregisters[10],
            VRegister::VB => &mut self.vregisters[11],
            VRegister::VC => &mut self.vregisters[12],
            VRegister::VD => &mut self.vregisters[13],
            VRegister::VE => &mut self.vregisters[14],
            VRegister::VF => &mut self.vregisters[15],
        }
    }

    /// Sets the current value of the register
    pub fn set_register(&mut self, register: VRegister, value: VRegisterValue) {
        *self.register_mut(register) = value;
    }
}

impl Index<VRegister> for VRegisters {
    type Output = VRegisterValue;

    fn index(&self, index: VRegister) -> &Self::Output {
        &self.register(index)
    }
}

impl IndexMut<VRegister> for VRegisters {
    fn index_mut(&mut self, index: VRegister) -> &mut Self::Output {
        self.register_mut(index)
    }
}

/// Virtual machine
#[derive(Debug)]
pub struct VM {
    registers: VRegisters,
}

impl VM {
    fn new() -> Self {
        Self {
            registers: VRegisters::default(),
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match *instruction {
            // Sys(Addr)
            // Clear
            // Return
            // Jump(Addr)
            // Call(Addr)
            // SkipEqualOperand(Vx, Byte)
            // SkipNotEqualOperand(Vx, Byte)
            // SkipEqual(Vx, Vy)
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
            // SkipNotEqual(Vx, Vy),
            // LoadI(Addr),
            // LongJump(Addr)
            other => panic!("Unimplemented instruction: {:?}", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::{Instruction::*, VRegister::*};

    #[test]
    fn vregisters_set_get() {
        let mut registers = VRegisters::new();
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
            fn $test_name() {
                let mut vm = $crate::vm::VM::new();
                $(
                  vm.registers[$register_before] = $register_before_value;
                )+
                println!("Created VM: {:?}", vm);

                vm.execute_instruction(&$instruction);
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
            }
        };
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
}
