use flag_registers::FlagsRegister;

// 8-bit registers
pub struct Registers
{
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    F: FlagRegisters,
    H: u8,
    L: u8,
}

impl Registers 
{
    pub fn new() -> Registers
    {
        Registers
        {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            F: FlagsRegister::new(),
            H: 0,
            L: 0,
        }
    }

    // Read/Write functions for virtual 16-bits registers
    // BC:
    fn get_BC(&self) -> u16
    {
        (self.B as u16) << 8 | (self.C as u16)
    }

    fn set_BC(&mut self, value: u16)
    {
        self.B = ((value & 0xFF00) >> 8) as u8;
        self.C = (value & 0xFF) as u8;
    }

    // AF:
    fn get_AF(&self) -> u16
    {
        (self.A as u16) << 8 | (self.F as u16)
    }

    fn set_AF(&mut self, value: u16)
    {
        self.A = ((value & 0xFF00) >> 8) as u8;
        self.F = FlagsRegister::from((value & 0xFF) as u8);
    }

    // DE:
    fn get_DE(&self) -> u16
    {
        (self.D as u16) << 8 | (self.E as u16)
    }

    fn set_DE(&mut self, value: u16)
    {
        self.D = ((value & 0xFF00) >> 8) as u8;
        self.E = (value & 0xFF) as u8;
    }

    // HL:
    fn get_HL(&self) -> u16
    {
        (self.H as u16) << 8 | (self.L as u16)
    }

    fn set_HL(&mut self, value: u16)
    {
        self.H = ((value & 0xFF00) >> 8) as u8;
        self.L = (value & 0xFF) as u8;
    }
}