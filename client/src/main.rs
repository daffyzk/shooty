use minifb::{Key, Window, WindowOptions, MouseMode};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const MAP_WIDTH: usize = 8;
const MAP_HEIGHT: usize = 8;
const TILE_SIZE: usize = 64;

const MAP: [&str; MAP_HEIGHT] = [
    "########",
    "#      #",
    "#  ##  #",
    "#      #",
    "#      #",
    "#  ##  #",
    "#      #",
    "########",
];

struct Player {
    x: f64,
    y: f64,
    angle: f64, // aiming direction
}

impl Player {
    fn move_dir(&mut self, dx: f64, dy: f64) {
        let new_x = self.x + dx;
        let new_y = self.y + dy;
        if !is_wall(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    fn aim_toward(&mut self, mx: f64, my: f64) {
        self.angle = (my - self.y).atan2(mx - self.x);
    }
}

fn is_wall(x: f64, y: f64) -> bool {
    let mx = x as usize / TILE_SIZE;
    let my = y as usize / TILE_SIZE;
    if mx >= MAP_WIDTH || my >= MAP_HEIGHT {
        return true;
    }
    MAP[my].as_bytes()[mx] == b'#'
}

fn draw_rect(buffer: &mut [u32], x: usize, y: usize, w: usize, h: usize, color: u32) {
    for dy in 0..h {
        for dx in 0..w {
            let px = x + dx;
            let py = y + dy;
            if px < WIDTH && py < HEIGHT {
                buffer[py * WIDTH + px] = color;
            }
        }
    }
}

fn draw_line(buffer: &mut [u32], x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
    let dx = (x1 as isize - x0 as isize).abs();
    let dy = -(y1 as isize - y0 as isize).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let (mut x, mut y) = (x0 as isize, y0 as isize);

    while x >= 0 && y >= 0 && (x as usize) < WIDTH && (y as usize) < HEIGHT {
        buffer[y as usize * WIDTH + x as usize] = color;
        if x == x1 as isize && y == y1 as isize {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

fn main() {
    let mut window = Window::new("shooty", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut player = Player {
        x: 100.0,
        y: 100.0,
        angle: 0.0,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // aiming
        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Discard) {
            player.aim_toward(mx as f64, my as f64);
        }

        // movement
        let mut dx = 0.0;
        let mut dy = 0.0;
        if window.is_key_down(Key::W) {
            dy -= 2.0;
        }
        if window.is_key_down(Key::S) {
            dy += 2.0;
        }
        if window.is_key_down(Key::A) {
            dx -= 2.0;
        }
        if window.is_key_down(Key::D) {
            dx += 2.0;
        }
        player.move_dir(dx, dy);

        buffer.fill(0x111111);

        // map drawing
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let color = if MAP[y].as_bytes()[x] == b'#' {
                    0xCCCCCC
                } else {
                    0x222222
                };
                draw_rect(&mut buffer, x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE, color);
            }
        }

        // player drawing
        let px = player.x as usize;
        let py = player.y as usize;
        draw_rect(&mut buffer, px.saturating_sub(2), py.saturating_sub(2), 4, 4, 0x00FF00);

        // scope drawing
        let dx = (player.x + player.angle.cos() * 30.0) as usize;
        let dy = (player.y + player.angle.sin() * 30.0) as usize;
        draw_line(&mut buffer, px, py, dx, dy, 0x00FF00);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

