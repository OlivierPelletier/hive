use crate::engine::grid::piece::PieceColor;
use crate::engine::rules::queen_surrended_rule;
use crate::engine::{
  grid::{coordinate::hex::Hex, piece::Piece, Grid},
  rules::one_hive_rule,
};

fn initialize_grid() -> Grid {
  Grid::new()
    .place_piece_to_hex(Piece::queen_bee().white(), &Hex::new(2, -1))
    .place_piece_to_hex(Piece::queen_bee().black(), &Hex::new(-3, -0))
    .place_piece_to_hex(Piece::beetle().white(), &Hex::new(0, 0))
    .place_piece_to_hex(Piece::beetle().black(), &Hex::new(-2, 1))
    .place_piece_to_hex(Piece::beetle().black(), &Hex::new(-1, 0))
    .place_piece_to_hex(Piece::grasshopper().white(), &Hex::new(1, 0))
    .place_piece_to_hex(Piece::grasshopper().white(), &Hex::new(2, 0))
    .place_piece_to_hex(Piece::grasshopper().black(), &Hex::new(-2, 0))
    .place_piece_to_hex(Piece::soldier_ant().black(), &Hex::new(-3, 1))
    .place_piece_to_hex(Piece::soldier_ant().white(), &Hex::new(3, -1))
    .place_piece_to_hex(Piece::spider().white(), &Hex::new(0, 1))
    .place_piece_to_hex(Piece::ladybug().black(), &Hex::new(-1, -1))
    .place_piece_to_hex(Piece::mosquito().white(), &Hex::new(-2, -1))
}

fn initialize_white_losing_grid() -> Grid {
  Grid::new()
      .place_piece_to_hex(Piece::queen_bee().white(), &Hex::new(2, -1))
      .place_piece_to_hex(Piece::queen_bee().black(), &Hex::new(2, 0))
      .place_piece_to_hex(Piece::beetle().white(), &Hex::new(1, -1))
      .place_piece_to_hex(Piece::beetle().black(), &Hex::new(3, -1))
      .place_piece_to_hex(Piece::beetle().black(), &Hex::new(2, -2))
      .place_piece_to_hex(Piece::grasshopper().white(), &Hex::new(2, 0))
}

fn initialize_black_losing_grid() -> Grid {
  Grid::new()
      .place_piece_to_hex(Piece::queen_bee().black(), &Hex::new(2, -1))
      .place_piece_to_hex(Piece::queen_bee().white(), &Hex::new(2, 0))
      .place_piece_to_hex(Piece::beetle().white(), &Hex::new(1, -1))
      .place_piece_to_hex(Piece::beetle().black(), &Hex::new(3, -1))
      .place_piece_to_hex(Piece::beetle().black(), &Hex::new(2, -2))
      .place_piece_to_hex(Piece::grasshopper().white(), &Hex::new(2, 0))
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

  assert!(!queen_surrended_rule(&grid, PieceColor::WHITE))
}

#[test]
fn given_filled_grid_when_queen_surrended_rule_with_black_should_return_false() {
  let grid = initialize_grid();

  assert!(!queen_surrended_rule(&grid, PieceColor::BLACK))
}

#[test]
fn given_filled_white_loosing_grid_when_queen_surrended_rule_with_white_should_return_true() {
  let grid = initialize_white_losing_grid();

  assert!(!queen_surrended_rule(&grid, PieceColor::WHITE))
}

#[test]
fn given_filled_white_loosing_grid_when_queen_surrended_rule_with_black_should_return_false() {
  let grid = initialize_white_losing_grid();

  assert!(!queen_surrended_rule(&grid, PieceColor::BLACK))
}

#[test]
fn given_filled_white_loosing_grid_when_queen_surrended_rule_with_white_should_return_false() {
  let grid = initialize_black_losing_grid();

  assert!(!queen_surrended_rule(&grid, PieceColor::WHITE))
}

#[test]
fn given_filled_white_loosing_grid_when_queen_surrended_rule_with_black_should_return_true() {
  let grid = initialize_black_losing_grid();

  assert!(!queen_surrended_rule(&grid, PieceColor::BLACK))
}
