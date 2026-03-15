use crate::engine::grid::piece::{PieceColor, PieceType};
use crate::engine::grid::{
  coordinate::{cube::Cube, hex::Hex},
  Grid,
};

pub mod hive;

#[cfg(test)]
#[path = "../tests/rules_tests.rs"]
mod rules_tests;

pub fn one_hive_rule(grid: &Grid, from: &Hex, to: &Hex) -> bool {
  if !grid.is_hex_occupied(from) {
    return false;
  }

  let mut temp_grid = Grid {
    grid: grid.grid.clone(),
  };
  temp_grid.move_piece_from_to(*from, *to);

  hive::one_hive_rule_grid_validation(grid) && hive::one_hive_rule_grid_validation(&temp_grid)
}

pub fn freedom_to_move_rule(grid: &Grid, from: &Hex, to: &Hex) -> bool {
  if !grid.is_hex_neighbor_of(to, from) {
    return false;
  }

  let is_accessible;
  let cube = Cube::from(*to);
  let cube_from = Cube::from(*from);

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

    let h1 = c1.into();
    let h2 = c2.into();

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

    let h1 = c1.into();
    let h2 = c2.into();

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

    let h1 = c1.into();
    let h2 = c2.into();

    is_accessible = !(grid.is_hex_occupied(&h1) && grid.is_hex_occupied(&h2));
  }
  is_accessible
}

pub fn queen_surrounded_rule(grid: &Grid, color: PieceColor) -> bool {
  let mut is_queen_surrounded = false;

  for hex_pieces in &grid.grid {
    for piece in hex_pieces.1 {
      if piece.p_type != PieceType::QUEENBEE {
        continue;
      }

      if piece.p_color != color {
        continue;
      }

      if !grid.is_hex_surrounded(hex_pieces.0) {
        continue;
      }

      is_queen_surrounded = true
    }
  }

  is_queen_surrounded
}
