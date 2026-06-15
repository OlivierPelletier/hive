use crate::engine::{
  game::action::Action,
  grid::{Grid, coordinate::hex::Hex, piece::Piece},
  rules::{freedom_to_move_rule, one_hive_rule},
};

pub fn ladybug_moves(grid: &Grid, piece: &Piece, hex: &Hex) -> Vec<Action> {
  let mut moves = Vec::new();
  let mut trail = vec![*hex];

  ladybug_moves_it(grid, hex, hex, &mut trail, &mut moves);

  let mut actions = Vec::new();
  for m in moves {
    actions.push(Action {
      piece: *piece,
      from: *hex,
      to: m,
      in_hand: false,
      is_pillbug_special_move: false,
    })
  }

  actions
}

fn ladybug_moves_it(
  grid: &Grid,
  hex: &Hex,
  initial_hex: &Hex,
  trail: &mut Vec<Hex>,
  moves: &mut Vec<Hex>,
) {
  for neighbor in hex.neighbors() {
    if trail.contains(&neighbor) {
      continue;
    }
    if !one_hive_rule(grid, initial_hex, &neighbor) {
      continue;
    }
    if !freedom_to_move_rule(grid, hex, &neighbor) {
      continue;
    }
    if trail.len() == 1 && !grid.is_hex_occupied(&neighbor) {
      continue;
    }
    if trail.len() == 2 && !grid.is_hex_occupied(&neighbor) {
      continue;
    }
    if trail.len() == 3 && grid.is_hex_occupied(&neighbor) {
      continue;
    }

    trail.push(neighbor);

    if trail.len() == 4 {
      if !moves.contains(&neighbor) {
        moves.push(neighbor);
      }
    } else {
      ladybug_moves_it(grid, &neighbor, initial_hex, trail, moves);
    }

    trail.pop();
  }
}
