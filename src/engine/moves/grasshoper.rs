use crate::engine::{
  grid::{
    coordinate::{cube::Cube, hex::Hex},
    Grid,
  },
  rules::one_hive_rule,
};

pub fn grasshopper_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let cube = hex.to_cube();
  let mut moves: Vec<Hex> = Vec::new();
  let mut possible_line: Vec<Cube> = Vec::new();

  for neighbor in hex.neighbors() {
    if grid.is_hex_occupied(&neighbor) {
      possible_line.push(neighbor.to_cube());
    }
  }

  for line in possible_line {
    if line.x == cube.x {
      let y_offset = line.y - cube.y;
      let mut line_cube;
      let mut count = 0;

      loop {
        count += 1;
        line_cube = Cube::xy(line.x, line.y + count * y_offset);

        if !grid.is_hex_occupied(&line_cube.to_axial()) {
          break;
        }
      }

      let line_hex = line_cube.to_axial();

      if one_hive_rule(grid, hex, &line_hex) {
        moves.push(line_hex);
      }
    } else if line.y == cube.y {
      let z_offset = line.z - cube.z;
      let mut line_cube;
      let mut count = 0;

      loop {
        count += 1;
        line_cube = Cube::yz(line.y, line.z + count * z_offset);

        if !grid.is_hex_occupied(&line_cube.to_axial()) {
          break;
        }
      }

      let line_hex = line_cube.to_axial();

      if one_hive_rule(grid, hex, &line_hex) {
        moves.push(line_hex);
      }
    } else {
      let x_offset = line.x - cube.x;
      let mut line_cube;
      let mut count = 0;

      loop {
        count += 1;
        line_cube = Cube::xz(line.x + count * x_offset, line.z);

        if !grid.is_hex_occupied(&line_cube.to_axial()) {
          break;
        }
      }

      let line_hex = line_cube.to_axial();

      if one_hive_rule(grid, hex, &line_hex) {
        moves.push(line_hex);
      }
    }
  }

  moves
}
