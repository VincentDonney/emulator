use minifb::{Key, Window, WindowOptions, KeyRepeat};
use std::{time::{Duration, Instant}, sync::mpsc::Receiver};
use crate::mpsc::*;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub struct Screen{
    window: Window,
    buffer: Vec<u32>,
    pub joypad: Joypad,
}

#[derive(Clone)]
pub struct Joypad {
    pub buttons: u8,
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



    pub fn render_screen(&mut self, rx1: &Receiver<[u8;160*144]>, tx2: &Sender<Joypad>, tx3: &Sender<bool>) {

    
        self.window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60fps
    
    
        let mut fps_timer = Instant::now();
        let mut fps_counter = 0;
        let mut video_buffer: [u8; 160*144];

    
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            match rx1.try_recv() {
                Ok(data) => {
                    video_buffer = data;
                    
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
                },
                Err(TryRecvError::Disconnected) => {/* handle sender disconnected */}
                Err(TryRecvError::Empty) => {/* handle no data available yet */}

            }


            let mut button_pressed: bool = false;
            
    
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
                    Key::A => {self.joypad.buttons = Joypad::register_input(0, String::from("buttons"), self.joypad.clone()); button_pressed = true},
                    Key::W => {self.joypad.buttons = Joypad::register_input(0, String::from("buttons"), self.joypad.clone()); button_pressed = true},

                    //button B
                    Key::B => {self.joypad.buttons = Joypad::register_input(1, String::from("buttons"), self.joypad.clone()); button_pressed = true},
                    Key::X => {self.joypad.buttons = Joypad::register_input(1, String::from("buttons"), self.joypad.clone()); button_pressed = true},

                    //button Select
                    Key::R => {self.joypad.buttons = Joypad::register_input(2, String::from("buttons"), self.joypad.clone()); button_pressed = true},
                    Key::V => {self.joypad.buttons = Joypad::register_input(2, String::from("buttons"), self.joypad.clone()); button_pressed = true},

                    //button Start
                    Key::E => {self.joypad.buttons = Joypad::register_input(3, String::from("buttons"), self.joypad.clone()); button_pressed = true},
                    Key::C => {self.joypad.buttons = Joypad::register_input(3, String::from("buttons"), self.joypad.clone()); button_pressed = true},

                    //button Right
                    Key::D => {self.joypad.buttons = Joypad::register_input(0, String::from("dpad"), self.joypad.clone()); button_pressed = true},
                    Key::Right => {self.joypad.buttons = Joypad::register_input(0, String::from("dpad"), self.joypad.clone()); button_pressed = true},

                    //button Left
                    Key::Q => {self.joypad.buttons = Joypad::register_input(1, String::from("dpad"), self.joypad.clone()); button_pressed = true},
                    Key::Left => {self.joypad.buttons = Joypad::register_input(1, String::from("dpad"), self.joypad.clone()); button_pressed = true},

                    //button Up
                    Key::Z => {self.joypad.buttons = Joypad::register_input(2, String::from("dpad"), self.joypad.clone()); button_pressed = true},
                    Key::Up => {self.joypad.buttons = Joypad::register_input(2, String::from("dpad"), self.joypad.clone()); button_pressed = true},

                    //button Down
                    Key::S => {self.joypad.buttons = Joypad::register_input(3, String::from("dpad"), self.joypad.clone()); button_pressed = true},
                    Key::Down => {self.joypad.buttons = Joypad::register_input(3, String::from("dpad"), self.joypad.clone()); button_pressed = true},



                    _ => (),
                }
            );
            if button_pressed {
                let _= tx2.send(self.joypad.clone());
                let _= tx3.send(true);
                self.joypad.buttons = 0b00111111;
            }
            
            // Update the window buffer and display the changes
            self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();
        }
    }
    
}

impl Joypad {
    pub fn new() -> Joypad {
        let buttons_matrix = 0b00111111;
        Joypad {
            buttons: buttons_matrix,

        }
    }

    pub fn register_input(button: u8, mode: String, mut joypad: Joypad) -> u8 {
        match mode.as_str() {
            "dpad" => {
                if (joypad.buttons & (1 << 5) != 0){
                    let clear_bit_mask = !(1 << button); // Create a mask with the bit to clear set to 0
                    joypad.buttons = joypad.buttons & clear_bit_mask;
                    joypad.buttons = joypad.buttons & !(1 << 4);
                }
            },
            "buttons" => {
                if (joypad.buttons & (1 << 4) != 0){
                    let clear_bit_mask = !(1 << button); // Create a mask with the bit to clear set to 0
                    joypad.buttons = joypad.buttons & clear_bit_mask;
                    joypad.buttons = joypad.buttons & !(1 << 5);
                }
            },
            _ => panic!("Wrong mode selection on joypad")
            
        }
        joypad.buttons
    }
}