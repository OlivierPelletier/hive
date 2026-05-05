use crate::engine::{
  game::action::Action,
  grid::{Grid, coordinate::hex::Hex, piece::Piece},
  rules::{freedom_to_move_rule, one_hive_rule},
};

pub fn queen_bee_moves(grid: &Grid, piece: &Piece, hex: &Hex) -> Vec<Action> {
  let mut actions: Vec<Action> = Vec::new();

  for neighbor in hex.neighbors() {
    if grid.is_hex_occupied(&neighbor) {
      continue;
    }
    if !one_hive_rule(grid, hex, &neighbor) {
      continue;
    }
    if !freedom_to_move_rule(grid, hex, &neighbor) {
      continue;
    }

    actions.push(Action {
      piece: *piece,
      from: *hex,
      to: neighbor,
      in_hand: false,
      is_pillbug_special_move: false,
    });
  }

  actions
}
