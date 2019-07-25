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
    pc: u16,
    sp: u16,
    pub bus: MemoryBus,
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
                        // TODO: implement ADD on register C
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
    }
}