use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

const MAP: [&str; 8] = [
    "########",
    "#      #",
    "#  ##  #",
    "#      #",
    "#      #",
    "#  ##  #",
    "#      #",
    "########",
];

fn main() {
    let mut window = Window::new("shooty game", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // TODO: handle player movement
        // TODO: perform raycasting

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
