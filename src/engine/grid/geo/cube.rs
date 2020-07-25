use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Clone)]
pub struct Cube {
    pub x: i64,
    pub z: i64,
    pub y: i64,
}

impl Cube {
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

        return neighbors;
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "x: {}, z: {}, y: {}", self.x, self.z, self.y)
    }
}
