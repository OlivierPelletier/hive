use std::fmt;

fn main() {
    let hex = Hex { q: -1, r: -1 };
    println!("Hex:{{ {} }}", hex);
    let cube = axial_to_cube(hex);
    println!("Cube: {{ {} }}", cube);
    let hex = cube_to_axial(cube);
    println!("Hex:{{ {} }}", hex);
}

fn axial_to_cube(hex: Hex) -> Cube {
    Cube {
        x: hex.q,
        z: hex.r,
        y: -hex.q - hex.r,
    }
}

fn cube_to_axial(cube: Cube) -> Hex {
    Hex {
        q: cube.x,
        r: cube.z,
    }
}

struct Cube {
    x: i64,
    z: i64,
    y: i64,
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, z: {}, y: {}", self.x, self.z, self.y)
    }
}

struct Hex {
    q: i64,
    r: i64,
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "q: {}, r: {}", self.q, self.r)
    }
}
