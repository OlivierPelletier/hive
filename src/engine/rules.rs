use crate::engine::grid::{
  coordinate::{cube::Cube, hex::Hex},
  Grid,
};

pub mod hive;

#[cfg(test)]
#[path = "../tests/rules_tests.rs"]
mod rules_tests;

pub fn one_hive_rule(grid: &Grid, from: &Hex, to: &Hex) -> bool {
  let is_valid;
  let mut temp_grid = Grid {
    grid: grid.grid.clone(),
  };
  temp_grid = temp_grid.move_piece_from_to(from, to);

  if grid.is_hex_occupied(from) {
    is_valid =
      hive::one_hive_rule_grid_validation(grid) && hive::one_hive_rule_grid_validation(&temp_grid);
  } else {
    is_valid = false;
  }

  is_valid
}

pub fn freedom_to_move_rule(grid: &Grid, from: &Hex, to: &Hex) -> bool {
  let is_accessible;

  if grid.is_hex_neighbor_of(to, from) {
    let cube = to.clone().to_cube();
    let cube_from = from.clone().to_cube();

    if cube.x == cube_from.x {
      let xz_offset = cube.z - cube_from.z;
      let xy_offset = cube.y - cube_from.y;

      let c1 = Cube {
        x: cube_from.x - xz_offset,
        y: cube_from.y,
        z: cube.z,
      };
      let c2 = Cube {
        x: cube_from.x - xy_offset,
        y: cube.y,
        z: cube_from.z,
      };

      let h1 = c1.to_axial();
      let h2 = c2.to_axial();

      is_accessible = !(grid.is_hex_occupied(&h1) && grid.is_hex_occupied(&h2));
    } else if cube.z == cube_from.z {
      let zx_offset = cube.x - cube_from.x;
      let zy_offset = cube.y - cube_from.y;

      let c1 = Cube {
        x: cube.x,
        y: cube_from.y,
        z: cube_from.z - zx_offset,
      };
      let c2 = Cube {
        x: cube_from.x,
        y: cube.y,
        z: cube_from.z - zy_offset,
      };

      let h1 = c1.to_axial();
      let h2 = c2.to_axial();

      is_accessible = !(grid.is_hex_occupied(&h1) && grid.is_hex_occupied(&h2));
    } else {
      let yx_offset = cube.x - cube_from.x;
      let yz_offset = cube.z - cube_from.z;

      let c1 = Cube {
        x: cube.x,
        y: cube_from.y - yx_offset,
        z: cube_from.z,
      };
      let c2 = Cube {
        x: cube_from.x,
        y: cube_from.y - yz_offset,
        z: cube.z,
      };

      let h1 = c1.to_axial();
      let h2 = c2.to_axial();

      is_accessible = !(grid.is_hex_occupied(&h1) && grid.is_hex_occupied(&h2));
    }
  } else {
    is_accessible = false;
  }
  is_accessible
}
