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
    let mut screen = gpu::Screen::new();
    let mut i = 0;
    loop {
        print!("{} ",i+1);
        cpu.step();
        if cpu.bus.timer.cycles_counter >= 456 {
            cpu.bus.ppu.ppu_step();
            if cpu.bus.ppu.ly == 144{
                screen.render_screen(cpu.bus.ppu.video_buffer);
            }
            cpu.bus.timer.cycles_counter = cpu.bus.timer.cycles_counter % 456;
        }
        i+=1;
    }
}