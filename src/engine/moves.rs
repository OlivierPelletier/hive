use std::collections::HashSet;

use crate::engine::{
  grid::{
    coordinate::hex::Hex,
    piece::{PieceColor, PieceType},
    Grid,
  },
  moves::{
    beetle::beetle_moves, grasshoper::grasshopper_moves, ladybug::ladybug_moves,
    mosquito::mosquito_moves, queen_bee::queen_bee_moves, soldier_ant::soldier_ant_moves,
    spider::spider_moves,
  },
};

pub mod beetle;
pub mod grasshoper;
pub mod ladybug;
pub mod mosquito;
pub mod queen_bee;
pub mod soldier_ant;
pub mod spider;

pub fn available_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let piece = grid.find_top_piece(hex);

  match piece {
    Some(p) => match p.p_type {
      PieceType::BEETLE => beetle_moves(grid, hex),
      PieceType::GRASSHOPPER => grasshopper_moves(grid, hex),
      PieceType::LADYBUG => ladybug_moves(grid, hex),
      PieceType::MOSQUITO => mosquito_moves(grid, hex),
      PieceType::NONE => Vec::new(),
      PieceType::QUEENBEE => queen_bee_moves(grid, hex),
      PieceType::SOLDIERANT => soldier_ant_moves(grid, hex),
      PieceType::SPIDER => spider_moves(grid, hex),
    },
    None => Vec::new(),
  }
}

pub fn available_actions_for_piece_color(grid: &Grid, piece_color: &PieceColor) -> Vec<Hex> {
  let mut moves: HashSet<Hex> = HashSet::new();

  if grid.number_of_pieces() == 0 {
    moves.insert(Hex::new(0, 0));
  } else if grid.number_of_pieces() == 1 {
    moves.insert(Hex::new(-1, 0));
  } else {
    for hex in grid.grid.keys() {
      if grid.is_hex_of_color(hex, piece_color) {
        for neighbor in hex.neighbors() {
          if !grid.is_hex_occupied(&neighbor)
            && grid.is_hex_neighbors_only_piece_color(&neighbor, piece_color)
          {
            moves.insert(neighbor);
          }
        }
      }
    }
  }

  moves.into_iter().collect()
}

fn extract_moves_from_paths(paths: Vec<Vec<Hex>>, path_expected_length: usize) -> Vec<Hex> {
  let mut moves: Vec<Hex> = Vec::new();

  for path in paths {
    if path.len() == path_expected_length {
      if let Some(h) = path.last() {
        if !moves.contains(h) {
          moves.push(*h);
        }
      }
    }
  }

  moves
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

  #[test]
  fn given_grid_when_available_moves_queenbee_should_return_correct_moves() {
    let grid = initialize_grid();
    let mut correct_moves = vec![Hex { q: -4, r: 1 }, Hex { q: -3, r: -1 }];

    let mut moves = available_moves(&grid, &Hex::new(-3, -0));

    moves.sort();
    correct_moves.sort();

    assert_eq!(moves, correct_moves);
  }

  #[test]
  fn given_grid_when_available_moves_beetle_should_return_correct_moves() {
    let grid = initialize_grid();
    let mut correct_moves = vec![
      Hex { q: -1, r: 0 },
      Hex { q: -1, r: 1 },
      Hex { q: -3, r: 2 },
      Hex { q: -3, r: 1 },
      Hex { q: -2, r: 0 },
    ];

    let mut moves = available_moves(&grid, &Hex::new(-2, 1));

    moves.sort();
    correct_moves.sort();

    assert_eq!(moves, correct_moves);
  }

  #[test]
  fn given_grid_when_available_moves_grasshopper_should_return_correct_moves() {
    let grid = initialize_grid();
    let mut correct_moves = vec![
      Hex { q: 4, r: -2 },
      Hex { q: -4, r: 0 },
      Hex { q: 2, r: -2 },
    ];

    let mut moves = available_moves(&grid, &Hex::new(2, 0));

    moves.sort();
    correct_moves.sort();

    assert_eq!(moves, correct_moves);
  }

  #[test]
  fn given_grid_when_available_moves_spider_should_return_correct_moves() {
    let grid = initialize_grid();
    let mut correct_moves = vec![Hex { q: 3, r: 0 }, Hex { q: -3, r: 2 }];

    let mut moves = available_moves(&grid, &Hex::new(0, 1));

    moves.sort();
    correct_moves.sort();

    assert_eq!(moves, correct_moves);
  }

  #[test]
  fn given_grid_when_available_moves_ladybug_should_return_correct_moves() {
    let grid = initialize_grid();
    let mut correct_moves = vec![
      Hex { q: 1, r: -1 },
      Hex { q: -1, r: 1 },
      Hex { q: 0, r: -1 },
      Hex { q: -2, r: 2 },
      Hex { q: -3, r: 2 },
      Hex { q: -4, r: 2 },
      Hex { q: -4, r: 1 },
      Hex { q: -4, r: 0 },
      Hex { q: -3, r: -1 },
      Hex { q: -1, r: -2 },
      Hex { q: -2, r: -2 },
    ];

    let mut moves = available_moves(&grid, &Hex::new(-1, -1));

    moves.sort();
    correct_moves.sort();

    assert_eq!(moves, correct_moves);
  }

  #[test]
  fn given_grid_when_available_moves_solider_ant_should_return_correct_moves() {
    let grid = initialize_grid();
    let mut correct_moves = vec![
      Hex { q: 3, r: 0 },
      Hex { q: 2, r: 1 },
      Hex { q: 1, r: 1 },
      Hex { q: 0, r: 2 },
      Hex { q: -1, r: 2 },
      Hex { q: -2, r: 2 },
      Hex { q: -1, r: 1 },
      Hex { q: -3, r: 2 },
      Hex { q: -4, r: 2 },
      Hex { q: -4, r: 1 },
      Hex { q: -4, r: 0 },
      Hex { q: -3, r: -1 },
      Hex { q: -2, r: -2 },
      Hex { q: -1, r: -2 },
      Hex { q: 0, r: -2 },
      Hex { q: 0, r: -1 },
      Hex { q: 1, r: -1 },
      Hex { q: 2, r: -2 },
      Hex { q: 3, r: -2 },
    ];

    let mut moves = available_moves(&grid, &Hex::new(3, -1));

    moves.sort();
    correct_moves.sort();

    assert_eq!(moves, correct_moves);
  }

  #[test]
  fn given_grid_when_available_moves_mosquito_should_return_correct_moves() {
    let grid = initialize_grid();
    let mut correct_moves = vec![
      Hex { q: 0, r: -1 },
      Hex { q: -1, r: 1 },
      Hex { q: 0, r: -2 },
      Hex { q: -1, r: -2 },
      Hex { q: -3, r: 2 },
      Hex { q: -2, r: 2 },
      Hex { q: -4, r: 2 },
      Hex { q: -4, r: 1 },
      Hex { q: -3, r: -1 },
      Hex { q: -4, r: 0 },
    ];

    let mut moves = available_moves(&grid, &Hex::new(-2, -1));
    moves.sort();
    correct_moves.sort();

    assert_eq!(moves, correct_moves);
  }

  #[test]
  fn given_grid_when_available_moves_by_color_should_return_correct_moves() {
    let grid = initialize_grid();
    let mut correct_moves = vec![
      Hex { q: 0, r: -2 },
      Hex { q: -4, r: 1 },
      Hex { q: -4, r: 0 },
      Hex { q: -4, r: 2 },
      Hex { q: -3, r: 2 },
      Hex { q: -2, r: 2 },
    ];

    let mut moves = available_actions_for_piece_color(&grid, &PieceColor::BLACK);
    moves.sort();
    correct_moves.sort();

    assert_eq!(moves, correct_moves);
  }
}
