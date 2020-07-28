use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone)]
pub struct Piece {
    pub p_type: PieceType,
}

impl Piece {
    pub fn queen_bee() -> Piece {
        Piece {
            p_type: PieceType::QUEENBEE,
        }
    }

    pub fn beetle() -> Piece {
        Piece {
            p_type: PieceType::BEETLE,
        }
    }

    pub fn grasshopper() -> Piece {
        Piece {
            p_type: PieceType::GRASSHOPPER,
        }
    }

    pub fn ladybug() -> Piece {
        Piece {
            p_type: PieceType::LADYBUG,
        }
    }

    pub fn mosquito() -> Piece {
        Piece {
            p_type: PieceType::MOSQUITO,
        }
    }

    pub fn soldier_ant() -> Piece {
        Piece {
            p_type: PieceType::SOLDIERANT,
        }
    }

    pub fn spider() -> Piece {
        Piece {
            p_type: PieceType::SPIDER,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let str;

        if self.p_type == PieceType::QUEENBEE {
            str = "QBEE"
        } else if self.p_type == PieceType::BEETLE {
            str = "BETL"
        } else if self.p_type == PieceType::GRASSHOPPER {
            str = "GRHP"
        } else if self.p_type == PieceType::LADYBUG {
            str = "LDBG"
        } else if self.p_type == PieceType::MOSQUITO {
            str = "MSQT"
        } else if self.p_type == PieceType::SOLDIERANT {
            str = "SLDR"
        } else if self.p_type == PieceType::SPIDER {
            str = "SPDR"
        } else {
            str = "N//A"
        }
        write!(f, "{}", str)
    }
}
