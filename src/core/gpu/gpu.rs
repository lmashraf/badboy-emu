/*
 * Tiles are stored between 0x8000 and 0x97FF for what's worth 6144 bytes of Memory
 * Each Tile is encoded in 16 bytes.
 * 
 * 8000 - 87FF
 *      Tile Set #1 - PART I
 * 8800 - 8FFF
 *      Tile Set #1 - PART II
 *      Tile Set #2 - PART I
 * 9000 - 97FF
 *      Tile Set #2 - PART II
 * 
 * The bit value to colour mapping is as follows:
 * 
 * 0b11 : white
 * 0b10 : dark-grey
 * 0b01 : light-grey
 * 0b00 : black
 * 
 * 
 *               Bit Position
 * A            7 6 5 4 3 2 1 0
 * d          +-----------------+
 * d  0x8000  | 1 0 1 1 0 1 0 1 |
 * r          |-----------------|
 * e  0x8001  | 0 1 1 0 0 1 0 1 |
 * s          +-----------------+
 * s            D L W D B W B W
 *                    Color
 */ 

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = (VRAM_END - VRAM_BEGIN) + 1;

#[derive(Debug)]
enum TilePixelValue 
{
    NIL,
    ONE,
    TWO,
    THREE,
}

type Tile = [[TilePixelValue; 8]; 8];

fn empty_tile() -> Tile 
{
    [[TilePixelValue::NIL; 8]; 8]
}

struct GPU
{
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}