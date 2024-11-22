use std::{
  collections::HashMap,
  fmt::{Display, Formatter},
};

use serde::{Deserialize, Serialize};

use crate::engine::grid::{
  coordinate::hex::Hex,
  piece::{Piece, PieceColor},
};

pub mod coordinate;
pub mod piece;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grid {
  pub grid: HashMap<Hex, Vec<Piece>>,
}

impl Grid {
  pub fn new() -> Grid {
    let grid: HashMap<Hex, Vec<Piece>> = HashMap::new();

    Grid { grid }
  }

  pub fn place_piece_to_hex(&mut self, piece: Piece, hex: Hex) {
    let mut pieces: Vec<Piece> = match self.grid.get(&hex) {
      None => Vec::new(),
      Some(v) => v.to_vec(),
    };
    pieces.push(piece);
    self.grid.insert(hex, pieces);
  }

  pub fn remove_top_piece_from_hex(&mut self, hex: Hex) -> Option<Piece> {
    let mut pieces: Vec<Piece> = match self.grid.get(&hex) {
      None => Vec::new(),
      Some(v) => v.to_vec(),
    };
    let piece = pieces.pop();
    self.grid.insert(hex, pieces);

    piece
  }

  pub fn move_piece_from_to(&mut self, from: Hex, to: Hex) {
    let removed_piece = self.remove_top_piece_from_hex(from);
    if let Some(p) = removed_piece {
      self.place_piece_to_hex(p, to)
    };
  }

  pub fn find_top_piece(&self, hex: &Hex) -> Option<&Piece> {
    if self.is_hex_occupied(hex) {
      match self.grid.get(hex) {
        Some(v) => v.last(),
        None => None,
      }
    } else {
      None
    }
  }

  pub fn is_hex_surrounded(&self, hex: &Hex) -> bool {
    let neighbors = hex.neighbors();

    let mut is_surrended = true;

    for neighbor in &neighbors {
      let pieces = self.grid.get(neighbor);
      is_surrended = is_surrended
        && match pieces {
          Some(p) => !p.is_empty(),
          None => false,
        }
    }

    is_surrended
  }

  pub fn is_hex_neighbor_of(&self, hex: &Hex, of: &Hex) -> bool {
    let mut is_neighbor = false;

    for neighbor in hex.neighbors() {
      is_neighbor = is_neighbor || neighbor == *of;
    }

    is_neighbor
  }

  pub fn is_hex_occupied(&self, hex: &Hex) -> bool {
    let pieces = self.grid.get(hex);

    match pieces {
      Some(p) => !p.is_empty(),
      None => false,
    }
  }

  pub fn is_hex_alone(&self, hex: &Hex) -> bool {
    let neighbors = hex.neighbors();

    let mut is_alone = true;

    for neighbor in neighbors {
      is_alone = is_alone && !self.is_hex_occupied(&neighbor);
    }

    is_alone
  }

  pub fn is_hex_of_color(&self, hex: &Hex, piece_color: &PieceColor) -> bool {
    let piece = self.find_top_piece(hex);

    match piece {
      Some(p) => p.p_color == *piece_color,
      None => false,
    }
  }

  pub fn is_hex_neighbors_only_piece_color(&self, hex: &Hex, piece_color: &PieceColor) -> bool {
    let mut is_hex_surrounded_by_piece_color = true;

    if !self.is_hex_alone(hex) {
      for neighbor in hex.neighbors() {
        if self.is_hex_occupied(&neighbor) && !self.is_hex_of_color(&neighbor, piece_color) {
          is_hex_surrounded_by_piece_color = false;
        }
      }
    } else {
      is_hex_surrounded_by_piece_color = false;
    }

    is_hex_surrounded_by_piece_color
  }

  pub fn number_of_pieces(&self) -> usize {
    let mut count = 0;
    for vec in self.grid.values() {
      count += vec.len()
    }

    count
  }
}

impl Default for Grid {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for Grid {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    let mut initialized = false;
    let mut min_q = 0;
    let mut max_q = 0;
    let mut min_r = 0;
    let mut max_r = 0;

    for key in self.grid.keys() {
      if !initialized {
        min_q = key.q;
        max_q = key.q;
        min_r = key.r;
        max_r = key.r;
        initialized = true;
      } else {
        if min_q > key.q {
          min_q = key.q;
        } else if max_q < key.q {
          max_q = key.q;
        }

        if min_r > key.r {
          min_r = key.r;
        } else if max_r < key.r {
          max_r = key.r;
        }
      }
    }

    let mut count_r = -1;

    write!(f, "GRID START")?;
    for r in min_r..=max_r {
      for m in 1..=2 {
        writeln!(f)?;

        if m % 2 == 1 {
          count_r += 1;
        }

        for _i in 0..count_r {
          write!(f, "    ")?;
        }

        for q in min_q..=max_q {
          let hex = Hex::new(q, r);

          let occupied = self.is_hex_occupied(&hex);

          if occupied {
            if m % 2 == 0 {
              write!(f, "{}", hex)?;
            } else {
              let piece = self.find_top_piece(&hex);
              match piece {
                Some(p) => write!(f, " {} ", p)?,
                None => write!(f, " NA ")?,
              }
            }
          } else if m % 2 == 1 {
            write!(f, "   __   ")?;
          } else {
            write!(f, "{}", hex)?;
          }
        }
      }
    }

    write!(f, "\nGRID END")
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn given_grid_when_placing_piece_to_hex_then_hex_contains_piece() {
    let mut grid = Grid::new();
    let hex = Hex::new(0, 0);
    let queen_bee = Piece::queen_bee();

    grid.place_piece_to_hex(queen_bee, hex);

    match grid.grid.get(&hex) {
      Some(p) => {
        let piece = p;
        assert_eq!(*piece.last().unwrap(), queen_bee);
      }
      None => panic!(),
    }
  }

  #[test]
  fn given_grid_when_removing_piece_from_hex_then_piece_is_removed_from_hex() {
    let mut grid = Grid::new();
    let hex = Hex::new(0, 0);

    grid.place_piece_to_hex(Piece::queen_bee(), hex);
    grid.remove_top_piece_from_hex(hex);

    match grid.grid.get(&hex) {
      Some(p) => assert_eq!(p.len(), 0),
      None => panic!(),
    }
  }

  #[test]
  fn given_grid_when_removing_piece_from_hex_containing_two_pieces_then_top_piece_is_removed_from_hex(
  ) {
    let mut grid = Grid::new();
    let hex = Hex::new(0, 0);
    let queen_bee = Piece::queen_bee();
    let beetle = Piece::beetle();
    grid.place_piece_to_hex(queen_bee, hex);
    grid.place_piece_to_hex(beetle, hex);
    grid.remove_top_piece_from_hex(hex);

    match grid.grid.get(&hex) {
      Some(p) => {
        let piece = p;
        assert_eq!(piece.last(), Some(&queen_bee));
      }
      None => panic!(),
    }
  }

  #[test]
  fn given_grid_when_adding_two_pieces_on_same_hex_then_hex_contains_both_pieces() {
    let mut grid = Grid::new();
    let hex = Hex::new(0, 0);
    let queen_bee = Piece::queen_bee();
    let spider = Piece::spider();

    grid.place_piece_to_hex(queen_bee, hex);
    grid.place_piece_to_hex(spider, hex);

    match grid.grid.get(&hex) {
      Some(p) => {
        let piece = p;
        assert_eq!(p.len(), 2);
        assert_eq!(piece.first(), Some(&queen_bee));
        assert_eq!(piece.last(), Some(&spider));
      }
      None => panic!(),
    }
  }

  #[test]
  fn given_grid_when_moving_piece_from_hex_to_hex_then_piece_is_moved() {
    let mut grid = Grid::new();
    let from = Hex::new(0, 0);
    let to = Hex::new(0, 1);
    let queen_bee = Piece::queen_bee();
    grid.place_piece_to_hex(queen_bee, from);

    grid.move_piece_from_to(from, to);

    match grid.grid.get(&from) {
      Some(p) => {
        assert_eq!(p.len(), 0);
      }
      None => panic!(),
    }

    match grid.grid.get(&to) {
      Some(p) => {
        let piece = p;
        assert_eq!(piece.last(), Some(&queen_bee));
      }
      None => panic!(),
    }
  }

  #[test]
  fn given_grid_when_moving_piece_from_hex_to_occupied_hex_then_piece_is_moved_and_hex_contains_both_pieces(
  ) {
    let mut grid = Grid::new();
    let from = Hex::new(0, 0);
    let to = Hex::new(0, 1);
    let queen_bee = Piece::queen_bee();
    let beetle = Piece::beetle();
    grid.place_piece_to_hex(beetle, from);
    grid.place_piece_to_hex(queen_bee, to);

    grid.move_piece_from_to(from, to);

    match grid.grid.get(&from) {
      Some(p) => {
        assert_eq!(p.len(), 0);
      }
      None => panic!(),
    }

    match grid.grid.get(&to) {
      Some(p) => {
        let piece = p;
        assert_eq!(piece.last(), Some(&beetle));
        assert_eq!(piece.first(), Some(&queen_bee));
      }
      None => panic!(),
    }
  }

  #[test]
  fn given_grid_when_finding_top_piece_then_top_piece_is_returned() {
    let mut grid = Grid::new();
    let hex = Hex::new(0, 0);
    let queen_bee = Piece::queen_bee();
    let ladybug = Piece::ladybug();

    grid.place_piece_to_hex(queen_bee, hex);
    grid.place_piece_to_hex(ladybug, hex);

    let piece = grid.find_top_piece(&hex).unwrap();

    assert_eq!(piece, &ladybug);
  }

  #[test]
  fn given_empty_grid_when_finding_top_piece_then_no_piece_is_returned() {
    let grid = Grid::new();
    let hex = Hex::new(0, 0);

    let piece = grid.find_top_piece(&hex);

    assert!(piece.is_none());
  }

  #[test]
  fn given_empty_grid_when_is_hex_surrended_should_return_false() {
    let grid = Grid::new();
    let hex = Hex::new(0, 0);

    assert!(!grid.is_hex_surrounded(&hex))
  }

  #[test]
  fn given_grid_with_surrended_hex_when_is_hex_surrended_should_return_true() {
    let center = Hex::new(-2, 0);
    let hex1 = Hex::new(-3, 0);
    let hex2 = Hex::new(-3, 1);
    let hex3 = Hex::new(-2, 1);
    let hex4 = Hex::new(-1, 0);
    let hex5 = Hex::new(-1, -1);
    let hex6 = Hex::new(-2, -1);
    let mut grid = Grid::new();
    grid.place_piece_to_hex(Piece::beetle(), hex1);
    grid.place_piece_to_hex(Piece::beetle(), hex2);
    grid.place_piece_to_hex(Piece::beetle(), hex3);
    grid.place_piece_to_hex(Piece::beetle(), hex4);
    grid.place_piece_to_hex(Piece::beetle(), hex5);
    grid.place_piece_to_hex(Piece::beetle(), hex6);

    assert!(grid.is_hex_surrounded(&center))
  }

  #[test]
  fn given_grid_with_partially_surrended_hex_when_is_hex_surrended_should_return_false() {
    let center = Hex::new(-2, 0);
    let hex1 = Hex::new(-3, 0);
    let hex2 = Hex::new(-3, 1);
    let hex3 = Hex::new(-2, 1);
    let hex4 = Hex::new(-1, 0);
    let hex5 = Hex::new(-1, -1);
    let mut grid = Grid::new();
    grid.place_piece_to_hex(Piece::beetle(), hex1);
    grid.place_piece_to_hex(Piece::beetle(), hex2);
    grid.place_piece_to_hex(Piece::beetle(), hex3);
    grid.place_piece_to_hex(Piece::beetle(), hex4);
    grid.place_piece_to_hex(Piece::beetle(), hex5);

    assert!(!grid.is_hex_surrounded(&center))
  }

  #[test]
  fn given_grid_with_two_connected_hex_when_is_hex_neighbor_of_should_return_true() {
    let hex1 = Hex::new(-3, 0);
    let hex2 = Hex::new(-3, 1);
    let grid = Grid::new();

    assert!(grid.is_hex_neighbor_of(&hex1, &hex2))
  }

  #[test]
  fn given_grid_with_two_speparated_hex_when_is_hex_neighbor_of_should_return_false() {
    let hex1 = Hex::new(-3, 0);
    let hex2 = Hex::new(-3, 4);
    let grid = Grid::new();

    assert!(!grid.is_hex_neighbor_of(&hex1, &hex2))
  }

  #[test]
  fn given_grid_empty_hex_when_is_hex_occupied_should_return_false() {
    let hex = Hex::new(0, 0);
    let grid = Grid::new();

    assert!(!grid.is_hex_occupied(&hex))
  }

  #[test]
  fn given_grid_occupied_hex_when_is_hex_occupied_should_return_true() {
    let hex = Hex::new(0, 0);
    let mut grid = Grid::new();
    grid.place_piece_to_hex(Piece::beetle(), hex);

    assert!(grid.is_hex_occupied(&hex))
  }

  #[test]
  fn given_empty_grid_when_is_hex_alone_should_return_true() {
    let grid = Grid::new();
    let hex = Hex::new(0, 0);

    assert!(grid.is_hex_alone(&hex))
  }

  #[test]
  fn given_grid_with_partially_surrended_hex_when_is_hex_alone_should_return_false() {
    let center = Hex::new(-2, 0);
    let hex1 = Hex::new(-3, 0);
    let mut grid = Grid::new();
    grid.place_piece_to_hex(Piece::beetle(), hex1);

    assert!(!grid.is_hex_alone(&center))
  }

  #[test]
  fn given_empty_grid_when_number_of_pieces_should_return_zero() {
    let grid = Grid::new();

    assert_eq!(grid.number_of_pieces(), 0)
  }

  #[test]
  fn given_filled_grid_when_number_of_pieces_should_return_correct_size() {
    let hex1 = Hex::new(-3, 0);
    let hex2 = Hex::new(-3, 1);
    let hex3 = Hex::new(-2, 1);
    let hex4 = Hex::new(-1, 0);
    let hex5 = Hex::new(-1, -1);
    let mut grid = Grid::new();
    grid.place_piece_to_hex(Piece::beetle(), hex1);
    grid.place_piece_to_hex(Piece::beetle(), hex2);
    grid.place_piece_to_hex(Piece::beetle(), hex3);
    grid.place_piece_to_hex(Piece::beetle(), hex4);
    grid.place_piece_to_hex(Piece::beetle(), hex5);

    assert_eq!(grid.number_of_pieces(), 5)
  }
}
