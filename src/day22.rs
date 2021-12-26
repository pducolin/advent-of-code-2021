use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
  x: i64,
  y: i64,
  z: i64
}

#[derive(Debug, Clone)]
struct Range {
  min: i64,
  max: i64,
}
impl Range {
  fn new(min:i64, max:i64) -> Self {
    Self{
      min,
      max
    }
  }

  fn is_in_range(&self, other: &Range) -> bool {
    self.min <= other.max && self.max >= other.min
  }
}

#[derive(Debug, Clone)]
struct Instruction {
  cuboid: Cuboid,
  on: bool
}

impl Instruction {
  fn parse(data: &String) -> Self {
    let mut iter = data.split_whitespace();
    let on: bool = iter.next().unwrap() == "on";

    let mut iter = iter.next().unwrap().split(",");
    let mut numbers = iter.next().unwrap().split("=").last().unwrap().split("..").map(|x|i64::from_str_radix(x, 10).unwrap());
    let x_min = numbers.next().unwrap();
    let x_max = numbers.next().unwrap();
    
    let mut numbers = iter.next().unwrap().split("=").last().unwrap().split("..").map(|x|i64::from_str_radix(x, 10).unwrap());
    let y_min = numbers.next().unwrap();
    let y_max = numbers.next().unwrap();

    let mut numbers = iter.next().unwrap().split("=").last().unwrap().split("..").map(|x|i64::from_str_radix(x, 10).unwrap());
    let z_min = numbers.next().unwrap();
    let z_max = numbers.next().unwrap();

    Self {
      cuboid: Cuboid{
        x: Range{
          min: x_min,
          max: x_max,
        },
        y: Range{
          min: y_min,
          max: y_max,
        },
        z: Range{
          min: z_min,
          max: z_max,
        },
        off: Vec::new()
      },
      on
    }
  }
}

fn parse_instructions(data: &Vec<String>) -> Vec<Instruction> {
  let mut instructions: Vec<Instruction> = Vec::new();
  for d in data {
    instructions.push(Instruction::parse(d));
  } 
  return instructions;
}

#[derive(Debug, Clone)]
struct Cuboid {
  x: Range,
  y: Range,
  z: Range,

  off: Vec<Cuboid>
}

impl Cuboid {
  fn intersects(&self, other: &Cuboid) -> bool {
    self.x.is_in_range(&other.x) &&
    self.y.is_in_range(&other.y) &&
    self.z.is_in_range(&other.z)
  }

  fn subtract(&mut self, other: &Cuboid) {
    if !self.intersects(other) {
      return;
    }
    let intersect_cube = Cuboid {
      x: Range{
        min: i64::max(self.x.min, other.x.min),
        max: i64::min(self.x.max, other.x.max),
      },
      y: Range{
        min: i64::max(self.y.min, other.y.min),
        max: i64::min(self.y.max, other.y.max),
      },
      z: Range{
        min: i64::max(self.z.min, other.z.min),
        max: i64::min(self.z.max, other.z.max),
      },
      off: Vec::new()
    };
    self.off.iter_mut().for_each(|c|c.subtract(other));
    self.off.push(intersect_cube);
  }

  fn volume(&self) -> u128 {
    let off_volume : u128= self.off.iter().map(|c|c.volume()).sum();
    ((self.x.max - self.x.min + 1) as u128
    * (self.y.max - self.y.min + 1) as u128
    * (self.z.max - self.z.min + 1) as u128)
    - off_volume
  }
}


fn solution_1(data: &Vec<String>) -> u128 {
  let valid_cuboid = Cuboid {
    x: Range::new(-50, 50),
    y: Range::new(-50, 50),
    z: Range::new(-50, 50),
    off: Vec::new()
  };
  let instructions = parse_instructions(data);

  let mut cubes: Vec<Cuboid> = Vec::new();
  instructions.iter().filter(|i|i.cuboid.intersects(&valid_cuboid)).for_each(|i|{
    let new_cube = i.cuboid.clone();
    cubes.iter_mut().for_each(|c|c.subtract(&new_cube));
    if i.on {
      cubes.push(new_cube);
    }
  });

  return cubes.iter().map(|c|c.volume()).sum();
}

fn solution_2(data: &Vec<String>) -> u128 {
  let instructions = parse_instructions(data);

  let mut cubes: Vec<Cuboid> = Vec::new();
  instructions.iter().for_each(|i|{
    let new_cube = i.cuboid.clone();
    cubes.iter_mut().for_each(|c|c.subtract(&new_cube));
    if i.on {
      cubes.push(new_cube);
    }
  });

  return cubes.iter().map(|c|c.volume()).sum();
}

fn main() {
  let test = true;
  let mut file_path: String = "inputs/day22".to_string();
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
