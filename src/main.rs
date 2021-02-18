use hive::engine::game::Game;
use hive::utils::{get_usize_input, set_panic_hook};

fn main() {
  set_panic_hook();

  let game = Game::tournement();

  let moves = game.list_moves_for_player(&game.players[0]);

  for (i, m) in moves.iter().enumerate() {
    println!("{}: {}", i, m)
  }

  let chosen_move = get_usize_input("Choose a move");

  println!("{}", moves[chosen_move])
}
