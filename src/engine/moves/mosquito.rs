use crate::engine::{
  game::action::Action,
  grid::{
    Grid,
    coordinate::hex::Hex,
    piece::{Piece, PieceType},
  },
  moves::{
    beetle::beetle_moves, grasshoper::grasshopper_moves, ladybug::ladybug_moves,
    queen_bee::queen_bee_moves, soldier_ant::soldier_ant_moves, spider::spider_moves,
  },
};

pub fn mosquito_moves(grid: &Grid, piece: &Piece, hex: &Hex) -> Vec<Action> {
  let mut actions: Vec<Action> = Vec::new();
  let mut found_types: Vec<PieceType> = Vec::new();

  for neighbor in hex.neighbors() {
    let piece = grid.find_top_piece(&neighbor);

    if let Some(p) = piece {
      found_types.push(p.p_type)
    }
  }

  for found_type in found_types {
    let moves: Vec<Action> = match found_type {
      PieceType::BEETLE => beetle_moves(grid, piece, hex),
      PieceType::GRASSHOPPER => grasshopper_moves(grid, piece, hex),
      PieceType::LADYBUG => ladybug_moves(grid, piece, hex),
      PieceType::MOSQUITO => Vec::new(),
      PieceType::NONE => Vec::new(),
      PieceType::QUEENBEE => queen_bee_moves(grid, piece, hex),
      PieceType::SOLDIERANT => soldier_ant_moves(grid, piece, hex),
      PieceType::SPIDER => spider_moves(grid, piece, hex),
    };

    for m in moves {
      if !actions.contains(&m) {
        actions.push(m)
      }
    }
  }

  actions
}
