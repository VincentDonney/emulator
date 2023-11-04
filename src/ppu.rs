use crate::tile::extract_tile;

pub(crate) struct PPU {
    lcdc:u8,
    lcds:u8,
    scy:u8,
    scx:u8,
    ly:u8,
    lyc:u8,
    dma:u8,
    bg_palette:u8,
    obp0:u8,
    obp1:u8,
    wy:u8,
    wx:u8,
    video_buffer:[u8;160*144],
    oam:[u8;0xA0],
    vram:[u8;0x2000],
    bg_tileset:[u8;256*256],
    win_tileset:[u8;256*256],

}

impl PPU{

    pub fn new()->PPU{
        PPU{
            lcdc: 0,
            lcds: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0x00,
            dma: 0,
            bg_palette: 0xFF,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
            video_buffer: [0u8;160*144],
            oam: [0u8;0xA0],
            vram: [0u8;0x2000],
            bg_tileset: [0u8;256*256],
            win_tileset: [0u8;256*256],
        }
    }

    fn ppu_step(&mut self){
        if self.get_bit(self.lcdc, 7) == 1 {
            self.render_line();
            self.ly+= 1;
        }   
    }

    fn get_bit(&self,byte:u8,bit:u8)->u8{
        byte >> bit & 1
    }

    pub fn oam_read(&self,address:u16)->u8{
        self.oam[(address &0xFF) as usize]
    }

    pub fn oam_write(&mut self,address:u16,val:u8){
        self.oam[(address & 0xFF) as usize] = val;
    }

    pub fn vram_write(&mut self,address:u16,value:u8){
        self.vram[(address & 0x1FFF) as usize] = value;
    }
    
    pub fn vram_read(&self,address:u16) -> u8{
        self.vram[(address & 0x1FFF) as usize]
    }

    pub fn lcd_read(&self,address:u16)->u8{
        match address{
            0xFF40 => self.lcdc,
            0xFF41 => self.lcds,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma,
            0xFF47 => self.bg_palette,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ =>panic!()
        }
    }

    pub fn lcd_write(&mut self,address:u16,val:u8){
        match address{
            0xFF40 => self.lcdc = val,
            0xFF41 => self.lcds = val,
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => self.ly = val,
            0xFF45 => self.lyc = val,
            0xFF46 => self.dma = val,
            0xFF47 => self.bg_palette = val,
            0xFF48 => self.obp0 = val,
            0xFF49 => self.obp1 = val,
            0xFF4A => self.wy = val,
            0xFF4B => self.wx = val,
            _ =>panic!()
        }
    }

    fn render_line(&mut self){
        let y = self.ly;
        for x in 0..160 {
            self.video_buffer[(x+160*y) as usize] = self.render_pixel(x,y);
        }
        self.render_sprites();
        
    }
    fn render_pixel(&self,x:u8,y:u8)->u8{
        //If BG/WIN Enabled
        if self.get_bit(self.lcdc, 0) != 0 {
            //If WIN Enabled
            if self.get_bit(self.lcdc, 5) != 0 {
                if self.pixel_in_window(x, y){
                    self.pixel_from_window(x, y)
                }else{
                    self.pixel_from_background(x,y)
                }
            }else{
                self.pixel_from_background(x,y)
            }
        }else {
            0
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
        let x = (x - (self.wx - 7)) as u32;
        let y = (y - self.wy) as u32;
        self.win_tileset[(x+256*y) as usize]
    }

    fn pixel_from_background(&self,x:u8,y:u8)->u8{
        let x = (x + self.scx) as u32 % 256;
        let y = (y + self.scy) as u32 % 256;
        self.bg_tileset[(x+256*y) as usize]
        
    }

    fn tilesets(&self,set_type:&str)->[u8;256*256]{
        let bit = match set_type{
            "bg" => 3,
            "win" => 6,
            &_ =>panic!()
        };
        let tilemap = self.tile_map(bit);
        let mut tile_line = 0;
        let mut background = [0u8;256*256];
        for i in tilemap.0..tilemap.1 {
            let tile_index =  self.vram_read(i);
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
                    background[(256*tile_line+256*k+l+8*i%32) as usize] = tile_pixels[k as usize][l as usize];
                }
            }
            
        }
        background        
    }

    fn is_visible(&self,x:u8,y:u8)->bool{
        let h = match self.get_bit(self.lcdc,2){
            0 => 8,
            1 => 16,
            _ =>panic!()
        };
        (x != 0) && (self.ly + 16 >= y) && (self.ly < y + h)
    }
        
    fn render_sprites(&mut self){
        if self.get_bit(self.lcdc, 1) == 1 {
            for i in 0..40 {
                let y = self.oam[i];
                let x = self.oam[i+1];
                let tile_index = self.oam[i+2];
                let tile_address = 0x0 + (tile_index as u16) * 16;
                let flags = self.oam[i+3];
                let ly = self.ly;
                let priority =self.get_bit(flags, 7);
                let y_flip = self.get_bit(flags, 6);
                let x_flip = self.get_bit(flags, 5);
                if self.is_visible(x, y){
                    match self.get_bit(self.lcdc, 2){
                        0 =>{
                            let mut tile = [0u8;16];
                            for j in 0..16 {
                                tile[j as usize] = self.vram_read(tile_address+j);
                            }
                            let tile_pixels = extract_tile(tile);
                            
                            for k in (x-8)..x{
                                if (priority == 0) || (priority == 1 && self.video_buffer[(ly*160+k) as usize] == 0){
                                    let mut pixel = tile_pixels[k as usize][(ly-y+16) as usize];
                                    //Y flip
                                    if y_flip == 1 {
                                        pixel = tile_pixels[k as usize][7-(ly-y+16) as usize];
                                    }
                                    //X flip
                                    if  x_flip == 1 {
                                        pixel = tile_pixels[7-k as usize][(ly-y+16) as usize];
                                    }
                                    if x_flip == 1 && y_flip == 1 {
                                        pixel = tile_pixels[7-k as usize][7-(ly-y+16) as usize];
                                    }
                                    self.video_buffer[(ly*160+k) as usize] = pixel;
                                }
                            }
                        },
                        1 =>{
                            let mut tile1 = [0u8;16];
                            let mut tile2 = [0u8;16];
                            for j in 0..16 {
                                tile1[j as usize] = self.vram_read(tile_address+j);
                            }
                            let tile1_pixels = extract_tile(tile1);
                            for j in 0..16 {
                                tile2[j as usize] = self.vram_read(tile_address+16+j);
                            }
                            let tile2_pixels = extract_tile(tile2);
                            let tile_pixels = self.double_tile(tile1_pixels, tile2_pixels);

                            for k in (x-8)..x{
                                if (priority == 0) || (priority == 1 && self.video_buffer[(ly*160+k) as usize] == 0){
                                    let mut pixel = tile_pixels[k as usize][(ly-y+16) as usize];
                                    //Y flip
                                    if y_flip == 1 {
                                        pixel = tile_pixels[k as usize][15-(ly-y+16) as usize];
                                    }
                                    //X flip
                                    if x_flip == 1 {
                                        pixel = tile_pixels[7-k as usize][(ly-y+16) as usize];
                                    }
                                    if x_flip == 1 && y_flip == 1 {
                                        pixel = tile_pixels[7-k as usize][15-(ly-y+16) as usize];
                                    }
                                    self.video_buffer[(ly*160+k) as usize] = pixel;
                                }
                            }
                        }
                        _ =>panic!()
                    }
                }
            }
        } 
    }

    fn double_tile(&self,tile1:[[u8; 8]; 8],tile2:[[u8; 8]; 8])->[[u8; 8]; 16]{
        let mut tile = [[0u8; 8]; 16];
        for i in 0..8 {
            tile[i] = tile1[i];
            tile[i + 8] = tile2[i];
        }
        tile
    }
    

    fn tile_map(&self,bit:u8)->(u16,u16){
        match self.get_bit(self.lcdc, bit){
            0 =>(0x9800,0x9BFF),
            1 =>(0x9C00,0x9FFF),
            _ =>panic!()
        }
    }

    fn addressing_mode(&self,tile_index:u8)->u16{
        match self.get_bit(self.lcdc, 4){
            0 =>(0x1000 + (((tile_index as i8) as i16) * 16)) as u16,
            1 =>0x0 + (tile_index as u16) * 16,
            _ =>panic!()
        }
    }
    
}