use hive::grid::geo::hex::Hex;
use hive::grid::piece::Piece;
use hive::grid::{init_grid, place_piece_to_hex};

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

    let mut grid = init_grid();
    grid = place_piece_to_hex(grid, piece.clone(), hex1);
    grid = place_piece_to_hex(grid, piece.clone(), hex2);
    grid = place_piece_to_hex(grid, piece.clone(), hex3);
    grid = place_piece_to_hex(grid, piece.clone(), hex4);
    grid = place_piece_to_hex(grid, piece.clone(), hex5);
    grid = place_piece_to_hex(grid, piece.clone(), hex6);
    grid = place_piece_to_hex(grid, piece.clone(), hex7);
    grid = place_piece_to_hex(grid, piece.clone(), hex8);
    grid = place_piece_to_hex(grid, piece.clone(), hex9);

    println!("{}", grid);
}
