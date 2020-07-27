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
