use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

use crate::engine::grid::coordinate::cube::Cube;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Hex {
  pub q: i64,
  pub r: i64,
}

impl Hex {
  pub fn new(q: i64, r: i64) -> Hex {
    Hex { q, r }
  }

  pub fn zero() -> Hex {
    Hex { q: 0, r: 0 }
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

impl From<Cube> for Hex {
  fn from(cube: Cube) -> Self {
    Hex {
      q: cube.x,
      r: cube.z,
    }
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
