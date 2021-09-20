use serde::{Deserialize, Serialize};

use crate::engine::{
  game::{action::Action, player::Player},
  grid::{
    coordinate::hex::Hex,
    piece::{Piece, PieceType},
    Grid,
  },
  moves::{available_actions_for_piece_color, available_moves},
};

pub mod action;
pub mod player;

#[derive(Debug, Serialize, Deserialize)]
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

  pub fn current_player_index(&self) -> usize {
    return if self.turn % 2 == 0 { 0 } else { 1 };
  }

  pub fn list_actions_for_player(&self, player: &Player) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();

    if player.is_queen_played {
      for from in self.grid.grid.keys() {
        let piece = self.grid.find_top_piece(from);

        if piece.p_type != PieceType::NONE && piece.p_color == player.color {
          for to in available_moves(&self.grid, from) {
            actions.push(Action {
              piece: piece.clone(),
              from: *from,
              to,
              in_hand: false,
            })
          }
        }
      }
    }

    for to in available_actions_for_piece_color(&self.grid, player.color) {
      for piece in player.pieces.clone() {
        if self.can_play_piece(&piece) {
          actions.push(Action {
            piece,
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
    let current_player = &self.players[self.current_player_index()];

    return if self.is_tournement_rule
      && piece.p_type == PieceType::QUEENBEE
      && (self.turn == 0 || self.turn == 1)
    {
      false
    } else if piece.p_type != PieceType::QUEENBEE
      && !current_player.is_queen_played
      && (self.turn >= 6)
    {
      false
    } else {
      true
    };
  }

  pub fn play_action(&self, action: Action) -> Game {
    let mut grid = self.grid.clone();
    let mut players = self.players.clone();
    let mut actions_history = self.actions_history.clone();
    let current_player_index = self.current_player_index();

    let mut current_player = players[current_player_index].clone();

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

      if action.piece.p_type == PieceType::QUEENBEE {
        current_player.is_queen_played = true
      }

      actions_history.push(action.clone());
    }

    players[current_player_index] = current_player;

    Game {
      grid,
      players,
      actions_history,
      turn: self.turn,
      is_tournement_rule: self.is_tournement_rule,
    }
  }

  pub fn next_turn(&self) -> Game {
    let mut turn = self.turn;

    turn = turn + 1;

    Game {
      grid: self.grid.clone(),
      players: self.players.clone(),
      actions_history: self.actions_history.clone(),
      turn,
      is_tournement_rule: self.is_tournement_rule,
    }
  }
}
