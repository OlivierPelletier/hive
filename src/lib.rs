#![allow(
  clippy::enum_variant_names,
  clippy::unused_unit,
  clippy::let_and_return,
  clippy::not_unsafe_ptr_arg_deref,
  clippy::cast_lossless,
  clippy::blacklisted_name,
  clippy::too_many_arguments,
  clippy::trivially_copy_pass_by_ref,
  clippy::let_unit_value,
  clippy::clone_on_copy
)]

use crate::engine::game::{action::Action, Game};

pub mod engine;
mod jni_c_header;
pub mod utils;

include!(concat!(env!("OUT_DIR"), "/java_glue.rs"));

#[allow(dead_code)]
struct HiveEngine {
  game: Game,
}

impl HiveEngine {
  #[allow(dead_code)]
  fn new() -> HiveEngine {
    HiveEngine {
      game: Game::tournement(),
    }
  }

  #[allow(dead_code)]
  fn list_actions_for_player(&self, player_index: usize) -> String {
    let actions = self
      .game
      .list_actions_for_player(&self.game.players[player_index]);

    serde_json::to_string(&actions).unwrap()
  }

  #[allow(dead_code)]
  fn play_action(&mut self, action: &str) {
    let action: Action = serde_json::from_str(action).unwrap();
    self.game = self.game.play_action(action);
  }
}
