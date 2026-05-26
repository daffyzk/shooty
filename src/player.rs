use serde::Serialize;
use socketioxide::socket::Sid;

#[derive(Debug, Serialize, Clone)]
pub struct PlayerState {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: Sid,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub speed: f32,
}

impl Player {
    pub fn new(id: Sid, x: f32, y: f32) -> Self {
        Player { id, x, y, angle: 0.0, speed: 0.3 } // grid units per action
    }

    pub fn apply_action(&mut self, action: &str) {
        match action {
            "moveup" => self.y -= self.speed,
            "movedown" => self.y += self.speed,
            "moveleft" => self.x -= self.speed,
            "moveright" => self.x += self.speed,
            _ => {}
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
