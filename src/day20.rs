use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pixel {
  row: i64,
  col: i64,
}

impl Pixel {
  fn new(row: i64, col: i64) -> Self {
    Self {
      row,
      col
    }
  }
}

struct Algorithm {
  samples: Vec<bool>
}

impl Algorithm {
  fn parse(s: String) -> Self {
    Self {
      samples: s.chars().map(|x| x== '#').collect()
    }
  }
}

#[derive(Clone)]
struct Size {
  rows: usize,
  cols: usize,
}

impl Size {
  fn new(rows: usize, cols: usize) -> Self {
    Self {
      rows,
      cols
    }
  }
}

struct Image {
  matrix: HashMap<Pixel, bool>,
  size: Size,
}


impl Image {
  fn parse(data: &Vec<String>) -> Self {
    let size = Size::new(data.len(), data.first().unwrap().len());
    let mut matrix: HashMap<Pixel, bool> = HashMap::new();

    for r_i in 0..size.rows {
      let row: Vec<char> = data[r_i].chars().into_iter().collect();
      for c_i in 0..size.cols {
        let pixel_on: bool = row[c_i] == '#';
        matrix.insert(Pixel{row: r_i as i64, col: c_i as i64}, pixel_on);
      }
    }
    Self{
      matrix,
      size
    }
  }

  fn increase_size(&self, offset: i64, border_on: bool) -> Self {
    // increase adding a 2 pixel border
    // all pixels increase row and col by 2 
    let size = Size::new(self.size.rows + (offset as usize)*2, self.size.cols + (offset as usize)*2);
    let mut matrix: HashMap<Pixel, bool> = HashMap::new();

    for r_i in 0..size.rows {
      for c_i in 0..size.cols {
        matrix.insert(Pixel{row: r_i as i64, col: c_i as i64}, border_on);
      }
    }

    for (pixel, is_on) in &self.matrix {
      matrix.insert(Pixel::new(pixel.row+offset, pixel.col+offset), is_on.clone());
    }

    Self{
      matrix,
      size
    }
  }

  fn get_algo_index(&self, target: &Pixel, border_on: bool) -> usize {
    let mut neighbours = "".to_owned();
    let char_by_bool: HashMap<bool, char> = HashMap::from([
      (true, '1'),
      (false, '0'),
    ]);
    // find neighbours for target
    // top-left 
    let neighbour = self.matrix.get(&Pixel{row: target.row - 1, col: target.col - 1});
    if neighbour.is_some() {
      neighbours.push(char_by_bool.get(neighbour.unwrap()).unwrap().clone());
    } else {
      neighbours.push(char_by_bool.get(&border_on).unwrap().clone());
    }
    // top
    let neighbour = self.matrix.get(&Pixel{row: target.row - 1, col: target.col});
    if neighbour.is_some() {
      neighbours.push(char_by_bool.get(neighbour.unwrap()).unwrap().clone());
    } else {
      neighbours.push(char_by_bool.get(&border_on).unwrap().clone());
    }
    // top-right
    let neighbour = self.matrix.get(&Pixel{row: target.row - 1, col: target.col + 1});
    if neighbour.is_some() {
      neighbours.push(char_by_bool.get(neighbour.unwrap()).unwrap().clone());
    } else {
      neighbours.push(char_by_bool.get(&border_on).unwrap().clone());
    }
    // left
    let neighbour = self.matrix.get(&Pixel{row: target.row, col: target.col - 1});
    if neighbour.is_some() {
      neighbours.push(char_by_bool.get(neighbour.unwrap()).unwrap().clone());
    } else {
      neighbours.push(char_by_bool.get(&border_on).unwrap().clone());
    }
    // center
    let neighbour = self.matrix.get(&Pixel{row: target.row, col: target.col});
    if neighbour.is_some() {
      neighbours.push(char_by_bool.get(neighbour.unwrap()).unwrap().clone());
    } else {
      neighbours.push(char_by_bool.get(&border_on).unwrap().clone());
    }
    // right
    let neighbour = self.matrix.get(&Pixel{row: target.row, col: target.col + 1});
    if neighbour.is_some() {
      neighbours.push(char_by_bool.get(neighbour.unwrap()).unwrap().clone());
    } else {
      neighbours.push(char_by_bool.get(&border_on).unwrap().clone());
    }
    // bottom-left 
    let neighbour = self.matrix.get(&Pixel{row: target.row + 1, col: target.col - 1});
    if neighbour.is_some() {
      neighbours.push(char_by_bool.get(neighbour.unwrap()).unwrap().clone());
    } else {
      neighbours.push(char_by_bool.get(&border_on).unwrap().clone());
    }
    // bottom
    let neighbour = self.matrix.get(&Pixel{row: target.row + 1, col: target.col});
    if neighbour.is_some() {
      neighbours.push(char_by_bool.get(neighbour.unwrap()).unwrap().clone());
    } else {
      neighbours.push(char_by_bool.get(&border_on).unwrap().clone());
    }
    // bottom-right
    let neighbour = self.matrix.get(&Pixel{row: target.row + 1, col: target.col + 1});
    if neighbour.is_some() {
      neighbours.push(char_by_bool.get(neighbour.unwrap()).unwrap().clone());
    } else {
      neighbours.push(char_by_bool.get(&border_on).unwrap().clone());
    }

    usize::from_str_radix(&neighbours, 2).unwrap()
  }

  fn apply_algo(&self, algo: &Algorithm, border_on: bool) -> Self {
    let mut matrix: HashMap<Pixel, bool> = HashMap::new();

    for r_i in 0..self.size.rows {
      for c_i in 0..self.size.cols {
        let target = Pixel::new(r_i as i64, c_i as i64);
        let algo_index = self.get_algo_index(&target, border_on);
        matrix.insert(target.clone(), algo.samples[algo_index]);
      }
    }

    Self {
      matrix,
      size: self.size.clone()
    }
  }
}


fn solution_1(data: &Vec<String>) -> usize {
  let algo_str = data[0].clone();
  let algo = Algorithm::parse(algo_str);
  let mut image = Image::parse(&data[2..].to_vec());

  let mut border_on = false;
  for _ in 0..2 {
    image = image.increase_size(2, border_on);
    image = image.apply_algo(&algo, border_on);
    
    if !border_on {
      border_on = algo.samples.first().unwrap().clone();
    } else {
      border_on = algo.samples.last().unwrap().clone();
    }
  }

  return image.matrix.iter().filter(|(_,is_on)|*is_on.clone()).count();
}

fn solution_2(data: &Vec<String>) -> usize {
  let algo_str = data[0].clone();
  let algo = Algorithm::parse(algo_str);
  let mut image = Image::parse(&data[2..].to_vec());

  let mut border_on = false;
  for _ in 0..50 {
    image = image.increase_size(2, border_on);
    image = image.apply_algo(&algo, border_on);
    
    if !border_on {
      border_on = algo.samples.first().unwrap().clone();
    } else {
      border_on = algo.samples.last().unwrap().clone();
    }
  }

  return image.matrix.iter().filter(|(_,is_on)|*is_on.clone()).count();
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day20".to_string();
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
