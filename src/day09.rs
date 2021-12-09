use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
  row: usize,
  col: usize,
}

struct Location {
  value: u32,
  position: Point,
}

struct Heightmap {
  matrix: Vec<Vec<Location>>,
  tot_rows: usize,
  tot_cols: usize,
}

impl Heightmap {
  pub fn parse(data: &Vec<String>) -> Self { 
    let mut heightmap = Self {
      matrix: Vec::new(),
      tot_cols: data[0].len(),
      tot_rows: data.len(),
    };
    for row in 0..data.len() {
      let values: Vec<u32> = data[row]
        .chars()
        .map(|c|c.to_digit(10).unwrap())
        .collect();
      let mut cols: Vec<Location> = Vec::new();
      for col in 0 .. values.len() {
        cols.push(Location{
          value: values[col],
          position: Point{row, col}
        })
      }
      heightmap.matrix.push(cols);
    }

    return heightmap;
  }

  fn get_neighbours(&self, point: Point) -> Vec<&Location> {
    let mut neighbours: Vec<&Location> = Vec::new();

    if point.row > 0 {
      // add top
      neighbours.push(&self.matrix[point.row - 1][point.col])
    }
    if point.col > 0 {
      // add left
      neighbours.push(&self.matrix[point.row][point.col - 1])
    }
    if point.row < self.tot_rows - 1 {
      // add bottom
      neighbours.push(&self.matrix[point.row + 1][point.col])
    }
    if point.col < self.tot_cols - 1 {
      // add bottom
      neighbours.push(&self.matrix[point.row][point.col + 1]);
    }

    return neighbours;
  }

  fn evaluate_basin_size(&mut self, low: Point) -> u32 {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut basin: Vec<Point> = Vec::new();
    basin.push(low);
    visited.insert(low);
  
    let mut index = 0;
    while index < basin.len() {
      let current: Point = basin[index].clone();
      let current_value = self.matrix[current.row][current.col].value;

      let neighbours = self.get_neighbours(current);
      for n in neighbours {
        if visited.contains(&n.position) {
          continue;
        }
        if n.value == 9 {
          continue
        }
        if n.value <= current_value {
          continue;
        }
        visited.insert(n.position);
        basin.push(n.position.clone());
      }

      index += 1
    }

  
    return basin.len() as u32;
  }
}


fn solution_1(data: &Vec<String>) -> u32 {
  let mut counter = 0;

  let hm = Heightmap::parse(data);

  for row in 0..hm.tot_rows {
    for col in 0..hm.tot_cols {
      let val: u32 = hm.matrix[row][col].value;
      let neighbours = hm.get_neighbours(Point{row, col});
      if neighbours.iter().filter(|x| x.value <= val).count() == 0 {
        counter += val + 1;
      }
    }
  }

  return counter;
}

fn solution_2(data: &Vec<String>) -> u32 {

  let mut hm = Heightmap::parse(data);
  let mut lows: Vec<Point> = Vec::new();

  for row in 0..hm.tot_rows {
    for col in 0..hm.tot_cols {
      let val: u32 = hm.matrix[row][col].value;
      let neighbours = hm.get_neighbours(Point{row, col});
      if neighbours.iter().filter(|x| x.value <= val).count() == 0 {
        lows.push(Point{row, col});
      }
    }
  }

  let mut sizes: Vec<u32> = Vec::new();
  // recursion on lows
  for low in lows{
    let size = hm.evaluate_basin_size(low);
    sizes.push(size);
  }  
  sizes.sort();

  let mut ret = 1;
  for x in sizes.iter().rev().take(3) {
    ret *= x;
  }

  return ret;
}

fn main() {
  let test = false;
  if test {
    let file = File::open("inputs/day09.test.txt").unwrap();
    let data: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line|line.unwrap())
                .collect();

    println!(r#"🧪 Part 1 test result is {:?}"#, solution_1(&data));

    println!(r#"🧪 Part 2 test result is {:?}"#, solution_2(&data));
  } else {

    let file = File::open("inputs/day09.txt").unwrap();
    let data: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line|line.unwrap())
                .collect();

    println!(r#"🎉 Part 1 result is {:?}"#, solution_1(&data));

    println!(r#"🎉 Part 2 result is {:?}"#, solution_2(&data));
  }
}