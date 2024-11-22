use crate::engine::grid::{
  coordinate::{cube::Cube, hex::Hex},
  piece::{PieceColor, PieceType},
  Grid,
};

pub mod hive;

pub fn one_hive_rule(grid: &Grid, from: &Hex, to: &Hex) -> bool {
  let mut is_valid = false;
  let mut temp_grid = Grid {
    grid: grid.grid.clone(),
  };
  temp_grid.move_piece_from_to(*from, *to);

  if grid.is_hex_occupied(from) {
    is_valid =
      hive::one_hive_rule_grid_validation(grid) && hive::one_hive_rule_grid_validation(&temp_grid);
  }

  is_valid
}

pub fn freedom_to_move_rule(grid: &Grid, from: &Hex, to: &Hex) -> bool {
  let is_accessible;

  if grid.is_hex_neighbor_of(to, from) {
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
  } else {
    is_accessible = false;
  }
  is_accessible
}

pub fn queen_surrounded_rule(grid: &Grid, color: PieceColor) -> bool {
  let mut is_queen_surrounded = false;

  for hex_pieces in &grid.grid {
    for piece in hex_pieces.1 {
      if piece.p_type == PieceType::QUEENBEE
        && piece.p_color == color
        && grid.is_hex_surrounded(hex_pieces.0)
      {
        is_queen_surrounded = true
      }
    }
  }

  is_queen_surrounded
}
#[cfg(test)]
mod tests {
  use super::*;
  use crate::engine::grid::piece::Piece;

  /*
    __    W MSQT  B LDBG    __      __    W QBEE  W SANT
   -3,-1   -2,-1   -1,-1   0,-1    1,-1    2,-1    3,-1
      B QBEE  B GRHP  B BETL  W BETL  W GRHP  W GRHP    __
       -3,0    -2,0    -1,0     0,0     1,0     2,0     3,0
          B SANT  B BETL    __    W SPDR    __      __      __
           -3,1    -2,1    -1,1     0,1     1,1     2,1     3,1
  */
  fn initialize_grid() -> Grid {
    let mut grid = Grid::new();

    grid.place_piece_to_hex(Piece::queen_bee().white(), Hex::new(2, -1));
    grid.place_piece_to_hex(Piece::queen_bee().black(), Hex::new(-3, -0));
    grid.place_piece_to_hex(Piece::beetle().white(), Hex::new(0, 0));
    grid.place_piece_to_hex(Piece::beetle().black(), Hex::new(-2, 1));
    grid.place_piece_to_hex(Piece::beetle().black(), Hex::new(-1, 0));
    grid.place_piece_to_hex(Piece::grasshopper().white(), Hex::new(1, 0));
    grid.place_piece_to_hex(Piece::grasshopper().white(), Hex::new(2, 0));
    grid.place_piece_to_hex(Piece::grasshopper().black(), Hex::new(-2, 0));
    grid.place_piece_to_hex(Piece::soldier_ant().black(), Hex::new(-3, 1));
    grid.place_piece_to_hex(Piece::soldier_ant().white(), Hex::new(3, -1));
    grid.place_piece_to_hex(Piece::spider().white(), Hex::new(0, 1));
    grid.place_piece_to_hex(Piece::ladybug().black(), Hex::new(-1, -1));
    grid.place_piece_to_hex(Piece::mosquito().white(), Hex::new(-2, -1));

    grid
  }

  /*
    __    B BETL  W BETL
   -3,-1   -2,-1   -1,-1
      W GRHP  W QBEE  B QBEE
       -3,0    -2,0    -1,0
          W SANT  W SANT    __
           -3,1    -2,1    -1,1
  */
  fn initialize_white_losing_grid() -> Grid {
    let mut grid = Grid::new();

    grid.place_piece_to_hex(Piece::queen_bee().white(), Hex::new(-2, 0));
    grid.place_piece_to_hex(Piece::grasshopper().white(), Hex::new(-3, 0));
    grid.place_piece_to_hex(Piece::queen_bee().black(), Hex::new(-1, 0));
    grid.place_piece_to_hex(Piece::beetle().black(), Hex::new(-2, -1));
    grid.place_piece_to_hex(Piece::beetle().white(), Hex::new(-1, -1));
    grid.place_piece_to_hex(Piece::soldier_ant().white(), Hex::new(-3, 1));
    grid.place_piece_to_hex(Piece::soldier_ant().white(), Hex::new(-2, 1));

    grid
  }

  /*
    __    B BETL  W BETL
   -3,-1   -2,-1   -1,-1
      W GRHP  B QBEE  W QBEE
       -3,0    -2,0    -1,0
          W SANT  W SANT    __
           -3,1    -2,1    -1,1
  */
  fn initialize_black_losing_grid() -> Grid {
    let mut grid = Grid::new();

    grid.place_piece_to_hex(Piece::queen_bee().black(), Hex::new(-2, 0));
    grid.place_piece_to_hex(Piece::grasshopper().white(), Hex::new(-3, 0));
    grid.place_piece_to_hex(Piece::queen_bee().white(), Hex::new(-1, 0));
    grid.place_piece_to_hex(Piece::beetle().black(), Hex::new(-2, -1));
    grid.place_piece_to_hex(Piece::beetle().white(), Hex::new(-1, -1));
    grid.place_piece_to_hex(Piece::soldier_ant().white(), Hex::new(-3, 1));
    grid.place_piece_to_hex(Piece::soldier_ant().white(), Hex::new(-2, 1));

    grid
  }

  #[test]
  fn given_filled_grid_when_one_hive_rule_with_correct_hex_should_return_true() {
    let grid = initialize_grid();

    assert!(one_hive_rule(&grid, &Hex::new(-1, -1), &Hex::new(-3, -1)))
  }

  #[test]
  fn given_filled_grid_when_one_hive_rule_with_correct_hex_should_return_false() {
    let grid = initialize_grid();

    assert!(!one_hive_rule(&grid, &Hex::new(0, 0), &Hex::new(1, -1)))
  }

  #[test]
  fn given_filled_grid_when_queen_surrended_rule_with_white_should_return_false() {
    let grid = initialize_grid();

    assert!(!queen_surrounded_rule(&grid, PieceColor::WHITE))
  }

  #[test]
  fn given_filled_grid_when_queen_surrended_rule_with_black_should_return_false() {
    let grid = initialize_grid();

    assert!(!queen_surrounded_rule(&grid, PieceColor::BLACK))
  }

  #[test]
  fn given_filled_white_loosing_grid_when_queen_surrended_rule_with_white_should_return_true() {
    let grid = initialize_white_losing_grid();

    assert!(queen_surrounded_rule(&grid, PieceColor::WHITE))
  }

  #[test]
  fn given_filled_white_loosing_grid_when_queen_surrended_rule_with_black_should_return_false() {
    let grid = initialize_white_losing_grid();

    assert!(!queen_surrounded_rule(&grid, PieceColor::BLACK))
  }

  #[test]
  fn given_filled_black_loosing_grid_when_queen_surrended_rule_with_white_should_return_false() {
    let grid = initialize_black_losing_grid();

    assert!(!queen_surrounded_rule(&grid, PieceColor::WHITE))
  }

  #[test]
  fn given_filled_black_loosing_grid_when_queen_surrended_rule_with_black_should_return_true() {
    let grid = initialize_black_losing_grid();

    assert!(queen_surrounded_rule(&grid, PieceColor::BLACK))
  }
}
