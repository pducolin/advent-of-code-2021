// inspired by https://github.com/jeffomatic/adventofcode/blob/main/2021-rust/day24a/src/main.rs

use std::{collections::HashMap, fs::File};
use std::io::{self, BufRead};

mod day24_alu;

#[derive(Debug, Clone)]
struct Solver {
  z_out_by_z_in: Vec<HashMap<i64, Option<i64>>>,
}

impl Solver {
  fn solve_max(&mut self, digit_index: usize, prev_z: i64) -> Option<i64> {
    if digit_index >= 14 {
        if prev_z == 0 {
            return Some(0);
        }

        return None;
    }

    if let Some(&cached) = self.z_out_by_z_in[digit_index].get(&prev_z) {
      return cached;
    }

    for w in (1..=9).rev() {
      let mem_in = day24_alu::MemoryState::new(w, prev_z);
      // check if already evaluated
      let mem = day24_alu::VALIDATORS[digit_index](&mem_in);
      if let Some(best_suffix) = self.solve_max(digit_index + 1, mem.z()) {
          let exp = 14 - digit_index - 1;
          let new_suffix = 10_i64.pow(exp as u32) * w + best_suffix;

          self.z_out_by_z_in[digit_index].insert(prev_z, Some(new_suffix));
          return Some(new_suffix);
      }
    }

    self.z_out_by_z_in[digit_index].insert(prev_z, None);
    None
  }

  fn solve_min(&mut self, digit_index: usize, prev_z: i64) -> Option<i64> {
    if digit_index >= 14 {
        if prev_z == 0 {
            return Some(0);
        }

        return None;
    }

    if let Some(&cached) = self.z_out_by_z_in[digit_index].get(&prev_z) {
      return cached;
    }

    for w in 1..=9 {
      let mem_in = day24_alu::MemoryState::new(w, prev_z);
      // check if already evaluated
      let mem = day24_alu::VALIDATORS[digit_index](&mem_in);
      if let Some(best_suffix) = self.solve_min(digit_index + 1, mem.z()) {
          let exp = 14 - digit_index - 1;
          let new_suffix = 10_i64.pow(exp as u32) * w + best_suffix;

          self.z_out_by_z_in[digit_index].insert(prev_z, Some(new_suffix));
          return Some(new_suffix);
      }
    }

    self.z_out_by_z_in[digit_index].insert(prev_z, None);
    None
  }
}

fn solution_1(_: &Vec<String>) -> String {
  let mut solver = Solver{
    z_out_by_z_in: vec![HashMap::new(); 14],
  };

  let solution = solver.solve_max(0, 0);

  solution.unwrap().to_string()
}

fn solution_2(_: &Vec<String>) -> String {
  let mut solver = Solver{
    z_out_by_z_in: vec![HashMap::new(); 14],
  };

  let solution = solver.solve_min(0, 0);

  solution.unwrap().to_string()
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day24".to_string();
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
