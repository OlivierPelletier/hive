use std::fmt::{Display, Formatter, Result};

pub struct Cube {
    pub x: i64,
    pub z: i64,
    pub y: i64,
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "x: {}, z: {}, y: {}", self.x, self.z, self.y)
    }
}
