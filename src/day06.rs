use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap};

fn iterate_days(fishes_map: &HashMap<i32,i64>, days: i32) -> HashMap<i32,i64> {
  let mut ret: HashMap<i32,i64> = HashMap::new();
  for (fish,fish_count) in fishes_map.iter() {
    let mut updated_fish = fish - days;
    if updated_fish < 0 {
      // create a new fish with a counter of 8 - the updated_fish + 1 = 9 + updated_fish
      // example:
      // if new fish days is -1, created fish with counter of 8 = 9 + updated_fish
      *ret.entry(9 + updated_fish).or_insert(0) += fish_count;
      updated_fish += days;
    }
    *ret.entry(updated_fish).or_insert(0) += fish_count;
  }
  return ret;
}

fn solve (data: &Vec<String>, days: i32) -> i64 {
  let fishes: Vec<i32> = data.get(0)
    .unwrap()
    .split(",")
    .map(|n|n.parse().unwrap())
    .collect();

  let mut fishes_map: HashMap<i32,i64> = HashMap::new();

  for f in fishes {
    *fishes_map.entry(f).or_insert(0) += 1;
  }

  let weeks = days / 7;

  // iterate on 7 days cycles at once for weeks
  for _ in 0 .. weeks {
    fishes_map.clone_from(&iterate_days(&fishes_map, 7));
  }

  // iterate on 7 days cycles at once for weeks
  let remaining_days = days % 7;
  fishes_map.clone_from(&iterate_days(&fishes_map, remaining_days));

  return fishes_map.values().sum();
}


fn solution_1(data: &Vec<String>) -> i64 {
  solve(data, 80)
}

fn solution_2(data: &Vec<String>) -> i64 {
  solve(data, 256)
}

fn main() {
  let test = false;
  if test {
    let file = File::open("inputs/day06.test.txt").unwrap();
    let data: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line|line.unwrap())
                .collect();

    println!(r#"🧪 Part 1 test result is {:?}"#, solution_1(&data));

    println!(r#"🧪 Part 2 test result is {:?}"#, solution_2(&data));
  } else {

    let file = File::open("inputs/day06.txt").unwrap();
    let data: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line|line.unwrap())
                .collect();

    println!(r#"🎉 Part 1 result is {:?}"#, solution_1(&data));

    println!(r#"🎉 Part 2 result is {:?}"#, solution_2(&data));
  }
}