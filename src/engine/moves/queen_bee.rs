use crate::grid::geo::hex::Hex;
use crate::grid::Grid;
use crate::rules::{freedom_to_move_rule, one_hive_rule};

pub fn queen_bee_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let mut moves: Vec<Hex> = Vec::new();

  for neighbor in hex.neighbors() {
    if one_hive_rule(grid, hex, &neighbor)
      && freedom_to_move_rule(grid, hex, &neighbor)
      && !grid.is_hex_occupied(&neighbor)
    {
      moves.push(neighbor);
    }
  }

  moves
}
