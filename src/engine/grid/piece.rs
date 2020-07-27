use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum PieceType {
    QUEENBEE,
    BEETLE,
    GRASSHOPPER,
    LADYBUG,
    MOSQUITO,
    SOLDIERANT,
    SPIDER,
}

#[derive(Clone)]
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

    // pub fn available_moves(&self, self_hex: Hex, grid: Grid) -> Vec<Hex> {
    //     Vec::new()
    // }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.p_type)
    }
}
