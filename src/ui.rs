use std::time::Duration;
use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window, pixels::Color, rect::Point};

use crate::bus;

enum PixelColors{
    White = 0xFFFFFFFF,
    Lightgray = 0xFFAAAAAA,
    Darkgray = 0xFF555555,
    Black = 0xFF000000
}

pub fn ui_init()->Result<(),String>{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("GBEmulator", 800, 600)
        .position_centered()
        .build()
        .expect("Could not initialize the window");
        
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::GRAY);
    canvas.draw_line(Point::new(0,100),Point::new(14,50));
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    
    'running: loop {
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                Event::KeyDown{keycode: Some(Keycode::Escape), ..} =>{
                    break 'running;
                },
                _ => {}
            }
        }
    }
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));  

    Ok(())
}

fn update_ui(canvas:Canvas<Window>){
    let mut tile_number = 0;
    let mut x = 0;
    let y = 0;

    let address:u16 =0x8000;
    for i in 0..24{
        for j in 0..16{
            display_tile(canvas,address,tile_number,x + i,y +j);
            tile_number+=1;
            x += 8;
        }
    }
}

fn display_tile(mut canvas:Canvas<Window>,address:u16, tile_number:u16,x:u32,y:u32){
    for i in (0..16).step_by(2) {
        let byte1 = bus::bus_read(address + tile_number*16 + i);
        let byte2 = bus::bus_read(address + tile_number*16 + i + 1);

        for bit in (0..8).rev(){
            let high_bit = !!(byte1 & (1 << bit)) << 1;
            let low_bit = !!(byte2 & (1 << bit));
            let color = high_bit | low_bit;
            canvas.set_draw_color(Color::WHITE);
            canvas.draw_point(Point::new((x+ (7-bit)) as i32,(y +(i as u32)/2)as i32)); 
        }
    }
}