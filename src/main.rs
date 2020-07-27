use hive::grid::geo::hex::Hex;
use hive::grid::piece::Piece;
use hive::grid::Grid;

fn main() {
    let piece = Piece::queen_bee();

    let hex1 = Hex { q: 2, r: 2 };
    let hex2 = Hex { q: 3, r: 2 };
    let hex3 = Hex { q: 1, r: 3 };
    let hex4 = Hex { q: 2, r: 3 };
    let hex5 = Hex { q: 3, r: 3 };
    let hex6 = Hex { q: 1, r: 4 };
    let hex7 = Hex { q: 2, r: 4 };
    let hex8 = Hex { q: 6, r: 0 };
    let hex9 = Hex { q: 3, r: 0 };

    let mut grid = Grid::new()
        .place_piece_to_hex(piece.clone(), hex1)
        .place_piece_to_hex(piece.clone(), hex2)
        .place_piece_to_hex(piece.clone(), hex2)
        .place_piece_to_hex(piece.clone(), hex3)
        .place_piece_to_hex(piece.clone(), hex4)
        .place_piece_to_hex(piece.clone(), hex5)
        .place_piece_to_hex(piece.clone(), hex6)
        .place_piece_to_hex(piece.clone(), hex7)
        .place_piece_to_hex(piece.clone(), hex8)
        .place_piece_to_hex(piece.clone(), hex9);

    println!("{}", grid);

    let hex10 = Hex { q: 3, r: 1 };

    grid = grid.move_piece_from_to(hex9, hex10);

    println!("{}", grid);

    println!(
        "{}",
        grid.is_move_piece_from_to_valid(hex8, Hex { q: 6, r: 1 })
    );
    println!(
        "{}",
        grid.is_move_piece_from_to_valid(hex2, Hex { q: 4, r: 2 })
    );

    println!("{}", grid);
}
