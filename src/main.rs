use hive::{
  engine::game::Game,
  utils::{get_usize_input, set_panic_hook},
};

fn main() {
  set_panic_hook();

  let mut game = Game::tournement();

  loop {
    println!("{:?}", game);
    println!("{:}", game.grid);

    let actions = game.list_actions_for_player(&game.players[game.current_player_index]);

    for (i, m) in actions.iter().enumerate() {
      println!("{}: {}", i, m)
    }

    let chosen_action = get_usize_input("Choose an action");

    game = game.play_action(actions[chosen_action].clone());
  }
}
