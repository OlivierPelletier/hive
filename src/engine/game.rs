use serde::{Deserialize, Serialize};

use crate::engine::{game::{action::Action, player::Player}, grid::{
  coordinate::hex::Hex,
  piece::{Piece, PieceColor, PieceType},
  Grid,
}, moves::{available_actions_for_piece_color, available_moves}, rules};

pub mod action;
pub mod player;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
  pub grid: Grid,
  pub players: Vec<Player>,
  pub actions_history: Vec<Action>,
  pub turn: u64,
  pub is_tournement_rule: bool,
  pub current_player_index: usize,
}

impl Game {
  pub fn tournement() -> Game {
    Game {
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

  pub fn play_action(&self, action: Action) -> Game {
    let mut grid = self.grid.clone();
    let mut players = self.players.clone();
    let mut actions_history = self.actions_history.clone();
    let current_player_index = self.current_player_index;
    let mut current_player = players[current_player_index].clone();

    if self
      .list_actions_for_player(&current_player)
      .contains(&action)
    {
      if action.in_hand {
        grid = grid.place_piece_to_hex(action.piece, action.to);
        let index = current_player
          .pieces
          .iter()
            .position(|p| p.id == action.piece.id)
          .unwrap();
        current_player.pieces.remove(index);
      } else {
        grid = grid.move_piece_from_to(action.from, action.to)
      }

      if action.piece.p_type == PieceType::QUEENBEE {
        current_player.is_queen_played = true
      }

      actions_history.push(action);
      players[current_player_index] = current_player;

      let updated_game = Game {
        grid,
        players,
        actions_history,
        turn: self.turn,
        is_tournement_rule: self.is_tournement_rule,
        current_player_index,
      };

      updated_game.next_turn()
    } else {
      self.clone()
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

  fn next_turn(&self) -> Game {
    let mut turn = self.turn;

    turn += 1;

    Game {
      grid: self.grid.clone(),
      players: self.players.clone(),
      actions_history: self.actions_history.clone(),
      turn,
      is_tournement_rule: self.is_tournement_rule,
      current_player_index: Game::current_player_index(turn),
    }
  }

  fn current_player_index(turn: u64) -> usize {
    if turn % 2 == 0 {
      0
    } else {
      1
    }
  }
}
