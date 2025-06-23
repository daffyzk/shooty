use map::Coordinates;
use socketioxide::socket::Sid;

#[derive(Debug)]
pub struct Player {
    pub id: Sid,
    pub location: Coordinates, // location coords in pixels
    pub angle: i8 // angle in radians
}

impl Player {
    pub fn new(id: Sid, location: Coordinates, angle: i8) -> Self {
        Player { id, location, angle }
    }

    // Example method to update the player's location
    pub fn update(&mut self, new_location: Coordinates, new_angle: i8) {
        self.location = new_location;
        self.angle = new_angle;
    }
}
