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

pub fn cube_neighbors(cube: Cube) -> Vec<Cube> {
    let x = cube.x;
    let z = cube.z;
    let y = cube.y;

    let v: Vec<Cube> = vec![
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

    return v;
}
