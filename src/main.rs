mod register;
mod cpu;
mod instruction;
mod timer;
mod launch;
mod ppu;
mod tile;
mod gpu;
use std::thread;
use std::sync::mpsc;

fn main() {  
     


    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    thread::spawn(move || {
        let mut screen = gpu::Screen::new();
        let video_buff = rx1.recv().unwrap();
        screen.render_screen(video_buff);
        let _= tx2.send(screen.joypad);
    });

    thread::spawn(move || {
        let mut cpu = cpu::CPU::new();
        let mut i = 0;
        while i < 16052 {
            let _= tx1.send(cpu.bus.ppu.video_buffer);
            let _joypad = rx2.recv().unwrap();
            let _ = cpu.interrupts();
            print!("{} ",i);
            cpu.step();
            if cpu.bus.timer.cycles_counter >= 456 {
                if cpu.bus.ppu.ly == 144{
                }
                cpu.bus.ppu.ppu_step();
                
                cpu.bus.timer.cycles_counter = cpu.bus.timer.cycles_counter % 456;
            }
            i+=1;
        }
    });
    
}