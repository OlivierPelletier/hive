use crate::engine::{
  game::action::Action,
  grid::{
    Grid,
    coordinate::{cube::Cube, hex::Hex},
    piece::Piece,
  },
  rules::{freedom_to_move_rule, one_hive_rule},
};

pub fn pillbug_moves(
  grid: &Grid,
  piece: &Piece,
  hex: &Hex,
  actions_history: &[Action],
) -> Vec<Action> {
  let mut actions: Vec<Action> = Vec::new();
  let mut potential_hexes: Vec<Hex> = Vec::new();

  // check for 1 hex move around
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

  // look for potential pieces to flip over
  for neighbor in hex.neighbors() {
    // skip if hex is empty or stacked
    if grid.get_stack_size(&neighbor) != 1 {
      continue;
    }

    // skip if piece was just move by opponent
    if let Some(last_opposite_action) = actions_history
      .iter()
      .rev()
      .find(|&action| action.piece.p_color != piece.p_color)
      && last_opposite_action.to == neighbor
    {
      continue;
    }

    potential_hexes.push(neighbor)
  }

  // check if flipping potential piece a legal move
  for potential_hex in potential_hexes {
    let Some(potential_piece) = grid.find_top_piece(&potential_hex) else {
      continue;
    };
    if !one_hive_rule(grid, &potential_hex, hex) {
      continue;
    }
    if !freedom_to_move_rule(grid, &potential_hex, hex) {
      continue;
    }

    let cube = Cube::from(*hex);
    let potential_cube = Cube::from(potential_hex);

    for neighbor in hex.neighbors() {
      if neighbor == potential_hex {
        continue;
      }
      if grid.is_hex_occupied(&neighbor) {
        continue;
      }

      let neighbor_cube = Cube::from(neighbor);

      // make sure potential piece, main piece and neighbor is on the same line
      if (cube.x == neighbor_cube.x && cube.x == potential_cube.x)
        || (cube.y == neighbor_cube.y && cube.y == potential_cube.y)
        || (cube.z == neighbor_cube.z && cube.z == potential_cube.z)
      {
        actions.push(Action {
          piece: *potential_piece,
          from: Hex::from(potential_cube),
          to: neighbor,
          in_hand: false,
          is_pillbug_special_move: true,
        });
        break;
      }
    }
  }

  actions
}
