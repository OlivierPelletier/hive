use std::io::{stdin, stdout, Write};

pub fn set_panic_hook() {
  // When the `console_error_panic_hook` feature is enabled, we can call the
  // `set_panic_hook` function at least once during initialization, and then
  // we will get better error messages if our code ever panics.
  //
  // For more details see
  // https://github.com/rustwasm/console_error_panic_hook#readme
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}

pub fn get_input(message: &str) -> String {
  let mut buffer = String::new();
  print!("{}: ", message);
  let _ = stdout().flush();
  stdin()
    .read_line(&mut buffer)
    .expect("Did not enter a correct string");

  if let Some('\n') = buffer.chars().next_back() {
    buffer.pop();
  }
  if let Some('\r') = buffer.chars().next_back() {
    buffer.pop();
  }

  buffer
}

pub fn get_usize_input(message: &str) -> usize {
  let input = get_input(message);

  match input.parse::<usize>() {
    Ok(v) => v,
    Err(_e) => get_usize_input(message),
  }
}
