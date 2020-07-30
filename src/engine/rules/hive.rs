use crate::grid::geo::hex::Hex;
use crate::grid::Grid;
use std::collections::HashSet;

pub fn one_hive_rule_grid_validation(grid: &Grid) -> bool {
  let mut is_valid = true;
  let mut keys_it = grid.grid.keys();
  let mut found_pieces = HashSet::new();

  match keys_it.next() {
    Some(start) => found_pieces = one_hive_rule_iterative_pieces_search(grid, found_pieces, *start),
    None => (),
  }

  if found_pieces.len() != grid.number_of_pieces() {
    is_valid = false;
  }

  is_valid
}

fn one_hive_rule_iterative_pieces_search(
  grid: &Grid,
  found_pieces: HashSet<(Hex, usize)>,
  hex: Hex,
) -> HashSet<(Hex, usize)> {
  let mut _found_pieces = found_pieces;
  let pieces = match grid.grid.get(&hex) {
    Some(v) => v.clone(),
    None => Vec::new(),
  };

  for i in 0..pieces.len() {
    _found_pieces.insert((hex, i));
  }

  for neighbor in hex.neighbors() {
    let zero: usize = 0;
    if !_found_pieces.contains(&(neighbor, zero)) && grid.is_hex_occupied(&neighbor) {
      _found_pieces = one_hive_rule_iterative_pieces_search(grid, _found_pieces, neighbor);
    }
  }
  _found_pieces
}
