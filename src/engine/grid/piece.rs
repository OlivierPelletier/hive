use crate::grid::geo::hex::Hex;
use crate::grid::Grid;

pub mod queen_bee;

pub trait Piece {
    fn name(&self) -> String;
    fn available_moves(&self, self_hex: Hex, grid: Grid) -> Vec<Hex>;
}
