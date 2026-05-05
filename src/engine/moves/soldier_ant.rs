use crate::engine::{
  game::action::Action,
  grid::{Grid, coordinate::hex::Hex, piece::Piece},
  rules::{freedom_to_move_rule, one_hive_rule},
};

pub fn soldier_ant_moves(grid: &Grid, piece: &Piece, hex: &Hex) -> Vec<Action> {
  let mut start_trail = vec![*hex];
  let mut moves = Vec::new();

  soldier_ant_moves_it(grid, hex, hex, &mut start_trail, &mut moves);

  let mut actions = Vec::new();

  for m in moves {
    actions.push(Action {
      piece: *piece,
      from: *hex,
      to: m,
      in_hand: false,
      is_pillbug_special_move: false,
    });
  }

  actions
}

fn soldier_ant_moves_it(
  grid: &Grid,
  initital_hex: &Hex,
  hex: &Hex,
  trail: &mut Vec<Hex>,
  moves: &mut Vec<Hex>,
) {
  for neighbor in hex.neighbors() {
    if grid.is_hex_occupied(&neighbor) {
      continue;
    }
    if trail.contains(&neighbor) {
      continue;
    }
    if !one_hive_rule(grid, initital_hex, &neighbor) {
      continue;
    }
    if !freedom_to_move_rule(grid, hex, &neighbor) {
      continue;
    }

    trail.push(neighbor);

    if !moves.contains(&neighbor) {
      moves.push(neighbor)
    }

    soldier_ant_moves_it(grid, initital_hex, &neighbor, trail, moves);
  }
}
