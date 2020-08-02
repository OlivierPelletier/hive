use crate::engine::grid::piece::PieceType;
use crate::grid::geo::cube::Cube;
use crate::grid::geo::hex::Hex;
use crate::grid::Grid;

pub mod hive;
pub mod moves;

pub fn available_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let piece = grid.find_top_piece(hex);
  let mut moves: Vec<Hex> = Vec::new();

  match piece.p_type {
    PieceType::QUEENBEE => {
      moves = moves::queen_moves(grid, hex);
    }
    PieceType::BEETLE => {
      moves = moves::beetle_moves(grid, hex);
    }
    PieceType::GRASSHOPPER => {
      moves = moves::grasshopper_moves(grid, hex);
    }
    PieceType::LADYBUG => {
      moves = moves::ladybug_moves(grid, hex);
    }
    PieceType::MOSQUITO => {}
    PieceType::SOLDIERANT => {
      moves = moves::soldier_ant_moves(grid, hex);
    }
    PieceType::SPIDER => {
      moves = moves::spider_moves(grid, hex);
    }
    PieceType::NONE => {
      moves == Vec::new();
    }
  }

  moves
}

pub fn one_hive_rule(grid: &Grid, from: &Hex, to: &Hex) -> bool {
  let is_valid;
  let temp_grid = Grid {
    grid: grid.grid.clone(),
  };

  if grid.is_hex_occupied(from) {
    is_valid = hive::one_hive_rule_grid_validation(grid)
      && hive::one_hive_rule_grid_validation(&temp_grid.move_piece_from_to(from, to));
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
