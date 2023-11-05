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
    while !cpu.is_halted{
        cpu.step();
    }
    
    let mut cpu = cpu::CPU::new();
    
    while !cpu.is_halted{
        cpu.step();
        if cpu.bus.timer.cycles_counter >= 456 {
            cpu.bus.ppu.ppu_step();
            cpu.bus.timer.cycles_counter = cpu.bus.timer.cycles_counter % 456;
        }
        
    }
    
}
