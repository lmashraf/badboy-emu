// 8-bit registers
struct Registers
{
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagRegisters,
    h: u8,
    l: u8,
}

impl Registers 
{
    pub fn new() -> Registers
    {
        Registers
        {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::new(),
            h: 0,
            l: 0,
        }
    }
    // Read from virtual 16-bits register
    fn get_bc(&self) -> u16 
    {
        (self.b as u16) << 8 | self.c as u16
    }

    // Write to virtual 16-bits register
    fn set_bc(&mut self, value: u16) 
    {
        self.b = ((value & 0xFF00) >> 8)  as u8;
        self.c = (value & 0xFF) as u8;
    }
}