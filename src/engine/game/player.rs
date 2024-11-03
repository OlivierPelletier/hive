use serde::{Deserialize, Serialize};

use crate::engine::grid::piece::{Piece, PieceColor};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
  pub color: PieceColor,
  pub pieces: Vec<Piece>,
  pub is_queen_played: bool,
}

impl Player {
    pub fn white() -> Player {
    Player {
      pieces: vec![
          Piece::queen_bee().white(),
          Piece::spider().white(),
          Piece::spider().white(),
          Piece::beetle().white(),
          Piece::beetle().white(),
          Piece::grasshopper().white(),
          Piece::grasshopper().white(),
          Piece::grasshopper().white(),
          Piece::soldier_ant().white(),
          Piece::soldier_ant().white(),
          Piece::soldier_ant().white(),
          Piece::mosquito().white(),
          Piece::ladybug().white(),
      ],
        color: PieceColor::WHITE,
      is_queen_played: false,
    }
  }

    pub fn black() -> Player {
    Player {
        pieces: vec![
            Piece::queen_bee().black(),
            Piece::spider().black(),
            Piece::spider().black(),
            Piece::beetle().black(),
            Piece::beetle().black(),
            Piece::grasshopper().black(),
            Piece::grasshopper().black(),
            Piece::grasshopper().black(),
            Piece::soldier_ant().black(),
            Piece::soldier_ant().black(),
            Piece::soldier_ant().black(),
            Piece::mosquito().black(),
            Piece::ladybug().black(),
        ],
      color: PieceColor::BLACK,
        is_queen_played: false,
    }
  }
}
