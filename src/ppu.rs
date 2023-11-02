use crate::tile::extract_tile;

struct PPU {
    lcdc:u8,
    lcds:u8,
    scy:u8,
    scx:u8,
    ly:u8,
    lyc:u8,
    dma:u8,
    bg_palette:[u32;4],
    obp0:[u32;4],
    obp1:[u32;4],
    wy:u8,
    wx:u8,
    video_buffer:[u8;160*144],
    oam:[u8;0xA0],
    vram:[u8;0x2000],
    bg_tileset:[u8;256*256],
    win_tileset:[u8;256*256],

}

impl PPU{

    fn ppu_tick(&self){
        self.ly+= 1;
        //Match mode
    }

    fn get_bit(&self,byte:u8,bit:u8)->u8{
        byte >> bit & 1
    }

    fn read_oam(&self,mut address:u16)->u8{
        if address>=0xFE00{
            address-=0xFE00;
        }
        self.oam[address as usize]
    }

    fn write_oam(&self,mut address:u16,val:u8){
        if address>=0xFE00{
            address-=0xFE00;
        }
        self.oam[address as usize] = val;
    }

    fn vram_write(&self,address:u16,value:u8){
        self.vram[address as usize] = value;
    }
    
    fn vram_read(&self,address:u16) -> u8{
        self.vram[address as usize]
    }

    fn render_line(&self){
        let y = self.ly;
        for x in 0..160 {
            self.video_buffer[(x+160*y) as usize] = self.render_pixel(x,y);
        }
        
    }
    fn render_pixel(&self,x:u8,y:u8)->u8{
        //If BG/WIN Enabled
        if self.get_bit(self.lcdc, 0) != 0 {
            //If WIN Enabled
            if self.get_bit(self.lcdc, 5) != 0 {
                if self.pixel_in_window(x, y){
                    return self.pixel_from_window(x, y)
                }else{
                    return self.pixel_from_background(x,y)
                }
            }else{
                return self.pixel_from_background(x,y)
            }
        }else {
            return 0;
        }
    }

    fn pixel_in_window(&self,x:u8,y:u8)->bool{
        let signed_x = x as i32;
        let signed_y = y as i32;
        let signed_wx = (self.wx as i32) - 7;
        let signed_wy = self.wy as i32;

        (signed_x >= signed_wx) && (signed_y >= signed_wy)
    }

    fn pixel_from_window(&self,x:u8,y:u8)->u8{
        let x = x - (self.wx - 7);
        let y = y - self.wy;
        self.win_tileset[(x+256*y) as usize]
    }

    fn pixel_from_background(&self,x:u8,y:u8)->u8{
        let x = (x + self.scx)%256;
        let y = (y + self.scy)%256;
        self.bg_tileset[(x+256*y) as usize]
        
    }

    fn background_tilesets(&self)->[u8;256*256]{
        let tilemap = self.tile_map();
        let mut tile_line = 0;
        let mut background = [0u8;256*256];
        for i in tilemap.0..tilemap.1 {
            let tile_index =  self.vram_read(i - 0x8000);
            let tile_address = self.addressing_mode(tile_index);
            let mut tile = [0u8;16];
            for j in 0..16 {
                tile[j as usize] = self.vram_read(tile_address+j);
            }
            let tile_pixels = extract_tile(tile);
            
            if i%32 == 0 {
                tile_line += 8;
            }
            for k in 0..8 {
                for l in 0..8 {
                    //Use palettes
                    background[(256*tile_line+256*k+l+8*i%32) as usize] = tile_pixels[k as usize][l as usize];
                }
            }
            
        }
        background        
    }

    fn render_sprites(&self)->[u8;10]{
        let mut i = 0;
        let mut visible_sprites =[0;10];
        for sprite in self.oam {
            
            let h = match self.get_bit(self.lcdc,2){
                0 => 8,
                1 => 16
            };
            let y = self.get_bit(sprite, 0);
            let x = self.get_bit(sprite, 1);
            if (x != 0) && (self.ly + 16 >= y) && (self.ly < y + h){
                visible_sprites[i] = sprite;
                i+=1;
            }   
        }
        visible_sprites
    }

    fn tile_map(&self)->(u16,u16){
        match self.get_bit(self.lcdc, 3){
            0 =>(0x9800,0x9BFF),
            1 =>(0x9C00,0x9FFF)
        }
    }

    fn addressing_mode(&self,tile_index:u8)->u16{
        match self.get_bit(self.lcdc, 4){
            0 =>(0x1000 + (((tile_index as i8) as i16) * 16)) as u16,
            1 =>0x0 + (tile_index as u16) * 16
        }
    }
    
}