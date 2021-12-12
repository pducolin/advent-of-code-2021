use std::fs::File;
use std::io::{self, BufRead};


fn solution_1(data: &Vec<String>) -> i64 {
  0
}

fn solution_2(data: &Vec<String>) -> i64 {
  0
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/dayXX".to_string();
  let mut emoji: String = "🎉".to_string();
  if test {
    file_path += ".test";
    emoji = "🧪".to_string();
  }
  file_path += ".txt";

  let file = File::open(file_path).unwrap();
  let data: Vec<String> = io::BufReader::new(file)
              .lines()
              .map(|line|line.unwrap())
              .collect();

  println!(r#"{:?} Part 1 result is {:?}"#, emoji, solution_1(&data));

  println!(r#"{:?} Part 2 test result is {:?}"#, emoji, solution_2(&data));
}