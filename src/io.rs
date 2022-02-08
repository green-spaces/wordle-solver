use super::guess_result::GuessResult;
use std::io::{self, prelude::*};
use std::fs::File;


pub fn read_wordle_output() -> GuessResult {
  let mut buffer = String::new();
  while buffer.parse::<GuessResult>().is_err() {
      buffer = "".to_string();
      println!("Enter wordle result: g = green, y = yellow, b = black");
      io::stdin().read_line(&mut buffer).unwrap();
  }
  buffer.parse::<GuessResult>().unwrap()
}

pub fn load_dictionary(file: &str) -> Vec<String> {
  let file = File::open(file).unwrap();
  let lines = io::BufReader::new(file).lines();

  let mut flw: Vec<String> = Vec::new();

  for line in lines.flatten() {
      if line.len() == 5 {
          flw.push(line);
      }
  }

  flw
}

