use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub struct Screen{
    window: Window,
    buffer: Vec<u32>
}

impl Screen{
    pub fn new() -> Screen{
        let buff: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let wind = Window::new(
            "Rust Game",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });
        Screen {
            window : wind,
            buffer : buff,
        }
    }

    pub fn render_screen(&mut self, video_buffer: [u8;160*144]) {
        self.window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60fps
        let mut fps_timer = Instant::now();
        let mut fps_counter = 0;
    
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
    
            //loop through buffer to change the pixels color
            let mut i = 0;
            while i < self.buffer.len() {
                let mut color_value = 0x00FFFFFF;   //default value
                match video_buffer[i] {
                    0 => color_value = 0x00FFFFFF,  //white pixel
                    1 => color_value = 0x00A9A9A9,  //light gray pixel
                    2 => color_value = 0x00545454,  //dark gray pixel
                    3 => color_value = 0x00000000,  //black pixel
                    _ => println!("Wrong pixel value while updating the buffer\n"),
                }
                self.buffer[i] = color_value;
                i = i+1;
            }
    
            // Calculate fps
            fps_counter += 1;
            let elapsed = fps_timer.elapsed();
            if elapsed >= Duration::from_secs(1) {
                let fps = fps_counter as f64 / elapsed.as_secs_f64();
                self.window.set_title(&format!("Rust Game (FPS: {:.2})", fps));
                fps_counter = 0;
                fps_timer = Instant::now();
            }
    
            
            // Update the window buffer and display the changes
            self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();
        }
    }
    
}