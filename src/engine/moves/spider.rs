use crate::engine::{
  grid::{coordinate::hex::Hex, Grid},
  moves::extract_moves_from_paths,
  rules::{freedom_to_move_rule, one_hive_rule},
};

pub fn spider_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let initial_path: Vec<Hex> = vec![*hex];
  let paths: Vec<Vec<Hex>> = spider_moves_it(grid, hex, hex, initial_path);

  extract_moves_from_paths(paths, 4)
}

fn spider_moves_it(grid: &Grid, hex: &Hex, initital_hex: &Hex, path: Vec<Hex>) -> Vec<Vec<Hex>> {
  let mut _path = path;
  let mut paths: Vec<Vec<Hex>> = Vec::new();

  for neighbor in hex.neighbors() {
    if !_path.contains(&neighbor)
      && !grid.is_hex_occupied(&neighbor)
      && one_hive_rule(grid, initital_hex, &neighbor)
      && freedom_to_move_rule(grid, hex, &neighbor)
    {
      let mut current_path = _path.clone();
      current_path.push(neighbor);

      if current_path.len() < 4 {
        let mut new_paths = spider_moves_it(grid, &neighbor, initital_hex, current_path);
        paths.append(&mut new_paths);
      } else {
        paths.push(current_path);
      }
    }
  }

  paths
}
