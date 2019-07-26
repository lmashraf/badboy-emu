use std;

// Arithmetic Instructions
pub enum ArithmeticTarget
{
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

// Increment/Decrement Instructions
pub enum IncDecTarget
{
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    BC,
    DE,
    HL,
    SP,
}

// Jump Instructions
pub enum JumpTest
{
    NZ,     // Not Zero
    Z,      // Zero
    NC,     // Not Carry
    C,      // Carry
    A       // Always
}

//
pub enum ADDHLTarget
{
    BC,
    DE,
    HL,
    SP,
}

// Load Instructions
pub enum LoadByteTarget
{
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI
}

pub enum LoadByteSource
{
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,     // Direct 8-bit value
    HLI
}

pub enum LoadType
{
    Byte(LoadByteTarget, LoadByteTarget),
}

pub enum Instruction
{
    ADD(ArithmeticTarget),
    JP(JumpTest),
    LD(LoadType),
    POP(StackTarget),
    PUSH(StackTarget),
}

// Stack Targets (PUSH, POP)
pub enum StackTarget
{
    AF,
    BC,
    DE,
    HL,
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
