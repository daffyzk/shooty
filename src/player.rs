use serde::{Deserialize, Serialize};
use socketioxide::socket::Sid;
use crate::Level;

#[derive(Debug, Serialize, Clone)]
pub struct PlayerState {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

#[derive(Debug, Clone, Default)]
pub struct Movement {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeyInput {
    pub key: String,
    pub pressed: bool,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: Sid,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub speed: f32,
    pub movement: Movement,
}

impl Player {
    pub fn new(id: Sid, x: f32, y: f32) -> Self {
        Player { id, x, y, angle: 0.0, speed: 0.15, movement: Movement::default() }
    }

    pub fn set_key(&mut self, key: &str, pressed: bool) {
        match key {
            "w" => self.movement.up = pressed,
            "s" => self.movement.down = pressed,
            "a" => self.movement.left = pressed,
            "d" => self.movement.right = pressed,
            _ => {}
        }
    }

    pub fn update(&mut self, level: &Level) {
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;

        if self.movement.up    { dy -= self.speed; }
        if self.movement.down  { dy += self.speed; }
        if self.movement.left  { dx -= self.speed; }
        if self.movement.right { dx += self.speed; }

        // normalize diagonal movement
        if dx != 0.0 && dy != 0.0 {
            let len = (dx * dx + dy * dy).sqrt();
            dx = dx / len * self.speed;
            dy = dy / len * self.speed;
        }

        // check X and Y separately so player can slide along walls
        let new_x = self.x + dx;
        if !level.collision(new_x as usize, self.y as usize) {
            self.x = new_x;
        }
        let new_y = self.y + dy;
        if !level.collision(self.x as usize, new_y as usize) {
            self.y = new_y;
        }
    }

    pub fn state(&self) -> PlayerState {
        PlayerState {
            id: self.id.to_string(),
            x: self.x,
            y: self.y,
            angle: self.angle,
        }
    }
}
