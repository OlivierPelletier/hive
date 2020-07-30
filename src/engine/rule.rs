use crate::engine::grid::piece::PieceType;
use crate::grid::geo::cube::Cube;
use crate::grid::geo::hex::Hex;
use crate::grid::piece::Piece;
use crate::grid::Grid;
use std::collections::HashSet;

pub fn single_hive_rule_validation(grid: &Grid, from: &Hex, to: &Hex) -> bool {
    let is_valid;
    let temp_grid = Grid {
        grid: grid.grid.clone(),
    };

    if grid.is_hex_occupied(from) {
        is_valid = validate_grid(grid) && validate_grid(&temp_grid.move_piece_from_to(from, to));
    } else {
        is_valid = false;
    }

    is_valid
}

fn validate_grid(grid: &Grid) -> bool {
    let mut is_valid = true;
    let mut keys_it = grid.grid.keys();

    let first_key = keys_it.next();
    let mut found_keys = HashSet::new();

    match first_key {
        Some(fk) => found_keys = add_keys(grid, found_keys, *fk),
        None => (),
    }

    if found_keys.len() != grid.number_of_pieces() {
        is_valid = false;
    }

    is_valid
}

fn add_keys(grid: &Grid, found_keys: HashSet<(Hex, usize)>, hex: Hex) -> HashSet<(Hex, usize)> {
    let mut key_set = found_keys;
    let mut pieces = match grid.grid.get(&hex) {
        Some(v) => v.clone(),
        None => Vec::new(),
    };

    for i in 0..pieces.len() {
        key_set.insert((hex, i));
    }

    for neighbor in hex.neighbors() {
        let zero: usize = 0;
        if !key_set.contains(&(neighbor, zero)) && grid.is_hex_occupied(&neighbor) {
            key_set = add_keys(grid, key_set, neighbor);
        }
    }
    key_set
}

pub fn available_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
    let piece = grid.get_piece_copy(hex);
    let mut moves: Vec<Hex> = Vec::new();

    match piece.p_type {
        PieceType::QUEENBEE => {
            moves = queen_moves(grid, hex);
        }
        PieceType::BEETLE => {
            moves = beetle_moves(grid, hex);
        }
        PieceType::GRASSHOPPER => {
            moves = grasshopper_moves(grid, hex);
        }
        PieceType::LADYBUG => {}
        PieceType::MOSQUITO => {}
        PieceType::SOLDIERANT => {}
        PieceType::SPIDER => {}
        PieceType::NONE => {}
    }

    moves
}

fn queen_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
    let mut moves: Vec<Hex> = Vec::new();

    for neighbor in hex.neighbors() {
        if single_hive_rule_validation(grid, hex, &neighbor)
            && grid.is_hex_accessible_from(&neighbor, hex)
            && !grid.is_hex_occupied(&neighbor)
        {
            moves.push(neighbor);
        }
    }

    moves
}

fn beetle_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
    let mut moves: Vec<Hex> = Vec::new();

    for neighbor in hex.neighbors() {
        if single_hive_rule_validation(grid, hex, &neighbor)
            && grid.is_hex_accessible_from(&neighbor, hex)
        {
            moves.push(neighbor);
        }
    }

    moves
}

fn grasshopper_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
    let cube = hex.to_cube();
    let mut moves: Vec<Hex> = Vec::new();
    let mut possible_line: Vec<Cube> = Vec::new();

    for neighbor in hex.neighbors() {
        if grid.is_hex_occupied(&neighbor) {
            possible_line.push(neighbor.to_cube());
        }
    }

    for line in possible_line {
        if line.x == cube.x {
            let y_offset = line.y - cube.y;
            let mut line_cube;
            let mut count = 0;

            loop {
                count += 1;
                line_cube = Cube::xy(line.x, line.y + count * y_offset);

                if !grid.is_hex_occupied(&line_cube.to_axial()) {
                    break;
                }
            }

            let line_hex = line_cube.to_axial();

            if single_hive_rule_validation(grid, hex, &line_hex) {
                moves.push(line_hex);
            }
        } else if line.y == cube.y {
            let z_offset = line.z - cube.z;
            let mut line_cube;
            let mut count = 0;

            loop {
                count += 1;
                line_cube = Cube::yz(line.y, line.z + count * z_offset);

                if !grid.is_hex_occupied(&line_cube.to_axial()) {
                    break;
                }
            }

            let line_hex = line_cube.to_axial();

            if single_hive_rule_validation(grid, hex, &line_hex) {
                moves.push(line_hex);
            }
        } else {
            let x_offset = line.x - cube.x;
            let mut line_cube;
            let mut count = 0;

            loop {
                count += 1;
                line_cube = Cube::xz(line.x + count * x_offset, line.z);

                if !grid.is_hex_occupied(&line_cube.to_axial()) {
                    break;
                }
            }

            let line_hex = line_cube.to_axial();

            if single_hive_rule_validation(grid, hex, &line_hex) {
                moves.push(line_hex);
            }
        }
    }

    moves
}
