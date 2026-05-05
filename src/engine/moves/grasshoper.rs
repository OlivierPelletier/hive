use crate::engine::{
  game::action::Action,
  grid::{
    Grid,
    coordinate::{cube::Cube, hex::Hex},
    piece::Piece,
  },
  rules::one_hive_rule,
};

pub fn grasshopper_moves(grid: &Grid, piece: &Piece, hex: &Hex) -> Vec<Action> {
  let cube = Cube::from(*hex);
  let mut actions = Vec::new();
  let mut possible_line = Vec::new();

  for neighbor in hex.neighbors() {
    if grid.is_hex_occupied(&neighbor) {
      possible_line.push(Cube::from(neighbor));
    }
  }

  for line in possible_line {
    if line.x == cube.x {
      let y_offset = line.y - cube.y;
      let mut line_hex;
      let mut count = 0;

      loop {
        count += 1;
        line_hex = Cube::xy(line.x, line.y + count * y_offset).into();

        if !grid.is_hex_occupied(&line_hex) {
          break;
        }
      }

      if one_hive_rule(grid, hex, &line_hex) {
        actions.push(Action {
          piece: *piece,
          from: *hex,
          to: line_hex,
          in_hand: false,
          is_pillbug_special_move: false,
        });
      }
    } else if line.y == cube.y {
      let z_offset = line.z - cube.z;
      let mut line_hex;
      let mut count = 0;

      loop {
        count += 1;
        line_hex = Cube::yz(line.y, line.z + count * z_offset).into();

        if !grid.is_hex_occupied(&line_hex) {
          break;
        }
      }

      if one_hive_rule(grid, hex, &line_hex) {
        actions.push(Action {
          piece: *piece,
          from: *hex,
          to: line_hex,
          in_hand: false,
          is_pillbug_special_move: false,
        });
      }
    } else {
      let x_offset = line.x - cube.x;
      let mut line_hex;
      let mut count = 0;

      loop {
        count += 1;
        line_hex = Cube::xz(line.x + count * x_offset, line.z).into();

        if !grid.is_hex_occupied(&line_hex) {
          break;
        }
      }

      if one_hive_rule(grid, hex, &line_hex) {
        actions.push(Action {
          piece: *piece,
          from: *hex,
          to: line_hex,
          in_hand: false,
          is_pillbug_special_move: false,
        });
      }
    }
  }

  actions
}
