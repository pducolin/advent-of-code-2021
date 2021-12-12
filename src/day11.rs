use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
  row: usize,
  col: usize,
}

#[derive(PartialEq, Copy, Clone)]
enum FlashState {
  Idle,
  Flashing,
  Flashed,
}

struct Pown {
  value: u32,
  status: FlashState,
  neighbours: Vec<Point>,
}

impl Pown {
  fn increase(&mut self) {
    if self.status == FlashState::Flashed {
      return;
    }

    if self.status == FlashState::Flashing {
      self.status = FlashState::Flashed;
      return;
    }
    
    if self.value < 9 {
      self.value += 1;
      return;
    }

    self.value = 0;
    self.status = FlashState::Flashing;
  }

  fn reset_flash(&mut self) {
    self.status = FlashState::Idle;
  }
}

struct Game {
  matrix: Vec<Vec<Pown>>,
  size: usize,
}

impl Game {
  fn parse(data: &Vec<String>) -> Self {
    let mut matrix: Vec<Vec<Pown>> = Vec::new();
    let size: usize = data.len();

    for row in 0 .. size {
      let numbers: Vec<u32> = data[row].chars().map(|c|c.to_digit(10).unwrap()).collect();
      let mut powns: Vec<Pown> = Vec::new();
      for col in 0 .. size {
        let mut neighbours: Vec<Point> = Vec::new();
        // top
        if row > 0 {
          neighbours.push(Point{row: row - 1, col});
        }
        // top right
        if row > 0 && col < size - 1 {
          neighbours.push(Point{row: row - 1, col: col + 1});
        }
        // right
        if col < size - 1 {
          neighbours.push(Point{row, col: col + 1});
        }
        // bottom right
        if row < size - 1 && col < size - 1 {
          neighbours.push(Point{row: row + 1, col: col + 1});
        }
        // bottom
        if row < size - 1  {
          neighbours.push(Point{row: row + 1, col});
        }
        // bottom left
        if row < size - 1 && col > 0 {
          neighbours.push(Point{row: row + 1, col: col - 1});
        }
        // left
        if col > 0 {
          neighbours.push(Point{row, col: col - 1});
        }
        // top left
        if row > 0 && col > 0 {
          neighbours.push(Point{row: row - 1, col: col - 1});
        }
        powns.push(Pown{value: numbers[col], status: FlashState::Idle, neighbours});
      }
      matrix.push(powns);
    }

    return Game{matrix,size};
  }

  fn increase(&mut self, position: &Point) -> (u64, Vec<Point>) {
    let row = position.row;
    let col = position.col;
    let mut neighbours: Vec<Point> = Vec::new();
    let mut flash_count: u64 = 0;
    self.matrix[row][col].increase();
    if self.matrix[row][col].status == FlashState::Flashing {
      flash_count += 1;
      for n in &self.matrix[row][col].neighbours {
        let neighbour_status: FlashState = self.matrix[n.row][n.col].status;
        if neighbour_status == FlashState::Idle {
          neighbours.push(n.clone());
        }
      }
    }
    return (flash_count, neighbours);
  }

  fn play(&mut self, times: i32) -> u64 {
    let mut tot_flashes: u64 = 0;

    for _ in 0 .. times {
      let mut neighbours: Vec<Point> = Vec::new();
      // increase all by 1
      for row in 0 .. self.size {
        for col in 0 .. self.size {
          let mut ret = self.increase(&Point{row, col});
          tot_flashes += ret.0;
          neighbours.append(&mut ret.1);
        }
      }
      // iterate on neighbours
      while neighbours.len() > 0 {
        let point = neighbours.pop().unwrap();
        let mut ret = self.increase(&point.clone());
        tot_flashes += ret.0;
        neighbours.append(&mut ret.1);
      }

      // reset all statuses
      for row in 0 .. self.size {
        for col in 0 .. self.size {
          self.matrix[row][col].reset_flash();
        }
      }
    }

    return tot_flashes;
  }

  fn count_flashing(&self) -> u64 {
    let mut counter = 0;
    for row in 0 .. self.size {
      for col in 0 .. self.size {
        if self.matrix[row][col].value == 0 {
          counter += 1;
        }
      }
    }
    return counter;
  }
}


fn solution_1(data: &Vec<String>) -> u64 {
  let mut game = Game::parse(data);
  return game.play(100);
}

fn solution_2(data: &Vec<String>) -> u64 {
  let mut game = Game::parse(data);
  let mut iterations = 0;
  while true && iterations < 2000 {
    iterations += 1;
    game.play(1);
    if game.count_flashing() == 100 {
      break;
    }
  }
  return iterations;
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day11".to_string();
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