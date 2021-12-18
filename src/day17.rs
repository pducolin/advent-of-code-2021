use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Target {
  x_min: i64,
  x_max: i64,
  y_min: i64,
  y_max: i64,
}

fn parse_target(data: &String) -> Target {
  let instruction_str = data["target area: ".len()..].to_string();
  let mut splits = instruction_str.split(", ");
  // x
  let mut x_splits = splits.next().unwrap()["x=".len()..].split("..");
  let x_min = i64::from_str_radix(x_splits.next().unwrap(), 10).unwrap();
  let x_max = i64::from_str_radix(x_splits.next().unwrap(), 10).unwrap();
  // y
  let mut y_splits = splits.next().unwrap()["y=".len()..].split("..");
  let y_min = i64::from_str_radix(y_splits.next().unwrap(), 10).unwrap();
  let y_max = i64::from_str_radix(y_splits.next().unwrap(), 10).unwrap();

  return Target {
    x_min,
    x_max,
    y_min,
    y_max
  }
}

#[derive(Clone, Debug)]
struct Point {
  x: i64,
  y: i64,
}

impl Point {
  fn apply_velocity(&mut self, velocity: Velocity) {
    self.x += velocity.x;
    self.y += velocity.y;
  }

  fn is_in_target(&self, target: &Target) -> bool {
    return self.x >= target.x_min && 
    self.x <= target.x_max &&
    self.y >= target.y_min && 
    self.y <= target.y_max; 
  }
}

#[derive(Clone, Debug)]
struct Velocity {
  x: i64,
  y: i64,
}

impl Velocity {
  fn step(&mut self) {
    if self.x > 0 {
      self.x -= 1;
    }
    self.y -= 1;
  }
}

struct Player {
  point: Point,
  velocity: Velocity,
  max_y: i64,
}

impl Player {
  fn fire_step(&mut self) {
    self.point.apply_velocity(self.velocity.clone());
    self.max_y = cmp::max(self.max_y, self.point.y);
    self.velocity.step();
    // println!("Point: {:?}, velocity: {:?}", self.point, self.velocity);
  }

  fn is_out_of_target(&self, target: &Target) -> bool {
    if self.velocity.x == 0 {
      // fell over target
      return self.point.x < target.x_min || self.point.x > target.x_max || self.point.y < target.y_min;
    }

    return self.point.y < target.y_min;
  }
}

fn fire(velocity: &Velocity, target: &Target) -> Option<i64> {
  let mut player = Player{
    point: Point{x:0, y:0},
    velocity: velocity.clone(),
    max_y: i64::MIN,
  };
  loop {
    player.fire_step();
    if player.point.is_in_target(&target) {
      return Some(player.max_y);
    }
    if player.is_out_of_target(&target) {
      return None;
    }
  }
}

fn evaluate_final_x(x_v: i64) -> i64 {
  let mut x_v = x_v.clone();
  let mut final_x = 0;
  while x_v > 0 {
    final_x += x_v;
    x_v -= 1;
  }
  return final_x;
}

fn find_min_x_v(target: &Target) -> i64 {
  let mut x = 0;
  // reach x min
  loop {
    let final_x = evaluate_final_x(x);
    if final_x >= target.x_min {
      break;
    }
    x += 1;
  }

  return x;
}

fn solution_1(data: &Vec<String>) -> i64 {
  let target: Target = parse_target(&data[0]);
  
  let x_min_v = find_min_x_v(&target);
  let mut y_max = i64::MIN;

  // reach target
  for x_v in x_min_v..100 {
    for y_v in -200..200 {
      let velocity = Velocity{x: x_v, y:y_v};
      let res = fire(&velocity, &target);
      if res.is_some() {
        // println!("🎯 Reached target at velocity {:?}", velocity);
        y_max = cmp::max(y_max, res.unwrap());
      }
    }
  }
  return y_max;
}

fn solution_2(data: &Vec<String>) -> i64 {
  let target: Target = parse_target(&data[0]);
  
  let x_min_v = find_min_x_v(&target);
  let mut count = 0;

  // reach target
  for x_v in x_min_v..500 {
    for y_v in -1000..1000 {
      let velocity = Velocity{x: x_v, y:y_v};
      let res = fire(&velocity, &target);
      if res.is_some() {
        count += 1;
      }
    }
  }
  return count;
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day17".to_string();
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
