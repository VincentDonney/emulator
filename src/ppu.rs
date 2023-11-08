use crate::tile::extract_tile;


pub struct PPU {
    pub lcdc:u8,
    pub lcds:u8,
    pub scy:u8,
    pub scx:u8,
    pub ly:u8,
    pub lyc:u8,
    pub bg_palette:u8,
    pub obp0:u8,
    pub obp1:u8,
    pub wy:u8,
    pub wx:u8,
    pub video_buffer:[u8;160*144],
    pub oam:[u8;0xA0],
    pub vram:[u8;0x2000],
    pub bg_tileset:[u8;256*256],
    pub win_tileset:[u8;256*256],
    pub vblank_interrupt:u8,
    pub stat_interrupt:u8
}

impl PPU{

    pub fn new()->PPU{
        PPU{
            lcdc: 0x91,
            lcds: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0x00,
            bg_palette: 0xFF,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
            video_buffer: [0u8;160*144],
            oam: [0u8;0xA0],
            vram: [0;0x2000],
            bg_tileset: [0u8;256*256],
            win_tileset: [0u8;256*256],
            vblank_interrupt: 0,
            stat_interrupt: 0,
        }
    }

    pub fn ppu_step(&mut self){
        if self.ly == self.lyc && (self.lcds & 0x40) > 0{
            //raise a STAT interrupt
            self.stat_interrupt = 1;
        }

        if self.get_bit(self.lcdc, 7) == 1 {
            match self.ly{
                144 => {
                    self.ly += 1;
                    // raise VBlank interrupt
                    self.vblank_interrupt = 1;
                    
                }
                145..=153 => self.ly += 1,
                154 => self.ly = 0,
                _=>()
                
            }
            if self.ly < 144 {
                self.bg_tileset = self.tilesets("bg");
                self.win_tileset = self.tilesets("win");
                self.render_line();
                self.ly+= 1;
            }
        }   
    }

    

    fn get_bit(&self,byte:u8,bit:u8)->u8{
        (byte >> bit) & 1
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

    fn render_line(&mut self){
        let y = self.ly;
        for x in 0..160 {
            self.video_buffer[(x as u32+160*y as u32) as usize] = self.render_pixel(x,y);
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
        let mut pos_in_line = 0u32;
        //For each address in tilemap
        for i in tilemap.0..tilemap.1 {
            //Read tile index from VRAM
            let tile_index =  self.vram_read(i);
            //Use corresponding addressing mode to get tile address in VRAM
            let tile_address = self.addressing_mode(tile_index);
            let mut tile = [0u8;16];
            //Build tile
            for j in 0..16 {
                tile[j as usize] = self.vram_read(tile_address+j);
            }
            //Get pixels from tile
            let tile_pixels = extract_tile(tile);
            
            if pos_in_line%32 == 0 && pos_in_line != 0{
                tile_line += 8;
            }
            //For each  line
            for k in 0..8 {
                //For each pixel
                for l in 0..8 {
                    background[(256*tile_line+256*k+l+8*(pos_in_line%32)) as usize] = tile_pixels[k as usize][l as usize];
                }
            }
            pos_in_line += 1;
            
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
                                if (priority == 0) || (priority == 1 && self.video_buffer[(ly*160+k) as usize] == 0) && !self.pixel_in_window(k, ly-y+16){
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