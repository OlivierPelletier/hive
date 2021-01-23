use crate::engine::game::player::Player;
use crate::engine::grid::coordinate::hex::Hex;
use crate::engine::grid::piece::Piece;
use crate::engine::grid::Grid;

pub mod player;

pub struct Game {
  pub grid: Grid,
  pub players: Vec<Player>,
  pub moves: Vec<Move>,
  pub turn: u64,
}

impl Game {
  pub fn new() -> Game {
    Game {
      grid: Grid::new(),
      players: vec![Player::white(), Player::black()],
      moves: Vec::new(),
      turn: 1,
    }
  }
}

pub struct Move {
  pub piece: Piece,
  pub from: Hex,
  pub to: Hex,
}
