use std::collections::{HashMap};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone)]
struct BoardNumber {
  value: u32,
  is_marked: bool,
  column: usize,
  row: usize,
}

#[derive(Clone)]
struct Board {
  board_numbers: Vec<Vec<BoardNumber>>,
  board_numbers_by_value: HashMap<u32, BoardNumber>,

  visited_count_by_row: Vec<u32>,
  visited_count_by_column: Vec<u32>,
  is_winner: bool,
}

impl Board {
  
    fn init(&mut self)  {
      for r in 0..5 {
        let mut row: Vec<BoardNumber> = Vec::new();
        for c in 0..5 {
          let board_number = BoardNumber{
            value:0,
            is_marked: false,
            column: c,
            row: r,
          };
          row.push(board_number);
        }
        self.board_numbers.push(row);
      }
    }

    pub fn parse(data: &Vec<String>) -> Self { 
      let mut board = Self {
        board_numbers: Vec::new(),
        board_numbers_by_value: HashMap::new(),
        visited_count_by_row: vec![0; 5],
        visited_count_by_column: vec![0; 5],
        is_winner: false,
      };

      board.init();

      for row_index in 0..5 {
        let values: Vec<u32> = data[row_index]
        .split_whitespace()
        .map(|v|v.parse().unwrap())
        .collect();
        for column_index in 0..5 {
          let value = values[column_index];
          board.board_numbers[row_index][column_index].value = value;
          board.board_numbers_by_value.insert(value, board.board_numbers[row_index][column_index]);
        }
      }

      return board;
    }

    fn mark_number(&mut self, number: u32) {
      if !self.board_numbers_by_value.contains_key(&number) {
        return;
      }

      let position = self.board_numbers_by_value.get(&number).unwrap();

      self.board_numbers[position.row][position.column].is_marked = true;
      self.visited_count_by_column[position.column] += 1;
      self.visited_count_by_row[position.row] += 1;
      
      if self.visited_count_by_column[position.column] == 5 || self.visited_count_by_row[position.row] == 5 {
        self.is_winner = true;
      }
    }

    fn sum_unmarked(&self) -> u32 {
      let mut sum: u32 = 0;
      for row in &self.board_numbers {
        for board_number in row {
          if !board_number.is_marked {
            sum += board_number.value;
          }
        }
      }
      return sum;
    }
}

struct Match {
  boards: Vec<Board>,
  random_numbers: Vec<u32>,
  boards_count: usize,
  winners_count: usize,
}

impl Match {
  pub fn parse_data(data: &Vec<String>) -> Self {
    // first row contains random numbers
    let random_numbers: Vec<u32> = data[0]
    .split(",")
    .map(|n|n.parse().unwrap())
    .collect();
  
    let mut boards: Vec<Board> = Vec::new();
    let mut index = 1;
    while index < data.len() {
      // skip empty lines
      if data[index].is_empty() {
        index +=1;
        continue;
      }
  
      boards.push(Board::parse(&data[index .. index + 5].to_vec()));
      index += 5;
    }

    let boards_count = boards.len();
  
    return Self{random_numbers: random_numbers, boards: boards, winners_count: 0, boards_count: boards_count};
  }

  pub fn play(&mut self) -> u32 {
    for value in &self.random_numbers {
      for index in 0 .. self.boards.len() {
        let board = self.boards.get_mut(index).unwrap();
        board.mark_number(*value);
        if board.is_winner {
          return board.sum_unmarked() * value;
        }
      }
    }
    return 0;
  }

  pub fn play_last(&mut self) -> u32 {
    for value in &self.random_numbers {
      for index in 0 .. self.boards.len() {
        let board = self.boards.get_mut(index).unwrap();
        if board.is_winner {
          continue;
        }
        board.mark_number(*value);
        if !board.is_winner {
          continue;
        }

        self.winners_count += 1;
        if self.winners_count < self.boards_count {
          continue;
        }
        // last winner board
        return board.sum_unmarked() * value;
      }
    }
    return 0;
  }
}
fn solution_1(data: &Vec<String>) -> u32 {
  let mut m = Match::parse_data(data);
  return m.play();
}

fn solution_2(data: &Vec<String>) -> u32 {
  let mut m = Match::parse_data(data);
  return m.play_last();
}

fn main() {
    let file = File::open("inputs/day04.txt").unwrap();
    let data: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line|line.unwrap())
                .collect();

    println!(r#"🎉 Part 1 result is {:?}"#, solution_1(&data));

    println!(r#"🎉 Part 2 result is {:?}"#, solution_2(&data));
}