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
use std::collections::HashSet;

pub mod beetle;
pub mod grasshoper;
pub mod ladybug;
pub mod mosquito;
pub mod queen_bee;
pub mod soldier_ant;
pub mod spider;

#[cfg(test)]
#[path = "../tests/moves_tests.rs"]
mod moves_tests;

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
