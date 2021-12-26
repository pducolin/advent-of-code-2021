use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use lazy_static::lazy_static;

static BOARD_WIDTH: i64 = 13;
static  BOARD_HEIGHT_2: i64 = 7;

lazy_static! {
  static ref COST_BY_LETTER: HashMap<char, i64> = HashMap::from([
    ('A', 1),
    ('B', 10),
    ('C', 100),
    ('D', 1000)
  ]);

  static ref HALLWAY: Vec<Point> = Vec::from([
    Point::new(1,1),
    Point::new(1,2),
    Point::new(1,4),
    Point::new(1,6),
    Point::new(1,8),
    Point::new(1,10),
    Point::new(1,11),
  ]);

  static ref TARGET_ROOM_BY_LETTER: HashMap<char, i64> = HashMap::from([
    ('A', 3),
    ('B', 5),
    ('C', 7),
    ('D', 9),
  ]);


  static ref TARGET_BY_DEPTH: HashMap<i64,HashMap<Point, char>> =  HashMap::from([
    (2, HashMap::from([
    // room 1
    (Point::new(2,3), 'A'),
    (Point::new(3,3), 'A'),
    // room 2
    (Point::new(2,5), 'B'),
    (Point::new(3,5), 'B'),
    // room 3
    (Point::new(2,7), 'C'),
    (Point::new(3,7), 'C'),
    // room 4
    (Point::new(2,9), 'D'),
    (Point::new(3,9), 'D'),
    ])),
    (4, HashMap::from([
      // room 1
      (Point::new(2,3), 'A'),
      (Point::new(3,3), 'A'),
      (Point::new(4,3), 'A'),
      (Point::new(5,3), 'A'),
      // room 2
      (Point::new(2,5), 'B'),
      (Point::new(3,5), 'B'),
      (Point::new(4,5), 'B'),
      (Point::new(5,5), 'B'),
      // room 3
      (Point::new(2,7), 'C'),
      (Point::new(3,7), 'C'),
      (Point::new(4,7), 'C'),
      (Point::new(5,7), 'C'),
      // room 4
      (Point::new(2,9), 'D'),
      (Point::new(3,9), 'D'),
      (Point::new(4,9), 'D'),
      (Point::new(5,9), 'D'),
    ])),
  ]);

  static ref WALLS_2: HashSet<Point> = build_walls();
}

fn build_walls() -> HashSet<Point> {
  let mut walls: HashSet<Point> = HashSet::new();

  // row 0
  for col in 0..BOARD_WIDTH {
    walls.insert(Point::new(0, col));
  }

  // left 
  walls.insert(Point::new(1,0));
  walls.insert(Point::new(2,0));
  walls.insert(Point::new(2,1));
  for row in 2..BOARD_HEIGHT_2 {
    walls.insert(Point::new(row,2));
  }
  // floor
  for col in 3..BOARD_WIDTH - 3 {
    walls.insert(Point::new(BOARD_HEIGHT_2 -1, col));
  }
  // right
  for row in 2..BOARD_HEIGHT_2 {
    walls.insert(Point::new(row,BOARD_WIDTH - 3));
  }
  walls.insert(Point::new(1,BOARD_WIDTH - 1));
  walls.insert(Point::new(2,BOARD_WIDTH - 1));
  walls.insert(Point::new(2,BOARD_WIDTH - 2));

  // room walls
  for row in 2..BOARD_HEIGHT_2 -1 {
    walls.insert(Point::new(row, 4));
    walls.insert(Point::new(row, 6));
    walls.insert(Point::new(row, 8));
  }

  return walls;
}

#[derive(Debug, Clone)]
struct Step {
  letter: char,
  from: Point,
  to: Point,
}
impl Step {
  fn distance(&self) -> i64 {
    (self.from.row - 1).abs() // distance from self.row to row 1
    + (1 - self.to.row).abs() // distance from 1 to other.row
    + (self.from.col - self.to.col).abs()
  }

  fn cost(&self) -> i64 {
    self.distance() * COST_BY_LETTER[&self.letter]
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
  row: i64,
  col: i64,
}

impl Point {
  fn new(row: i64, col: i64) -> Self {
    Self { row, col }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
  original: HashMap<Point, char>,
  amphipods: HashMap<Point, char>,
  room_depth: i64,
}


impl Board {  
  fn parse(data: &Vec<String>) -> Self {
    let mut amphipods: HashMap<Point, char> = HashMap::new();
    for r in 0..data.len() {
      let row: Vec<char> = data[r].chars().collect();
      for c in 0..row.len() {
        let letter = row[c];
        match letter {
            'A' | 'B' | 'C' | 'D' => {
              amphipods.insert(Point{ row: r as i64, col: c as i64}, letter);
            },
            _ => continue
        }
      }
    }
    Self {
      original: amphipods.clone(),
      amphipods,
      room_depth: (data.len() - 3) as i64
    }
  } 

  fn is_in_hallway(point: &Point) -> bool {
    point.row == 1
  }

  fn left_hallway(point: &Point) -> Vec<Point> {
    HALLWAY.iter().filter(|p|p.col < point.col)
    .map(|p|p.clone())
    .rev()
    .collect()
  }

  fn right_hallway(point: &Point) -> Vec<Point> {
    HALLWAY.iter().filter(|p|p.col > point.col)
    .map(|p|p.clone())
    .collect()
  }

  fn can_move_left(&self, point: &Point) -> bool {
    Self::left_hallway(point).first().is_some() &&
    !self.amphipods.contains_key(Self::left_hallway(point).first().unwrap())
  }

  fn can_move_right(&self, point: &Point) -> bool {
    Self::right_hallway(point).first().is_some() &&
    !self.amphipods.contains_key(Self::right_hallway(point).first().unwrap())
  }

  fn can_move_to_target_room(&self, letter: &char, point: &Point) -> bool {
    if !Self::is_in_hallway(point) && !self.can_move_up(point) {
      return false
    }

    // check all hallway points are available
    let target_room: i64 = TARGET_ROOM_BY_LETTER[&letter];
    if target_room == point.col {
      return false;
    }

    // is hallway slice accessible
    if target_room < point.col {
      if Self::left_hallway(point).iter()
      .filter(|x|x.col > target_room)
      .any(|x|self.amphipods.contains_key(x)) {
        return false;
      }
    } else {
      if Self::right_hallway(point).iter()
      .filter(|x|x.col < target_room)
      .any(|x|self.amphipods.contains_key(x)) {
        return false;
      }
    }

    !self.amphipods.iter().filter(|(x,_)|x.col == target_room).any(|(_, l)|l != letter)
  }

  fn can_move(&self, letter: &char, point: &Point) -> bool {
    if self.is_done(letter, point) {
      return false;
    }
    if Self::is_in_hallway(point) {
      return self.can_move_to_target_room(letter, point);
    } 

    // it is in a room
    // can move up ?
    if !self.can_move_up(point) {
      return false;
    }

    // can move left or right ?
    return self.can_move_left(point) 
    || self.can_move_right(point)
  }

  fn can_move_up(&self, point: &Point) -> bool {
    !self.amphipods.contains_key(&Point::new(point.row - 1, point.col))
  }

  fn is_done(&self, letter: &char, point: &Point) -> bool {
    point.col == TARGET_ROOM_BY_LETTER[letter] 
    && point.row > 1
    && self.amphipods.clone().into_iter()
    .filter(|(other_point, _)| other_point.col == TARGET_ROOM_BY_LETTER[letter])
    .all(|(_,l)| l == *letter)
  }

  fn deepest_cell_in_target_room(&self, letter: &char) -> Point {
    for row in 0..self.room_depth {
      let possible_move= Point::new(5 - row, TARGET_ROOM_BY_LETTER[letter]);
      if self.amphipods.contains_key(&possible_move) {
        continue
      }
      return possible_move;
    }
    panic!("Should have found one empty cell in target room");
  }

  fn possible_moves(&self, letter: &char, point: &Point) -> Vec<Point> {
    let mut ret: Vec<Point> = Vec::new();
    if !self.can_move(letter, point){
     return ret;   
    }

    if Self::is_in_hallway(point) {
      // can move to deepest cell in target room
      if !self.can_move_to_target_room(letter, point) {
        panic!("Should be able to go to target room");
      } 
      ret.push(self.deepest_cell_in_target_room(letter));
      return ret;
    } 

    // it's in a room
    if !self.can_move_up(point ) {
      panic!("Should be able to move up");
    }

    // it's in a room, can go to target room or to hallway
    if self.can_move_to_target_room(letter, point) {
      // can move to deepest cell
      ret.push(self.deepest_cell_in_target_room(letter));
      return ret;
    }

    // move to hallway
    // move left
    for cell in Self::left_hallway(point) {
      if self.amphipods.contains_key(&cell) {
        break;
      }
      ret.push(cell);
    }

    // move right
    for cell in Self::right_hallway(point) {
      if self.amphipods.contains_key(&cell) {
        break;
      }
      ret.push(cell);
    }

    return ret;
  }

  fn move_amphipod(&self, from: &Point, to: &Point) -> Self{
    let mut new_amphipods = self.amphipods.clone();
    let letter = self.amphipods.get(from).unwrap().clone();
    new_amphipods.remove(from);
    new_amphipods.insert(to.clone(), letter);
    return Self {amphipods: new_amphipods, original: self.original.clone(), room_depth: self.room_depth};
  }

  fn is_space_point(&self, point: &Point) -> bool {
    point.row > 2 && (point.col < 2 || point.col > BOARD_WIDTH - 3)
  }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // The `f` value implements the `Write` trait, which is what the
      // write! macro is expecting. Note that this formatting ignores the
      // various flags provided to format strings.
      let mut s = "".to_owned();
      for row in 0..BOARD_HEIGHT_2 {
        for col in 0..BOARD_WIDTH {
          let point = Point::new(row,col);
          if WALLS_2.contains(&point) {
            s.push('#');
            continue;
          }
          if self.is_space_point(&point) {
            s.push(' ');
            continue;
          }
          let maybe_letter = self.amphipods.get(&point);
          if maybe_letter.is_none() {
            s.push('.');
            continue;
          }
          s.push(maybe_letter.unwrap().clone());
        }
        s.push('\n');
      }
      write!(f, "{}", s)
  }
}

#[derive(Debug,Clone)]
struct Solution {
  original_board: Board,
  steps: Vec<Step>,
  cost: i64,
}

impl Solution {
  fn print(&self) {
    let mut board = self.original_board.clone();
    println!("{}", board);
    for step in self.steps.iter() {
      board = board.move_amphipod(&step.from, &step.to);
      println!("{}", board);
    }
    println!("💸 Cost: {}", self.cost);
  }
}

fn solve(original_board: &Board, 
  current_board: &Board, 
  current_cost: i64, 
  current_min: i64, 
  current_steps: &Vec<Step>, 
  solutions: &Vec<Solution>) -> (Vec<Solution>, i64) {
    let mut new_solutions = solutions.clone();
    let mut new_min = current_min.clone();

    if !current_board.amphipods.iter().any(|(point, letter)|{
      current_board.possible_moves(letter, point).len() > 0
    }) {
      // println!("🚫");
      return (new_solutions, new_min);
    }

    // move while possible moves
    for (point, letter) in &current_board.amphipods {
      for destination in current_board.possible_moves(&letter, &point) {
        let step = Step{
          from: point.clone(),
          to: destination.clone(),
          letter: letter.clone()
        };
        let mut new_steps = current_steps.clone();
        new_steps.push(step.clone()); 
        let new_cost = step.cost() + current_cost;
        if new_cost > new_min {
          // println!("💸 current min {}", current_min);
          continue;
        }
        let new_board = current_board.move_amphipod(&step.from, &step.to);
        if new_board.amphipods == TARGET_BY_DEPTH[&original_board.room_depth].clone() {
          let solution = Solution{
            original_board: original_board.clone(),
            steps: new_steps.clone(),
            cost: new_cost
          };
          new_solutions.push(solution.clone());
          if new_cost < new_min {
            new_min = new_cost.clone();
            println!("✨ {}", new_min);
            solution.print();
          }
          continue;
        }

        let ret = solve(original_board,
          &new_board, 
          new_cost, 
          new_min, 
          &new_steps, 
          &new_solutions);
        
          new_solutions = ret.0;
          new_min = ret.1;
      }
    }
    return (new_solutions, new_min);
}

fn solution_1(data: &Vec<String>) -> i64 {
  let board = Board::parse(&data);
  let ret = solve(&board, 
    &board, 
    0, 
    i64::MAX, 
    &Vec::new(),
  &Vec::new());

    return ret.1;
}

fn solution_2(data: &Vec<String>) -> i64 {
  let mut data = data.clone();
  data.insert(3, "  #D#C#B#A#".to_string());
  data.insert(4, "  #D#B#A#C#".to_string());
  let board = Board::parse(&data);
  let ret = solve(&board, 
    &board, 
    0, 
    i64::MAX, 
    &Vec::new(),
  &Vec::new());

    return ret.1;
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day23".to_string();
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

  println!(r#"{:?} Part 2 test result is {:?}"#, emoji, solution_2(&data));
}