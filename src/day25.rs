use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
  row: usize,
  col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
  cucumbers: HashMap<Point, char>,
  tot_columns: usize,
  tot_rows: usize,
}

impl Board {
  fn parse(data: &Vec<String>) -> Self {
    let tot_columns = data[0].len();
    let tot_rows = data.len();
    let mut cucumbers: HashMap<Point, char> = HashMap::new();

    for row in 0..tot_rows {
      let row_chars: Vec<char> = data[row].chars().collect();
      for col in 0..tot_columns {
        let c = row_chars[col];
        match c {
            '>' | 'v' => {
              cucumbers.insert(Point{row, col}, c);
            },
            _ => continue
        }
      }
    }

    return Self {
      tot_columns,
      tot_rows,
      cucumbers
    };
  }

  fn print(&self) {
    for row in 0..self.tot_rows {
      let mut row_str = "".to_owned();
      for col in 0..self.tot_columns {
        row_str.push(self.cucumbers.get(&Point{row,col}).or(Some(&'.')).unwrap().clone());
      }
      println!("{}", row_str);
    }
  }
}

fn step_east(board: &Board) -> Board {
  let cucumbers: HashMap<Point, char> = board.cucumbers.iter()
  .map(|(point, direction)| {
    if *direction != '>' {
      return (point.clone(), direction.clone());
    }
    // find next position
    let mut next_col = point.col + 1;

    if next_col > board.tot_columns {
      panic!("next col should not be higher than tot columns");
    }
    if next_col == board.tot_columns {
      next_col = 0;
    }
    let new_point = Point{row: point.row, col: next_col};
        // stay still if new position is not empty
    if board.cucumbers.contains_key(&new_point) {
      return (point.clone(), direction.clone());
    }
    return (new_point, direction.clone());
  }).collect();

  return Board { cucumbers, tot_columns: board.tot_columns, tot_rows: board.tot_rows };
}

fn step_south(board: &Board) -> Board {
  let cucumbers: HashMap<Point, char> = board.cucumbers.iter()
  .map(|(point, direction)| {
    if *direction != 'v' {
      return (point.clone(), direction.clone());
    }
    // find next position
    let mut next_row = point.row + 1;

    if next_row > board.tot_rows {
      panic!("next col should not be higher than tot columns");
    }
    if next_row == board.tot_rows {
      next_row = 0;
    }
    let new_point = Point{row: next_row, col: point.col};
    // stay still if new position is not empty
    if board.cucumbers.contains_key(&new_point) {
      return (point.clone(), direction.clone());
    }
    return (new_point, direction.clone());
  }).collect();

  return Board { cucumbers, tot_columns: board.tot_columns, tot_rows: board.tot_rows };
}

fn step(board: &Board) -> Board {
  let new_board = step_east(board);
  return step_south(&new_board);
}

fn solution_1(data: &Vec<String>) -> i64 {
  let mut board = Board::parse(data);
  board.print();

  let mut step_count = 0;
  loop {
    step_count += 1;
    let new_board = step(&board);
    if new_board == board {
      break;
    }
    board = new_board.clone();
  }

  step_count
}

fn solution_2(data: &Vec<String>) -> i64 {
  0
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day25".to_string();
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
