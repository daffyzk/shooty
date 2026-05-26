use socketioxide::socket::Sid;

use crate::player::PlayerState;
use crate::Player;
use crate::Level;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Game {
    pub scenario: Level,
    pub players: HashMap<Sid, Player>,
}

impl Game {
    pub fn new(scenario: Level) -> Self {
        Game { scenario, players: HashMap::new() }
    }

    pub fn join(&mut self, id: Sid) {
        let spawn = self.scenario.get_spawn();
        let player = Player::new(id, spawn.x as f32, spawn.y as f32);
        tracing::info!("Player joined: {:?} at ({}, {})", id, player.x, player.y);
        self.players.insert(id, player);
    }

    pub fn disconnect(&mut self, id: Sid) {
        self.players.remove(&id);
        tracing::info!("Player left: {:?}", id);
    }

    pub fn apply_action(&mut self, id: Sid, action: &str) {
        if let Some(player) = self.players.get_mut(&id) {
            player.apply_action(action);
        }
    }

    pub fn get_state(&self) -> Vec<PlayerState> {
        self.players.values().map(|p| p.state()).collect()
    }
}
