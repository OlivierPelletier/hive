use crate::engine::grid::coordinate::hex::Hex;
use crate::engine::grid::piece::Piece;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Clone)]
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
