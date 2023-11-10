//const TILE_SIZE: usize = 16;
//const TILE_DATA_ADDRESS_RANGE: std::ops::Range<usize> = 0x8000..0x9800;
//const TILE_DATA_ADDRESS_RANGE_8800: std::ops::Range<usize> = 0x8800..0x9000;

/*
#[derive(Clone)]
pub struct Tile {
    pixel_data: [[u8; 8]; 8],
}
*/
/*
pub struct TileDataTable {
    data: Vec<Tile>,
}
*/

//16 bytes from vram into Tile object
pub fn extract_tile(tile_data: [u8; 16]) -> [[u8; 8]; 8] {
    //tile data should be read from adresses between 0x8000 and 0x97FF
    let mut tile = [[0u8; 8]; 8];

    for row in 0..8 {
        // Get two bytes that represent a row of pixel data
        let byte1 = tile_data[row * 2];
        let byte2 = tile_data[row * 2 + 1];

        for col in 0..8 {
            // Calculate the shift amount to extract the color for this pixel
            let shift = 7 - col;

            // Extract the color (2 bits) for this pixel
            let color1 = (byte1 >> shift) & 0b01;
            let color2 = (byte2 >> shift) & 0b01;

            // Combine the color bits from both bytes to get the final color
            let color = (color2 << 1) | color1;

            tile[row][col] = color;
        }
    }
    // return 2d array filled with 0, 1, 2 and 3.
    //0 = transparent; 1 = light gray: 2 = dark gray: 3 = black;
    tile
}

/*

//get 0x8000 adressing from 0x8800 adressing method (in case it is needed)
fn get_tile_index(tile_number: i8) -> usize {
    if tile_number >= 0 {
        tile_number as usize
    } else {
        256 + (tile_number as usize)
    }
}

impl TileDataTable {
    pub fn new() -> TileDataTable{
        TileDataTable {
            data: vec![Tile {
                pixel_data: [[0; 8]; 8]
            }; 384], // 384 tiles in total (0x9800 - 0x8000)
        }
    }

    // Populate the table with tile data using the extract_tile function
    pub fn populate(&mut self, tile_number: i8, tile_data: [u8; 16]) {
        let index = tile_number as usize;
        self.data[index] = Tile {
            pixel_data: extract_tile(tile_data),
        };
    }

    // Access a tile by index
    pub fn get_tile(&self, tile_number: i8) -> &Tile {
        let index = tile_number as usize;
        &self.data[index]
    }
    
}
*/