use crate::engine::{
  grid::{coordinate::hex::Hex, Grid},
  rules::{freedom_to_move_rule, one_hive_rule},
};

pub fn soldier_ant_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let start_trail: Vec<Hex> = vec![*hex];

  let moves: Vec<Hex> = soldier_ant_moves_it(grid, hex, hex, start_trail, Vec::new());

  moves
}

fn soldier_ant_moves_it(
  grid: &Grid,
  initital_hex: &Hex,
  hex: &Hex,
  trail: Vec<Hex>,
  moves: Vec<Hex>,
) -> Vec<Hex> {
  let mut _trail = trail;
  let mut _moves = moves;

  for neighbor in hex.neighbors() {
    if !_trail.contains(&neighbor)
      && !grid.is_hex_occupied(&neighbor)
      && one_hive_rule(grid, initital_hex, &neighbor)
      && freedom_to_move_rule(grid, hex, &neighbor)
    {
      let mut temp_trail = _trail.clone();
      temp_trail.push(neighbor);

      if !_moves.contains(&neighbor) {
        _moves.push(neighbor)
      }

      _moves = soldier_ant_moves_it(grid, initital_hex, &neighbor, temp_trail, _moves);
    }
  }

  _moves
}
