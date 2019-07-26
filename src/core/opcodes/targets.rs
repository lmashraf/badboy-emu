// -------------------------------------------------------
// ALU Targets
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

pub enum ADDHLTarget
{
    BC,
    DE,
    HL,
    SP,
}

// -------------------------------------------------------
// Prefixed Targets
pub enum PrefixTarget 
{
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

// -------------------------------------------------------
// Incr. & Decr. Targets
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

// -------------------------------------------------------
// Jump Tests
pub enum JumpTest
{
    NZ,     // Not Zero
    Z,      // Zero
    NC,     // Not Carry
    C,      // Carry
    A       // Always
}

// -------------------------------------------------------
// Bit Positions
pub enum BitPosition 
{
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
}

impl std::convert::From<BitPosition> for u8 
{
    fn from(position: BitPosition) -> u8 
    { 
        match position 
        {
            BitPosition::B0 => 0,
            BitPosition::B1 => 1,
            BitPosition::B2 => 2,
            BitPosition::B3 => 3,
            BitPosition::B4 => 4,
            BitPosition::B5 => 5,
            BitPosition::B6 => 6,
            BitPosition::B7 => 7,
        }
    }
}

// -------------------------------------------------------
//  Restart Positions
pub enum RSTPosition
{
    X00,
    X08,
    X10,
    X18,
    X20,
    X28,
    X30,
    X38,
}

impl RSTPosition 
{
    pub fn to_hex(&self) -> u16 
    {
        match self 
        {
            RSTPosition::X00 => 0x00,
            RSTPosition::X08 => 0x08,
            RSTPosition::X10 => 0x10,
            RSTPosition::X18 => 0x18,
            RSTPosition::X20 => 0x20,
            RSTPosition::X28 => 0x28,
            RSTPosition::X30 => 0x30,
            RSTPosition::X38 => 0x38,
        }
    }
}

// -------------------------------------------------------
// Loads Targets, Sources and Types
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

pub enum LoadWordTarget 
{
    BC,
    DE,
    HL,
    SP,
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
    D8,
    HLI
}

pub enum Indirect 
{
    BCIndirect,
    DEIndirect,
    HLIndirectMinus,
    HLIndirectPlus,
    WordIndirect,
    LastByteIndirect,
}

pub enum LoadType
{
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget),
    AFromIndirect(Indirect),
    IndirectFromA(Indirect),
    AFromByteAddress,
    ByteAddressFromA,
    SPFromHL,
    HLFromSPN,
    IndirectFromSP,
}

// -------------------------------------------------------
// Stack Targets
pub enum StackTarget
{
    AF,
    BC,
    DE,
    HL,
}
