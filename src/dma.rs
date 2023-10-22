use crate::bus::bus_read;

struct Context{
    is_active:bool,
    byte:u8,
    val:u8,
    start_delay:u8
}

static CONTEXT:Context= Context{
    is_active:false,
    byte:0u8,
    val:0u8,
    start_delay:0u8
};

fn dma_start(start:u8){
    CONTEXT.is_active = true;
    CONTEXT.start_delay = 2;
    CONTEXT.val = start;
}

fn dma_tick(){
    if !CONTEXT.is_active{
        return;
    }
    if CONTEXT.start_delay != 0 {
        CONTEXT.start_delay -= 1;
        return;
    }
    ppu::ppu_oam_write(CONTEXT.byte as u16,bus_read((CONTEXT.val * 0x100 + CONTEXT.byte) as u16));
    CONTEXT.byte+=1;
    CONTEXT.is_active = CONTEXT.byte < 0xA0;
}

pub fn dma_transferring()->bool{
    return CONTEXT.is_active;
}