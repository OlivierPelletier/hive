use crate::engine::{
  grid::{coordinate::hex::Hex, Grid},
  rules::{freedom_to_move_rule, one_hive_rule},
};

pub fn beetle_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let mut moves: Vec<Hex> = Vec::new();

  for neighbor in hex.neighbors() {
    if one_hive_rule(grid, hex, &neighbor)
      && ((!grid.is_hex_occupied(hex) && freedom_to_move_rule(grid, hex, &neighbor))
        || grid.is_hex_occupied(hex))
    {
      moves.push(neighbor);
    }
  }

  moves
}
