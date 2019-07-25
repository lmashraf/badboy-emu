use std;

// Arithmetic Instructions
enum ArithmeticTarget
{
    A, B, C, D, E, H, L,
}

// Jump Instructions
enum JumpTarget
{
    NZ,     // Not Zero
    Z,      // Zero
    NC,     // Not Carry
    C,      // Carry
    A       // Always
}

enum Instruction
{
    // ADD can target all of the 8-bit registers except F
    ADD(ArithmeticTarget),
    JP(JumpTarget),
}

impl Instruction
{
    // Decodes an instruction into a byte
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction>
    {
        if prefixed
        {
            Instruction::from_byte_prefixed(byte)
        }
        else
        {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    // Decodes an instruction into a byte, if prefixed then one extra byte
    fn from_byte_prefixed(byte: u8) -> Option<Instruction>
    {
        match byte
        {
            0x00 => Some(Instruction::RLC(PrefixTArget::B)),
            _ => None
            // TODO: add mapping of the rest of prefixed instructions
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction>
    {
        match byte
        {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            _ => None
            // TODO: add mapping for the rest of none-prefixed instructions
        }
    }
}
