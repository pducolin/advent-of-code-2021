use std::collections::{HashSet};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
  x: usize,
  y: usize,
}

impl Point {
  fn parse(data: &String) -> Self {
    let splits: Vec<usize> = data.split(",").map(|n|n.parse().unwrap()).collect();
    return Self{x:splits[0], y:splits[1]};
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FoldDirection {
  Up,
  Left
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Fold {
  direction: FoldDirection,
  value: usize,
}

impl Fold {
  fn parse(data: &String) -> Self {
    let fold_data: String = data.split_whitespace().filter(|x|x.contains("=")).last().unwrap().to_string();
    let fold_splits: Vec<&str> = fold_data.split("=").collect();

    let mut direction: FoldDirection = FoldDirection::Left;
    if fold_splits[0].chars().next() == Some('y') {
      direction = FoldDirection::Up;
    } 
    let value: usize = fold_splits[1].parse().unwrap();
    return Self{direction, value };
  }
}

#[derive(Clone, Copy)]
struct Size {
  columns: usize,
  rows: usize,
}

#[derive(Clone)]
struct Board {
  points: HashSet<Point>,
  folds: Vec<Fold>,
} 

fn get_board_size(points: &Vec<Point>) -> Size {
  let mut max_x = 0;
  let mut max_y = 0;
  for point in points {
    if point.x > max_x {
      max_x = point.x;
    }
    if point.y > max_y {
      max_y = point.y;
    }
  }
  return Size{columns: max_x + 1, rows: max_y + 1};
}

impl Board {
  fn init() -> Self {
    return Self {
      points: HashSet::new(),
      folds: Vec::new(),
    };
  }

  fn get_size(&self) -> Size {
    return get_board_size(&Vec::from_iter(self.points.clone()));
  }

  fn parse(data: &Vec<String>) -> Self {
    let mut board = Self::init();

    for line in data {
      if line.is_empty() {
        continue;
      }
      if line.starts_with("fold") {
        board.folds.push(Fold::parse(line));
        continue;
      }
      board.points.insert(Point::parse(line));
    }

    return board.clone();
  }

 
  fn fold_left(&mut self, fold_x: usize) {
    let mut points: HashSet<Point> = HashSet::new();

    let tot_cols = fold_x * 2 + 1;

    for point in &self.points {
      if point.x < fold_x {
        points.insert(point.clone());
        continue;
      }

      points.insert(Point{x: tot_cols - point.x - 1, y: point.y});
    }
    self.points = points;
  }

  fn fold_up(&mut self, fold_y: usize) {
    let mut points: HashSet<Point> = HashSet::new();

    let tot_rows = fold_y * 2 + 1;

    for point in &self.points {
      if point.y < fold_y {
        points.insert(point.clone());
        continue;
      }

      points.insert(Point{y: tot_rows - point.y - 1, x: point.x});
    }
    self.points = points;
  }

  fn fold(&mut self, fold_count: usize) {
    for i in 0 .. fold_count {
      let fold_instruction = self.folds.get(i).unwrap().clone();
      match fold_instruction.direction {
          FoldDirection::Left => self.fold_left(fold_instruction.value),
          FoldDirection::Up =>  self.fold_up(fold_instruction.value),
      }
   } 
  } 

  fn print(&self) {
    let mut printable_matrix: Vec<String> = Vec::new();
    let size = self.get_size();
    for y in 0 .. size.rows {
      let mut row = "".to_owned();
      for x in 0 .. size.columns {
        if self.points.contains(&Point{x,y}) {
          row.push_str("#");
        } else {
          row.push_str(".");
        }
      }
      printable_matrix.push(row);
    }

    for row in printable_matrix {
      println!(r#"{:?}"#, row);
    }
  }
}

fn solution_1(data: &Vec<String>) -> usize {
  let mut board = Board::parse(data);
  // println!(r#"===== Board before folding ====="#);
  // board.print();
  board.fold(1);
  // println!(r#"===== Board after folding ====="#);
  // board.print();
  return board.points.len();
}

fn solution_2(data: &Vec<String>) -> usize {
  let mut board = Board::parse(data);
  let tot_folds = board.folds.len();
  board.fold(tot_folds);
  board.print();
  return board.points.len();
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day13".to_string();
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
