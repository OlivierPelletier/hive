use crate::grid::geo::cube::Cube;
use crate::grid::geo::hex::Hex;
use crate::grid::Grid;
use crate::rules::{freedom_to_move_rule, one_hive_rule};

pub fn queen_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let mut moves: Vec<Hex> = Vec::new();

  for neighbor in hex.neighbors() {
    if one_hive_rule(grid, hex, &neighbor)
      && freedom_to_move_rule(grid, hex, &neighbor)
      && !grid.is_hex_occupied(&neighbor)
    {
      moves.push(neighbor);
    }
  }

  moves
}

pub fn beetle_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let mut moves: Vec<Hex> = Vec::new();

  for neighbor in hex.neighbors() {
    if one_hive_rule(grid, hex, &neighbor)
      && ((!grid.is_hex_occupied(hex) && freedom_to_move_rule(grid, hex, &neighbor))
        || grid.is_hex_occupied(hex))
    {
      moves.push(neighbor);
    }
  }

  moves
}

pub fn grasshopper_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
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

      if one_hive_rule(grid, hex, &line_hex) {
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

      if one_hive_rule(grid, hex, &line_hex) {
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

      if one_hive_rule(grid, hex, &line_hex) {
        moves.push(line_hex);
      }
    }
  }

  moves
}

pub fn spider_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let mut moves: Vec<Hex> = Vec::new();
  let start_trail: Vec<Hex> = vec![*hex];

  let trails: Vec<Vec<Hex>> = spider_moves_it(grid, hex, hex, start_trail);

  for trail in trails {
    if trail.len() == 4 {
      match trail.last() {
        Some(h) => {
          if !moves.contains(h) {
            moves.push(*h);
          }
        }
        None => (),
      }
    }
  }

  moves
}

fn spider_moves_it(grid: &Grid, initital_hex: &Hex, hex: &Hex, trail: Vec<Hex>) -> Vec<Vec<Hex>> {
  let mut _trail = trail.clone();
  let mut trails: Vec<Vec<Hex>> = Vec::new();

  for neighbor in hex.neighbors() {
    if !_trail.contains(&neighbor)
      && !grid.is_hex_occupied(&neighbor)
      && one_hive_rule(grid, initital_hex, &neighbor)
      && freedom_to_move_rule(grid, &hex, &neighbor)
    {
      let mut temp_trail = _trail.clone();
      temp_trail.push(neighbor);

      if temp_trail.len() < 4 {
        let mut new_trails = spider_moves_it(grid, initital_hex, &neighbor, temp_trail);
        trails.append(&mut new_trails);
      } else {
        trails.push(temp_trail);
      }
    }
  }

  trails
}

pub fn ladybug_moves(grid: &Grid, hex: &Hex) -> Vec<Hex> {
  let mut moves: Vec<Hex> = Vec::new();
  let start_trail: Vec<Hex> = vec![*hex];

  let trails: Vec<Vec<Hex>> = ladybug_moves_it(grid, hex, hex, start_trail);

  for trail in trails {
    if trail.len() == 4 {
      match trail.last() {
        Some(h) => {
          if !moves.contains(h) {
            moves.push(*h);
          }
        }
        None => (),
      }
    }
  }

  moves
}

fn ladybug_moves_it(grid: &Grid, initital_hex: &Hex, hex: &Hex, trail: Vec<Hex>) -> Vec<Vec<Hex>> {
  let mut _trail = trail.clone();
  let mut trails: Vec<Vec<Hex>> = Vec::new();

  for neighbor in hex.neighbors() {
    if !_trail.contains(&neighbor)
      && one_hive_rule(grid, initital_hex, &neighbor)
      && (((_trail.len() == 1 || _trail.len() == 2) && grid.is_hex_occupied(&neighbor))
        || (_trail.len() == 3 && !grid.is_hex_occupied(&neighbor)))
    {
      let mut temp_trail = _trail.clone();
      temp_trail.push(neighbor);

      if temp_trail.len() < 4 {
        let mut new_trails = ladybug_moves_it(grid, initital_hex, &neighbor, temp_trail);
        trails.append(&mut new_trails);
      } else {
        trails.push(temp_trail);
      }
    }
  }

  trails
}
