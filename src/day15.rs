use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

/*
function Dijkstra(Graph, source):

    create vertex set Q

    for each vertex v in Graph:            
        dist[v] ← INFINITY                 
        prev[v] ← UNDEFINED                
        add v to Q                     
    dist[source] ← 0                       
   
    while Q is not empty:
        u ← vertex in Q with min dist[u]   
                                           
        remove u from Q
       
        for each neighbor v of u still in Q:
            alt ← dist[u] + length(u, v)
            if alt < dist[v]:              
                dist[v] ← alt
                prev[v] ← u

    return dist[], prev[]
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
  row: usize,
  col: usize,
}

#[derive(Clone, Debug)]
struct Size {
  tot_cols: usize,
  tot_rows: usize,
}

#[derive(Clone, Debug)]
struct ChitonMatrix {
  chitons_by_point: HashMap<Point, u64>,
  size: Size,
}

impl ChitonMatrix {
  fn new() -> Self {
    Self {
      chitons_by_point: HashMap::new(),
      size: Size {
        tot_cols: 0,
        tot_rows: 0,
      }
    }
  }

  fn parse(data: &Vec<String>) -> Self {
    let mut chitons_by_point: HashMap<Point, u64> = HashMap::new();
    
    // parse matrix
    let tot_rows = data.len();
    let tot_cols = data[0].len();
    for row in 0..tot_rows {
      let numbers = string_to_numbers(&data[row]);
      for col in 0..tot_cols{
        let point = Point{row, col};
        chitons_by_point.insert(point.clone(), numbers[col]);
      }
    }

    let previous_by_point: HashMap<Point, Point> = HashMap::new();
    return Self {
      chitons_by_point,
      size: Size { tot_cols, tot_rows }
    };
  }
}

#[derive(Clone, Debug)]
struct DijkstraMatrix {
  chitons_by_point: HashMap<Point, u64>,
  risk_from_source_by_point: HashMap<Point, u64>,
  previous_by_point: HashMap<Point, Point>,
  points_matrix: Vec<Vec<Point>>,
}

impl DijkstraMatrix {
  fn init(chiton_matrix: &ChitonMatrix) -> Self {
    let chitons_by_point: HashMap<Point, u64> = chiton_matrix.chitons_by_point.clone();
    let mut risk_from_source_by_point: HashMap<Point, u64> = HashMap::new();
    let mut points_matrix: Vec<Vec<Point>> = Vec::new();
    // parse matrix
    let tot_rows = chiton_matrix.size.tot_rows;
    let tot_cols = chiton_matrix.size.tot_cols;
    for row in 0..tot_rows {
      let mut points_row: Vec<Point> = Vec::new();
      for col in 0..tot_cols{
        let point = Point{row, col};
        points_row.push(point.clone());
        risk_from_source_by_point.insert(point.clone(), u64::MAX);
      }
      points_matrix.push(points_row);
    }

    let previous_by_point: HashMap<Point, Point> = HashMap::new();
    return Self {
      chitons_by_point,
      points_matrix,
      risk_from_source_by_point,
      previous_by_point,
    }
  }

  fn print(&self) {
    println!("====== Chitons ========");
    for row in self.points_matrix.clone() {
      let mut s = "".to_owned();
      for p in row {
        s += self.chitons_by_point.get(&p).unwrap().to_string().as_str();
      }
      println!("{}",s);
    }
  }
}

fn string_to_numbers(s: &String) -> Vec<u64> {
  return s.chars().map(|x|x.to_digit(10).unwrap() as u64).collect();
}

fn get_neighbours(point: &Point, size: &Size) -> Vec<Point> {
  let mut neighbours: Vec<Point> = Vec::new();

  // left
  if point.col > 0 {
    let neighbour = Point{row: point.row, col: point.col - 1};
    neighbours.push(neighbour);
  }

  // top
  if point.row > 0 {
    let neighbour = Point{row: point.row - 1, col: point.col};
    neighbours.push(neighbour);
  }

  // right
  if point.col < size.tot_cols - 1 {
    let neighbour = Point{row: point.row, col: point.col + 1};
    neighbours.push(neighbour);
  }

  // bottom
  if point.row < size.tot_rows - 1 {
    let neighbour = Point{row: point.row + 1, col: point.col};
    neighbours.push(neighbour);
  }

  return neighbours;
}
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    point: Point,
    cost: u64,
}
 
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
 
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



fn dijkstra_shortest_path(origin_matrix: &DijkstraMatrix, source: &Point, target: &Point) -> u64 {
  let mut matrix = origin_matrix.clone();
  let mut heap: BinaryHeap<State> = BinaryHeap::new();
  let mut visited: HashSet<Point> = HashSet::new();
  let points_matrix = matrix.points_matrix.clone();
 
  // initially we now source to source distance cost is 0
  matrix.risk_from_source_by_point.insert(source.clone(), 0);
  heap.push(State{
    point: source.clone(),
    cost: 0
  });

  while let Some(State { point, cost }) = heap.pop() {
    // find cheapest point from source
    if point == *target {
      break;
    }

    visited.insert(point.clone());

    let size: Size = Size { 
      tot_cols: points_matrix.first().unwrap().len(), 
      tot_rows: points_matrix.len(), 
    };

    if cost > *matrix.risk_from_source_by_point.get(&point).unwrap() {
      continue;
    }

    let neighbours: Vec<Point> = get_neighbours(&point.clone(), &size); 
    for n in neighbours {
      if visited.contains(&n) {
        // we already have a shortest path to this point
        // skip it
        continue;
      }

      let next = State {
        point: n.clone(),
        cost: cost + matrix.chitons_by_point.get(&n).unwrap(),
      };
      if next.cost < *matrix.risk_from_source_by_point.get(&next.point).unwrap() {
          matrix.risk_from_source_by_point.insert(n.clone(), next.cost);
          matrix.previous_by_point.insert(n.clone(), point.clone());
          heap.push(next);
      }
    }
  }

  return matrix.risk_from_source_by_point.get(target).unwrap().clone();
}

fn solution_1(data: &Vec<String>) -> u64 {
  let chiton_matrix = ChitonMatrix::parse(data);
  let matrix = DijkstraMatrix::init(&chiton_matrix);

  let source = Point{row:0, col:0};
  let target: Point = matrix.points_matrix.last().unwrap().last().unwrap().clone();

  return dijkstra_shortest_path(&matrix, &source, &target);
}

fn solution_2(data: &Vec<String>) -> u64 {
  let sub_matrix = ChitonMatrix::parse(data);

  let mut matrix: ChitonMatrix = ChitonMatrix::new();
  matrix.size.tot_cols = 5 * sub_matrix.size.tot_cols;
  matrix.size.tot_rows = 5 * sub_matrix.size.tot_rows;

  for row in 0 .. matrix.size.tot_rows  {
    for col in 0 .. matrix.size.tot_cols {
      let point = Point {row, col};

      if sub_matrix.chitons_by_point.contains_key(&point) {
        matrix.chitons_by_point.insert(point, sub_matrix.chitons_by_point.get(&point).unwrap().clone());
        continue;
      }     
      // find matching point in sub_matrix
      let sub_row = row % sub_matrix.size.tot_rows;
      let increment = row / sub_matrix.size.tot_rows;

      let sub_col = col % sub_matrix.size.tot_cols;
      let increment = increment +  col / sub_matrix.size.tot_cols;

      let mut value = sub_matrix.chitons_by_point.get(&Point{row: sub_row, col: sub_col}).unwrap().clone();
      value += increment as u64;
      while value > 9 {
        value -= 9
      }

      matrix.chitons_by_point.insert(point, value);
    }
  }

  let dijkstra_matrix = DijkstraMatrix::init(&matrix);

  let source = Point{row:0, col:0};
  let target: Point = Point{row: matrix.size.tot_rows - 1, col: matrix.size.tot_cols - 1};

  return dijkstra_shortest_path(&dijkstra_matrix, &source, &target);
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day15".to_string();
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
