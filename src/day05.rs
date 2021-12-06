use std::cmp;
use std::collections::{HashMap};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

struct Point {
  x: i32,
  y: i32,
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "({}:{})", self.x, self.y)
  }
}

struct Line {
  start: Point,
  end: Point,
}

fn parse_line(data: &String) -> Line {
  let line_splits: Vec<&str> = data.split("->").collect();

  let start = line_splits[0].replace(" ", "");
  let start: Vec<&str> = start.split(",").collect();
  let end = line_splits[1].replace(" ", "");
  let end: Vec<&str> = end.split(",").collect();
  return Line{
    start: Point{
      x: start[0].parse().unwrap(), 
      y: start[1].parse().unwrap()
    }, 
    end: Point{
      x: end[0].parse().unwrap(), 
      y: end[1].parse().unwrap()
    }
  };
}

fn solution_1(data: &Vec<String>) -> i32 {
  let mut count_per_point: HashMap<String, i32> = HashMap::new();
  
  let mut double_lines_counter = 0;
  for d in data {
    let line = parse_line(d);
    if line.start.x == line.end.x {
      // move on y
      let mut min_y = cmp::min(line.start.y, line.end.y);
      let max_y = cmp::max(line.start.y, line.end.y);
      while min_y <= max_y {
        let point = (Point {x: line.start.x, y: min_y}).to_string();
        let counter = count_per_point.entry(point).or_insert(0);
        *counter += 1;
        if *counter == 2 {
          double_lines_counter += 1;
        }
        min_y += 1;
      }
    } else if line.start.y == line.end.y {
      // move on x
      let mut min_x = cmp::min(line.start.x, line.end.x);
      let max_x = cmp::max(line.start.x, line.end.x);
      while min_x <= max_x {
        let point = (Point {x: min_x, y: line.start.y}).to_string();
        let counter = count_per_point.entry(point).or_insert(0);
        *counter += 1;
        if *counter == 2 {
          double_lines_counter += 1;
        }
        min_x += 1;
      }
    }
    // else do nothing
  }
  return double_lines_counter;
}

fn solution_2(data: &Vec<String>) -> i32 {
  let mut count_per_point: HashMap<String, i32> = HashMap::new();
  
  let mut double_lines_counter = 0;
  for d in data {
    let line = parse_line(d);

    // move on diagonal 
    let mut incr_x = 1;
    if line.start.x > line.end.x {
      incr_x = -1;
    } else if line.start.x == line.end.x {
      incr_x = 0;
    }
    let mut incr_y = 1;
    if line.start.y > line.end.y {
      incr_y = -1;
    }else if line.start.y== line.end.y {
      incr_y = 0;
    }

    let mut x = line.start.x;
    let mut y = line.start.y;
    while {
      let point = (Point {x, y}).to_string();
      let counter = count_per_point.entry(point).or_insert(0);
      *counter += 1;
      if *counter == 2 {
        double_lines_counter += 1;
      }
      x += incr_x;
      y += incr_y;

      x - incr_x != line.end.x || y - incr_y != line.end.y
    }{}
  }
  return double_lines_counter;
}

fn main() {
    let file = File::open("inputs/day05.txt").unwrap();
    let data: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line|line.unwrap())
                .collect();

    println!(r#"🎉 Part 1 result is {:?}"#, solution_1(&data));

    println!(r#"🎉 Part 2 result is {:?}"#, solution_2(&data));
}