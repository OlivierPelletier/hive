use std::fmt::{Display, Formatter, Result};

use crate::engine::game::player::Player;
use crate::engine::grid::coordinate::hex::Hex;
use crate::engine::grid::piece::{Piece, PieceType};
use crate::engine::grid::Grid;
use crate::engine::moves::{available_actions_for_piece_color, available_moves};

pub mod player;

#[derive(Debug)]
pub struct Game {
  pub grid: Grid,
  pub players: Vec<Player>,
  pub actions_history: Vec<Action>,
  pub turn: u64,
  pub is_tournement_rule: bool,
}

impl Game {
  pub fn tournement() -> Game {
    Game {
      grid: Grid::new(),
      players: vec![Player::white(), Player::black()],
      actions_history: Vec::new(),
      turn: 0,
      is_tournement_rule: true,
    }
  }

  pub fn list_actions_for_player(&self, player: &Player) -> Vec<Action> {
    let mut moves: Vec<Action> = Vec::new();

    for from in self.grid.grid.keys() {
      let piece = self.grid.find_top_piece(from);

      if piece.p_type != PieceType::NONE && piece.p_color == player.color {
        for to in available_moves(&self.grid, from) {
          moves.push(Action {
            piece: piece.clone(),
            from: *from,
            to,
            in_hand: false,
          })
        }
      }
    }

    for to in available_actions_for_piece_color(&self.grid, player.color) {
      for piece in player.pieces.clone() {
        if self.can_play_piece(&piece) {
          moves.push(Action {
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
      && (self.turn == 0 || self.turn == 1))
  }

  pub fn play_action(&self, action: Action) -> Game {
    let mut grid = self.grid.clone();
    let players = self.players.clone();
    let mut actions_history = self.actions_history.clone();
    let mut turn = self.turn;

    let mut current_player;
    let other_player;
    if players[0].color == action.piece.p_color {
      current_player = players[0].clone();
      other_player = players[1].clone();
    } else {
      current_player = players[1].clone();
      other_player = players[0].clone();
    }

    if self
      .list_actions_for_player(&current_player)
      .contains(&action)
    {
      if action.in_hand {
        grid = grid.place_piece_to_hex(action.piece.clone(), &action.to);
        let index = current_player
          .pieces
          .iter()
          .position(|p| p.clone().id == action.piece.id)
          .unwrap();
        current_player.pieces.remove(index);
      } else {
        grid = grid.move_piece_from_to(&action.from, &action.to)
      }

      actions_history.push(action.clone());
      turn = turn + 1;
    }

    Game {
      grid,
      players: vec![current_player, other_player],
      actions_history,
      turn,
      is_tournement_rule: self.is_tournement_rule,
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Action {
  pub piece: Piece,
  pub from: Hex,
  pub to: Hex,
  pub in_hand: bool,
}

impl Display for Action {
  fn fmt(&self, f: &mut Formatter) -> Result {
    if self.in_hand {
      write!(f, "{} from: HAND, to: {}", self.piece, self.to)
    } else {
      write!(f, "{} from: {}, to: {}", self.piece, self.from, self.to)
    }
  }
}
