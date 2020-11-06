use crate::grid::geo::cube::Cube;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Hex {
  pub q: i64,
  pub r: i64,
}

impl Hex {
  pub fn new(q: i64, r: i64) -> Hex {
    Hex { q, r }
  }

  pub fn to_cube(&self) -> Cube {
    Cube {
      x: self.q,
      z: self.r,
      y: -self.q - self.r,
    }
  }

  pub fn neighbors(&self) -> Vec<Hex> {
    let q = self.q;
    let r = self.r;

    let neighbors: Vec<Hex> = vec![
      Hex::new(q + 1, r - 1),
      Hex::new(q + 1, r),
      Hex::new(q, r + 1),
      Hex::new(q - 1, r + 1),
      Hex::new(q - 1, r),
      Hex::new(q, r - 1),
    ];

    neighbors
  }
}

impl Display for Hex {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let mut str: String = self.q.to_string() + "," + self.r.to_string().as_str();

    if str.len() <= 6 {
      for i in str.len()..8 {
        if i % 2 == 1 {
          str = " ".to_owned() + &str;
        } else {
          str += " ";
        }
      }
      write!(f, "{}", str)
    } else {
      write!(f, "   ?,?  ")
    }
  }
}
