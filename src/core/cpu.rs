pub mod flags_register;
pub mod instructions;
pub mod registers;

use self::instruction::{
    ArithmeticTarget,
    ADDHLTarget,
    JumpTest,
    LoadByteSource,
    LoadByteTarget,
    LoadType,
    StackTarget,
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
    // Constructor
    pub fn new(boot_rom: Option<Vec<u8>>, game_rom: Vec<u8>) -> CPU
    {
        CPU
        {
            registers: Registers::new(),
            program_counter: 0x0,
            stack_pointer: 0x0,
            bus: MemoryBus::new(boot_rom, game_rom),
            is_halted: false,
            is_interrupted: true,
        }
    }

    // Program Counter's step to next OpCode
    pub fn step(&mut self) -> u8
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
        let next_program_counter = if let Some(instruction) = Instruction::from_byte(instruction_byte)
        {
            self.execute(instruction)
        }
        else
        {
            let error_description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unknown instruction found for: {}", error_description);
        };

        self.program_counter = next_program_counter;
    }


    // Executes OpCodes
    pub fn execute(&mut self, instruction: Instruction)
    {
        match instruction
        {
            // ADD
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
            // JP
            Instruction::JP(target) =>
            {
                let jump_condition = match target
                {
                    JumpTest::NZ => !self.registers.F.zero,
                    JumpTest::NC => !self.registers.F.carry,
                    JumpTest::Z  => !self.registers.F.zero,
                    JumpTest::C  => !self.registers.F.carry,
                    JumpTest::A  => true
                };
                self.jump(jump_condition)
            }
            // LD
            Instruction::LD(load_type) =>
            {
                LoadType::Byte(target, source) =>
                {
                    let source_value = match source
                    {
                        LoadByteSource::A => self.registers.A,
                        LoadByteSource::D8 => self.read_next_byte()),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_HL()),
                        _ =>
                        {
                            // TODO: implement other sources
                        }
                    };
                    match target
                    {
                        LoadByteTarget::A => self.registers.A = source_value,
                        LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_HL(), source_value),
                        _ =>
                        {
                            // TODO: implement other targets
                        }
                    };
                    match source
                    {
                        LoadByteSource::D8  => self.program_counter.wrapping_add(2),
                        _                   => self.program_counter.wrapping_add(1),
                    }
                    _=>
                    {
                        // TODO: implement other load types
                    }
                }
            }
            // PUSH
            Instruction::PUSH(target)
            {
                let value = match target
                {
                    StackTarget::BC => self.registers.get_BC(),
                    _ =>
                    {
                        // TODO : support more targets
                    }
                    self.push(value);
                    self.program_counter.wrapping_add(1)
                }
            }
            // POP
            Instruction::POP(target)
            {
                let result = self.pop();
                match target
                {
                    StackTarget::BC => self.registers.set_BC(result),
                    _ =>
                    {
                        // TODO: support more targets
                    }
                }
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

    // Push
    fn push(&mut self, value: u16)
    {
        // Decrease SP by 1: the Stack grows downward in memory.
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);

        // Write the MSB of the 16-bit value into memory at the location SP is pointing
        self.bus.write_byte(self.stack_pointer,((value & 0xFF00) >> 8) as u8);

        // Decrease SP by 1
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);

        // Write the LSB of the 16-bit value into memory at the location SP is pointing
        self.bus.write_byte(self.stack_pointer,(value & 0x00FF) as u8);
    }

    // Pop
    fn pop(&mut self) -> u16
    {
        // Read the LSB of the 16-bit value which is pointed by the SP
        let least_significant_byte = self.bus.read_byte(self.stack_pointer) as u16;

        // Increase SP by 1
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        // Read the MSB of the 16-bit value which is pointed by the SP
        let most_significant_byte = self.bus.read_byte(self.stack_pointer) as u16;

        // Increase SP by 1
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        // Pop the 16-bit value
        (most_significant_byte << 8) | least_significant_byte
    }

    // Call
    fn call(&mut self, should_jump: bool) -> u16
    {
        // Push the next PC on to the stack
        let next_program_counter = self.program_counter.wrapping_add(3);

        // Jump to the address specified in the next 2 bytes of the memory
        if should_jump
        {
            self.push(next_program_counter);
            self.read_next_word()
        }
        else
        {
            next_program_counter
        }
    }

    // Return
    fn return(&mut self, should_jump: bool) -> u16
    {
        if should_jump
        {
            self.pop()
        }
        else
        {
            self.program_counter.wrapping_add(1)
        }
    }

    // Read and Write functions
    fn read_next_byte(&self) -> u8
    {
        self.bus.read_byte(self.program_counter + 1)
    }

    fn read_next_word(&self) -> u16
    {
        ((self.bus.read_byte(self.program_counter + 2) as u16) << 8) | (self.bus.read_byte(self.program_counter +1) as u16)
    }
}