use crate::engine::{
  grid::{coordinate::hex::Hex, Grid},
  moves::extract_moves_from_paths,
  rules::{freedom_to_move_rule, one_hive_rule},
};

pub fn ladybug_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let initial_path: Vec<Hex> = vec![*hex];
  let paths: Vec<Vec<Hex>> = ladybug_moves_it(grid, hex, hex, &initial_path);

  extract_moves_from_paths(paths, 4)
}

fn ladybug_moves_it(grid: &Grid, hex: &Hex, initital_hex: &Hex, path: &Vec<Hex>) -> Vec<Vec<Hex>> {
  let mut paths: Vec<Vec<Hex>> = Vec::new();

  for neighbor in hex.neighbors() {
    if path.contains(&neighbor) {
      continue;
    }
    if !one_hive_rule(grid, initital_hex, &neighbor) {
      continue;
    }
    if !freedom_to_move_rule(grid, hex, &neighbor) {
      continue;
    }
    if path.len() == 1 && !grid.is_hex_occupied(&neighbor) {
      continue;
    }
    if path.len() == 2 && !grid.is_hex_occupied(&neighbor) {
      continue;
    }
    if path.len() == 3 && grid.is_hex_occupied(&neighbor) {
      continue;
    }

    let mut current_path = path.clone();

    current_path.push(neighbor);

    if current_path.len() < 4 {
      let mut new_paths = ladybug_moves_it(grid, &neighbor, initital_hex, &current_path);
      paths.append(&mut new_paths);
    } else {
      paths.push(current_path);
    }
  }

  paths
}
