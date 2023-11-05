use minifb::{Key, Window, WindowOptions};
use crate::HEIGHT;
use crate::WIDTH;
use std::time::{Duration, Instant};




pub fn render_screen(mut window: Window, mut buffer: Vec<u32>, mut video_buffer: [u8;160*144]) {

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60fps


    let mut fps_timer = Instant::now();
    let mut fps_counter = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        //loop through buffer to change the pixels color
        let mut i = 0;
        while i < buffer.len() {
            let mut color_value = 0x00FFFFFF;   //default value
            match video_buffer[i] {
                0 => color_value = 0x00FFFFFF,  //white pixel
                1 => color_value = 0x00A9A9A9,  //light gray pixel
                2 => color_value = 0x00545454,  //dark gray pixel
                3 => color_value = 0x00000000,  //black pixel
                _ => println!("Wrong pixel value while updating the buffer\n"),
            }
            buffer[i] = color_value;
            i = i+1;
        }

        // Calculate fps
        fps_counter += 1;
        let elapsed = fps_timer.elapsed();
        if elapsed >= Duration::from_secs(1) {
            let fps = fps_counter as f64 / elapsed.as_secs_f64();
            window.set_title(&format!("Rust Game (FPS: {:.2})", fps));
            fps_counter = 0;
            fps_timer = Instant::now();
        }

        
        // Update the window buffer and display the changes
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}


