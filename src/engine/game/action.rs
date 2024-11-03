use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

use crate::engine::grid::{coordinate::hex::Hex, piece::Piece};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Action {
  pub piece: Piece,
  pub from: Hex,
  pub to: Hex,
  pub in_hand: bool,
}

impl Display for Action {
  fn fmt(&self, f: &mut Formatter) -> Result {
    if self.in_hand {
      write!(f, "{} from: HAND, to: {}", self.piece, self.to)
    } else {
      write!(f, "{} from: {}, to: {}", self.piece, self.from, self.to)
    }
  }
}
