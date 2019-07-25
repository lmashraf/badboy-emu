pub mod flags_register;
pub mod instructions;
pub mod registers;

use self::instruction::{
    ArithmeticTarget,
};

use self::registers::Registers;

pub struct CPU
{
    pub registers: Registers,
    pub bus: MemoryBus,
    program_counter: u16,
    stack_pointer: u16,
    is_halted: bool,
    is_interrupted: bool,
}

impl CPU
{
    fn execute(&mut self, instruction: Instruction)
    {
        match instruction
        {
            Instruction::ADD(target) =>
            {
                match target
                {
                    ArithmeticTarget::C =>
                    {
                        let value = self.registers.C;
                        let new_value = self.add(value);
                        self.registers.A = new_value;
                    }
                }
                _ => 
                {
                    // TODO: support more targets
                }
            }
            _ =>
            {
                // TODO: support more instructions
            }
        }
    }

    fn add(&mut self, value: u8) -> u8
    {
        let (new_value, did_overflow) = self.registers.A.overflowing_add(value);

        // Set the flags
        self.registers.F.zero = (new_value == 0);
        self.registers.F.substract = false;
        self.registers.F.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and
        // register A together results in a value bigger than 0xF.
        self.registers.F.half_carry = ((self.registers.A & 0xF) + (value + 0xF)) > 0xF;

        new_value
    }
}