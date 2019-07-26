use std;
use targets::*;

// -------------------------------------------------------
// OpCodes Summary
pub enum Instruction
{
    // 8-bit & 16-bit Loads
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),

    // 8-bit ALU
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),

    // 8-bit & 16-bit Arithmetic
    INC(IncDecTarget),
    DEC(IncDecTarget),

    // 16-bit Arithmetic
    ADDHL(ADDHLTarget),
    ADDSP,

    // Miscellaneous
    SWAP(PrefixTarget),
    DAA,
    CPL,

    CCF,
    SCF,

    NOP,
    HALT,
    STOP,
    DI,
    EI,    

    // Rotates & Shifts
    RLCA,
    RLA,
    RRCA,
    RRA,

    RLC(PrefixTarget),
    RL(PrefixTarget),
    RRC(PrefixTarget),
    RR(PrefixTarget),

    SLA(PrefixTarget),
    SRA(PrefixTarget),
    SRL(PrefixTarget),
    
    // Bit Opcodes
    BIT(PrefixTarget, BitPosition),
    RES(PrefixTarget, BitPosition),
    SET(PrefixTarget, BitPosition),

    // Jumps
    JP(JumpTest),
    JR(JumpTest),
    JPI,

    // Calls
    CALL(JumpTest),

    // Restarts
    RST(RSTLocation),

    // Returns
    RET(JumpTest),
    RETI,
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
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
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
