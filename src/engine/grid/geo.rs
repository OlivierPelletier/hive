use crate::grid::geo::cube::Cube;
use crate::grid::geo::hex::Hex;

pub mod cube;
pub mod hex;

pub fn axial_to_cube(hex: Hex) -> Cube {
    Cube {
        x: hex.q,
        z: hex.r,
        y: -hex.q - hex.r,
    }
}

pub fn cube_to_axial(cube: Cube) -> Hex {
    Hex {
        q: cube.x,
        r: cube.z,
    }
}
