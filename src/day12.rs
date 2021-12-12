use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;


#[derive(Clone)]
struct Node {
  value: String,
  neighbours: Vec<Rc<RefCell<Node>>>,
  visited: bool,
}

impl Node {
  fn is_big(&self) -> bool {
    return self.value.chars().next().unwrap().is_uppercase();
  }

  fn can_visit(&self) -> bool {
    return self.is_big() || !self.visited;
  }
}

struct CaveMap {
  neighbours_by_value: HashMap<String, Vec<String>>,
}

impl CaveMap {
  fn init() -> Self {
    Self {
      neighbours_by_value: HashMap::new(),
    }
  } 

  fn parse(&mut self, data: &Vec<String>) {
    for line in data {
      let splits: Vec<&str> = line.split("-").collect();
      let node_a = self.neighbours_by_value
        .entry(splits[0].to_string())
        .or_insert(Vec::new());
      node_a.push(splits[1].to_string());
      let node_b = self.neighbours_by_value
        .entry(splits[1].to_string())
        .or_insert(Vec::new());
      node_b.push(splits[0].to_string());
    }
  }

  fn visit(&mut self, 
    node: &String, 
    path: &Vec<String>, 
    visited: &HashMap<String, i32>, 
    max_visits: i32) -> i32 {
    let mut counter = 0;
    for i in 0 .. self.neighbours_by_value.get(node).unwrap().len() {
      let n = self.neighbours_by_value.get(node).unwrap().get(i).unwrap().clone();

      // skip "start" neighbour
      if n == "start" {
        continue;
      }
      if n == "end" {
        counter += 1;
        continue;
      }
      let mut current_path: Vec<String> = path.to_vec();
      current_path.push(n.clone());
      let mut current_visited: HashMap<String, i32> = visited.clone();
      let mut current_max = max_visits.clone();
      // skip visited nodes 
      if n.chars().next().unwrap().is_lowercase()  {
        // check if we visited this small cave already max times
        if !current_visited.contains_key(&n) {
          current_visited.insert(n.clone(), 1);
        } else {
          let current_count = current_visited.get_mut(&n).unwrap();
          if *current_count >= current_max {
            continue;
          }
          *current_count += 1;
          current_max = 1;
        }
      }
      counter += self.visit(&n, &current_path, &current_visited, current_max); 
    }

    return counter;
  }
}

fn solution_1(data: &Vec<String>) -> i32 {
  // println!(r#"Map: {:?}"#, nodes_by_value);
  let mut cave_map = CaveMap::init();
  cave_map.parse(data);
  let mut path: Vec<String> = Vec::new();
  path.push("start".to_string());
  return cave_map.visit(&"start".to_string(), &path, &HashMap::new(), 1);
}

fn solution_2(data: &Vec<String>) -> i32 {
  let mut cave_map = CaveMap::init();
  cave_map.parse(data);
  let mut path: Vec<String> = Vec::new();
  path.push("start".to_string());
  return cave_map.visit(&"start".to_string(), &path, &HashMap::new(), 2);
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day12".to_string();
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