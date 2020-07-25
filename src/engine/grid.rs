use crate::engine::grid::geo::cube_to_axial;
use crate::grid::geo::axial_to_cube;
use crate::grid::geo::cube::Cube;
use crate::grid::geo::hex::Hex;
use crate::grid::piece::Piece;
use std::collections::HashMap;

pub mod geo;
pub mod piece;

pub fn init_grid() -> Grid {
    let mut grid: HashMap<Hex, Vec<dyn Piece>> = HashMap::new();
    let hex_zero = Hex { q: 0, r: 0 };

    grid.insert(hex_zero, Vec::new());

    return Grid { grid };
}

pub struct Grid {
    pub grid: HashMap<Hex, Vec<dyn Piece>>,
}

impl Grid {
    pub fn is_hex_surrounded(&self, hex: Hex) -> bool {
        let neighbors = hex.neighbors();

        let mut is_surrended = true;

        for neighbor in &neighbors {
            let pieces = self.grid.get(neighbor);
            is_surrended = is_surrended
                && match pieces {
                    Some(p) => !p.is_empty(),
                    None => false,
                }
        }

        is_surrended
    }

    pub fn is_hex_accessible_from(&self, hex: Hex, from: Hex) -> bool {
        let mut is_accessible = true;

        if self.is_hex_neighbor_of(hex, from) {
            let cube = axial_to_cube(hex.clone());
            let cube_from = axial_to_cube(from.clone());

            if cube.x == cube_from.x {
                let xz_offset = cube.z - cube_from.z;
                let xy_offset = cube.y - cube_from.y;

                let c1 = Cube {
                    x: cube_from.x - xz_offset,
                    y: cube_from.y,
                    z: cube.z,
                };
                let c2 = Cube {
                    x: cube_from.x - xy_offset,
                    y: cube.y,
                    z: cube_from.z,
                };

                let h1 = cube_to_axial(c1);
                let h2 = cube_to_axial(c2);

                is_accessible = !(self.is_hex_occupied(h1) && self.is_hex_occupied(h2));
            } else if cube.z == cube_from.z {
                let zx_offset = cube.x - cube_from.x;
                let zy_offset = cube.y - cube_from.y;

                let c1 = Cube {
                    x: cube.x,
                    y: cube_from.y,
                    z: cube_from.z - zx_offset,
                };
                let c2 = Cube {
                    x: cube_from.x,
                    y: cube.y,
                    z: cube_from.z - zy_offset,
                };

                let h1 = cube_to_axial(c1);
                let h2 = cube_to_axial(c2);

                is_accessible = !(self.is_hex_occupied(h1) && self.is_hex_occupied(h2));
            } else {
                let yx_offset = cube.x - cube_from.x;
                let yz_offset = cube.z - cube_from.z;

                let c1 = Cube {
                    x: cube.x,
                    y: cube_from.y - yx_offset,
                    z: cube_from.z,
                };
                let c2 = Cube {
                    x: cube_from.x,
                    y: cube_from.y - yz_offset,
                    z: cube.z,
                };

                let h1 = cube_to_axial(c1);
                let h2 = cube_to_axial(c2);

                is_accessible = !(self.is_hex_occupied(h1) && self.is_hex_occupied(h2));
            }
        } else {
            is_accessible = false;
        }
        is_accessible
    }

    pub fn is_hex_neighbor_of(&self, hex: Hex, of: Hex) -> bool {
        let mut is_neighbor = false;

        for neighbor in &hex.neighbors() {
            is_neighbor = is_neighbor || neighbor == of;
        }

        is_neighbor
    }

    pub fn is_hex_occupied(&self, hex: Hex) -> bool {
        let pieces = self.grid.get(*hex);

        match pieces {
            Some(p) => !p.is_empty(),
            None => false,
        }
    }
}
