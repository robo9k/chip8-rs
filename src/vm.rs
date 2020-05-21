//! Virtual machine

use crate::instructions::{Instruction, VRegister};
use std::ops::{Index, IndexMut};

/// Type of a general purpose register in the VM
pub type VRegisterValue = u8;

/// General purpose registers
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
        match instruction {
            Instruction::Add(vx, vy) => {
                let x = self.registers[*vx] as u16;
                let y = self.registers[*vy] as u16;

                let res = x + y;

                // VF is carryover
                self.registers[VRegister::VF] =
                    (res > VRegisterValue::MAX as u16) as VRegisterValue;

                self.registers[*vx] = res as VRegisterValue;
            }

            other => panic!("Unimplemented instruction: {:?}", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vregisters_set_get() {
        let mut registers = VRegisters::new();
        registers[VRegister::V0] = 42;
        assert_eq!(registers[VRegister::V0], 42);
    }

    #[test]
    fn vm_execute_instruction_add() {
        let mut vm = VM::new();
        vm.registers[VRegister::V2] = 0xFE;
        vm.registers[VRegister::V3] = 0x01;

        vm.execute_instruction(&Instruction::Add(VRegister::V2, VRegister::V3));

        assert_eq!(vm.registers[VRegister::V2], 0xFF);
        assert_eq!(vm.registers[VRegister::V3], 0x01);

        assert_eq!(vm.registers[VRegister::VF], 0);
    }

    #[test]
    fn vm_execute_instruction_add_overflow() {
        let mut vm = VM::new();
        vm.registers[VRegister::V2] = 0xFF;
        vm.registers[VRegister::V3] = 0x01;

        vm.execute_instruction(&Instruction::Add(VRegister::V2, VRegister::V3));

        assert_eq!(vm.registers[VRegister::V2], 0x00);
        assert_eq!(vm.registers[VRegister::V3], 0x01);

        assert_eq!(vm.registers[VRegister::VF], 1);
    }
}
