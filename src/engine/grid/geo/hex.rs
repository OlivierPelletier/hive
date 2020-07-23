use std::fmt::{Display, Formatter, Result};

pub struct Hex {
    pub q: i64,
    pub r: i64,
}

impl Display for Hex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "q: {}, r: {}", self.q, self.r)
    }
}
