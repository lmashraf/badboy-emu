pub mod FlagRegisters;

// 8-bit registers
struct Registers
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
    // Read from virtual 16-bits register
    fn get_BC(&self) -> u16
    {
        (self.B as u16) << 8 | (self.C as u16)
    }

    // Write to virtual 16-bits register
    fn set_BC(&mut self, value: u16)
    {
        self.B = ((value & 0xFF00) >> 8) as u8;
        self.C = (value & 0xFF) as u8;
    }
}