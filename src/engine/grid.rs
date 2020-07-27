use crate::engine::grid::geo::cube_to_axial;
use crate::grid::geo::axial_to_cube;
use crate::grid::geo::cube::Cube;
use crate::grid::geo::hex::Hex;
use crate::grid::piece::Piece;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

pub mod geo;
pub mod piece;

#[derive(Clone)]
pub struct Grid {
    pub grid: HashMap<Hex, Vec<Piece>>,
}

impl Grid {
    pub fn new() -> Grid {
        let grid: HashMap<Hex, Vec<Piece>> = HashMap::new();

        return Grid { grid };
    }

    pub fn place_piece_to_hex(&self, piece: Piece, hex: &Hex) -> Grid {
        let mut _grid: HashMap<Hex, Vec<Piece>> = self.grid.clone();
        let mut _vec: Vec<Piece> = match _grid.get(&hex) {
            None => Vec::new(),
            Some(v) => v.to_vec(),
        };
        _vec.push(piece);
        _grid.insert(*hex, _vec);

        Grid { grid: _grid }
    }

    pub fn remove_top_piece_from_hex(&self, hex: &Hex) -> (Grid, Option<Piece>) {
        let mut _grid: HashMap<Hex, Vec<Piece>> = self.grid.clone();
        let mut _vec: Vec<Piece> = match _grid.get(&hex) {
            None => Vec::new(),
            Some(v) => v.to_vec(),
        };
        let piece = _vec.pop();
        _grid.insert(*hex, _vec);

        (Grid { grid: _grid }, piece)
    }

    pub fn move_piece_from_to(&self, from: &Hex, to: &Hex) -> Grid {
        let mut _grid_piece = self.remove_top_piece_from_hex(from);
        match _grid_piece.1 {
            Some(p) => _grid_piece.0.place_piece_to_hex(p, to),
            None => _grid_piece.0,
        }
    }

    pub fn is_hex_surrounded(&self, hex: &Hex) -> bool {
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

    pub fn is_hex_accessible_from(&self, hex: &Hex, from: &Hex) -> bool {
        let is_accessible;

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

                is_accessible = !(self.is_hex_occupied(&h1) && self.is_hex_occupied(&h2));
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

                is_accessible = !(self.is_hex_occupied(&h1) && self.is_hex_occupied(&h2));
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

                is_accessible = !(self.is_hex_occupied(&h1) && self.is_hex_occupied(&h2));
            }
        } else {
            is_accessible = false;
        }
        is_accessible
    }

    pub fn is_hex_neighbor_of(&self, hex: &Hex, of: &Hex) -> bool {
        let mut is_neighbor = false;

        for neighbor in hex.neighbors() {
            is_neighbor = is_neighbor || neighbor == *of;
        }

        is_neighbor
    }

    pub fn is_hex_occupied(&self, hex: &Hex) -> bool {
        let pieces = self.grid.get(&hex);

        match pieces {
            Some(p) => !p.is_empty(),
            None => false,
        }
    }

    pub fn is_hex_alone(&self, hex: &Hex) -> bool {
        let neighbors = hex.neighbors();

        let mut is_alone = true;

        for neighbor in neighbors {
            is_alone = is_alone && !self.is_hex_occupied(&neighbor);
        }

        is_alone
    }

    pub fn number_of_pieces(&self) -> usize {
        let mut count = 0;
        for vec in self.grid.values() {
            count = count + vec.len()
        }

        count
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut initialized = false;
        let mut min_q = 0;
        let mut max_q = 0;
        let mut min_r = 0;
        let mut max_r = 0;

        for key in self.grid.keys() {
            if !initialized {
                min_q = key.q;
                max_q = key.q;
                min_r = key.r;
                max_r = key.r;
                initialized = true;
            } else {
                if min_q > key.q {
                    min_q = key.q;
                } else if max_q < key.q {
                    max_q = key.q;
                }

                if min_r > key.r {
                    min_r = key.r;
                } else if max_r < key.r {
                    max_r = key.r;
                }
            }
        }

        let mut count_r = -1;
        let mut reverse_c_r = max_r - min_r;

        write!(f, "GRID START")?;
        for r in min_r..=max_r {
            write!(f, "\n")?;
            count_r += 1;
            reverse_c_r -= 1;

            for i in 0..=reverse_c_r {
                if i % 2 == 0 {
                    write!(f, "   ")?;
                } else {
                    write!(f, "    ")?;
                }
            }

            for _i in 1..=count_r {
                write!(f, "       ")?;
            }

            for q in min_q..=max_q {
                let hex = Hex { q, r };
                let q_str = str_number_with_sign(q);
                let r_str = str_number_with_sign(r);

                let occuped = self.is_hex_occupied(&hex);

                if occuped {
                    write!(f, "[{},{}]", q_str, r_str)?;
                } else {
                    write!(f, "       ")?;
                }
            }
            write!(f, "\n")?;
        }

        write!(f, "GRID END")
    }
}

fn str_number_with_sign(number: i64) -> String {
    let mut str = number.to_string();

    if number > -1 {
        str = "+".to_string() + &str
    };

    str
}
