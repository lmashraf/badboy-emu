pub struct MemoryBus 
{
    memory: [u8; 0xFFFF]
}

impl MemoryBus
{
    fn read_byte(&self, address: u16) -> u8
    {
        self.memory[address as usize]
    }

    fn write_byte(&self, address: u16, byte: u8)
    {
        // TODO: write byte to given memory address
    }
}