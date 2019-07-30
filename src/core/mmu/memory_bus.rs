pub struct MemoryBus 
{
    memory: [u8; 0xFFFF]
    boot_rom: Option<[u8; 0xFFFF]>,
    game_rom: [u8; 0xFFFF]
    // TODO: review MMU logic
}

impl MemoryBus
{
    pub fn new(boot_rom: Option<Vec<u8>>, game_rom: Vec<u8>) -> MemoryBus
    {
        MemoryBus
        {
            memory,
            boot_rom,
            game_rom,
        }
    }

    // Read and Write to and from Memory
    fn read_byte(&self, address: u16) -> u8
    {
        let address = address as usize;

        match address
        {
            VRAM_BEGIN ... VRAM_END =>
            {
                self.gpu.read_vram(address - VRAM_BEGIN, value)
            }
            _ =>
            {
                // TODO: support other areas of memory
            }
        }

        self.memory[address as usize]
    }

    fn write_byte(&self, address: u16, byte: u8)
    {
        // TODO: write byte to given memory address
    }
}