//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate web_sys;

use js_sys::JSON;
use wasm_bindgen_test::*;

use hive::{engine::game::action::Action, list_actions_for_player, new_game, play_action};
use wasm_bindgen::JsValue;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
  assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_new_game() {
  let mut game = new_game();
  let actions = list_actions_for_player(&game, 0);
  let mut actions: Vec<Action> = actions.into_serde().unwrap();
  let action = actions.get(0).unwrap();
  let action_js = &JsValue::from_serde(action).unwrap();

  log(action_js);

  game = play_action(&game, action_js);

  log(&game)
}

fn log(object: &JsValue) {
  let obj_as_string: String = JSON::stringify(&object).unwrap().into();
  let obj_as_str: &str = &*obj_as_string;

  web_sys::console::log_1(&obj_as_str.into())
}
