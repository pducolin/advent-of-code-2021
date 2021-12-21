use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::iter::Scan;
use std::thread::park;

// Solution inspired by 
// https://www.reddit.com/r/adventofcode/comments/rjpf7f/comment/hp7tpyf/?utm_source=share&utm_medium=web2x&context=3

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Point {
  x: i64,
  y: i64,
  z: i64,
}

impl Point {
  fn new(x: i64, y: i64, z: i64) -> Self {
    Self {
      x,
      y,
      z,
    }
  } 

  fn from_str(s: &String) -> Self {
    let coordinates: Vec<i64> = s
    .split(",")
    .map(|x|i64::from_str_radix(x, 10).unwrap())
    .collect();
    Self {
      x: coordinates[0],
      y: coordinates[1],
      z: coordinates[2],
    }
  }

  fn manhattan_distance(&self, other: &Point) -> i64 {
    let d_x = self.x - other.x;
    let d_y = self.y - other.y;
    let d_z = self.z - other.z;
    return i64::abs(d_x) + i64::abs(d_y) + i64::abs(d_z);
  }

  fn offset_from(&self, other: &Point) -> Point {
    return Point::new(self.x - other.x, self.y - other.y, self.z - other.z);
  }

  fn add_offset(&self, offset: &Point) -> Point {
    return Point::new(self.x + offset.x, self.y + offset.y, self.z + offset.z);
  }

  fn rotate(&self, rotate_index: u8) -> Self {
    let mut new_point = Point::new(self.x,self.y,self.z);
    // rotate on axis x and y so that z is facing one of the 6 axes
    match rotate_index / 4 {
      0 => {
          // Unchanged
      }
      1 => {
          // Rotate 180 degrees around the y-axis so z points towards -z
          new_point = Point::new(-new_point.x, new_point.y, -new_point.z);
      }
      2 => {
          // Rotate 90 degrees around the y-axis so that z points towards -x
          new_point = Point::new(new_point.z, new_point.y, -new_point.x);
      }
      3 => {
          // Rotate 90 degrees around the y-axis so that z points towards +x
          new_point = Point::new(-new_point.z, new_point.y, new_point.x);
      }
      4 => {
          // Rotate 90 degrees around the x-axis so that z points towards -y
          new_point = Point::new(new_point.x, new_point.z, -new_point.y);
      }
      5 => {
          // Rotate 90 degrees around the x-axis so that z points towards +y
          new_point = Point::new(new_point.x, -new_point.z, new_point.y);
      }
      _ => panic!("Invalid rotate index"),
    }

    // rotate on z axis 
    match rotate_index % 4 {
      0 => {} // Unchanged
      1 => {
          // Rotate 90 degrees around z
          new_point = Point::new(-new_point.y, new_point.x, new_point.z);
      }
      2 => {
          // Rotate 180 degrees around z
          new_point = Point::new(-new_point.x, -new_point.y, new_point.z);
      }
      3 => {
          // Rotate 270 degrees around z
          new_point = Point::new(new_point.y, -new_point.x, new_point.z);
      }
      _ => panic!("Invalid rotate index"),
    }

    return new_point;
  }
}

#[derive(Clone)]
struct RotationAndOffset {
  rotation_index: u8,
  offset: Point,
}

#[derive(Clone, Debug)]
struct Scanner {
  beacons: Vec<Point>,
}

impl Scanner {
  fn from_data(data: &Vec<String>) -> Self {
    let mut beacons: Vec<Point> = Vec::new();
    for d in data {
      beacons.push(Point::from_str(d));
    }
    Self {
      beacons
    }
  }

  fn rotate(&mut self, rotation_index: u8) {
    // update beacons
    let mut new_beacons: Vec<Point> = Vec::new();
    for beacon in &self.beacons {
      new_beacons.push(beacon.rotate(rotation_index));
    }
    self.beacons.clear();
    self.beacons.append(&mut new_beacons);
  }


  fn add_offset(&mut self, offset: &Point) {
    // update beacons
    let mut new_beacons: Vec<Point> = Vec::new();
    for beacon in &self.beacons {
      new_beacons.push(beacon.add_offset(offset));
    }
    self.beacons = new_beacons;
  }

  fn matches(&self, other: &Scanner) -> Option<Point> {
    // iterate over other scanners 
    for my_beacon in self.beacons.clone() {
      for other_beacon in other.beacons.clone() {
        let offset = other_beacon.offset_from(&my_beacon);

        // count how many points in both scanners
        let mut count = 0;
        for a in self.beacons.clone() {
          if other.beacons.contains(&a.add_offset(&offset)) {
            count += 1;
          }
        }

        if count == 0 {
          // not possible, at least my beacon and other beacon should match
          println!("my_beacon {:?}", my_beacon);
          println!("other_beacon {:?}", other_beacon);
          println!("offset {:?}", offset);
          println!("replaced {:?}", my_beacon.add_offset(&offset));
          panic!("Should not happen")
        }

        if count >= 12 {
          // match found
          return Some(offset);
        }
      }
    }
    
    None
  }

  fn add_beacons(&mut self, beacons: &Vec<Point>) {
    let beacons_set: HashSet<Point> = HashSet::from_iter(self.beacons.iter().map(|b|b.clone()));
    for b in beacons {
      if !beacons_set.contains(b) {
        self.beacons.push(b.clone());
      }
    }
  }
}

fn parse_scanners(data: &Vec<String>) -> Vec<Scanner> {
  let mut scanners: Vec<Scanner> = Vec::new();
  let mut scanner_data: Vec<String> = Vec::new();

  for d in data {
    if d.starts_with("---"){
      continue;
    }
    if d.is_empty() {
      let scanner = Scanner::from_data(&scanner_data);
      scanners.push(scanner);
      scanner_data.clear();
      continue;
    }
    scanner_data.push(d.clone());
  }

  if !scanner_data.is_empty() {
    let scanner = Scanner::from_data(&scanner_data);
    scanners.push(scanner);
    scanner_data.clear();
  }

  println!("Parsed {} scanners", scanners.len());
  return scanners;
}

fn rotate_all_scanners(scanners: &Vec<Scanner>) -> Vec<Vec<Scanner>> {
  let mut rotated_scanners: Vec<Vec<Scanner>> = Vec::with_capacity(scanners.len());
  for scanner in scanners {
    let mut rotated_scanner_i: Vec<Scanner> = Vec::with_capacity(24);
    for rotation_index in 0..24 {
      let mut scanner_rotated = scanner.clone();
      scanner_rotated.rotate(rotation_index);
      rotated_scanner_i.push(scanner_rotated);
    }
    rotated_scanners.push(rotated_scanner_i);
  }
  return rotated_scanners;
}

fn align_scanners(scanners: &Vec<Scanner>) -> (HashMap<usize, Point>, Scanner) {
  let all_rotated_scanners = rotate_all_scanners(scanners);
  let mut global_scanner: Scanner = scanners[0].clone();
  let mut aligned : HashMap<usize, Scanner> = HashMap::new();
  let mut offsets: HashMap<usize, Point> = HashMap::new();

  aligned.insert(0, scanners[0].clone());
  offsets.insert(0, Point::new(0, 0, 0));

  while aligned.len() < scanners.len() {
    let mut match_found = false;
    for i in 1..scanners.len() {
      if aligned.contains_key(&i) {
        continue;
      }

      // try aligning scanner i with aligned scanners
      // iterate over aligned scanners 
      for (j, scanner_aligned) in aligned.clone().iter() {
        if i == *j {
          panic!("i and j should not be the same");
        }
        for rotated_scanner in all_rotated_scanners.get(i).unwrap() {
          let res = rotated_scanner.matches(&scanner_aligned);
          if res.is_none() {
            continue
          }
          let offset = res.unwrap();
          println!("Scanner {} matched with scanner {}, offset {:?}", i, j, offset);

          let mut aligned_scanner = rotated_scanner.clone();
          aligned_scanner.add_offset(&offset);
          global_scanner.add_beacons(&aligned_scanner.clone().beacons);

          println!("Offset to 0 {:?}", offset);
          
          offsets.insert(i, offset);

          aligned.insert(i, aligned_scanner.clone());
          match_found = true;
          break;
        }

        if match_found {
          break;
        }
      }

      if match_found {
        break;
      }
    }
    if !match_found {
      for i in 1..scanners.len() {
        if aligned.contains_key(&i) {
          continue;
        }
        println!("No match for scanner {}", i);
      }
      panic!("Oh no");
    }
  }

  return (offsets, global_scanner);
}

fn solution_1(data: &Vec<String>) -> usize {
  let scanners = parse_scanners(data);

  let (_, global_scanner) = align_scanners(&scanners);

  return global_scanner.beacons.len();
  442
}

fn solution_2(data: &Vec<String>) -> i64 {
  let scanners = parse_scanners(data);

  let (offsets, _) = align_scanners(&scanners);

  let mut max_distance = i64::MIN;

  for i in 0..offsets.len()-1 {
    for j in i+1..offsets.len() {
      let manhattan_distance = offsets.get(&i).unwrap().manhattan_distance(&offsets.get(&j).unwrap());
      max_distance = i64::max(max_distance, manhattan_distance);
    }
  }

  return max_distance;
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day19".to_string();
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
