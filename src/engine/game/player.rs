use serde::{Deserialize, Serialize};

use crate::engine::grid::piece::{Piece, PieceColor};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
  pub color: PieceColor,
  pub pieces: Vec<Piece>,
  pub is_queen_played: bool,
}

impl Player {
  pub fn new() -> Player {
    Player {
      pieces: vec![
        Piece::queen_bee(),
        Piece::spider(),
        Piece::spider(),
        Piece::beetle(),
        Piece::beetle(),
        Piece::grasshopper(),
        Piece::grasshopper(),
        Piece::grasshopper(),
        Piece::soldier_ant(),
        Piece::soldier_ant(),
        Piece::soldier_ant(),
        Piece::mosquito(),
        Piece::ladybug(),
      ],
      color: PieceColor::NONE,
      is_queen_played: false,
    }
  }

  pub fn black() -> Player {
    let player = Player::new();

    let mut colored_pieces = player.pieces.clone();
    for piece in colored_pieces.iter_mut() {
      *piece = piece.black();
    }

    Player {
      pieces: colored_pieces,
      color: PieceColor::BLACK,
      is_queen_played: player.is_queen_played,
    }
  }

  pub fn white() -> Player {
    let player = Player::new();

    let mut colored_pieces = player.pieces.clone();
    for piece in colored_pieces.iter_mut() {
      *piece = piece.white();
    }

    Player {
      pieces: colored_pieces,
      color: PieceColor::WHITE,
      is_queen_played: player.is_queen_played,
    }
  }
}
