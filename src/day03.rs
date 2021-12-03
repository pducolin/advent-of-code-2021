use std::collections::{HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn binary_string_to_decimal(binary_string: &String) -> u32 {
  return u32::from_str_radix(binary_string, 2).unwrap()
}

fn gamma_rate(binary_strings: &Vec<String>) -> String {
  let word_length = binary_strings.first().unwrap().len();
    let mut zeros: Vec<u32> = vec![0; word_length];
    let mut ones: Vec<u32> =vec![0; word_length];

    for word in binary_strings {
      let chars: Vec<char> = word.chars().collect();
      for index in 0..word_length {
        let c: char = chars[index];
        match c {
          '0'=> zeros[index] += 1,
          '1' => ones[index] += 1,
          _ => panic!(),
        }
      }
    }  

    let mut most_common_bits: String = String::from("");
    for index in 0..word_length {
      if ones[index] >= zeros[index] {
        most_common_bits += "1";
      } else {
        most_common_bits += "0";
      }
    }

    return most_common_bits;
}

fn binary_not(s: &String) -> String {
  let mut ret = String::from("");
  for c in s.chars() {
    match c {
      '0'=> ret += "1",
      '1' => ret += "0",
      _ => panic!(),
    }
  }
  return ret;
}

fn solution_1(diagnostics: &Vec<String>) -> u32 {
    let word_length = diagnostics.first().unwrap().len();
    let mut zeros: Vec<u32> = vec![0; word_length];
    let mut ones: Vec<u32> =vec![0; word_length];

    for word in diagnostics {
      let chars: Vec<char> = word.chars().collect();
      for index in 0..word_length {
        let c: char = chars[index];
        match c {
          '0'=> zeros[index] += 1,
          '1' => ones[index] += 1,
          _ => panic!(),
        }
      }
    }  
    
    let gamma_rate = gamma_rate(diagnostics);
    let epsilon_rate: String = binary_not(&gamma_rate);

    return binary_string_to_decimal(&gamma_rate) * binary_string_to_decimal(&epsilon_rate);
}


struct DiagnosticByBit {
  ones: HashSet<String>,
  zeros: HashSet<String>,
}

struct BitCount {
  ones: u32,
  zeros: u32,
}

fn build_diagnostic_by_bits(diagnostics: &Vec<String>) -> Vec<DiagnosticByBit> {
  let word_length = diagnostics.first().unwrap().len();
  let mut diagnostics_by_bits: Vec<DiagnosticByBit> = Vec::new();
  for _ in 0..word_length {
    diagnostics_by_bits.push(DiagnosticByBit {ones:HashSet::new(),zeros: HashSet::new()});
  }

  for word in diagnostics {
    let bits: Vec<char> = word.chars().collect();
    for index in 0..word_length {
      let b: char = bits[index];
      match b {
        '0'=> {
          diagnostics_by_bits[index].zeros.insert(word.to_string());
        },
        '1' => {
          diagnostics_by_bits[index].ones.insert(word.to_string());
        },
        _ => panic!(),
      };
    }
  }
  return diagnostics_by_bits;
}

fn count_bits_at_index(set: &HashSet<String>, index: usize) -> BitCount {
  let mut bit_count = BitCount{ones:0, zeros:0};
  for word in set.iter() {
    let bits: Vec<char> = word.chars().collect();
    let bit: char = bits[index];
    match bit {
      '0'=> {
        bit_count.zeros += 1;
      },
      '1' => {
        bit_count.ones += 1;
      },
      _ => panic!(),
    }
  }
  return bit_count;
}

fn solution_2(diagnostics: &Vec<String>) -> u32 {
  // build a dictionary of diagnostics by bits
  let diagnostics_by_bits: Vec<DiagnosticByBit> = build_diagnostic_by_bits(diagnostics);

  let mut oxygen_set :HashSet<String> = HashSet::from_iter(diagnostics.iter().map(|d|d.to_string()));
  let mut co2_set :HashSet<String> = HashSet::from_iter(diagnostics.iter().map(|d|d.to_string()));
  // oxygen
  let mut index = 0;
  while oxygen_set.len() > 1 {
    let bit_count = count_bits_at_index(&oxygen_set, index);
    if bit_count.zeros > bit_count.ones {
      oxygen_set = oxygen_set.intersection(&diagnostics_by_bits[index].zeros).map(|d|d.to_string()).collect();
    } else {
      oxygen_set = oxygen_set.intersection(&diagnostics_by_bits[index].ones).map(|d|d.to_string()).collect();
    }
    index += 1;
  }
  
  // co2
  index = 0;
  while co2_set.len() > 1 {
    let bit_count = count_bits_at_index(&co2_set, index);
    if bit_count.zeros > bit_count.ones {
      co2_set = co2_set.intersection(&diagnostics_by_bits[index].ones).map(|d|d.to_string()).collect();
    } else {
      co2_set = co2_set.intersection(&diagnostics_by_bits[index].zeros).map(|d|d.to_string()).collect();
    }
    index += 1;
  }

  let oxygen = binary_string_to_decimal( oxygen_set.iter().next().unwrap());
  let co2 = binary_string_to_decimal(co2_set.iter().next().unwrap());

  return oxygen * co2;
}

fn main() {
    let file = File::open("inputs/day03.txt").unwrap();
    let diagnostics: Vec<String> = io::BufReader::new(file)
                .lines()
                .map(|line|line.unwrap())
                .collect();

    println!(r#"🎉 Part 1 result is {:?}"#, solution_1(&diagnostics));

    println!(r#"🎉 Part 2 result is {:?}"#, solution_2(&diagnostics));
}