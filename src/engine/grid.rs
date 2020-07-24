use crate::grid::geo::hex::Hex;
use crate::grid::piece::Piece;
use std::collections::HashMap;

pub mod geo;
pub mod piece;

pub fn init_grid() -> Grid {
    let mut grid: HashMap<Hex, Vec<dyn Piece>> = HashMap::new();
    let hex_zero = Hex { q: 0, r: 0 };

    grid.insert(hex_zero, Vec::new());

    return Grid { grid };
}

pub struct Grid {
    pub grid: HashMap<Hex, Vec<dyn Piece>>,
}
