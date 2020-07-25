use std::fmt::{Display, Formatter, Result};

#[derive(Hash, PartialEq, Clone)]
pub struct Hex {
    pub q: i64,
    pub r: i64,
}

impl Hex {
    pub fn neighbors(&self) -> Vec<Hex> {
        let q = self.q;
        let r = self.r;

        let neighbors: Vec<Hex> = vec![
            Hex { q: q + 1, r: r - 1 },
            Hex { q: q + 1, r },
            Hex { q, r: r + 1 },
            Hex { q: q - 1, r: r + 1 },
            Hex { q: q - 1, r },
            Hex { q, r: r - 1 },
        ];

        return neighbors;
    }
}

impl Display for Hex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "q: {}, r: {}", self.q, self.r)
    }
}
