use crate::engine::grid::geo::hex::Hex;
use crate::engine::grid::piece::Piece;
use crate::engine::grid::Grid;

struct QueenBee;

impl Piece for QueenBee {
    fn name(&self) -> String {
        String::from("Queen Bee")
    }

    fn available_moves(&self, self_hex: Hex, grid: Grid) -> Vec<Hex> {
        let neighbors = self_hex.neighbors();

        unimplemented!()
    }
}
