use crate::engine::game::player::Player;
use crate::engine::grid::coordinate::hex::Hex;
use crate::engine::grid::piece::{Piece, PieceType};
use crate::engine::grid::Grid;
use crate::engine::moves::{available_moves, available_moves_for_piece_color};
use std::fmt::{Display, Formatter, Result};

pub mod player;

pub struct Game {
  pub grid: Grid,
  pub players: Vec<Player>,
  pub moves: Vec<Move>,
  pub turn: Box<u64>,
  pub is_tournement_rule: bool,
}

impl Game {
  pub fn tournement() -> Game {
    Game {
      grid: Grid::new(),
      players: vec![Player::white(), Player::black()],
      moves: Vec::new(),
      turn: Box::new(0),
      is_tournement_rule: true,
    }
  }

  pub fn current_player(&self) -> &Player {
    let player: Option<&Player>;

    if *self.turn % 2 == 0 {
      player = self.players.get(0)
    } else {
      player = self.players.get(1)
    }

    match player {
      Some(p) => p,
      None => &Player::new()
    }
  }

  pub fn list_moves_for_player(&self, player: &Player) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    for from in self.grid.grid.keys() {
      let piece = self.grid.find_top_piece(from);

      if piece.p_type != PieceType::NONE && piece.p_color == player.color {
        for to in available_moves(&self.grid, from) {
          moves.push(Move {
            piece: piece.clone(),
            from: *from,
            to,
            in_hand: false,
          })
        }
      }
    }

    for to in available_moves_for_piece_color(&self.grid, player.color) {
      for piece in player.pieces.clone() {
        if self.can_play_piece(&piece) {
          moves.push(Move {
            piece,
            from: Hex::zero(),
            to,
            in_hand: true,
          })
        }
      }
    }

    moves
  }

  fn can_play_piece(&self, piece: &Piece) -> bool {
    !(self.is_tournement_rule
      && piece.p_type == PieceType::QUEENBEE
      && (*self.turn == 0 || *self.turn == 1))
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Move {
  pub piece: Piece,
  pub from: Hex,
  pub to: Hex,
  pub in_hand: bool,
}

impl Display for Move {
  fn fmt(&self, f: &mut Formatter) -> Result {
    if self.in_hand {
      write!(f, "{} from: HAND, to: {}", self.piece, self.to)
    } else {
      write!(f, "{} from: {}, to: {}", self.piece, self.from, self.to)
    }
  }
}
