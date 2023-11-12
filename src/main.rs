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
use crate::mpsc::TryRecvError;

fn main() {     

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();

    thread::spawn(move || {
        let mut screen = gpu::Screen::new();
        screen.render_screen(&rx1, &tx2, &tx3);

    });

    

    let mut cpu = cpu::CPU::new();
    let mut i = 0;
    loop{
        match rx2.try_recv() {
            Ok(data) => {cpu.bus.joypad = data.buttons},
            Err(TryRecvError::Disconnected) => {/* handle sender disconnected */}
            Err(TryRecvError::Empty) => {/* handle no data available yet */}
        }
        match rx3.try_recv() {
            Ok(data) => {cpu.jpad_interrupt = data},
            Err(TryRecvError::Disconnected) => {/* handle sender disconnected */}
            Err(TryRecvError::Empty) => {/* handle no data available yet */}
        }
        let _ = cpu.interrupts();
        print!("{} ",i+1);
        cpu.step();
        if cpu.bus.timer.cycles_counter >= 456 {
            if cpu.bus.ppu.ly == 144{
                let _= tx1.send(cpu.bus.ppu.video_buffer);
            }
            cpu.bus.ppu.ppu_step();

            
            cpu.bus.timer.cycles_counter %= 456;
        }
        println!(" ly: {}",cpu.bus.ppu.ly);

        println!(" Cycles : {}",cpu.bus.timer.cycles_counter);
        i+=1;
       
    }
}