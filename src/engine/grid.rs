use crate::grid::geo::hex::Hex;
use crate::grid::piece::Piece;
use crate::grid::piece::{PieceColor, PieceType};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

pub mod geo;
pub mod piece;

#[derive(Debug, Clone)]
pub struct Grid {
  pub grid: HashMap<Hex, Vec<Piece>>,
}

impl Grid {
  pub fn new() -> Grid {
    let grid: HashMap<Hex, Vec<Piece>> = HashMap::new();

    Grid { grid }
  }

  pub fn place_piece_to_hex(&self, piece: Piece, hex: &Hex) -> Grid {
    let mut _grid: HashMap<Hex, Vec<Piece>> = self.grid.clone();
    let mut _vec: Vec<Piece> = match _grid.get(hex) {
      None => Vec::new(),
      Some(v) => v.to_vec(),
    };
    _vec.push(piece);
    _grid.insert(*hex, _vec);

    Grid { grid: _grid }
  }

  pub fn remove_top_piece_from_hex(&self, hex: &Hex) -> (Grid, Option<Piece>) {
    let mut _grid: HashMap<Hex, Vec<Piece>> = self.grid.clone();
    let mut _vec: Vec<Piece> = match _grid.get(hex) {
      None => Vec::new(),
      Some(v) => v.to_vec(),
    };
    let piece = _vec.pop();
    _grid.insert(*hex, _vec);

    (Grid { grid: _grid }, piece)
  }

  pub fn move_piece_from_to(&self, from: &Hex, to: &Hex) -> Grid {
    let mut _grid_piece = self.remove_top_piece_from_hex(from);
    match _grid_piece.1 {
      Some(p) => _grid_piece.0.place_piece_to_hex(p, to),
      None => _grid_piece.0,
    }
  }

  pub fn find_top_piece(&self, hex: &Hex) -> Piece {
    let none = Piece {
      p_type: PieceType::NONE,
      p_color: PieceColor::NONE,
    };

    if self.is_hex_occupied(hex) {
      match self.grid.get(hex) {
        Some(v) => match v.clone().pop() {
          Some(p) => p,
          None => none,
        },
        None => none,
      }
    } else {
      none
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
  fn fmt(&self, f: &mut Formatter) -> Result {
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
              write!(f, " {} ", piece)?;
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
  use super::geo::hex::Hex;
  use super::piece::Piece;
  use super::Grid;

  #[test]
  fn given_a_grid_when_placing_piece_to_hex_then_hex_contains_piece() {
    let mut grid = Grid::new();
    let hex = Hex::new(0, 0);
    grid = grid.place_piece_to_hex(Piece::queen_bee(), &hex);
    assert!(grid.grid.contains_key(&hex));
  }

  #[test]
  fn given_a_grid_when_removing_piece_from_hex_then_piece_is_removed_from_hex() {
    let mut grid = Grid::new();
    let hex = Hex::new(0, 0);
    grid = grid.place_piece_to_hex(Piece::queen_bee(), &hex);
    grid = grid.remove_top_piece_from_hex(&hex).0;
    match grid.grid.get(&hex) {
      Some(p) => assert_eq!(p.len(), 0),
      None => assert!(false),
    }
  }

  #[test]
  fn given_a_grid_when_adding_two_pieces_on_same_hex_then_hex_contains_both_pieces() {
    let mut grid = Grid::new();
    let hex = Hex::new(0, 0);
    grid = grid.place_piece_to_hex(Piece::queen_bee(), &hex);
    grid = grid.place_piece_to_hex(Piece::spider(), &hex);
    match grid.grid.get(&hex) {
      Some(p) => {
        let mut piece = p.clone();
        assert_eq!(p.len(), 2);
        assert_eq!(piece.pop(), Some(Piece::spider()));
        assert_eq!(piece.pop(), Some(Piece::queen_bee()));
      }
      None => assert!(false),
    }
  }

  #[test]
  fn given_a_grid_when_moving_piece_from_hex_to_hex_then_piece_is_moved() {
    let mut grid = Grid::new();
    let from = Hex::new(0, 0);
    let to = Hex::new(0, 1);
    grid = grid.place_piece_to_hex(Piece::queen_bee(), &from);
    grid = grid.move_piece_from_to(&from, &to);
    match grid.grid.get(&from) {
      Some(p) => {
        assert_eq!(p.len(), 0);
      }
      None => assert!(false),
    }
    assert!(grid.grid.contains_key(&to));
  }
}
