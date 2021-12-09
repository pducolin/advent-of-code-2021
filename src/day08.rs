use core::num;
use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

Map segments to a vector of 7 boolean

 aaaa           0000
b    c        1      2
b    c        1      2
 dddd   ---->   3333
e    f        4      5
e    f        4      5
 gggg           6666 

Each digit has some segment on:
0: 0,1,2,4,5,6
1: 2,5
2: 0,2,3,4,6
3: 0,2,3,5,6
4: 1,2,3,5
5: 0,1,3,5,6
6: 0,1,3,4,5,6
7: 0,2,5
8: 0,1,2,3,4,5,6
9: 0,1,2,3,5,6

Given all digits =>
We can find 1,4,7,8 as they have an unique segments.len()

digits[1] = digits.find(digit => digit.len() == 2)
digits[4] = digits.find(digit => digit.len() = 4)
digits[7] = digits.find(digit => digit.len() = 3)
digits[8] = digits.find(digit => digit.len() = 7)

Then we can find:
segment 0 = digits[7].segments - digits[1].segments

We have:
1: 2,5
4: 1,2,3,5
7: 0,2,5
8: 0,1,2,3,4,5,6

We need to find:
2: 0,2,3,4,6
3: 0,2,3,5,6
5: 0,1,3,5,6

0: 0,1,2,4,5,6
6: 0,1,3,4,5,6
9: 0,1,2,3,5,6

digit 6 has 6 segments and includes only one of the two segments from digits[1]
digits[6] = digits.find(digit => (digits.segments.len() == 6 && digits[1] - digit.segments).len() == 1 )
This also get us both segments 2 and 5, among with segment 0

segments 3 are in all 2,3,5,6,9 digits, but not in 0

any of digits (2,3,5) - digit 0 gives segment 3

segments (0,2,3,5) are one segment away (6) from digit 3 

segments (0,2,3,5,6) are one segment away (1) from digit 9

we get digit 0 by exclusion

we get digit 5 given segment (0,1,3,5,6)

we get digit 2 by exclusion
*/

#[derive(Clone)]
struct Digit {
  value: i32,
  segments: HashSet<char>,
}

struct Line {
  digits: HashMap<String, Digit>,
  display: Vec<String>
}

fn find_digits(digit_data: &Vec<String>) -> HashMap<String, Digit> {
  let known_len_by_value: HashMap<i32, usize> = HashMap::from([
    (1, 2),
    (4, 4),
    (7, 3),
    (8, 7),
  ]);

  let mut digits_by_segment: HashMap<String, Digit> = HashMap::new();
  let mut digits_by_value: HashMap<i32, Digit> = HashMap::new();

  let mut digits: Vec<String> = digit_data.clone();

  // find 1, 4, 7, 8
  for (val, len) in known_len_by_value{
    let index = digits.iter().position(|d| d.len() == len).unwrap();
    let s = &digits[index].clone();
    let segments: HashSet<char> = s.chars().collect();
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a,b| b.cmp(a));
    digits_by_segment.insert(chars.into_iter().collect(), Digit{value: val, segments: segments});
    let segments: HashSet<char> = s.chars().collect();
    digits_by_value.insert(val, Digit{value: val, segments: segments});
    digits.remove(index);
  }

  // find 6
  {
    let index = digits.iter().position(|d | {
      if d.len() != 6 {
        return false;
      }
      let d_set: HashSet<char> = d.chars().collect();
      let delta: HashSet<char> = digits_by_value[&1].segments.difference(&d_set).map(|c|c.clone()).collect();
      return delta.len() == 1;
    }).unwrap();
    let six: &String = &digits[index].clone();
    let segments: HashSet<char> = six.chars().collect();
    let mut chars: Vec<char> = six.chars().collect();
    chars.sort_by(|a,b| b.cmp(a));
    digits_by_segment.insert(chars.into_iter().collect(), Digit{value: 6, segments: segments});
    let segments: HashSet<char> = six.clone().chars().collect();
    digits_by_value.insert(6, Digit{value: 6, segments: segments});
    digits.remove(index);
  }

  // find 5 
  {
    let index = digits.iter().position(|d | {
      if d.len() != 5 {
        return false;
      }
      let d_set: HashSet<char> = d.chars().collect();
      let delta: HashSet<char> = digits_by_value[&6].segments.difference(&d_set).map(|c|c.clone()).collect();
      return delta.len() == 1;
    }).unwrap();
    let five: &String = &digits[index].clone();
    let mut chars: Vec<char> = five.chars().collect();
    chars.sort_by(|a,b| b.cmp(a));
    let segments: HashSet<char> = five.clone().chars().collect();
    digits_by_segment.insert(chars.into_iter().collect(), Digit{value: 5, segments: segments});
    let segments: HashSet<char> = five.clone().chars().collect();
    digits_by_value.insert(5, Digit{value: 5, segments: segments});
    digits.remove(index);
  }

  // find 3
  {  
    let index = digits.iter().position(|d | {
      if d.len() != 5 {
        return false;
      }
      let d_set: HashSet<char> = d.chars().collect();
      let delta: HashSet<char> = digits_by_value[&5].segments.difference(&d_set).map(|c|c.clone()).collect();
      return delta.len() == 1;
    }).unwrap();
    let three: &String = &digits[index].clone();
    let mut chars: Vec<char> = three.chars().collect();
    chars.sort_by(|a,b| b.cmp(a));
    let segments: HashSet<char> = three.chars().collect();
    digits_by_segment.insert(chars.into_iter().collect(),  Digit{value: 3, segments: segments});
    let segments: HashSet<char> = three.chars().collect();
    digits_by_value.insert(3,  Digit{value: 3, segments: segments});
    digits.remove(index);
  }

  // find 2
  {
    let index = digits.iter().position(|d | d.len() == 5).unwrap();
    let two: &String = &digits[index].clone();
    let mut chars: Vec<char> = two.chars().collect();
    chars.sort_by(|a,b| b.cmp(a));
    let segments: HashSet<char> = two.chars().collect();
    digits_by_segment.insert(chars.into_iter().collect(), Digit{value: 2, segments: segments});
    let segments: HashSet<char> = two.chars().collect();
    digits_by_value.insert(2, Digit{value: 2, segments: segments});
    digits.remove(index);
  }

  // find 9
  {
    let index = digits.iter().position(|d | {
      if d.len() != 6 {
        return false;
      }
      let d_set: HashSet<char> = d.chars().collect();
      let delta: HashSet<char> = digits_by_value[&5].segments.difference(&d_set).map(|c|c.clone()).collect();
      return delta.len() == 0;
    }).unwrap();
    let nine: &String = &digits[index].clone();
    let mut chars: Vec<char> = nine.chars().collect();
    chars.sort_by(|a,b| b.cmp(a));
    let segments: HashSet<char> = nine.chars().collect();
    digits_by_value.insert(9, Digit{value: 9, segments: segments});
    let segments: HashSet<char> = nine.chars().collect();
    digits_by_segment.insert(chars.into_iter().collect(), Digit{value: 9, segments: segments});
    digits.remove(index);
  }

  // find 0
  {
    let index = 0;
    let zero: &String = &digits[0].clone();
    let mut chars: Vec<char> = zero.chars().collect();
    chars.sort_by(|a,b| b.cmp(a));
    let segments: HashSet<char> = zero.chars().collect();
    digits_by_value.insert(0, Digit{value: 0, segments: segments});
    let segments: HashSet<char> = zero.chars().collect();
    digits_by_segment.insert(chars.into_iter().collect(), Digit{value: 0, segments: segments});
    digits.remove(index);
  }

  return digits_by_segment;
}

fn parse_line(data: &String) -> Line {
  let data_splits: Vec<&str> = data.split("|").collect();
  let digits_data: Vec<String> = data_splits[0].split_whitespace().map(|s|String::from(s)).collect();
  
  let display: Vec<String> = data_splits[1].split_whitespace().map(|s|String::from(s)).collect();
  return Line { digits: find_digits(&digits_data), display: display }
}

fn solution_1(data: &Vec<String>) -> i32 {
  let mut counter = 0;

  let unique_length: HashSet<&i32> = [2,3,4,7].iter().clone().collect();

  for d in data {
    let line: Line = parse_line(d);

    for d in line.display {
      if unique_length.contains(&(d.len() as i32)) {
        counter += 1;
      }
    }
  }

  return counter;
}


fn solution_2(data: &Vec<String>) -> i32 {
  let mut counter = 0;

  for d in data {
    let mut number = 0;
    let line: Line = parse_line(d);

    for d in line.display {
      let mut chars: Vec<char> = d.chars().collect();
      chars.sort_by(|a,b| b.cmp(a));
      let s: String = chars.into_iter().collect();
      number = line.digits.get(&s).unwrap().value + number * 10; 
    }

    counter += number;
  }

  return counter;
}

fn main() {
  let test = false;
  if test {
    let file = File::open("inputs/day08.test.txt").unwrap();
    let data: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line|line.unwrap())
                .collect();

    println!(r#"🧪 Part 1 test result is {:?}"#, solution_1(&data));

    println!(r#"🧪 Part 2 test result is {:?}"#, solution_2(&data));
  } else {

    let file = File::open("inputs/day08.txt").unwrap();
    let data: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line|line.unwrap())
                .collect();

    println!(r#"🎉 Part 1 result is {:?}"#, solution_1(&data));

    println!(r#"🎉 Part 2 result is {:?}"#, solution_2(&data));
  }
}