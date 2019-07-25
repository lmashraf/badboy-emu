pub mod flags_register;
pub mod instructions;
pub mod registers;

use self::instruction::{
    ArithmeticTarget, JumpTarget,
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
    // Executes OpCodes
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
            Instruction::JP(target) =>
            {
                let jump_condition = match target
                {
                    JumpTarget::NZ => !self.registers.F.zero,
                    JumpTarget::NC => !self.registers.F.carry,
                    JumpTarget::Z => !self.registers.F.zero,
                    JumpTarget::C => !self.registers.F.carry,
                    JumpTarget::A => true
                };
                self.jump(jump_condition)
            }
            _ =>
            {
                // TODO: support more instructions
                self.program_counter
            }
        }
    }

    // Accumulate
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

    // Jump
    fn jump(&self, should_jump: bool) -> u16
    {
        if should_jump
        {
            // GB is Little Endian, ie:
            // PC+2 is MSB and PC+1 is LSB
            let least_significant_byte = self.bus.read_byte(self.program_counter + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.program_counter + 2) as u16;
        }
        else
        {
            // Jump instruction is 3 bytes wide, we still need to move the PC if we don't jump
            self.program_counter.wrapping_add(3)
        }
    }

    // Program Counter's step to next OpCode
    fn step(&mut self)
    {
        // Read the instruction byte from memory using Program Counter register
        let mut instruction_byte = self.bus.read_byte(self.program_counter);

        // Check if the byte we read from memory is 0xCB, if it is, we read one
        // more byte and interpret the current as a "prefix instruction"
        let prefixed = (instruction_byte == 0xCB);
        if prefixed
        {
            instruction_byte = self.bus.read_byte(self.program_counter + 1);
        }

        // Translate the byte to one of the instancse of the Instruction enum
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte)
        {
            self.execute(instruction)
        }
        else
        {
            let error_description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unknown instruction found for: {}", error_description);
        };

        self.program_counter = next_pc;
    }
}