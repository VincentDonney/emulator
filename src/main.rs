mod register;
mod cpu;
mod instruction;
mod timer;
mod launch;
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
    let mut i = 0;
    while i<100 {
        cpu.step();
        i = i+1;
    }
    
}
