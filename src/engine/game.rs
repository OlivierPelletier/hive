use crate::engine::grid::coordinate::hex::Hex;
use crate::engine::grid::piece::{Piece, PieceColor};
use crate::engine::grid::Grid;

pub struct Game {
  pub grid: Grid,
  pub players: Vec<Player>,
  pub moves: Vec<Move>,
  pub turn: u64,
}

pub struct Player {
  pub color: PieceColor,
  pub pieces: Vec<Piece>,
  pub is_queen_played: bool,
}

pub struct Move {
  pub piece: Piece,
  pub from: Hex,
  pub to: Hex,
}
