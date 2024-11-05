use std::{
  collections::HashMap,
  fmt::{Display, Formatter},
};

use crate::engine::grid::{
  coordinate::hex::Hex,
  piece::{Piece, PieceColor},
};
use serde::{Deserialize, Serialize};

pub mod coordinate;
pub mod piece;

#[cfg(test)]
#[path = "../tests/grid_tests.rs"]
mod grid_tests;

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
    match removed_piece {
      Some(p) => self.place_piece_to_hex(p, to),
      None => (),
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
