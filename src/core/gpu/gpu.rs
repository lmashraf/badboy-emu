/* Tiles are stored between 0x8000 and 0x97FF (6144 bytes)
 * 
 * 8000 - 87FF
 *      Tile Set #1 - PART I
 * 8800 - 8FFF
 *      Tile Set #1 - PART II
 *      Tile Set #2 - PART I
 * 9000 - 97FF
 *      Tile Set #2 - PART II
 */ 

const TILES_SIZE: usize = 384;

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = (VRAM_END - VRAM_BEGIN) + 1;


/* The bit value to colour mapping is as follows:
 *
 * 0b11 : white
 * 0b10 : dark-grey
 * 0b01 : light-grey
 * 0b00 : black
 */
enum TilePixelValue 
{
    WHITE,
    DARK_GREY,
    LIGHT_GREY,
    BLACK,
}

// Each Tile is encoded in 16 bytes.
type Tile = [[TilePixelValue; 8]; 8];

fn empty_tile() -> Tile 
{
    [[TilePixelValue::WHITE; 8]; 8]
}

struct GPU
{
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; TILES_SIZE],
}

impl GPU
{
    // Read and Write to and from VRAM array
    fn write_vram(&mut self, address: usize, value: u8)
    {
        // If the address is greater than 0x1800, abort
        if address >= 0x1800
        {
            return
        }

        self.vram[address] = value;

        // Tiles rows are encoded in 16-bit, the first bit is always on an even address.
        // Using AND between tis value and 0xFFFE gives the address of the first 8-bits.
        let normalised_address = address & 0xFFFE;

        // Retrieve 16 bits that encodes the tile row.
        let tile_byte_1 = self.vram[normalised_address];
        let tile_byte_2 = self.vram[normalised_address + 1];

        // A tile is 8 rows tall, each encoded in 16 bits.
        let tile_index = address / 16;

        // Every 16 bits is a row
        let row_index = (address % 16) / 2;

        // Loop through the row of tiles (8 times)
        for pixel_index in 0..8
        {
            // Shift the pixels to read the bits left to right.
            let mask = 1 << (7 - pixel_index);

            let lsb = tile_byte_1 & mask;
            let msb = tile_byte_2 & mask;

            // Assess the pixel values
            let value = match( lsb != 0, msb != 0)
            {
                (true,false) => TilePixelValue::WHITE,
                (true,true) => TilePixelValue::LIGHT_GREY,
                (false,true) => TilePixelValue::DARK_GREY,
                (false,false) => TilePixelValue::BLACK,
            };

            // Set the pixel's value to render
            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }

    fn read_vram(&self, address: usize) -> u8
    {
        self.vram[address]
    }
}