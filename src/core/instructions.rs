use std;

enum ArithmeticTarget
{
    A, B, C, D, E, H, L,
}

enum Instruction
{
    // ADD can target all of the 8-bit registers except F
    ADD(ArithmeticTarget),
}

impl Instruction
{
    // Decodes an instruction into a byte
    fn from_byte(byte: u8) -> Option<Instruction>
    {
        match byte
        {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            _ => None
            // TODO : add mapping of all targets/instructions
        }
    }
}
