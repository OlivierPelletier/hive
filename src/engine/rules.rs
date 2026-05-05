use crate::engine::game::action::Action;
use crate::engine::grid::piece::{PieceColor, PieceType};
use crate::engine::grid::{
  Grid,
  coordinate::{cube::Cube, hex::Hex},
};

pub mod hive;

#[cfg(test)]
#[path = "../tests/rules_tests.rs"]
mod rules_tests;

pub fn one_hive_rule(grid: &Grid, from: &Hex, to: &Hex) -> bool {
  if !grid.is_hex_occupied(from) {
    return false;
  }

  let mut after_move_grid = Grid {
    grid: grid.grid.clone(),
  };
  after_move_grid.move_piece_from_to(*from, *to);
  let mut without_piece_grid = Grid {
    grid: grid.grid.clone(),
  };
  without_piece_grid.remove_top_piece_from_hex(*from);

  hive::one_hive_rule_grid_validation(grid)
    && hive::one_hive_rule_grid_validation(&after_move_grid)
    && hive::one_hive_rule_grid_validation(&without_piece_grid)
}

pub fn freedom_to_move_rule(grid: &Grid, from: &Hex, to: &Hex) -> bool {
  if !grid.is_hex_neighbor_of(to, from) {
    return false;
  }

  let to_stack_size = grid.get_stack_size(to);

  let cube = Cube::from(*to);
  let cube_from = Cube::from(*from);
  let h1;
  let h2;

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

    h1 = c1.into();
    h2 = c2.into();
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

    h1 = c1.into();
    h2 = c2.into();
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

    h1 = c1.into();
    h2 = c2.into();
  }

  !(grid.get_stack_size(&h1) > to_stack_size && grid.get_stack_size(&h2) > to_stack_size)
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

pub fn pillbug_special_move_rule(grid: &Grid, from: &Hex, actions_history: &[Action]) -> bool {
  let Some(piece) = grid.find_top_piece(from) else {
    return false;
  };

  let Some(last) = actions_history.last() else {
    return false;
  };

  if piece.p_color == last.piece.p_color && *from == last.to && last.is_pillbug_special_move {
    return true;
  }

  if actions_history.len() < 2 {
    return false;
  }

  let Some(second_from_last) = actions_history.get(actions_history.len() - 2) else {
    return false;
  };

  if piece.p_color == second_from_last.piece.p_color
    && *from == second_from_last.to
    && second_from_last.is_pillbug_special_move
  {
    return true;
  }

  false
}
