use std::fmt::{Debug, Formatter, Result};

use crate::engine::{
  game::{action::Action, player::Player},
  grid::{
    coordinate::hex::Hex,
    piece::{Piece, PieceColor, PieceType},
    Grid,
  },
  moves::{available_actions_for_piece_color, available_moves},
  rules,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod action;
pub mod player;

#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameWinnerState {
  WHITE,
  BLACK,
  DRAW,
  NONE,
}

impl Debug for GameWinnerState {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match *self {
      GameWinnerState::WHITE => write!(f, "WHITE"),
      GameWinnerState::BLACK => write!(f, "BLACK"),
      GameWinnerState::DRAW => write!(f, "DRAW"),
      GameWinnerState::NONE => write!(f, "NONE"),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
  pub id: Uuid,
  pub grid: Grid,
  pub players: Vec<Player>,
  pub actions_history: Vec<Action>,
  pub turn: u64,
  pub is_tournement_rule: bool,
  pub current_player_index: usize,
}

impl Game {
  pub fn tournament() -> Game {
    Game {
      id: Uuid::new_v4(),
      grid: Grid::new(),
      players: vec![Player::white(), Player::black()],
      actions_history: Vec::new(),
      turn: 0,
      is_tournement_rule: true,
      current_player_index: 0,
    }
  }

  pub fn list_actions_for_player(&self, player: &Player) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();

    if player.is_queen_played {
      for from in self.grid.grid.keys() {
        let Some(piece) = self.grid.find_top_piece(from) else {
          continue;
        };
        if piece.p_color != player.color {
          continue;
        }

        for to in available_moves(&self.grid, from) {
          actions.push(Action {
            piece: *piece,
            from: *from,
            to,
            in_hand: false,
          })
        }
      }
    }

    for to in available_actions_for_piece_color(&self.grid, &player.color) {
      for piece in &player.pieces {
        if self.can_play_piece(piece) {
          actions.push(Action {
            piece: *piece,
            from: Hex::zero(),
            to,
            in_hand: true,
          })
        }
      }
    }

    actions
  }

  fn can_play_piece(&self, piece: &Piece) -> bool {
    let current_player = &self.players[self.current_player_index];

    if self.is_tournement_rule
      && piece.p_type == PieceType::QUEENBEE
      && (self.turn == 0 || self.turn == 1)
    {
      return false;
    }

    if piece.p_type != PieceType::QUEENBEE && !current_player.is_queen_played && self.turn >= 6 {
      return false;
    }

    true
  }

  pub fn play_action(&mut self, action: Action) {
    let is_valid_action = self
      .list_actions_for_player(&self.players[self.current_player_index])
      .contains(&action);

    if !is_valid_action {
      return;
    }

    if action.in_hand {
      self.grid.place_piece_to_hex(action.piece, action.to);
      let index = self.players[self.current_player_index]
        .pieces
        .iter()
        .position(|p| p.p_type == action.piece.p_type && p.p_color == action.piece.p_color)
        .unwrap();
      self.players[self.current_player_index].pieces.remove(index);
    } else {
      self.grid.move_piece_from_to(action.from, action.to)
    }

    if action.piece.p_type == PieceType::QUEENBEE {
      self.players[self.current_player_index].is_queen_played = true
    }

    self.actions_history.push(action);

    self.next_turn()
  }

  pub fn winner(&self) -> GameWinnerState {
    let is_white_queen_surrounded = rules::queen_surrounded_rule(&self.grid, PieceColor::WHITE);
    let is_black_queen_surrounded = rules::queen_surrounded_rule(&self.grid, PieceColor::BLACK);

    if is_white_queen_surrounded && is_black_queen_surrounded {
      return GameWinnerState::DRAW;
    } else if is_white_queen_surrounded {
      return GameWinnerState::BLACK;
    } else if is_black_queen_surrounded {
      return GameWinnerState::WHITE;
    } else {
      GameWinnerState::NONE
    }
  }

  fn next_turn(&mut self) {
    self.turn += 1;
    self.current_player_index = Game::current_player_index(self.turn);
  }

  fn current_player_index(turn: u64) -> usize {
    if turn % 2 == 0 {
      0
    } else {
      1
    }
  }
}
