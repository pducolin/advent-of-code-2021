use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Inspect;

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
struct Range3D {
  range_x: Range,
  range_y: Range,
  range_z: Range,
}

impl Range3D {
  fn is_in_range(&self, other: &Range3D) -> bool {
    self.range_x.is_in_range(&other.range_x) &&
    self.range_y.is_in_range(&other.range_y) &&
    self.range_z.is_in_range(&other.range_z)
  }
}

#[derive(Debug, Clone)]
struct Instruction {
  range: Range3D,
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
      range: Range3D{
        range_x: Range{
          min: x_min,
          max: x_max,
        },
        range_y: Range{
          min: y_min,
          max: y_max,
        },
        range_z: Range{
          min: z_min,
          max: z_max,
        },
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
  cubes: HashSet<Point>
}

impl Cuboid {
  fn new() -> Self {
    Self {
      cubes: HashSet::new()
    }
  }

  fn apply_instruction(&mut self, instruction: &Instruction, valid_range: Option<&Range3D>) {
    let mut range_x = instruction.range.range_x.clone();
    let mut range_y = instruction.range.range_y.clone();
    let mut range_z = instruction.range.range_z.clone();
    if valid_range.is_some() {
      range_x.min = i64::max(range_x.min, valid_range.unwrap().range_x.min);
      range_x.max = i64::min(range_x.max, valid_range.unwrap().range_x.max);
      range_y.min = i64::max(range_y.min, valid_range.unwrap().range_x.min);
      range_y.max = i64::min(range_y.max, valid_range.unwrap().range_x.max);
      range_z.min = i64::max(range_z.min, valid_range.unwrap().range_x.min);
      range_z.max = i64::min(range_z.max, valid_range.unwrap().range_x.max);
    }
    for x in range_x.min..range_x.max + 1 {
      for y in range_y.min..range_y.max + 1 {
        for z in range_z.min..range_z.max + 1 {
          let cube = Point{x,y,z};
          if instruction.on {
            // turn on this cube
            self.cubes.insert(cube);
          } else {
            self.cubes.remove(&cube);
          }
        }
      } 
    }
  }
}


fn solution_1(data: &Vec<String>) -> usize {
  let valid_range = Range3D {
    range_x: Range::new(-50, 50),
    range_y: Range::new(-50, 50),
    range_z: Range::new(-50, 50),
  };
  let mut instructions = parse_instructions(data);

  let mut cuboid = Cuboid::new();
  for inst in instructions.iter().filter(|i|i.range.is_in_range(&valid_range)) {
    cuboid.apply_instruction(inst, Some(&valid_range));
  }

  cuboid.cubes.len()
}

fn solution_2(data: &Vec<String>) -> usize {
  let instructions = parse_instructions(data);

  let mut cuboid = Cuboid::new();
  for i in 0..instructions.len() {
    println!("Applying instruction {} of {}", i + 1, instructions.len());
    let inst: &Instruction = instructions.get(i).unwrap();
    cuboid.apply_instruction(&inst, None);
  }

  cuboid.cubes.len()
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
