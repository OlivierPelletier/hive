use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

use crate::engine::grid::coordinate::hex::Hex;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Cube {
  pub x: i64,
  pub z: i64,
  pub y: i64,
}

impl Cube {
  pub fn new(x: i64, z: i64, y: i64) -> Cube {
    Cube { x, z, y }
  }

  pub fn xy(x: i64, y: i64) -> Cube {
    Cube { x, y, z: -x - y }
  }

  pub fn xz(x: i64, z: i64) -> Cube {
    Cube { x, y: -x - z, z }
  }

  pub fn yz(y: i64, z: i64) -> Cube {
    Cube { x: -y - z, y, z }
  }

  pub fn neighbors(&self) -> Vec<Cube> {
    let x = self.x;
    let z = self.z;
    let y = self.y;

    let neighbors: Vec<Cube> = vec![
      Cube {
        x: x + 1,
        z: z - 1,
        y,
      },
      Cube {
        x: x + 1,
        z,
        y: y - 1,
      },
      Cube {
        x,
        z: z + 1,
        y: y - 1,
      },
      Cube {
        x: x - 1,
        z: z + 1,
        y,
      },
      Cube {
        x: x - 1,
        z,
        y: y + 1,
      },
      Cube {
        x,
        z: z - 1,
        y: y + 1,
      },
    ];

    neighbors
  }
}

impl From<Hex> for Cube {
  fn from(hex: Hex) -> Cube {
    Cube {
      x: hex.q,
      z: hex.r,
      y: -hex.q - hex.r,
    }
  }
}

impl Display for Cube {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "x: {}, z: {}, y: {}", self.x, self.z, self.y)
  }
}
