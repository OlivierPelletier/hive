pub mod misc;

use crate::misc::utils::get_usize_input;
use hive::engine::game::Game;

fn main() {
  let mut game = Game::tournament();
  let mut winner = game.winner();

  while winner.is_none() {
    println!("{:?}", game);
    println!("{:}", game.grid);

    let actions = game.list_actions_for_player(&game.players[game.current_player_index]);

    let mut moves_str = "".to_owned();
    for (i, m) in actions.iter().enumerate() {
      moves_str.push_str(format!("[({}): {}]", i, m).as_str());
    }

    println!("{}", moves_str);

    let mut chosen_action = get_usize_input("Choose an action");

    while chosen_action >= actions.len() {
      chosen_action = get_usize_input("Invalid number... Try again");
    }

    game.play_action(actions[chosen_action]);
    winner = game.winner();
  }

  println!("{:?} wins!", winner.unwrap());
}
