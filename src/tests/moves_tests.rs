use crate::engine::grid::coordinate::hex::Hex;
use crate::engine::grid::piece::Piece;
use crate::engine::grid::Grid;
use crate::engine::moves::available_moves;

fn setup_grid() -> Grid {
  Grid::new()
    .place_piece_to_hex(Piece::queen_bee().white(), &Hex::new(2, -1))
    .place_piece_to_hex(Piece::queen_bee().black(), &Hex::new(-3, -0))
    .place_piece_to_hex(Piece::beetle().white(), &Hex::new(0, 0))
    .place_piece_to_hex(Piece::beetle().black(), &Hex::new(-2, 1))
    .place_piece_to_hex(Piece::beetle().black(), &Hex::new(-1, 0))
    .place_piece_to_hex(Piece::grasshopper().white(), &Hex::new(1, 0))
    .place_piece_to_hex(Piece::grasshopper().white(), &Hex::new(2, 0))
    .place_piece_to_hex(Piece::grasshopper().black(), &Hex::new(-2, 0))
    .place_piece_to_hex(Piece::soldier_ant().black(), &Hex::new(-3, 1))
    .place_piece_to_hex(Piece::soldier_ant().white(), &Hex::new(3, -1))
    .place_piece_to_hex(Piece::spider().white(), &Hex::new(0, 1))
    .place_piece_to_hex(Piece::ladybug().black(), &Hex::new(-1, -1))
    .place_piece_to_hex(Piece::mosquito().white(), &Hex::new(-2, -1))
}

#[test]
fn given_grid_when_available_moves_queenbee_should_return_correct_moves() {
  let grid = setup_grid();
  let correct_moves: Vec<Hex> = vec![Hex { q: -4, r: 1 }, Hex { q: -3, r: -1 }];

  let moves = available_moves(&grid, &Hex::new(-3, -0));

  assert_eq!(moves.len(), correct_moves.len());
  iterative_moves_assert(&moves, &correct_moves);
}

#[test]
fn given_grid_when_available_moves_beetle_should_return_correct_moves() {
  let grid = setup_grid();
  let correct_moves: Vec<Hex> = vec![
    Hex { q: -1, r: 0 },
    Hex { q: -1, r: 1 },
    Hex { q: -3, r: 2 },
    Hex { q: -3, r: 1 },
    Hex { q: -2, r: 0 },
  ];

  let moves = available_moves(&grid, &Hex::new(-2, 1));

  assert_eq!(moves.len(), correct_moves.len());
  iterative_moves_assert(&moves, &correct_moves);
}

#[test]
fn given_grid_when_available_moves_grasshopper_should_return_correct_moves() {
  let grid = setup_grid();
  let correct_moves: Vec<Hex> = vec![
    Hex { q: 4, r: -2 },
    Hex { q: -4, r: 0 },
    Hex { q: 2, r: -2 },
  ];

  let moves = available_moves(&grid, &Hex::new(2, 0));

  assert_eq!(moves.len(), correct_moves.len());
  iterative_moves_assert(&moves, &correct_moves);
}

#[test]
fn given_grid_when_available_moves_spider_should_return_correct_moves() {
  let grid = setup_grid();
  let correct_moves: Vec<Hex> = vec![Hex { q: 3, r: 0 }, Hex { q: -3, r: 2 }];

  let moves = available_moves(&grid, &Hex::new(0, 1));

  assert_eq!(moves.len(), correct_moves.len());
  iterative_moves_assert(&moves, &correct_moves);
}

#[test]
fn given_grid_when_available_moves_ladybug_should_return_correct_moves() {
  let grid = setup_grid();
  let correct_moves: Vec<Hex> = vec![
    Hex { q: 1, r: -1 },
    Hex { q: -1, r: 1 },
    Hex { q: 0, r: -1 },
    Hex { q: -2, r: 2 },
    Hex { q: -3, r: 2 },
    Hex { q: -4, r: 2 },
    Hex { q: -4, r: 1 },
    Hex { q: -4, r: 0 },
    Hex { q: -3, r: -1 },
    Hex { q: -1, r: -2 },
    Hex { q: -2, r: -2 },
  ];

  let moves = available_moves(&grid, &Hex::new(-1, -1));

  assert_eq!(moves.len(), correct_moves.len());
  iterative_moves_assert(&moves, &correct_moves);
}

#[test]
fn given_grid_when_available_moves_solider_ant_should_return_correct_moves() {
  let grid = setup_grid();
  let correct_moves: Vec<Hex> = vec![
    Hex { q: 3, r: 0 },
    Hex { q: 2, r: 1 },
    Hex { q: 1, r: 1 },
    Hex { q: 0, r: 2 },
    Hex { q: -1, r: 2 },
    Hex { q: -2, r: 2 },
    Hex { q: -1, r: 1 },
    Hex { q: -3, r: 2 },
    Hex { q: -4, r: 2 },
    Hex { q: -4, r: 1 },
    Hex { q: -4, r: 0 },
    Hex { q: -3, r: -1 },
    Hex { q: -2, r: -2 },
    Hex { q: -1, r: -2 },
    Hex { q: 0, r: -2 },
    Hex { q: 0, r: -1 },
    Hex { q: 1, r: -1 },
    Hex { q: 2, r: -2 },
    Hex { q: 3, r: -2 },
  ];

  let moves = available_moves(&grid, &Hex::new(3, -1));

  assert_eq!(moves.len(), correct_moves.len());
  iterative_moves_assert(&moves, &correct_moves);
}

#[test]
fn given_grid_when_available_moves_mosquito_should_return_correct_moves() {
  let grid = setup_grid();
  let correct_moves: Vec<Hex> = vec![
    Hex { q: 0, r: -1 },
    Hex { q: -1, r: 1 },
    Hex { q: 0, r: -2 },
    Hex { q: -1, r: -2 },
    Hex { q: -2, r: 2 },
    Hex { q: -3, r: 2 },
    Hex { q: -4, r: 2 },
    Hex { q: -4, r: 1 },
    Hex { q: -4, r: 0 },
    Hex { q: -3, r: -1 },
  ];

  let moves = available_moves(&grid, &Hex::new(-2, -1));

  assert_eq!(moves.len(), correct_moves.len());
  iterative_moves_assert(&moves, &correct_moves);
}

fn iterative_moves_assert(moves: &Vec<Hex>, correct_moves: &Vec<Hex>) {
  for correct_move in correct_moves {
    assert!(moves.contains(correct_move));
  }
}
