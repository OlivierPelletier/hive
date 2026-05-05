use crate::engine::{
  game::action::Action,
  grid::{
    Grid,
    coordinate::hex::Hex,
    piece::{Piece, PieceColor},
  },
  moves::{available_moves, available_placements_for_piece_color},
};

/*
  __    W MSQT  B LDBG    __      __    W QBEE  W SANT
 -3,-1   -2,-1   -1,-1   0,-1    1,-1    2,-1    3,-1
    B QBEE  B GRHP  B BETL  W BETL  W GRHP  W GRHP    __
     -3,0    -2,0    -1,0     0,0     1,0     2,0     3,0
        B SANT  B BETL    __    W SPDR    __      __      __
         -3,1    -2,1    -1,1     0,1     1,1     2,1     3,1
*/
fn initialize_grid() -> Grid {
  let mut grid = Grid::new();

  grid.place_piece_to_hex(Piece::queen_bee().white(), Hex::new(2, -1));
  grid.place_piece_to_hex(Piece::queen_bee().black(), Hex::new(-3, -0));
  grid.place_piece_to_hex(Piece::beetle().white(), Hex::new(0, 0));
  grid.place_piece_to_hex(Piece::beetle().black(), Hex::new(-2, 1));
  grid.place_piece_to_hex(Piece::beetle().black(), Hex::new(-1, 0));
  grid.place_piece_to_hex(Piece::grasshopper().white(), Hex::new(1, 0));
  grid.place_piece_to_hex(Piece::grasshopper().white(), Hex::new(2, 0));
  grid.place_piece_to_hex(Piece::grasshopper().black(), Hex::new(-2, 0));
  grid.place_piece_to_hex(Piece::soldier_ant().black(), Hex::new(-3, 1));
  grid.place_piece_to_hex(Piece::soldier_ant().white(), Hex::new(3, -1));
  grid.place_piece_to_hex(Piece::spider().white(), Hex::new(0, 1));
  grid.place_piece_to_hex(Piece::ladybug().black(), Hex::new(-1, -1));
  grid.place_piece_to_hex(Piece::mosquito().white(), Hex::new(-2, -1));

  grid
}

fn move_action(piece: Piece, from: Hex, to: Hex) -> Action {
  Action {
    piece,
    from,
    to,
    in_hand: false,
    is_pillbug_special_move: false,
  }
}

fn move_action_pillbug_flip(piece: Piece, from: Hex, to: Hex) -> Action {
  Action {
    piece,
    from,
    to,
    in_hand: false,
    is_pillbug_special_move: true,
  }
}

fn sort_actions(actions: &mut [Action]) {
  actions.sort_by_key(|action| (action.from.q, action.from.r, action.to.q, action.to.r));
}

#[test]
fn given_grid_when_available_moves_queenbee_should_return_correct_moves() {
  let grid = initialize_grid();
  let from = Hex::new(-3, 0);
  let piece = Piece::queen_bee().black();
  let mut correct_moves = vec![
    move_action(piece, from, Hex::new(-4, 1)),
    move_action(piece, from, Hex::new(-3, -1)),
  ];

  let mut moves = available_moves(&grid, &from, &Vec::new());

  sort_actions(&mut moves);
  sort_actions(&mut correct_moves);

  assert_eq!(moves, correct_moves);
}

#[test]
fn given_grid_when_available_moves_beetle_should_return_correct_moves() {
  let grid = initialize_grid();
  let from = Hex::new(-2, 1);
  let piece = Piece::beetle().black();
  let mut correct_moves = vec![
    move_action(piece, from, Hex::new(-1, 0)),
    move_action(piece, from, Hex::new(-1, 1)),
    move_action(piece, from, Hex::new(-3, 2)),
    move_action(piece, from, Hex::new(-3, 1)),
    move_action(piece, from, Hex::new(-2, 0)),
  ];

  let mut moves = available_moves(&grid, &from, &Vec::new());

  sort_actions(&mut moves);
  sort_actions(&mut correct_moves);

  assert_eq!(moves, correct_moves);
}

#[test]
fn given_grid_when_available_moves_grasshopper_should_return_correct_moves() {
  let grid = initialize_grid();
  let from = Hex::new(2, 0);
  let piece = Piece::grasshopper().white();
  let mut correct_moves = vec![
    move_action(piece, from, Hex::new(4, -2)),
    move_action(piece, from, Hex::new(-4, 0)),
    move_action(piece, from, Hex::new(2, -2)),
  ];

  let mut moves = available_moves(&grid, &from, &Vec::new());

  sort_actions(&mut moves);
  sort_actions(&mut correct_moves);

  assert_eq!(moves, correct_moves);
}

#[test]
fn given_grid_when_available_moves_spider_should_return_correct_moves() {
  let grid = initialize_grid();
  let from = Hex::new(0, 1);
  let piece = Piece::spider().white();
  let mut correct_moves = vec![
    move_action(piece, from, Hex::new(3, 0)),
    move_action(piece, from, Hex::new(-3, 2)),
  ];

  let mut moves = available_moves(&grid, &from, &Vec::new());

  sort_actions(&mut moves);
  sort_actions(&mut correct_moves);

  assert_eq!(moves, correct_moves);
}

#[test]
fn given_grid_when_available_moves_ladybug_should_return_correct_moves() {
  let grid = initialize_grid();
  let from = Hex::new(-1, -1);
  let piece = Piece::ladybug().black();
  let mut correct_moves = vec![
    move_action(piece, from, Hex::new(1, -1)),
    move_action(piece, from, Hex::new(-1, 1)),
    move_action(piece, from, Hex::new(0, -1)),
    move_action(piece, from, Hex::new(-2, 2)),
    move_action(piece, from, Hex::new(-3, 2)),
    move_action(piece, from, Hex::new(-4, 2)),
    move_action(piece, from, Hex::new(-4, 1)),
    move_action(piece, from, Hex::new(-4, 0)),
    move_action(piece, from, Hex::new(-3, -1)),
    move_action(piece, from, Hex::new(-1, -2)),
    move_action(piece, from, Hex::new(-2, -2)),
  ];

  let mut moves = available_moves(&grid, &from, &Vec::new());

  sort_actions(&mut moves);
  sort_actions(&mut correct_moves);

  assert_eq!(moves, correct_moves);
}

#[test]
fn given_grid_when_available_moves_solider_ant_should_return_correct_moves() {
  let grid = initialize_grid();
  let from = Hex::new(3, -1);
  let piece = Piece::soldier_ant().white();
  let mut correct_moves = vec![
    move_action(piece, from, Hex::new(3, 0)),
    move_action(piece, from, Hex::new(2, 1)),
    move_action(piece, from, Hex::new(1, 1)),
    move_action(piece, from, Hex::new(0, 2)),
    move_action(piece, from, Hex::new(-1, 2)),
    move_action(piece, from, Hex::new(-2, 2)),
    move_action(piece, from, Hex::new(-1, 1)),
    move_action(piece, from, Hex::new(-3, 2)),
    move_action(piece, from, Hex::new(-4, 2)),
    move_action(piece, from, Hex::new(-4, 1)),
    move_action(piece, from, Hex::new(-4, 0)),
    move_action(piece, from, Hex::new(-3, -1)),
    move_action(piece, from, Hex::new(-2, -2)),
    move_action(piece, from, Hex::new(-1, -2)),
    move_action(piece, from, Hex::new(0, -2)),
    move_action(piece, from, Hex::new(0, -1)),
    move_action(piece, from, Hex::new(1, -1)),
    move_action(piece, from, Hex::new(2, -2)),
    move_action(piece, from, Hex::new(3, -2)),
  ];

  let mut moves = available_moves(&grid, &from, &Vec::new());

  sort_actions(&mut moves);
  sort_actions(&mut correct_moves);

  assert_eq!(moves, correct_moves);
}

#[test]
fn given_grid_when_available_moves_mosquito_should_return_correct_moves() {
  let grid = initialize_grid();
  let from = Hex::new(-2, -1);
  let piece = Piece::mosquito().white();
  let mut correct_moves = vec![
    move_action(piece, from, Hex::new(0, -1)),
    move_action(piece, from, Hex::new(-1, 1)),
    move_action(piece, from, Hex::new(0, -2)),
    move_action(piece, from, Hex::new(-1, -2)),
    move_action(piece, from, Hex::new(-3, 2)),
    move_action(piece, from, Hex::new(-2, 2)),
    move_action(piece, from, Hex::new(-4, 2)),
    move_action(piece, from, Hex::new(-4, 1)),
    move_action(piece, from, Hex::new(-3, -1)),
    move_action(piece, from, Hex::new(-4, 0)),
  ];

  let mut moves = available_moves(&grid, &from, &Vec::new());
  sort_actions(&mut moves);
  sort_actions(&mut correct_moves);

  assert_eq!(moves, correct_moves);
}

#[test]
fn given_grid_when_available_placements_by_color_should_return_correct_placements() {
  let grid = initialize_grid();
  let mut correct_placements = vec![
    Hex { q: 0, r: -2 },
    Hex { q: -4, r: 1 },
    Hex { q: -4, r: 0 },
    Hex { q: -4, r: 2 },
    Hex { q: -3, r: 2 },
    Hex { q: -2, r: 2 },
  ];

  let mut placements = available_placements_for_piece_color(&grid, &PieceColor::BLACK);
  placements.sort();
  correct_placements.sort();

  assert_eq!(placements, correct_placements);
}

/*
   __      __      __      __      __      __      __      __    W BETL
  -3,-1   -2,-1   -1,-1   0,-1    1,-1    2,-1    3,-1    4,-1    5,-1
     B QBEE  B SANT  B GRHP  W GRHP  W SANT (W BETL) W BETL  W BETL    __
      -3,0    -2,0    -1,0     0,0     1,0     2,0     3,0     4,0     5,0
         B SANT  B SANT  B SANT    __   (W BETL) W SANT (W BETL)   __      __
          -3,1    -2,1    -1,1     0,1     1,1     2,1     3,1     4,1     5,1
               __      __      __      __    W LDBG    __      __      __      __
              -3,2    -2,2    -1,2     0,2     1,2     2,2     3,2     4,2     5,2
*/
fn initialize_stacked_grid() -> Grid {
  let mut grid = Grid::new();

  grid.place_piece_to_hex(Piece::grasshopper().white(), Hex::new(0, 0));
  grid.place_piece_to_hex(Piece::soldier_ant().white(), Hex::new(1, 0));
  grid.place_piece_to_hex(Piece::queen_bee().white(), Hex::new(2, 0));
  grid.place_piece_to_hex(Piece::beetle().white(), Hex::new(2, 0));
  grid.place_piece_to_hex(Piece::soldier_ant().white(), Hex::new(1, 1));
  grid.place_piece_to_hex(Piece::soldier_ant().white(), Hex::new(2, 1));
  grid.place_piece_to_hex(Piece::soldier_ant().white(), Hex::new(3, 1));
  grid.place_piece_to_hex(Piece::beetle().white(), Hex::new(3, 0));
  grid.place_piece_to_hex(Piece::beetle().white(), Hex::new(1, 1));
  grid.place_piece_to_hex(Piece::beetle().white(), Hex::new(3, 1));
  grid.place_piece_to_hex(Piece::beetle().white(), Hex::new(3, 1));
  grid.place_piece_to_hex(Piece::beetle().white(), Hex::new(4, 0));
  grid.place_piece_to_hex(Piece::beetle().white(), Hex::new(5, -1));
  grid.place_piece_to_hex(Piece::ladybug().white(), Hex::new(1, 2));
  grid.place_piece_to_hex(Piece::grasshopper().black(), Hex::new(-1, 0));
  grid.place_piece_to_hex(Piece::soldier_ant().black(), Hex::new(-2, 0));
  grid.place_piece_to_hex(Piece::queen_bee().black(), Hex::new(-3, 0));
  grid.place_piece_to_hex(Piece::soldier_ant().black(), Hex::new(-1, 1));
  grid.place_piece_to_hex(Piece::soldier_ant().black(), Hex::new(-2, 1));
  grid.place_piece_to_hex(Piece::soldier_ant().black(), Hex::new(-3, 1));

  grid
}

#[test]
fn given_stacked_grid_when_available_moves_beetle_should_return_correct_moves() {
  let grid = initialize_stacked_grid();
  let from = Hex::new(3, 0);
  let piece = Piece::beetle().white();
  let mut correct_moves = vec![
    move_action(piece, from, Hex::new(2, 0)),
    move_action(piece, from, Hex::new(3, -1)),
    move_action(piece, from, Hex::new(3, 1)),
    move_action(piece, from, Hex::new(4, -1)),
    move_action(piece, from, Hex::new(4, 0)),
  ];

  let mut moves = available_moves(&grid, &from, &Vec::new());

  sort_actions(&mut moves);
  sort_actions(&mut correct_moves);

  assert_eq!(moves, correct_moves);
}

#[test]
fn given_stacked_grid_when_available_moves_ladybug_should_return_correct_moves() {
  let grid = initialize_stacked_grid();
  let from = Hex::new(1, 2);
  let piece = Piece::ladybug().white();
  let mut correct_moves = vec![
    move_action(piece, from, Hex::new(0, 2)),
    move_action(piece, from, Hex::new(0, 1)),
    move_action(piece, from, Hex::new(1, -1)),
    move_action(piece, from, Hex::new(2, -1)),
    move_action(piece, from, Hex::new(3, -1)),
    move_action(piece, from, Hex::new(2, 2)),
    move_action(piece, from, Hex::new(3, 2)),
    move_action(piece, from, Hex::new(4, 1)),
  ];

  let mut moves = available_moves(&grid, &from, &Vec::new());

  sort_actions(&mut moves);
  sort_actions(&mut correct_moves);

  assert_eq!(moves, correct_moves);
}

/*
   __      __    B PLBG (W PLBG) W PLBG
  -2,0    -1,0     0,0     1,0     2,0
     B PLBG  B PLBG    __    W PLBG (W PLBG)
      -2,1    -1,1     0,1     1,1     2,1
         B PLBG  B PLBG    __      __      __
          -2,2    -1,2     0,2     1,2     2,2
*/
fn initialize_pillbug_grid() -> Grid {
  let mut grid = Grid::new();

  grid.place_piece_to_hex(Piece::pillbug().black(), Hex::new(0, 0));
  grid.place_piece_to_hex(Piece::pillbug().black(), Hex::new(-1, 1));
  grid.place_piece_to_hex(Piece::pillbug().black(), Hex::new(-2, 1));
  grid.place_piece_to_hex(Piece::pillbug().black(), Hex::new(-2, 2));
  grid.place_piece_to_hex(Piece::pillbug().black(), Hex::new(-1, 2));
  grid.place_piece_to_hex(Piece::pillbug().white(), Hex::new(1, 1));
  grid.place_piece_to_hex(Piece::pillbug().white(), Hex::new(1, 0));
  grid.place_piece_to_hex(Piece::pillbug().white(), Hex::new(1, 0));
  grid.place_piece_to_hex(Piece::pillbug().white(), Hex::new(2, 1));
  grid.place_piece_to_hex(Piece::pillbug().white(), Hex::new(2, 1));
  grid.place_piece_to_hex(Piece::pillbug().white(), Hex::new(2, 0));

  grid
}

#[test]
fn given_grid_when_available_moves_pillbug_should_return_correct_moves() {
  let grid = initialize_pillbug_grid();
  let from = Hex::new(-2, 2);
  let piece = Piece::pillbug().black();
  let mut correct_moves = vec![
    move_action(piece, from, Hex::new(-3, 2)),
    move_action(piece, from, Hex::new(-2, 3)),
    move_action_pillbug_flip(piece, Hex::new(-1, 2), Hex::new(-3, 2)),
    move_action_pillbug_flip(piece, Hex::new(-2, 1), Hex::new(-2, 3)),
  ];

  let mut moves = available_moves(
    &grid,
    &from,
    &[Action {
      piece: Piece::pillbug().white(),
      from: Hex::new(1, 2),
      to: Hex::new(1, 1),
      in_hand: false,
      is_pillbug_special_move: false,
    }],
  );

  sort_actions(&mut moves);
  sort_actions(&mut correct_moves);

  assert_eq!(moves, correct_moves);
}

#[test]
fn given_grid_when_available_moves_pillbug_with_stacked_pieces_should_return_correct_moves() {
  let grid = initialize_pillbug_grid();
  let from = Hex::new(1, 1);
  let piece = Piece::pillbug().white();
  let mut correct_moves = vec![
    move_action(piece, from, Hex::new(0, 1)),
    move_action(piece, from, Hex::new(0, 2)),
    move_action(piece, from, Hex::new(1, 2)),
  ];

  let mut moves = available_moves(&grid, &from, &Vec::new());

  sort_actions(&mut moves);
  sort_actions(&mut correct_moves);

  assert_eq!(moves, correct_moves);
}

/*
   __   (W PLBG)   __
   0,0     1,0     2,0
       __    W PLBG (W PLBG)
       0,1     1,1     2,1
         W PLBG    __      __
           0,2     1,2     2,2
*/
fn initialize_pillbug_extra_stacked_scenario_grid() -> Grid {
  let mut grid = Grid::new();

  grid.place_piece_to_hex(Piece::pillbug().white(), Hex::new(1, 1));
  grid.place_piece_to_hex(Piece::pillbug().white(), Hex::new(1, 0));
  grid.place_piece_to_hex(Piece::pillbug().white(), Hex::new(1, 0));
  grid.place_piece_to_hex(Piece::pillbug().white(), Hex::new(2, 1));
  grid.place_piece_to_hex(Piece::pillbug().white(), Hex::new(2, 1));
  grid.place_piece_to_hex(Piece::pillbug().white(), Hex::new(0, 2));

  grid
}

#[test]
fn given_grid_when_available_moves_pillbug_with_extra_stacked_pieces_should_return_empty_moves() {
  let grid = initialize_pillbug_extra_stacked_scenario_grid();
  let from = Hex::new(1, 1);

  let moves = available_moves(&grid, &from, &Vec::new());

  assert!(moves.is_empty());
}
