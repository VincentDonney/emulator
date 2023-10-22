

pub fn bus_read(address:u16)->u8{
    if address < 0x8000 {
        return cartridge::rom_read(address);
    }else if address < 0xA000{
        return ppu::ppu_vram_read(address);
    }else if address < 0xC000{
        return cartridge::rom_read(address);
    }else if address < 0xE000{
        //Working RAM (WRAM)
        return 0;
    }else if address < 0xFE00{
        //reserved echo RAM
        return 0;
    }
    else if address < 0xFEA0{
        //OAM
        if dma::dma_transferring(){
            return 0xFF;
        }
        return ppu::ppu_oam_read(address);
    }else if address < 0xFF00{
        //not usable
        return 0;
    }else if address < 0xFF80{
        //I/O registers
        return 0;
    }else if address < 0xFFFF{
        //HighRAM
        return 0;
    }else{
        //Interrupt Enable register IE
        return 0;
    }
}

fn bus_write(address:u16,val:u8){
    if address < 0x8000 {
        cartridge::rom_write(address,val);
    }else if address < 0xA000{
        return ppu::ppu_vram_write(address,val);
    }else if address < 0xC000{
        return cartridge::rom_write(address,val);
    }else if address < 0xE000{
        //Working RAM (WRAM)
        return;
    }else if address < 0xFE00{
        //reserved echo RAM
        return;
    }
    else if address < 0xFEA0{
        //OAM
        return ppu::ppu_oam_write(address,val);
    }else if address < 0xFF00{
        //not usable
        return;
    }else if address < 0xFF80{
        //I/O registers
        return;
    }else if address < 0xFFFF{
        //HighRAM
        return;
    }else{
        //Interrupt Enable register IE
        return;
    }
}