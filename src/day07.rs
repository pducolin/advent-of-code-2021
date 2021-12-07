use std::fs::File;
use std::io::{self, BufRead};

fn evaluate_cost_constant(crabs: &Vec<i32>, pos: &i32) -> i32 {
  let mut cost = 0;

  for crab in crabs {
    cost += (crab - pos).abs();
  }

  return cost;
}


fn solution_1(data: &Vec<String>) -> i32 {
  let mut crabs: Vec<i32> = data.get(0)
    .unwrap()
    .split(",")
    .map(|n|n.parse().unwrap())
    .collect();

  crabs.sort();

  let min_crab = crabs.get(0).unwrap();
  let max_crab = crabs.get(crabs.len() -1).unwrap();
  let mut min_cost = i32::MAX;

  for pos in *min_crab .. *max_crab {
    let cost = evaluate_cost_constant(&crabs, &pos);
    if cost < min_cost {
      min_cost = cost;
    }
  }
  return min_cost;
}


fn evaluate_cost_increasing(crabs: &Vec<i32>, pos: &i32) -> i32 {
  let mut cost = 0;

  for crab in crabs {
    let distance = (crab - pos).abs();  
    cost += (1 .. distance + 1).fold(0, |a,b|a+b);
  }

  return cost;
}

fn solution_2(data: &Vec<String>) -> i32 {
  let mut crabs: Vec<i32> = data.get(0)
    .unwrap()
    .split(",")
    .map(|n|n.parse().unwrap())
    .collect();

  crabs.sort();

  let min_crab = crabs.get(0).unwrap();
  let max_crab = crabs.get(crabs.len() -1).unwrap();
  let mut min_cost = i32::MAX;

  for pos in *min_crab .. *max_crab {
    let cost = evaluate_cost_increasing(&crabs, &pos);
    if cost < min_cost {
      min_cost = cost;
    }
  }
  return min_cost;
}

fn main() {
  let test = false;
  if test {
    let file = File::open("inputs/day07.test.txt").unwrap();
    let data: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line|line.unwrap())
                .collect();

    println!(r#"🧪 Part 1 test result is {:?}"#, solution_1(&data));

    println!(r#"🧪 Part 2 test result is {:?}"#, solution_2(&data));
  } else {

    let file = File::open("inputs/day07.txt").unwrap();
    let data: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line|line.unwrap())
                .collect();

    println!(r#"🎉 Part 1 result is {:?}"#, solution_1(&data));

    println!(r#"🎉 Part 2 result is {:?}"#, solution_2(&data));
  }
}