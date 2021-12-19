use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Clone)]
struct Pair {
  left: u64,
  right: u64,
}

impl Pair {
  fn from_str(s: &String) -> Self {
    let pair_numbers: Vec<u64> = s
    .replace("[", "")
    .replace("]", "")
    .split(",")
    .map(|x|u64::from_str_radix(x,10).unwrap())
    .collect();
    return Self {
      left: pair_numbers[0],
      right: pair_numbers[1],
    };
  }
}

fn magnitude(pair: &Pair) -> u64 {
  return 3 * pair.left + 2 * pair.right;
}

fn replace_magnitude(expression_before: &String) -> String {
  let pair_re_str = r"\[\d+,\d+\]";
  let re = Regex::new(&pair_re_str).unwrap();
  let mat = re.find(&expression_before).unwrap();
  let mag = magnitude(&Pair::from_str(&mat.as_str().to_string()));
  return expression_before.replacen(mat.as_str(), &mag.to_string(), 1);
}

fn evaluate_expression_magnitude(expression: &String) -> u64 {
  let mut aux_expression = expression.clone();
  loop {
    let res = u64::from_str_radix(&aux_expression, 10);
    if res.is_ok() {
      return res.unwrap();
    }
    aux_expression = replace_magnitude(&aux_expression);
  }
}

fn sum(expression_a: &String, expression_b: &String) -> String {
  return format!("[{},{}]", expression_a, expression_b);
}

fn explode_expression(expression_before: &String) -> String {
  let mut expression = expression_before.clone();
  let re = Regex::new(r"\[\d+,\d+\]").unwrap();
  for mat in re.find_iter(&expression) {
    // count group level before this match
    let mut levels_count = 0;
    for i in 0..mat.start() {
      match &expression[i..i+1] {
        "[" => levels_count += 1,
        "]" => levels_count -= 1,
        _ => continue,
      }
    }
    if levels_count < 4 {
      continue;
    }
    // explode 
    let pair = Pair::from_str(&mat.as_str().to_string());
    let num_re = Regex::new(r"\d+").unwrap();
    let mut left_chunk = expression[0..mat.start()].to_string();
    let mut right_chunk = expression[mat.end()..].to_string();
    if num_re.is_match(&left_chunk) {
      let num_mat = num_re.find_iter(&left_chunk).last().unwrap();
      let num_str = num_mat.as_str();
      let num = u64::from_str_radix(num_str, 10).unwrap() + pair.left;
      let prefix: String = left_chunk[0..num_mat.start()].to_string();
      let suffix: String = left_chunk[num_mat.end()..].to_string();
      left_chunk = format!("{}{}{}", prefix, num, suffix); 
    }
    if num_re.is_match(&right_chunk) {
      let num_mat = num_re.find(&right_chunk).unwrap();
      let num_str = num_mat.as_str();
      let num = u64::from_str_radix(num_str, 10).unwrap() + pair.right;
      let prefix: String = right_chunk[0..num_mat.start()].to_string();
      let suffix: String = right_chunk[num_mat.end()..].to_string();
      right_chunk = format!("{}{}{}", prefix, num, suffix); 
    }
    expression = format!("{}0{}", left_chunk, right_chunk);
    break;
  }
  return expression;
}

fn split_expression(expression_before: &String) -> String {
  let mut expression = expression_before.clone();
  let re = Regex::new(r"\d{2,}").unwrap();
  if re.is_match(&expression) {
    // split
    let mat = re.find(&expression).unwrap();
    let prefix = expression[0..mat.start()].to_string();
    let suffix = expression[mat.end()..].to_string();
    let number = u64::from_str_radix(&mat.as_str(), 10).unwrap();
    let left = number / 2;
    let right = left + number % 2;
    let split = format!("[{},{}]", left, right);
    expression = format!("{}{}{}", prefix, split, suffix);
  }
  return expression;
}

fn reduce_expression(expression_before: &String) -> String {
  let mut expression = expression_before.clone();
  loop {
    // find explodes
    let expression_after_explodes = explode_expression(&expression);
    if expression_after_explodes != expression {
      expression = expression_after_explodes;
      continue;
    }
    // find splits
    let expression_after_splits = split_expression(&expression_after_explodes);
    if expression_after_splits == expression {
      break;
    }
    expression = expression_after_splits;
  }
  return expression;
}

fn solution_1(data: &Vec<String>) -> u64 {
  let mut data_iter = data.iter();
  let mut expression = reduce_expression(&data_iter.next().unwrap().to_string());
  for l in data_iter {
    if l.starts_with("#") {
      continue;
    }
    let to_add = reduce_expression(l);
    expression = sum(&expression, &to_add);
    expression = reduce_expression(&expression);
  }
  return evaluate_expression_magnitude(&expression);
}

fn solution_2(data: &Vec<String>) -> u64 {
  let mut max_mag = u64::MIN;
  let expressions: Vec<String> = data.into_iter().map(|x|reduce_expression(x)).collect();
  for i in 0..expressions.len() {
    for j in 0..expressions.len() {
      if i == j {
        continue;
      }
      println!("Evaluating {} + {}", i, j);
      let expression = sum(&expressions[i], &expressions[j]);
      let expression = reduce_expression(&expression);
      max_mag = cmp::max(evaluate_expression_magnitude(&expression), max_mag);
    }
  }
  return max_mag;
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day18".to_string();
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

