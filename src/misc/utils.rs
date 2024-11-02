use std::io::{stdin, stdout, Write};

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
