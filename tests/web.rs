//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate web_sys;

use js_sys::JSON;
use wasm_bindgen_test::*;

use hive::new_game;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
  assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_new_game() {
  let game = new_game();

  let game_as_string: String = JSON::stringify(&game).unwrap().into();
  let game_as_str: &str = &*game_as_string;

  web_sys::console::log_1(&game_as_str.into())
}
