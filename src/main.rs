mod utils;

use crate::utils::get_usize_input;
use hive::engine::game::Game;

fn main() {
    let mut game = Game::tournement();
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

        let chosen_action = get_usize_input("Choose an action");

        game = game.play_action(actions[chosen_action].clone());
        winner = game.winner();
    }

    println!("{:?} wins!", winner.unwrap());
}
