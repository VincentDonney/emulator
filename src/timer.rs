pub struct Timer{
    div:u16,
    tima:u8,
    tma:u8,
    tac:u8,
    pub cycles_counter:u32,
    div_counter:u32,
    tima_counter:u32,
    pub timer_interrupt:u8,
}

impl Timer{
    pub fn new()-> Timer{
        Timer{
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            cycles_counter: 0,
            div_counter: 0,
            tima_counter: 0,
            timer_interrupt:0,
        }
    }

    pub fn timer_tick(&mut self, cycles: u32,should_cycles:u8){
        if (should_cycles >> 7) & 1 == 1 {
            self.cycles_counter += cycles;
        }
        if (self.tac >> 2) & 1 != 0{
            self.tima_counter += cycles;
            let inc_rate = match self.tac & 0x03 {
                0 => 1024, //4096 kHz
                1 => 16, // 262 144 kHz
                2=> 64, //65 536 kHz
                3 => 256, //16 384 kHz
                _=> unreachable!()
            };
            if self.tima_counter >= inc_rate {
                if self.tima == 0xFF{
                    self.tima = self.tma;
                    //Trigger timer interrupt
                    self.timer_interrupt = 1;
                }else{
                    let _ = self.tima.wrapping_add(1);
                }
                self.tima_counter -= inc_rate;
            }
        }
        self.div_counter += cycles;
        if self.div_counter >= 256 {
            self.div = self.div.wrapping_add(1);
            self.div_counter -= 256;
        }   
    }

    pub fn timer_read(&self,address:u16)->u8{
        match address{
            0xFF04 =>self.div as u8,
            0xFF05 =>self.tima,
            0xFF06 =>self.tma,
            0xFF07 =>self.tac, 
            _=> unreachable!("Invalid timer register address: 0x{:04X}", address),
        }
    }
    
    pub fn timer_write(&mut self,address:u16, val:u8){
        match address{
            0xFF04 =>self.div = 0,
            0xFF05 =>self.tima= val,
            0xFF06 =>self.tma = val,
            0xFF07 =>self.tac = val,
            _=> unreachable!("Invalid timer register address: 0x{:04X}", address),
        }
    }

}

