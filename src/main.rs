use hive::grid::geo::hex::Hex;
use hive::grid::piece::Piece;
use hive::grid::Grid;
use hive::rule::single_hive_rule_validation;

fn main() {
    let mut grid = Grid::new()
        .place_piece_to_hex(Piece::queen_bee(), &Hex::new(0, 0))
        .place_piece_to_hex(Piece::mosquito(), &Hex::new(2, 2))
        .place_piece_to_hex(Piece::soldier_ant(), &Hex::new(3, 2))
        .place_piece_to_hex(Piece::soldier_ant(), &Hex::new(3, 3))
        .place_piece_to_hex(Piece::grasshopper(), &Hex::new(1, 4))
        .place_piece_to_hex(Piece::grasshopper(), &Hex::new(2, 4))
        .place_piece_to_hex(Piece::grasshopper(), &Hex::new(6, 0))
        .place_piece_to_hex(Piece::beetle(), &Hex::new(3, 0))
        .place_piece_to_hex(Piece::beetle(), &Hex::new(2, 3))
        .place_piece_to_hex(Piece::ladybug(), &Hex::new(1, 3));

    println!("{}", grid);

    grid = grid.move_piece_from_to(&Hex::new(1, 3), &Hex::new(3, 1));

    println!("{}", grid);

    println!(
        "{}",
        single_hive_rule_validation(&grid, &Hex::new(6, 0), &Hex::new(6, 1))
    );
    println!(
        "{}",
        single_hive_rule_validation(&grid, &Hex::new(3, 2), &Hex::new(4, 2))
    );

    println!("{}", grid);
}
