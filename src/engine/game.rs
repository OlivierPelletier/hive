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
        let piece = self.grid.find_top_piece(from);

        if let Some(piece) = piece {
          if piece.p_color == player.color {
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

    !(self.is_tournement_rule
      && piece.p_type == PieceType::QUEENBEE
      && (self.turn == 0 || self.turn == 1))
      && !(piece.p_type != PieceType::QUEENBEE
        && !current_player.is_queen_played
        && (self.turn >= 6))
  }

  pub fn play_action(&mut self, action: Action) {
    if self
      .list_actions_for_player(&self.players[self.current_player_index])
      .contains(&action)
    {
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
  }

  pub fn winner(&self) -> Option<PieceColor> {
    let mut winner: Option<PieceColor> = None;

    if rules::queen_surrounded_rule(&self.grid, PieceColor::WHITE) {
      winner = Option::from(PieceColor::BLACK)
    } else if rules::queen_surrounded_rule(&self.grid, PieceColor::BLACK) {
      winner = Option::from(PieceColor::WHITE)
    }

    winner
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
