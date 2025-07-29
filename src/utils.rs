use std::io::{self, Write};

pub fn user_prompt(prompt: &str) -> bool {
  print!("{} [y/N] :", prompt);
  io::stdout().flush().unwrap();
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}