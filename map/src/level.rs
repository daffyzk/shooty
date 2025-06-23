use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Coordinates { 
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Clone)]
pub struct Level { 
    pub map: BitVectorMap,
    pub tile_size: u8, 
    pub spawns: [Coordinates; 5],
}

impl Level { 
    /// Handles map collisions (for rays and players)
    pub fn new(map: BitVectorMap, tile_size: u8, spawns: [Coordinates; 5]) -> Self {
        Level { map, tile_size, spawns }
    }

    pub fn collision(self, x: usize, y: usize) -> bool {

        let mut crash: bool = false;

        if y >= self.map.height || x >= self.map.width {
            //bad error wooo
        } else {
            if self.get_tile(x, y).unwrap() == 1 {
                crash = true
            }
        }
        crash
    }

    /// Is the tile a wall???? If it is returns 1, otherwise 0
    fn get_tile(self, x: usize,y: usize) -> Option<u8>{
        self.map.get(x, y)
    }

    pub fn get_spawn(&self) -> Coordinates {
        let mut rng = thread_rng();
        self.spawns.choose(&mut rng).unwrap().clone()
    }
}

#[derive(Debug, Clone)]
pub struct BitVectorMap {
    rows: Vec<u64>,
    width: usize,
    height: usize,
}

impl BitVectorMap { 
    //  Why a bitvector? because I can! 
    //  ( also map information will be sent through to the player connection after
    //  the first websocket call, so it's good to save some bits of memory )
    pub fn new(rows: Vec<u64>, width: usize, height: usize) -> Self {
        BitVectorMap {rows, width, height}
    }

    pub fn get(&self, row: usize, col: usize) -> Option<u8> {
        if row < self.height && col < self.width {
            Some(((self.rows[row] >> col) & 1) as u8)
        }
        else {
            None
        }
    }
}
