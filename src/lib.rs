use wasm_bindgen::prelude::*;

use crate::engine::game::{action::Action, Game};

pub mod engine;
pub mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
  alert("Hello! You're correctly binded with the Hive game engine.");
}

#[wasm_bindgen]
pub fn new_game() -> JsValue {
  let game = Game::tournement();

  JsValue::from_serde(&game).unwrap()
}

#[wasm_bindgen]
pub fn list_actions_for_player(game: &JsValue, player_index: usize) -> JsValue {
  let game: Game = game.into_serde().unwrap();
  let actions = game.list_actions_for_player(&game.players[player_index]);

  JsValue::from_serde(&actions).unwrap()
}

#[wasm_bindgen]
pub fn play_action(game: &JsValue, action: &JsValue) -> JsValue {
  let mut game: Game = game.into_serde().unwrap();
  let action: Action = action.into_serde().unwrap();

  game = game.play_action(action);

  JsValue::from_serde(&game).unwrap()
}
