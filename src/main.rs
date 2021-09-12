use hive::engine::game::Game;
use hive::utils::{get_usize_input, set_panic_hook};

fn main() {
  set_panic_hook();

  let mut game = Game::tournement();

  println!("{:?}", game);

  let actions = game.list_actions_for_player(&game.players[0]);

  for (i, m) in actions.iter().enumerate() {
    println!("{}: {}", i, m)
  }

  let chosen_action = get_usize_input("Choose an action");

  game = game.play_action(actions[chosen_action].clone());

  println!("{:?}", game)
}
