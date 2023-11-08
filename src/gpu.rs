use minifb::{Key, Window, WindowOptions, KeyRepeat};
use std::time::{Duration, Instant};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub struct Screen{
    window: Window,
    buffer: Vec<u32>,
    joypad: Joypad,
}

pub struct Joypad {
    directions_select: bool,
    buttons_select: bool,
    buttons: u8,

}

impl Screen{
    pub fn new() -> Screen{
        let buff: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let wind = Window::new(
            "Rust Game",
            WIDTH,
            HEIGHT,
            WindowOptions {
                resize: true,
                ..WindowOptions::default()
            },
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });
        let joyp = Joypad::new();
        Screen {
            window: wind,
            buffer: buff,
            joypad: joyp
        }
    }



    pub fn render_screen(&mut self, video_buffer: [u8;160*144]) {

    
        self.window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60fps
    
    
        let mut fps_timer = Instant::now();
        let mut fps_counter = 0;
    
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {

            let (width, height) = self.window.get_size();

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

            //read inputs
            self.window.get_keys_pressed(KeyRepeat::No).iter().for_each(|key|
                match key {
                    //button A
                    Key::A => Joypad::register_input(0, self.joypad.buttons_select, self.joypad.buttons),
                    Key::W => Joypad::register_input(0, self.joypad.buttons_select, self.joypad.buttons),

                    //button B
                    Key::B => Joypad::register_input(1, self.joypad.buttons_select, self.joypad.buttons),
                    Key::X => Joypad::register_input(1, self.joypad.buttons_select, self.joypad.buttons),

                    //button Select
                    Key::R => Joypad::register_input(2, self.joypad.buttons_select, self.joypad.buttons),
                    Key::V => Joypad::register_input(2, self.joypad.buttons_select, self.joypad.buttons),

                    //button Start
                    Key::E => Joypad::register_input(3, self.joypad.buttons_select, self.joypad.buttons),
                    Key::C => Joypad::register_input(3, self.joypad.buttons_select, self.joypad.buttons),

                    //button Right
                    Key::D => Joypad::register_input(0, self.joypad.directions_select, self.joypad.buttons),
                    Key::Right => Joypad::register_input(0, self.joypad.directions_select, self.joypad.buttons),

                    //button Left
                    Key::Q => Joypad::register_input(1, self.joypad.directions_select, self.joypad.buttons),
                    Key::Left => Joypad::register_input(1, self.joypad.directions_select, self.joypad.buttons),

                    //button Up
                    Key::Z => Joypad::register_input(2, self.joypad.directions_select, self.joypad.buttons),
                    Key::Up => Joypad::register_input(2, self.joypad.directions_select, self.joypad.buttons),


                    //button Down
                    Key::S => Joypad::register_input(3, self.joypad.directions_select, self.joypad.buttons),
                    Key::Down => Joypad::register_input(3, self.joypad.directions_select, self.joypad.buttons),



                    _ => (),
                }
            );
    
            
            // Update the window buffer and display the changes
            self.window.update_with_buffer(&self.buffer, width, height).unwrap();
        }
    }
    
}

impl Joypad {
    pub fn new() -> Joypad {
        let directions_bool = false;
        let buttons_bool = false;
        let buttons_matrix = 0b00111111;
        Joypad {
            directions_select: directions_bool,
            buttons_select: buttons_bool,
            buttons: buttons_matrix,
        }
    }

    pub fn register_input(button: u8, select: bool, mut buttons_matrix: u8) {
        if select {
            let clear_bit_mask = !(1 << button); // Create a mask with the bit to clear set to 0
            let new_buttons_matrix = buttons_matrix & clear_bit_mask;
            buttons_matrix = new_buttons_matrix;
        }
    }
}