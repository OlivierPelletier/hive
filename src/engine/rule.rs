use crate::engine::grid::piece::PieceType;
use crate::grid::geo::hex::Hex;
use crate::grid::Grid;

pub fn single_hive_rule_validation(grid: &Grid, from: &Hex, to: &Hex) -> bool {
    let mut is_valid = true;
    let from_neighbors = from.neighbors();
    let to_neighbors = to.neighbors();
    let mut temp_grid = Grid {
        grid: grid.grid.clone(),
    };

    if grid.is_hex_occupied(from) {
        temp_grid = temp_grid.move_piece_from_to(from, to);

        for from_neighbor in from_neighbors {
            if temp_grid.is_hex_occupied(&from_neighbor) && temp_grid.is_hex_alone(&from_neighbor) {
                is_valid = false;
            }
        }

        for to_neighbor in to_neighbors {
            if temp_grid.is_hex_occupied(&to_neighbor) && temp_grid.is_hex_alone(&to_neighbor) {
                is_valid = false;
            }
        }
    } else {
        is_valid = false;
    }

    is_valid
}

pub fn available_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
    let piece = grid.get_piece_copy(hex);
    let mut moves: Vec<Hex> = Vec::new();

    if piece.p_type == PieceType::QUEENBEE {
        moves = queen_modes(grid, hex);
    }

    moves
}

fn queen_modes(grid: &Grid, hex: &Hex) -> Vec<Hex> {
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
