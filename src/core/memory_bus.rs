pub struct MemoryBus 
{
    memory: [u8; 0xFFFF]
}

impl MemoryBus
{
    fun read_byete(&self, address: u16) -> u8
    {
        self.memory[address as usize]
    }
}