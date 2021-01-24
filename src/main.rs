use hive::engine::game::Game;
use hive::engine::grid::coordinate::hex::Hex;
use hive::engine::grid::piece::Piece;
use hive::engine::grid::Grid;
use hive::engine::moves::available_moves;
use hive::utils::set_panic_hook;

fn main() {
  set_panic_hook();

  let game = Game::tournement();

  println!("{:?}", game.list_moves_for_player(&game.players[0]));
}
