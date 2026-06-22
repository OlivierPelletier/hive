use std::collections::HashSet;

use crate::engine::grid::{Grid, coordinate::hex::Hex};

pub fn one_hive_rule_grid_validation(grid: &Grid) -> bool {
  let mut is_valid = true;
  let mut keys_it = grid.cells.keys();
  let mut found_pieces = HashSet::new();

  if let Some(start) = keys_it.find(|h| grid.is_hex_occupied(h)) {
    one_hive_rule_iterative_pieces_search(grid, &mut found_pieces, start)
  }

  if found_pieces.len() != grid.number_of_pieces() {
    is_valid = false;
  }

  is_valid
}

fn one_hive_rule_iterative_pieces_search(
  grid: &Grid,
  found_pieces: &mut HashSet<(Hex, usize)>,
  hex: &Hex,
) {
  let piece_count = grid.cells.get(hex).map_or(0, Vec::len);

  for i in 0..piece_count {
    found_pieces.insert((*hex, i));
  }

  for neighbor in hex.neighbors() {
    let zero: usize = 0;
    if found_pieces.contains(&(neighbor, zero)) {
      continue;
    }
    if !grid.is_hex_occupied(&neighbor) {
      continue;
    }
    one_hive_rule_iterative_pieces_search(grid, found_pieces, &neighbor);
  }
}
