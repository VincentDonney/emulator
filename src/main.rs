use minifb::{ Window, WindowOptions};
mod gpu;


const WIDTH: usize = 600;
const HEIGHT: usize = 480;

fn main() {
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
}
