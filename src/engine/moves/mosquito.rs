use crate::engine::{
  grid::{coordinate::hex::Hex, piece::PieceType, Grid},
  moves::{
    beetle::beetle_moves, grasshoper::grasshopper_moves, ladybug::ladybug_moves,
    queen_bee::queen_bee_moves, soldier_ant::soldier_ant_moves, spider::spider_moves,
  },
};

pub fn mosquito_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let mut moves: Vec<Hex> = Vec::new();
  let mut found_types: Vec<PieceType> = Vec::new();

  for neighbor in hex.neighbors() {
    let piece = grid.find_top_piece(&neighbor);

    if let Some(p) = piece { found_types.push(p.p_type) }
  }

  for found_type in found_types {
    let temp_moves: Vec<Hex> = match found_type {
      PieceType::BEETLE => beetle_moves(grid, hex),
      PieceType::GRASSHOPPER => grasshopper_moves(grid, hex),
      PieceType::LADYBUG => ladybug_moves(grid, hex),
      PieceType::MOSQUITO => Vec::new(),
      PieceType::NONE => Vec::new(),
      PieceType::QUEENBEE => queen_bee_moves(grid, hex),
      PieceType::SOLDIERANT => soldier_ant_moves(grid, hex),
      PieceType::SPIDER => spider_moves(grid, hex),
    };

    for temp_move in temp_moves {
      if !moves.contains(&temp_move) {
        moves.push(temp_move)
      }
    }
  }

  moves
}
