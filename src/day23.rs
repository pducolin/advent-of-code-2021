use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

struct Cost {
  cost_by_letter: HashMap<char, i64>
}

impl Cost {
  fn new() -> Self {
    Self {
      cost_by_letter: HashMap::from([
        ('A', 1),
        ('B', 10),
        ('C', 100),
        ('D', 1000)
      ])
    }
  }
}

struct Step {
  letter: char,
  count: i64
}
impl Step {
  fn new(letter:char, count:i64) -> Self {
    Self {
      letter,
      count
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
  row: i64,
  col: i64,
}

impl Point {
  fn distance(&self, other: &Point) -> i64 {
    (self.row - 1).abs() // distance from self.row to row 1
    + (1 - other.row).abs() // distance from 1 to other.row
    + (self.col - other.col).abs()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Amphipod {
  letter: char,
  position: Point,
}

struct Board {
  amphipods: HashSet<Amphipod>
}

impl Board {
  fn parse(data: &Vec<String>) -> Self {
    let mut amphipods: HashSet<Amphipod> = HashSet::new();
    for r in 0..data.len() {
      let row: Vec<char> = data[r].chars().collect();
      for c in 0..row.len() {
        let letter = row[c];
        match letter {
            'A' | 'B' | 'C' | 'D' => {
              amphipods.insert(Amphipod { 
                letter, 
                position: Point{ row: r as i64, col: c as i64}
              });
            },
            _ => continue
        }
      }
    }
    Self {
      amphipods
    }
  }

  fn delta_step(&self, other: &Board) -> Step {
    let old_amphipod: &Amphipod = self.amphipods.difference(&other.amphipods).next().unwrap();
    let new_amphipod: &Amphipod = other.amphipods.difference(&self.amphipods).next().unwrap();

    let step = Step {
      letter: old_amphipod.letter,
      count: new_amphipod.position.distance(&old_amphipod.position),
    };

    println!("Letter {:?} did {:?} steps", step.letter, step.count);

    return step;
  }
}

fn solution_1(data: &Vec<String>) -> i64 {
  let cost = Cost::new();
  let mut step_sequence: Vec<Step> = Vec::new();

  let mut prev_board: Option<Board> = None;
  let mut iter = data.iter();
  loop {
    let mut board_str: Vec<String> = Vec::new();
    for _ in 0..5 {
      board_str.push(iter.next().unwrap().clone())
    }

    let board = Board::parse(&board_str);
    if prev_board.is_some() {
      let old_board = prev_board.as_ref().unwrap();
      step_sequence.push(board.delta_step(old_board));
    } 
    prev_board = Some(board);
    if iter.next().is_none() {
      break;
    }
  }
  step_sequence.iter().map(|s|{
    s.count*cost.cost_by_letter[&s.letter]
  }).sum()
}

fn solution_2(data: &Vec<String>) -> i64 {
  0
}

fn main() {
  let test = true;
  let mut file_path: String = "inputs/day23".to_string();
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
