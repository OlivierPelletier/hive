use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PieceType {
  QUEENBEE,
  BEETLE,
  GRASSHOPPER,
  LADYBUG,
  MOSQUITO,
  SOLDIERANT,
  SPIDER,
  NONE,
}

impl Debug for PieceType {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match *self {
      PieceType::QUEENBEE => write!(f, "QBEE"),
      PieceType::BEETLE => write!(f, "BETL"),
      PieceType::GRASSHOPPER => write!(f, "GRHP"),
      PieceType::LADYBUG => write!(f, "LDBG"),
      PieceType::MOSQUITO => write!(f, "MSQT"),
      PieceType::SOLDIERANT => write!(f, "SANT"),
      PieceType::SPIDER => write!(f, "SPDR"),
      PieceType::NONE => write!(f, " NA "),
    }
  }
}

#[derive(Clone, Eq, PartialEq, Copy)]
pub enum PieceColor {
  BLACK,
  WHITE,
  NONE,
}

impl Debug for PieceColor {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match *self {
      PieceColor::WHITE => write!(f, "W"),
      PieceColor::BLACK => write!(f, "B"),
      PieceColor::NONE => write!(f, " "),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
  pub p_type: PieceType,
  pub p_color: PieceColor,
}

impl Piece {
  pub fn queen_bee() -> Piece {
    Piece {
      p_type: PieceType::QUEENBEE,
      p_color: PieceColor::NONE,
    }
  }

  pub fn beetle() -> Piece {
    Piece {
      p_type: PieceType::BEETLE,
      p_color: PieceColor::NONE,
    }
  }

  pub fn grasshopper() -> Piece {
    Piece {
      p_type: PieceType::GRASSHOPPER,
      p_color: PieceColor::NONE,
    }
  }

  pub fn ladybug() -> Piece {
    Piece {
      p_type: PieceType::LADYBUG,
      p_color: PieceColor::NONE,
    }
  }

  pub fn mosquito() -> Piece {
    Piece {
      p_type: PieceType::MOSQUITO,
      p_color: PieceColor::NONE,
    }
  }

  pub fn soldier_ant() -> Piece {
    Piece {
      p_type: PieceType::SOLDIERANT,
      p_color: PieceColor::NONE,
    }
  }

  pub fn spider() -> Piece {
    Piece {
      p_type: PieceType::SPIDER,
      p_color: PieceColor::NONE,
    }
  }

  pub fn white(&self) -> Piece {
    Piece {
      p_type: self.p_type,
      p_color: PieceColor::WHITE,
    }
  }

  pub fn black(&self) -> Piece {
    Piece {
      p_type: self.p_type,
      p_color: PieceColor::BLACK,
    }
  }
}

impl Display for Piece {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?} {:?}", self.p_color, self.p_type)
  }
}
