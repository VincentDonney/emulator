mod register;
mod cpu;
mod instruction;
mod timer;
mod launch;
mod ppu;
mod tile;
mod gpu;



fn main() {  
     

    let mut cpu = cpu::CPU::new();
    let mut i = 0;
    while i < 62000{
        let _ = cpu.interrupts();
        print!("{} ",i);
        cpu.step();
        if cpu.bus.timer.cycles_counter >= 456 {
            if cpu.bus.ppu.ly == 144{
                //send video buffer
            }
            cpu.bus.ppu.ppu_step();
            
            cpu.bus.timer.cycles_counter = cpu.bus.timer.cycles_counter % 456;
        }
        i+=1;
    }
}