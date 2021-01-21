use hive::engine::grid::coordinate::hex::Hex;
use hive::engine::grid::piece::Piece;
use hive::engine::grid::Grid;
use hive::engine::moves::available_moves;
use hive::utils::set_panic_hook;

fn main() {
  set_panic_hook();

  let grid = Grid::new()
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
    .place_piece_to_hex(Piece::mosquito().white(), &Hex::new(-2, -1));

  println!("{}", grid);

  println!(
    "W QBEE MOVES: {:?}",
    available_moves(&grid, &Hex::new(2, -1))
  );

  println!(
    "B QBEE MOVES: {:?}",
    available_moves(&grid, &Hex::new(-3, -0))
  );

  println!(
    "W BETL MOVES: {:?}",
    available_moves(&grid, &Hex::new(0, 0))
  );

  println!(
    "B BETL MOVES: {:?}",
    available_moves(&grid, &Hex::new(-2, 1))
  );

  println!(
    "W GRHP MOVES: {:?}",
    available_moves(&grid, &Hex::new(2, 0))
  );

  println!(
    "W SPDR MOVES: {:?}",
    available_moves(&grid, &Hex::new(0, 1))
  );

  println!(
    "B LDBG MOVES: {:?}",
    available_moves(&grid, &Hex::new(-1, -1))
  );

  println!(
    "W SANT MOVES: {:?}",
    available_moves(&grid, &Hex::new(3, -1))
  );

  println!(
    "W MSQT MOVES: {:?}",
    available_moves(&grid, &Hex::new(-2, -1))
  );
}
