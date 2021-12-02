use std::fs::File;
use std::io::{self, BufRead};

struct Instruction {
  direction: String,
  count: i32,
}

struct Point {
  x: i32,
  depth: i32,
  aim: i32,
}

fn solution_1(instructions: &[Instruction]) -> i32 {
    let mut point = Point{x: 0, depth: 0, aim: 0};
    for instr in instructions {
      if instr.direction == "forward" {
        point.x += instr.count;
      } else if instr.direction == "down" {
        point.depth += instr.count;
      } else if instr.direction == "up" {
        point.depth -= instr.count;
      }
    }
    return point.x * point.depth;
}

fn solution_2(instructions: &[Instruction]) -> i32 {
  let mut point = Point{x: 0, depth: 0, aim: 0};
  for instr in instructions {
    if instr.direction == "forward" {
      point.x += instr.count;
      point.depth += instr.count * point.aim;
    } else if instr.direction == "down" {
      point.aim += instr.count;
    } else if instr.direction == "up" {
      point.aim -= instr.count;
    }
    // println!(r#"x: {:?}, depth: {:?}, aim {:?}"#, point.x, point.depth, point.aim)
  }
  return point.x * point.depth;
}


fn main() {
    let file = File::open("inputs/day02.txt").unwrap();
    let instructions: Vec<Instruction> = io::BufReader::new(file)
                .lines()
                .map(|line|line.unwrap())
                .map(|line|{
                  let splits: Vec<&str> = line.split_whitespace().collect();
                  Instruction{direction: String::from(splits[0]), count: splits[1].parse().unwrap()}
                })
                .collect();

    println!(r#"🎉 Part 1 result is {:?}"#, solution_1(&instructions));

    println!(r#"🎉 Part 2 result is {:?}"#, solution_2(&instructions));
}