use std::fs::File;
use std::io::{self, BufRead};

fn solution_1(entries: &[i32]) -> i32 {
    let mut previous = entries[0];
    let mut counter = 0;
    for num in entries.iter() {
        if *num > previous {
          counter += 1;
        }
        previous = *num
    }
    return counter;
}

fn solution_2(entries: &[i32]) -> i32 {
  let mut previous_sum: i32 = entries[0..3].iter().sum();
  let mut counter = 0;
  for index in 4..entries.len() + 1  {
      let sum: i32 = entries[index-3..index].iter().sum();
      if sum > previous_sum {
        counter += 1;
      }
      previous_sum = sum
  }
  return counter;
}


fn main() {
    let file = File::open("inputs/day01.txt").unwrap();
    let numbers: Vec<i32> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    println!(r#"🎉 Part 1 result is {:?}"#, solution_1(&numbers));

    println!(r#"🎉 Part 2 result is {:?}"#, solution_2(&numbers));
}