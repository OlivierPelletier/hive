use std::fmt::{Display, Formatter, Result};

use crate::engine::grid::{coordinate::hex::Hex, piece::Piece};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Action {
  pub piece: Piece,
  pub from: Hex,
  pub to: Hex,
  pub in_hand: bool,
  pub pillbug_flip: bool,
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
