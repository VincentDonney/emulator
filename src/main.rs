mod register;
mod cpu;
mod instruction;
mod timer;
mod launch;
mod ppu;
mod tile;
//mod gpu;


//const WIDTH: usize = 600;
//const HEIGHT: usize = 480;

fn main() {  
    /*   
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Rust Game",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    gpu::window_run(window, buffer);
    let mut cpu = cpu::CPU::new();
    while !cpu.is_halted{
        cpu.step();
    }
    */
    let mut cpu = cpu::CPU::new();
    
    while !cpu.is_halted{
        cpu.step();
        if cpu.bus.timer.cycles_counter >= 456 {
            cpu.bus.ppu.ppu_step();
            cpu.bus.timer.cycles_counter = cpu.bus.timer.cycles_counter % 456;
        }
        
    }
    
}
