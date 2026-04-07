use crate::engine::{
  game::action::Action,
  grid::{Grid, coordinate::hex::Hex, piece::Piece},
  moves::extract_moves_from_paths,
  rules::{freedom_to_move_rule, one_hive_rule},
};

pub fn spider_moves(grid: &Grid, piece: &Piece, hex: &Hex) -> Vec<Action> {
  let initial_path: Vec<Hex> = vec![*hex];
  let paths: Vec<Vec<Hex>> = spider_moves_it(grid, hex, hex, initial_path);

  let moves = extract_moves_from_paths(paths, 4);

  let mut actions = Vec::new();

  for m in moves {
    actions.push(Action {
      piece: *piece,
      from: *hex,
      to: m,
      in_hand: false,
    })
  }

  actions
}

fn spider_moves_it(grid: &Grid, hex: &Hex, initital_hex: &Hex, path: Vec<Hex>) -> Vec<Vec<Hex>> {
  let mut paths: Vec<Vec<Hex>> = Vec::new();

  for neighbor in hex.neighbors() {
    if grid.is_hex_occupied(&neighbor) {
      continue;
    }
    if path.contains(&neighbor) {
      continue;
    }
    if !one_hive_rule(grid, initital_hex, &neighbor) {
      continue;
    }
    if !freedom_to_move_rule(grid, hex, &neighbor) {
      continue;
    }

    let mut current_path = path.clone();
    current_path.push(neighbor);

    if current_path.len() < 4 {
      let mut new_paths = spider_moves_it(grid, &neighbor, initital_hex, current_path);
      paths.append(&mut new_paths);
    } else {
      paths.push(current_path);
    }
  }

  paths
}
