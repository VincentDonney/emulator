pub struct TimerContext{
    div:u16,
    tima:u8,
    tma:u8,
    tac:u8
}

impl TimerContext{
    pub fn new()-> TimerContext{
        TimerContext{
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0
        }
    }

    pub fn timer_tick(&mut self, cycles: u64){
        let tac_bit = (self.tac >> 2) & 0b11;
        let inc_rate = match tac_bit {
            0b00 => 1024,
            0b01 => 16,
            0b10=> 64,
            0b11 => 256,
            _=> 1024
        };
        let div_cycles = cycles / (inc_rate as u64);
        self.div = self.div.wrapping_add(div_cycles as u16);
        if self.tac & 0b00000001 != 0{
            for _ in 0..div_cycles{
                if self.tima == 0xFF{
                    self.tima = self.tma;
                }else{
                    self.tima += 1;
                }
            }
        }  
    }

    fn timer_read(&self,address:u16)->u8{
        match address{
            0xFF04 =>return self.div as u8,
            0xFF05 =>return self.tima,
            0xFF06 =>return self.tma,
            0xFF07 =>return self.tac, 
            _=> unreachable!("Invalid timer register address: 0x{:04X}", address),
        }
    }
    
    fn timer_write(&mut self,address:u16, val:u8){
        match address{
            0xFF04 =>{
                self.div = 0;
            },
            0xFF05 =>{
                self.tima= val;
            },
            0xFF06 =>{
                self.tma = val;
            },
            0xFF07 =>{
                self.tac = val;
            }
            _=> unreachable!("Invalid timer register address: 0x{:04X}", address),
        }
    }

}

