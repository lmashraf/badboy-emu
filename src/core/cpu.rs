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

    // 16-bit registers
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
                        self.program_counter.wrapping_add(1)
                    }
                }
                _ => 
                {
                    // TODO: support more targets
                    self.program_counter
                }
            }
            _ =>
            {
                // TODO: support more instructions
                self.program_counter
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

    fn step(&mut self)
    {
        // Read the instruction byte from memory using Program Counter register
        let mut instruction_byte = self.bus.read_byte(self.program_counter);

        // Translate the byte to one of the instancse of the Instruction enum
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte)
        {
            self.execute(instruction)
        }
        else
        {
            panic!("Unknown instruction found for: 0x{:x}", instruction_byte);
        };

        self.program_counter = next_pc;
    }
}