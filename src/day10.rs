use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap};

struct Line {
  illegal_char: Option<char>,
  expected_stack: Vec<char>,
}

fn parse_line(line: &String) -> Line {
  let mut expected_stack: Vec<char> = Vec::new();
  for char in line.chars().into_iter() {
    match char {
        // opening chars
        '(' => expected_stack.push(')'),
        '[' => expected_stack.push(']'),
        '{' => expected_stack.push('}'),
        '<' => expected_stack.push('>'),
        // closing chars
        _ => {
          if expected_stack.last() != Some(&char) {
            return Line{illegal_char: Some(char), expected_stack};
          }
          expected_stack.pop();
        },
    }
  }
  return Line{illegal_char: None, expected_stack};
}


fn solution_1(data: &Vec<String>) -> u32 {
  let points_by_char: HashMap<char, u32> = HashMap::from([
    (')', 3),
    (']', 57),
    ('}', 1197),
    ('>', 25137),
  ]);
  let mut counter = 0;
  for line in data {
    let illegal_char = parse_line(line).illegal_char;
    if illegal_char.is_none() {
      continue;
    }
    counter += points_by_char[&illegal_char.unwrap()];
  }
  return counter;
}

fn solution_2(data: &Vec<String>) -> u64 {
  let points_by_char: HashMap<char, u64> = HashMap::from([
    (')', 1),
    (']', 2),
    ('}', 3),
    ('>', 4),
  ]);
  let mut totals: Vec<u64> = Vec::new();
  for line in data {
    let line = parse_line(line);
    if line.illegal_char.is_some() {
      continue;
    }
    let mut total: u64 = 0;
    for c in line.expected_stack.into_iter().rev() {
      total = total*5 + points_by_char[&c];
    }
    totals.push(total);
  }
  totals.sort();
  return totals[totals.len() / 2];
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day10".to_string();
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