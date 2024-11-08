use super::{coordinate::hex::Hex, piece::Piece, Grid};

#[test]
fn given_grid_when_placing_piece_to_hex_then_hex_contains_piece() {
  let mut grid = Grid::new();
  let hex = Hex::new(0, 0);
  let queen_bee = Piece::queen_bee();

  grid.place_piece_to_hex(queen_bee, hex);

  match grid.grid.get(&hex) {
    Some(p) => {
      let piece = p;
      assert_eq!(*piece.last().unwrap(), queen_bee);
    }
    None => assert!(false),
  }
}

#[test]
fn given_grid_when_removing_piece_from_hex_then_piece_is_removed_from_hex() {
  let mut grid = Grid::new();
  let hex = Hex::new(0, 0);

  grid.place_piece_to_hex(Piece::queen_bee(), hex);
  grid.remove_top_piece_from_hex(hex);

  match grid.grid.get(&hex) {
    Some(p) => assert_eq!(p.len(), 0),
    None => assert!(false),
  }
}

#[test]
fn given_grid_when_removing_piece_from_hex_containing_two_pieces_then_top_piece_is_removed_from_hex(
) {
  let mut grid = Grid::new();
  let hex = Hex::new(0, 0);
  let queen_bee = Piece::queen_bee();
  let beetle = Piece::beetle();
  grid.place_piece_to_hex(queen_bee, hex);
  grid.place_piece_to_hex(beetle, hex);
  grid.remove_top_piece_from_hex(hex);

  match grid.grid.get(&hex) {
    Some(p) => {
      let piece = p;
      assert_eq!(piece.last(), Some(&queen_bee));
    }
    None => assert!(false),
  }
}

#[test]
fn given_grid_when_adding_two_pieces_on_same_hex_then_hex_contains_both_pieces() {
  let mut grid = Grid::new();
  let hex = Hex::new(0, 0);
  let queen_bee = Piece::queen_bee();
  let spider = Piece::spider();

  grid.place_piece_to_hex(queen_bee, hex);
  grid.place_piece_to_hex(spider, hex);

  match grid.grid.get(&hex) {
    Some(p) => {
      let piece = p;
      assert_eq!(p.len(), 2);
      assert_eq!(piece.first(), Some(&queen_bee));
      assert_eq!(piece.last(), Some(&spider));
    }
    None => assert!(false),
  }
}

#[test]
fn given_grid_when_moving_piece_from_hex_to_hex_then_piece_is_moved() {
  let mut grid = Grid::new();
  let from = Hex::new(0, 0);
  let to = Hex::new(0, 1);
  let queen_bee = Piece::queen_bee();
  grid.place_piece_to_hex(queen_bee, from);

  grid.move_piece_from_to(from, to);

  match grid.grid.get(&from) {
    Some(p) => {
      assert_eq!(p.len(), 0);
    }
    None => assert!(false),
  }

  match grid.grid.get(&to) {
    Some(p) => {
      let piece = p;
      assert_eq!(piece.last(), Some(&queen_bee));
    }
    None => assert!(false),
  }
}

#[test]
fn given_grid_when_moving_piece_from_hex_to_occupied_hex_then_piece_is_moved_and_hex_contains_both_pieces(
) {
  let mut grid = Grid::new();
  let from = Hex::new(0, 0);
  let to = Hex::new(0, 1);
  let queen_bee = Piece::queen_bee();
  let beetle = Piece::beetle();
  grid.place_piece_to_hex(beetle, from);
  grid.place_piece_to_hex(queen_bee, to);

  grid.move_piece_from_to(from, to);

  match grid.grid.get(&from) {
    Some(p) => {
      assert_eq!(p.len(), 0);
    }
    None => assert!(false),
  }

  match grid.grid.get(&to) {
    Some(p) => {
      let piece = p;
      assert_eq!(piece.last(), Some(&beetle));
      assert_eq!(piece.first(), Some(&queen_bee));
    }
    None => assert!(false),
  }
}

#[test]
fn given_grid_when_finding_top_piece_then_top_piece_is_returned() {
  let mut grid = Grid::new();
  let hex = Hex::new(0, 0);
  let queen_bee = Piece::queen_bee();
  let ladybug = Piece::ladybug();

  grid.place_piece_to_hex(queen_bee, hex);
  grid.place_piece_to_hex(ladybug, hex);

  let piece = grid.find_top_piece(&hex).unwrap();

  assert_eq!(piece, &ladybug);
}

#[test]
fn given_empty_grid_when_finding_top_piece_then_no_piece_is_returned() {
  let grid = Grid::new();
  let hex = Hex::new(0, 0);

  let piece = grid.find_top_piece(&hex);

  assert_eq!(piece.is_none(), true);
}

#[test]
fn given_empty_grid_when_is_hex_surrended_should_return_false() {
  let grid = Grid::new();
  let hex = Hex::new(0, 0);

  assert!(!grid.is_hex_surrounded(&hex))
}

#[test]
fn given_grid_with_surrended_hex_when_is_hex_surrended_should_return_true() {
  let center = Hex::new(-2, 0);
  let hex1 = Hex::new(-3, 0);
  let hex2 = Hex::new(-3, 1);
  let hex3 = Hex::new(-2, 1);
  let hex4 = Hex::new(-1, 0);
  let hex5 = Hex::new(-1, -1);
  let hex6 = Hex::new(-2, -1);
  let mut grid = Grid::new();
  grid.place_piece_to_hex(Piece::beetle(), hex1);
  grid.place_piece_to_hex(Piece::beetle(), hex2);
  grid.place_piece_to_hex(Piece::beetle(), hex3);
  grid.place_piece_to_hex(Piece::beetle(), hex4);
  grid.place_piece_to_hex(Piece::beetle(), hex5);
  grid.place_piece_to_hex(Piece::beetle(), hex6);

  assert!(grid.is_hex_surrounded(&center))
}

#[test]
fn given_grid_with_partially_surrended_hex_when_is_hex_surrended_should_return_false() {
  let center = Hex::new(-2, 0);
  let hex1 = Hex::new(-3, 0);
  let hex2 = Hex::new(-3, 1);
  let hex3 = Hex::new(-2, 1);
  let hex4 = Hex::new(-1, 0);
  let hex5 = Hex::new(-1, -1);
  let mut grid = Grid::new();
  grid.place_piece_to_hex(Piece::beetle(), hex1);
  grid.place_piece_to_hex(Piece::beetle(), hex2);
  grid.place_piece_to_hex(Piece::beetle(), hex3);
  grid.place_piece_to_hex(Piece::beetle(), hex4);
  grid.place_piece_to_hex(Piece::beetle(), hex5);

  assert!(!grid.is_hex_surrounded(&center))
}

#[test]
fn given_grid_with_two_connected_hex_when_is_hex_neighbor_of_should_return_true() {
  let hex1 = Hex::new(-3, 0);
  let hex2 = Hex::new(-3, 1);
  let grid = Grid::new();

  assert!(grid.is_hex_neighbor_of(&hex1, &hex2))
}

#[test]
fn given_grid_with_two_speparated_hex_when_is_hex_neighbor_of_should_return_false() {
  let hex1 = Hex::new(-3, 0);
  let hex2 = Hex::new(-3, 4);
  let grid = Grid::new();

  assert!(!grid.is_hex_neighbor_of(&hex1, &hex2))
}

#[test]
fn given_grid_empty_hex_when_is_hex_occupied_should_return_false() {
  let hex = Hex::new(0, 0);
  let grid = Grid::new();

  assert!(!grid.is_hex_occupied(&hex))
}

#[test]
fn given_grid_occupied_hex_when_is_hex_occupied_should_return_true() {
  let hex = Hex::new(0, 0);
  let mut grid = Grid::new();
  grid.place_piece_to_hex(Piece::beetle(), hex);

  assert!(grid.is_hex_occupied(&hex))
}

#[test]
fn given_empty_grid_when_is_hex_alone_should_return_true() {
  let grid = Grid::new();
  let hex = Hex::new(0, 0);

  assert!(grid.is_hex_alone(&hex))
}

#[test]
fn given_grid_with_partially_surrended_hex_when_is_hex_alone_should_return_false() {
  let center = Hex::new(-2, 0);
  let hex1 = Hex::new(-3, 0);
  let mut grid = Grid::new();
  grid.place_piece_to_hex(Piece::beetle(), hex1);

  assert!(!grid.is_hex_alone(&center))
}

#[test]
fn given_empty_grid_when_number_of_pieces_should_return_zero() {
  let grid = Grid::new();

  assert_eq!(grid.number_of_pieces(), 0)
}

#[test]
fn given_filled_grid_when_number_of_pieces_should_return_correct_size() {
  let hex1 = Hex::new(-3, 0);
  let hex2 = Hex::new(-3, 1);
  let hex3 = Hex::new(-2, 1);
  let hex4 = Hex::new(-1, 0);
  let hex5 = Hex::new(-1, -1);
  let mut grid = Grid::new();
  grid.place_piece_to_hex(Piece::beetle(), hex1);
  grid.place_piece_to_hex(Piece::beetle(), hex2);
  grid.place_piece_to_hex(Piece::beetle(), hex3);
  grid.place_piece_to_hex(Piece::beetle(), hex4);
  grid.place_piece_to_hex(Piece::beetle(), hex5);

  assert_eq!(grid.number_of_pieces(), 5)
}
