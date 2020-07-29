use hive::grid::geo::hex::Hex;
use hive::grid::piece::Piece;
use hive::grid::Grid;
use hive::rule::available_moves;

fn main() {
    let grid = Grid::new()
        .place_piece_to_hex(Piece::beetle().white(), &Hex::new(0, 0))
        .place_piece_to_hex(Piece::beetle().black(), &Hex::new(-1, 0))
        .place_piece_to_hex(Piece::grasshopper().white(), &Hex::new(1, 0))
        .place_piece_to_hex(Piece::grasshopper().black(), &Hex::new(-2, 0))
        .place_piece_to_hex(Piece::soldier_ant().white(), &Hex::new(2, -1))
        .place_piece_to_hex(Piece::soldier_ant().black(), &Hex::new(-3, 1))
        .place_piece_to_hex(Piece::queen_bee().white(), &Hex::new(1, 1))
        .place_piece_to_hex(Piece::queen_bee().black(), &Hex::new(-2, -1));

    println!("{}", grid);

    println!("{:?}", available_moves(&grid, &Hex::new(1, 1)))
}
